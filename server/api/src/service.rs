use anyhow::{anyhow, Result};
use chrono::prelude::*;
use ethers::prelude::{Address, Signer};
use pozk_db::ReDB;
use pozk_db::{MainController, Prover, Task};
use pozk_docker::{DockerManager, RunOption};
use pozk_monitor::PoolMessage;
use pozk_utils::{
    is_valid_url, is_valid_zkvm, remove_task_input, write_task_input, write_task_proof,
    ServiceMessage,
};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::Arc;
use std::time::Duration;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    time::sleep,
};

use crate::metrics::MetricsMessage;
use crate::p2p::P2pMessage;

struct WaitingTask {
    image: String,
    prover: Address,
    inputs: Vec<u8>,
    publics: Vec<u8>,
}

pub struct MainService {
    pool_sender: UnboundedSender<PoolMessage>,
    metrics_sender: UnboundedSender<MetricsMessage>,
    p2p_sender: UnboundedSender<P2pMessage>,
    service_receiver: UnboundedReceiver<ServiceMessage>,
    db: Arc<ReDB>,
    docker: Arc<DockerManager>,
    url: String,
    check_url: bool,
    zkvm: Option<String>,
    task_parallel: usize,
    /// task from API and not limit by parallel
    task_proxy: HashMap<String, i64>,
    /// task send to this pool when already a task running
    task_onchain: BTreeMap<u64, WaitingTask>,
    /// task need to accept if possible
    task_pending: VecDeque<u64>,
    /// running task with container, will check times, sid => (tid, created, overtime)
    task_working: HashMap<String, (u64, i64, i64)>,
}

impl MainService {
    pub fn new(
        pool_sender: UnboundedSender<PoolMessage>,
        metrics_sender: UnboundedSender<MetricsMessage>,
        p2p_sender: UnboundedSender<P2pMessage>,
        service_receiver: UnboundedReceiver<ServiceMessage>,
        db: Arc<ReDB>,
        docker: Arc<DockerManager>,
        task_parallel: usize,
        url: String,
        zkvm: Option<String>,
    ) -> Self {
        let check_url = is_valid_url(&url, true);
        if check_url {
            info!("[Service] checked url: {}", check_url);
        } else {
            warn!("[Service] checked url: {}", check_url);
        }
        Self {
            pool_sender,
            metrics_sender,
            p2p_sender,
            service_receiver,
            db,
            docker,
            url,
            check_url,
            zkvm,
            task_parallel,
            task_proxy: HashMap::new(),
            task_onchain: BTreeMap::new(),
            task_pending: VecDeque::new(),
            task_working: HashMap::new(),
        }
    }

    pub fn run(mut self, sender: UnboundedSender<ServiceMessage>) {
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(13)).await; // 13s
                sender
                    .send(ServiceMessage::TaskHeartbeat)
                    .expect("Missing service");
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = self.service_receiver.recv().await {
                if let Err(e) = handle(&mut self, msg).await {
                    error!("[Service] main error: {}", e);
                }
            }
        });
    }
}

