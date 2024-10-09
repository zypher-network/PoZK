use anyhow::Result;
use chrono::prelude::*;
use ethers::prelude::Signer;
use pozk_db::ReDB;
use pozk_db::{MainController, Prover, Task};
use pozk_docker::{DockerManager, RunOption};
use pozk_monitor::PoolMessage;
use pozk_utils::{remove_task_input, write_task_input, ServiceMessage};
use std::sync::Arc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct MainService {
    pool_sender: UnboundedSender<PoolMessage>,
    service_receiver: UnboundedReceiver<ServiceMessage>,
    db: Arc<ReDB>,
    docker: Arc<DockerManager>,
}

impl MainService {
    pub fn new(
        pool_sender: UnboundedSender<PoolMessage>,
        service_receiver: UnboundedReceiver<ServiceMessage>,
        db: Arc<ReDB>,
        docker: Arc<DockerManager>,
    ) -> Self {
        Self {
            pool_sender,
            service_receiver,
            db,
            docker,
        }
    }

    pub fn run(mut self) {
        tokio::spawn(async move {
            while let Some(msg) = self.service_receiver.recv().await {
                if let Err(e) = handle(&self, msg).await {
                    error!("[Service] main error: {}", e);
                }
            }
        });
    }
}

async fn handle(app: &MainService, msg: ServiceMessage) -> Result<()> {
    match msg {
        ServiceMessage::CreateTask(tid, prover, inputs, publics) => {
            // 1. check prover in local
            let key = Prover::to_key(&prover);
            if let Some(p) = app.db.get::<Prover>(key)? {
                let sid = tid.to_string();

                // 2. write data to file
                write_task_input(&sid, inputs, publics).await?;

                // 3. start docker container to run, TODO we can do more about cpu & memory
                let container = app.docker.run(&p.image, &sid, RunOption::default()).await?;

                // 4. save task to db
                let created = Utc::now().timestamp();
                let t = Task {
                    tid,
                    prover,
                    created,
                    container,
                    over: false,
                    is_me: false,
                    accepted: false,
                };

                app.db.add(&t)?;

                // 5. accept task
                app.pool_sender
                    .send(PoolMessage::AcceptTask(tid))
                    .expect("Missing pool");
            }
        }
        ServiceMessage::AcceptTask(tid, is_me) => {
            // 1. save task status in db
            let key = Task::to_key(tid);
            if let Some(mut t) = app.db.get::<Task>(&key)? {
                t.accepted = true;
                t.is_me = is_me;
                if !is_me {
                    t.over = true;
                    // 2. stop task
                    let _ = app.docker.stop(&t.container).await;
                }

                app.db.add(&t)?;
            }
        }
        ServiceMessage::UploadProof(sid, proof) => {
            tokio::spawn(upload_proof(
                app.db.clone(),
                sid,
                proof,
                app.pool_sender.clone(),
            ));
        }
        ServiceMessage::ApproveProver(prover, version, overtime) => {
            // 1. check prover in local
            let key = Prover::to_key(&prover);
            let new_tag = format!("v{}", version);

            if let Some(mut p) = app.db.remove::<Prover>(key)? {
                // 2. download new version
                let repo = format!("{:?}", p.prover);
                let image = app.docker.pull(&repo, &new_tag).await?;
                let old_image = p.image.clone();

                // 3. save new prover
                p.image = image;
                p.tag = new_tag;
                p.overtime = overtime;
                app.db.add(&p)?;

                // 4. delete old image
                app.docker.remove(&old_image).await?;
            }
        }
        ServiceMessage::PullProver(prover, tag, name, overtime) => {
            // 1. pull docker image
            let repo = format!("{:?}", prover);
            let image = app.docker.pull(&repo, &tag).await?;

            // 2. save to db
            let created = Utc::now().timestamp();
            let p = Prover {
                prover,
                tag,
                name,
                overtime,
                image,
                created,
            };
            app.db.add(&p)?;
        }
        ServiceMessage::RemoveProver(prover) => {
            let key = Prover::to_key(&prover);

            // 1. remove from db
            if let Some(p) = app.db.remove::<Prover>(key)? {
                // 2. delete docker image
                app.docker.remove(&p.image).await?;
            }
        }
        ServiceMessage::ChangeController(wallet) => {
            // 1. update controller in db
            let c = MainController {
                controller: wallet.address(),
            };
            app.db.add(&c)?;

            // 2. update pool signer
            app.pool_sender
                .send(PoolMessage::ChangeController(wallet))
                .expect("Missing pool");
        }
        ServiceMessage::MinerTest(id, prover, overtime, inputs, publics) => {
            // 1. check prover in local
            let key = Prover::to_key(&prover);
            if let Some(p) = app.db.get::<Prover>(key)? {
                let sid = format!("m-{}-{}", id, overtime);

                // 2. write data to file
                write_task_input(&sid, inputs, publics).await?;

                // 3. start docker container to run, TODO we can do more about cpu & memory
                let _container = app.docker.run(&p.image, &sid, RunOption::default()).await?;
            }
        }
    }

    Ok(())
}

async fn upload_proof(
    db: Arc<ReDB>,
    sid: String,
    proof: Vec<u8>,
    pool_sender: UnboundedSender<PoolMessage>,
) {
    // 0. check task is miner test or task by tid
    if sid.starts_with("m-") {
        // Miner test
        let s: Vec<&str> = sid.split('-').collect();
        if s.len() != 3 {
            return;
        }
        let id: u64 = s[1].parse().unwrap_or(0);
        let overtime: u64 = s[2].parse().unwrap_or(0);
        let now = Utc::now().timestamp() as u64;

        if overtime < now {
            error!("CANNOT complete the miner tests");
        } else {
            pool_sender
                .send(PoolMessage::SubmitMinerTest(id, proof))
                .expect("Missing pool");
        }

        let _ = remove_task_input(&sid).await;
        return;
    }

    let tid: u64 = sid.parse().unwrap_or(0);

    // 1. check task is me from db
    let key = Task::to_key(tid);
    let mut max_times = 100;

    loop {
        if let Ok(Some(mut t)) = db.get::<Task>(&key) {
            if t.accepted {
                if t.is_me {
                    // 2. if is_me, send tx
                    pool_sender
                        .send(PoolMessage::SubmitTask(tid, proof))
                        .expect("Missing pool");
                }
            } else {
                max_times -= 1;
                if max_times == 0 {
                    break; // over 200s
                }

                // sleep
                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                continue;
            }

            // 3. update in db
            t.over = true;
            let _ = db.add(&t);

            // 4. remove task input from disk
            let _ = remove_task_input(&sid).await;

            break;
        } else {
            break;
        }
    }
}