async fn handle(app: &mut MainService, msg: ServiceMessage) -> Result<()> {
    match msg {
        ServiceMessage::CreateTask(tid, prover, inputs, publics) => {
            // 1. check prover in local
            let key = Prover::to_key(&prover);
            if let Some(p) = app.db.get::<Prover>(key)? {
                // check url status
                if p.ptype.check_url() && !app.check_url {
                    return Ok(());
                }

                // check zkvm status
                if p.ptype.is_zkvm() {
                    if let Some(zkvm) = &app.zkvm {
                        if !is_valid_zkvm(zkvm, &p.types).await {
                            return Ok(());
                        }
                    } else {
                        return Ok(());
                    }
                }

                // 2. insert to waiting list
                app.task_onchain.insert(
                    tid,
                    WaitingTask {
                        image: p.image,
                        prover,
                        inputs,
                        publics,
                    },
                );

                // 3. parallel
                if app.task_parallel == 0 {
                    app.task_pending.push_back(tid);
                    return Ok(());
                }

                // 4. accept task
                app.pool_sender
                    .send(PoolMessage::AcceptTask(tid, app.url.clone()))
                    .expect("Missing pool");
            }
        }
        ServiceMessage::AcceptTask(tid, overtime, is_me) => {
            // 0. Cleanup waiting list
            if let Some(pos) = app.task_pending.iter().position(|x| *x == tid) {
                app.task_pending.remove(pos);
            }
            let task = app.task_onchain.remove(&tid).ok_or(anyhow!("No task"))?;
            if !is_me {
                return Ok(());
            }

            // 1. save task to db
            let sid = tid.to_string();
            let zkvm = app.zkvm.as_ref().map(|v| v.as_str()).unwrap_or("");

            // 2. write data to file
            write_task_input(&sid, task.inputs, task.publics).await?;

            // 3. start docker container to run, TODO we can do more about cpu & memory
            let container = app
                .docker
                .run(&task.image, &sid, zkvm, overtime, RunOption::default())
                .await?;

            // 4. save task to db
            let created = Utc::now().timestamp();
            let t = Task {
                tid,
                prover: task.prover,
                created,
                overtime,
                container,
                is_me: true,
                over: false,
            };
            app.db.add(&t)?;

            app.task_working.insert(sid, (tid, created, overtime));
            if app.task_parallel > 0 {
                app.task_parallel -= 1;
            }
        }
        ServiceMessage::UploadProof(sid, proof) => {
            if let Some(over_at) = app.task_proxy.remove(&sid) {
                let now = Utc::now().timestamp();
                // check overtime, if over, just ignore it.
                if now <= over_at {
                    write_task_proof(&sid, proof).await?;
                }
                // remove task input
                let _ = remove_task_input(&sid).await;
                return Ok(());
            }

            // remove task/minertest
            let _ = app.task_working.remove(&sid);
            app.task_parallel += 1;

            // check if has some task need accept
            if let Some(tid) = app.task_pending.pop_front() {
                app.pool_sender
                    .send(PoolMessage::AcceptTask(tid, app.url.clone()))
                    .expect("Missing pool");
            }

            tokio::spawn(upload_proof(
                app.db.clone(),
                sid,
                proof,
                app.pool_sender.clone(),
            ));
        }
        ServiceMessage::ApproveProver(prover, version, overtime, ptype, types) => {
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
                p.ptype = ptype;
                p.types = types;
                app.db.add(&p)?;

                // 4. delete old image
                app.docker.remove(&old_image).await?;
            }
        }
        ServiceMessage::PullProver(prover, tag, name, overtime, ptype, types) => {
            // 1. pull docker image
            let repo = format!("{:?}", prover);
            let image = app.docker.pull(&repo, &tag).await?;

            // 2. save to db
            let created = Utc::now().timestamp();
            let p = Prover {
                prover,
                tag,
                image,
                name,
                overtime,
                ptype,
                types,
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
        ServiceMessage::ChangeController(wallet, sk) => {
            // 1. update controller in db
            let c = MainController {
                controller: wallet.address(),
            };
            app.db.add(&c)?;

            // 2. update pool signer
            app.pool_sender
                .send(PoolMessage::ChangeController(wallet.clone()))
                .expect("Missing pool");

            app.metrics_sender
                .send(MetricsMessage::ChangeController(wallet.clone()))
                .expect("Missing metrics");

            app.p2p_sender
                .send(P2pMessage::ChangeController(sk))
                .expect("Missing p2p");
        }
        ServiceMessage::MinerTest(id, prover, overtime, inputs, publics) => {
            // 1. check prover in local
            let key = Prover::to_key(&prover);
            if let Some(p) = app.db.get::<Prover>(key)? {
                let sid = format!("m-{}-{}", id, overtime);
                let zkvm = app.zkvm.as_ref().map(|v| v.as_str()).unwrap_or("");

                // 2. write data to file
                write_task_input(&sid, inputs, publics).await?;

                // 3. start docker container to run, TODO we can do more about cpu & memory
                let _container = app
                    .docker
                    .run(&p.image, &sid, zkvm, overtime, RunOption::default())
                    .await?;

                app.task_working
                    .insert(sid, (0, Utc::now().timestamp(), overtime));
                if app.task_parallel > 0 {
                    app.task_parallel -= 1;
                }
            }
        }
        ServiceMessage::ApiTask(sid, over_at) => {
            app.task_proxy.insert(sid, over_at);
        }
        ServiceMessage::TaskHeartbeat => {
            let now = Utc::now().timestamp();
            let mut clean = vec![];
            for (sid, (_, created, overtime)) in app.task_working.iter() {
                let interval = overtime - created;
                let maxtime = interval * 2 + created;
                if now > maxtime {
                    clean.push(sid.clone());
                }
            }
            for i in clean {
                app.task_working.remove(&i);
                app.task_parallel += 1;
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
    // 0. cleanup task input
    let _ = remove_task_input(&sid).await;

    // 1. check task is miner test or task by tid
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

        return;
    }

    let tid: u64 = sid.parse().unwrap_or(0);

    // 2. submit to chain
    pool_sender
        .send(PoolMessage::SubmitTask(tid, proof))
        .expect("Missing pool");

    // 3. update in db
    if let Ok(Some(mut t)) = db.get::<Task>(&Task::to_key(tid)) {
        t.over = true;
        let _ = db.add(&t);
    }
}
