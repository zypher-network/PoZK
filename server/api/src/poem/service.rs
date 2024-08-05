use crate::poem::req::{
    ContainerNewReq, ControllerAddReq, ImagesUpdateReq, ProverNewReq, ProverPullReq,
};
use crate::poem::service::ApiTags::Controller;
use crate::poem::{ApiAuth, LoginReq, Pagination, User, SERVER_KEY};
use crate::{ApiConfig, Resp, RespData};
use anyhow::{anyhow, Result};
use db::{ControllerKey, ControllerValue, ReDB};
use docker::{ContainerNewOption, DockerManager};
use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::rand::thread_rng;
use ethers::prelude::{Http, LocalWallet, Middleware, Provider, ProviderExt, Wallet};
use ethers::types::Address;
use ethers::utils::hex;
use jwt::SignWithKey;
use once_cell::sync::OnceCell;
use poem::error::InternalServerError;
use poem::http::uri::Authority;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_openapi::param::{Path, Query};
use poem_openapi::payload::Json;
use poem_openapi::{OpenApi, OpenApiService, Tags};
use serde_json::json;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration as StdDuration;
use time::format_description::parse;
use time::format_description::well_known::Rfc3339;
use time::parsing::Parsed;
use tokio::spawn;
use uuid::Uuid;

pub static EIP712_DOMAIN_NAME: &str = "Zytron-Miner";

#[derive(Tags)]
enum ApiTags {
    /// Operations about `controller`
    Controller,

    /// Operations about `prover docker`
    Prover,

    ///
    Login,
}

pub struct ApiService {
    host: String,
    chain_id: u64,
    eth_cli: Provider<Http>,
    domain: Authority,
    db: Arc<ReDB>,
    docker_manager: DockerManager,
}

#[OpenApi]
impl ApiService {
    pub async fn new(
        cfg: &ApiConfig,
        db: Arc<ReDB>,
        docker_manager: DockerManager,
        eth_cli: Provider<Http>,
    ) -> Result<Self> {
        let host = format!("{}:{}", cfg.host, cfg.port);
        let domain = Authority::from_str(&cfg.login_domain)?;
        let chain_id = eth_cli.get_chainid().await?;

        Ok(Self {
            host,
            chain_id: chain_id.as_u64(),
            eth_cli,
            domain,
            db,
            docker_manager,
        })
    }

    pub async fn run(self) -> Result<()> {
        let host = self.host.clone();
        let server_config = format!("http://{}/api", &host);
        let api_service = OpenApiService::new(self, "miner", "").server(server_config);
        let swagger_ui = api_service.swagger_ui();

        spawn(async move {
            Server::new(TcpListener::bind(host))
                .run(
                    Route::new()
                        .nest("/api", api_service)
                        .nest("/", swagger_ui)
                        .data(SERVER_KEY.clone())
                        .with(Cors::new()),
                )
                .await
                .expect("");
        });

        tokio::time::sleep(StdDuration::from_secs(10)).await;

        log::info!(">>>>> api server run <<<<<");
        Ok(())
    }

    #[oai(path = "/hello", method = "get")]
    pub async fn hello(&self, auth: ApiAuth) -> poem::Result<Resp> {
        Ok(Resp::Ok(Json(RespData::new(&auth.0.address.to_string()))))
    }

    #[oai(path = "/login", method = "post", tag = "ApiTags::Login")]
    pub async fn login(&self, req: Json<LoginReq>) -> poem::Result<Resp> {
        let uid = Uuid::new_v4().to_string();
        log::info!("[login] uid: [{uid}], req: [{req:?}]");

        // to param
        let param = req.to_param().map_err(|e| {
            log::error!(
                "[login] uid: [{uid}], err: {:?}, backtrace: {:?}",
                e,
                e.backtrace()
            );
            e
        })?;

        // get block_num
        let block_num = self
            .eth_cli
            .get_block_number()
            .await
            .map_err(|e| anyhow!("{e:?}"))?;

        // check
        param
            .check(block_num.as_u64(), self.chain_id, &self.domain)
            .map_err(|e| {
                log::error!(
                    "[login] uid: [{uid}], err: {:?}, backtrace: {:?}",
                    e,
                    e.backtrace()
                );
                e
            })?;

        let token = User {
            address: Address::from(param.msg.address),
            create_time: param.now.unix_timestamp(),
            expiry_time: param.expiry_time.unix_timestamp(),
        }
        .sign_with_key(&SERVER_KEY.clone())
        .map_err(|e| {
            log::error!("[login] uid: [{uid}], err: {e:?}");
            anyhow!("{e:?}")
        })?;

        Ok(Resp::Ok(Json(RespData::new_data(
            &json!({
                "token": token
            }),
            &uid,
        ))))
    }

    #[oai(path = "/controller/list", method = "get", tag = "ApiTags::Controller")]
    pub async fn controller_list(
        &self,
        auth: ApiAuth,
        page_size: Query<Option<usize>>,
        page_count: Query<Option<usize>>,
    ) -> poem::Result<Resp> {
        let miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };
        let uid = Uuid::new_v4().to_string();
        log::info!(
            "[controller/list] uid: [{uid}], page_size: [{:?}], page_count: [{:?}]",
            page_size.0,
            page_count.0
        );

        let pagination = Pagination {
            page_count: page_count.0.unwrap_or_default(),
            page_size: page_size.0.unwrap_or_default(),
        };
        let (begin, take_count) = pagination.begin_and_take();
        log::debug!("[controller/list] uid: [{uid}], begin: [{begin}], take_count: [{take_count}]");

        if let Some(res) = self.db.controller_list(&miner, begin, take_count)? {
            Ok(Resp::Ok(Json(RespData::new_data(
                &json!({
                    "data": res.data,
                    "total": res.total
                }),
                &uid,
            ))))
        } else {
            Ok(Resp::Ok(Json(RespData::new(&uid))))
        }
    }

    #[oai(path = "/controller/add", method = "post", tag = "ApiTags::Controller")]
    pub async fn controller_add(
        &self,
        auth: ApiAuth,
        req: Json<ControllerAddReq>,
    ) -> poem::Result<Resp> {
        let miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };

        let uid = Uuid::new_v4().to_string();
        log::info!("[controller/add] uid: [{uid}], req: [{req:?}]");

        // to param
        let param = req.to_param().map_err(|e| {
            log::error!("[controller/add] uid: [{uid}], err: {:?}", e.backtrace());
            e
        })?;

        let key = ControllerKey::from(&param.signing_key);

        self.db.controller_add(&miner, &key, &param.signing_key)?;
        log::info!("[controller/add] uid: [{uid}] success");

        Ok(Resp::Ok(Json(RespData::new(&uid))))
    }

    #[oai(
        path = "/controller/set/:address",
        method = "post",
        tag = "ApiTags::Controller"
    )]
    pub async fn controller_set(&self, auth: ApiAuth, address: Path<String>) -> poem::Result<Resp> {
        let miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };

        let uid = Uuid::new_v4().to_string();
        log::info!("[controller/set] uid: [{uid}], req: [{}]", address.0);

        let address =
            Address::from_str(&address.0).map_err(|e| anyhow!("address parse err: {e:?}"))?;

        let key = ControllerKey(address);

        self.db.controller_set(&miner, &key)?;
        log::info!("[controller/set] uid: [{uid}] success");

        Ok(Resp::Ok(Json(RespData::new(&uid))))
    }

    #[oai(path = "/controller/set", method = "get", tag = "ApiTags::Controller")]
    pub async fn query_controller_set(&self, auth: ApiAuth) -> poem::Result<Resp> {
        let miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };

        let uid = Uuid::new_v4().to_string();
        log::info!("[get/controller/set] uid: [{uid}]");

        let controller = self.db.query_controller_set(&miner)?;
        Ok(Resp::Ok(Json(RespData::new_data(
            &json!({
                "controller": format!("{:?}",controller.0)
            }),
            &uid,
        ))))
    }

    #[oai(
        path = "/controller/export/:address",
        method = "get",
        tag = "ApiTags::Controller"
    )]
    pub async fn export_controller(
        &self,
        auth: ApiAuth,
        address: Path<String>,
    ) -> poem::Result<Resp> {
        let miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };

        let uid = Uuid::new_v4().to_string();
        log::info!("[get/controller/export] uid: [{uid}]");

        let (controller, singing_key) = {
            let address =
                Address::from_str(&address.0).map_err(|e| anyhow!("address parse err: {e:?}"))?;

            let key = ControllerKey(address);

            let singing_key = self.db.controller_export(&miner, &key)?;
            let singing_key_hex = hex::encode(singing_key.to_bytes().as_slice());

            (key, singing_key_hex)
        };
        Ok(Resp::Ok(Json(RespData::new_data(
            &json!({
                "controller": format!("{:?}",controller.0),
                "singing_key": format!("0x{}",singing_key)
            }),
            &uid,
        ))))
    }

    #[oai(path = "/controller/new", method = "post", tag = "ApiTags::Controller")]
    pub async fn controller_new(&self, auth: ApiAuth) -> poem::Result<Resp> {
        let miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };

        let uid = Uuid::new_v4().to_string();
        log::info!("[controller/new] uid: [{uid}]");

        let signing_key = SigningKey::random(&mut thread_rng());

        let key = ControllerKey::from(&signing_key);

        self.db.controller_add(&miner, &key, &signing_key)?;
        log::info!("[controller/new] uid: [{uid}] success");

        Ok(Resp::Ok(Json(RespData::new_data(
            &json!({
                "controller": format!("{:?}", key.0),
            }),
            &uid,
        ))))
    }

    #[oai(path = "/prover/pull", method = "post", tag = "ApiTags::Prover")]
    pub async fn prover_pull(&self, auth: ApiAuth, req: Json<ProverPullReq>) -> poem::Result<Resp> {
        let miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };

        let prover = Address::from_str(&req.prover).map_err(|e| anyhow!("{e:?}"))?;

        let uid = Uuid::new_v4().to_string();
        log::info!("[prover/pull] uid: [{uid}], req: [{req:?}]");

        self.docker_manager
            .pull_image(&req.repository, &req.tag)
            .await?;

        let Some(image_id) = self
            .docker_manager
            .get_image_by_repository(&req.repository)
            .await?
        else {
            return Ok(Resp::Ok(Json(RespData::new_msg(
                "pull image failed".to_string(),
                &uid,
                -1,
            ))));
        };

        self.db.prover_add(
            &miner,
            &image_id,
            &req.name,
            &req.repository,
            &prover,
            &req.tag,
            None,
        )?;

        Ok(Resp::Ok(Json(RespData::new(&uid))))
    }

    #[oai(path = "/prover/new", method = "post", tag = "ApiTags::Prover")]
    pub async fn prover_new(&self, auth: ApiAuth, req: Json<ProverNewReq>) -> poem::Result<Resp> {
        let miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };

        let prover = Address::from_str(&req.prover).map_err(|e| anyhow!("{e:?}"))?;

        let uid = Uuid::new_v4().to_string();
        log::info!("[prover/new] uid: [{uid}], req: [{req:?}]");

        let Some(meta) = self.db.prover_meta(&miner, &prover)? else {
            return Ok(Resp::Ok(Json(RespData::new_err("miner not exist".to_string(), &uid))));
        };

        let ccf = self
            .docker_manager
            .new_container(&meta.repository, &meta.tag, &req.option)
            .await?;

        self.db.prover_container_add(&miner, &prover, &ccf.id)?;

        Ok(Resp::Ok(Json(RespData::new_data(
            &json!({
                "container_id": ccf.id
            }),
            &uid,
        ))))
    }

    #[oai(path = "/prover/:container/start", method = "post", tag = "ApiTags::Prover")]
    pub async fn container_start(
        &self,
        auth: ApiAuth,
        container: Path<String>,
    ) -> poem::Result<Resp> {
        let _miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };

        let uid = Uuid::new_v4().to_string();
        log::info!("[container/start] uid: [{uid}], container: [{}]",container.0);

        self.docker_manager.start_container(&container.0).await?;

        Ok(Resp::Ok(Json(RespData::new(&uid))))
    }

    #[oai(path = "/prover/list", method = "get", tag = "ApiTags::Prover")]
    pub async fn prover_list(
        &self,
        auth: ApiAuth,
        page_size: Query<Option<usize>>,
        page_count: Query<Option<usize>>,
    ) -> poem::Result<Resp> {
        let miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };
        let uid = Uuid::new_v4().to_string();
        log::info!(
            "[prover/list] uid: [{uid}], page_size: [{:?}], page_count: [{:?}]",
            page_size.0,
            page_count.0
        );

        let pagination = Pagination {
            page_count: page_count.0.unwrap_or_default(),
            page_size: page_size.0.unwrap_or_default(),
        };
        let (begin, take_count) = pagination.begin_and_take();
        log::debug!("[prover/list] uid: [{uid}], begin: [{begin}], take_count: [{take_count}]");

        if let Some(data) = self.db.docker_image_list(&miner, begin, take_count)? {
            let data = serde_json::to_value(&data).map_err(|e| anyhow!(e))?;
            Ok(Resp::Ok(Json(RespData::new_data(&data, &uid))))
        } else {
            Ok(Resp::Ok(Json(RespData::new(&uid))))
        }
    }

    #[oai(path = "/prover/:prover/list", method = "get", tag = "ApiTags::Prover")]
    pub async fn container_list(
        &self,
        auth: ApiAuth,
        page_size: Query<Option<usize>>,
        page_count: Query<Option<usize>>,
        prover: Path<String>,
    ) -> poem::Result<Resp> {
        let miner = {
            let address = auth.0.address;
            ControllerKey(address)
        };

        let prover = Address::from_str(&prover).map_err(|e| anyhow!("{e:?}"))?;

        let uid = Uuid::new_v4().to_string();
        log::info!(
            "[prover/{:?}/list] uid: [{uid}], page_size: [{:?}], page_count: [{:?}]",
            prover,
            page_size.0,
            page_count.0
        );

        let pagination = Pagination {
            page_count: page_count.0.unwrap_or_default(),
            page_size: page_size.0.unwrap_or_default(),
        };
        let (begin, take_count) = pagination.begin_and_take();
        log::debug!(
            "[prover/{}/list] uid: [{uid}], begin: [{begin}], take_count: [{take_count}]",
            prover
        );

        if let Some(dcl) = self
            .db
            .docker_container_list(&miner, &prover, begin, take_count)?
        {
            let data = {
                let mut info = self
                    .docker_manager
                    .prover_image_list(&dcl.meta.image_id, dcl.data)
                    .await?;
                info.total = dcl.total;
                serde_json::to_value(&info).map_err(|e| anyhow!("{e:?}"))?
            };

            Ok(Resp::Ok(Json(RespData::new_data(&data, &uid))))
        } else {
            Ok(Resp::Ok(Json(RespData::new(&uid))))
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::poem::req::{ControllerAddReq, ControllerSetReq};
    use crate::ApiService;
    use db::{ControllerKey, DockerImageList, ReDB};
    use docker::DockerManager;
    use ethers::abi::AbiEncode;
    use ethers::core::k256::ecdsa::SigningKey;
    use ethers::core::rand::thread_rng;
    use ethers::prelude::{LocalWallet, Provider, ProviderExt, Signer};
    use ethers::utils::hex;
    use ethers::utils::hex::encode;
    use poem::http::uri::Authority;
    use reqwest::StatusCode;
    use serde_json::{json, Value};
    use siwe::Message;
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::sync::Arc;
    use std::time::Duration;

    static DOCKER_MANAGER_UPDATE_URL: &str = "http://127.0.0.1:9900/api/images";

    #[test]
    fn test_run() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let eth_cli = Provider::connect("http://127.0.0.1:8545").await;
            let domain = Authority::from_str("0.0.0.0:8090").unwrap();
            let db = {
                let db = ReDB::new(&PathBuf::from("/tmp/pozk/"), true).unwrap();
                Arc::new(db)
            };
            let docker_manager = DockerManager::new().unwrap();

            let api = ApiService {
                host: "0.0.0.0:8090".to_string(),
                chain_id: 31337,
                eth_cli,
                domain,
                db,
                docker_manager,
            };
            api.run().await.unwrap();

            tokio::signal::ctrl_c().await.unwrap();
        })
    }

    #[test]
    fn test_eip4361() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let msg = r#"localhost:4000 wants you to sign in with your Ethereum account:
0x28B9FEAE1f3d76565AAdec86E7401E815377D9Cc

Welcome to Zytron!

URI: http://0.0.0.0:9098/api/login
Version: 1
Chain ID: 5611
Nonce: 00000000
Issued At: 2024-08-02T10:35:18.807Z"#;

            let message: Message = msg.parse().unwrap();
            println!("message: {message:?}");
            let sk = "96dfba9d7b2967155b3e29d02df72a412bb935f899c99af7be41b68243949790";
            let wallet = LocalWallet::from_str(sk).unwrap();

            let sign = wallet.sign_message(message.to_string()).await.unwrap();

            println!("r: {:?}", sign.r.encode_hex());
            println!("s: {:?}", sign.s.encode_hex());
            println!("v: {:?}", sign.v);

            let sig: [u8; 65] = sign.into();
            println!("sig: {}", hex::encode(&sig));
            message.verify_eip191(&sig).unwrap();
        });
    }

    #[test]
    fn test_login() {
        env_logger::init();

        /*
        test data:
        eip4361 message:

        0.0.0.0:8090 wants you to sign in with your Ethereum account:
        0x28B9FEAE1f3d76565AAdec86E7401E815377D9Cc

        Welcome to Zytron!

        URI: http://0.0.0.0:8090/api/login
        Version: 1
        Chain ID: 31337
        Nonce: 00000000 // this is block number
        Issued At: 2024-07-08T11:42:18.807Z

        req data:
        {
            "domain": "0.0.0.0:8090",
            "address": "0x28B9FEAE1f3d76565AAdec86E7401E815377D9Cc",
            "uri": "http://0.0.0.0:8090/api/login",
            "version": "1",
            "chain_id": 31337,
            "nonce": "00000000",
            "issued_at": "2024-07-08T23:42:18.807Z",
            "v": 27,
            "r": "0xcb7d50792e40e536b3bfff99fb6428bc5f7b78ef36bb0309f28e0c9918f0bbdd",
            "s": "0x4b17270ff28b73e2a414dc7c619b0f090163e888b764bbd9480bbcb6b0d34026",
            "statement": "Welcome to Zytron!",
            "resources": []
        }
        */

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let eth_cli = Provider::connect("https://miner.zypher.game/api/login").await;
            let domain = Authority::from_str("localhost:4000").unwrap();
            let db = {
                let db = ReDB::new(&PathBuf::from("/tmp/pozk/"), true).unwrap();
                Arc::new(db)
            };
            let docker_manager = DockerManager::new().unwrap();

            let api = ApiService {
                host: "0.0.0.0:8090".to_string(),
                chain_id: 5611,
                eth_cli,
                domain,
                db,
                docker_manager
            };
            api.run().await.unwrap();

            {
                let client = reqwest::Client::new();

                let req = client.post("http://127.0.0.1:8090/api/login")
                    .json(&json!({
                            "domain": "localhost:4000",
                            "address": "0x28B9FEAE1f3d76565AAdec86E7401E815377D9Cc",
                            "uri": "http://0.0.0.0:8098/api/login",
                            "version": "1",
                            "chain_id": 5611,
                            "nonce": "00000000",
                            "issued_at": "2024-08-02T10:35:18.807Z",
                            "v": 28,
                            "r": "0x9870cecbc7a3db437951da275252b2c75e2eeb8c06f6daf6a32a6791da6e71d2",
                            "s": "0x470c3c2d3ee7979adc138a1de327588e458f4fdbd3edcb8d950d4d3ba3155531",
                            "statement": "Welcome to Zytron!",
                            "resources": []
                    }))
                    .build().unwrap();
                let result = client.execute(req).await.unwrap();
                if result.status() == StatusCode::OK {
                    let body = result.json::<Value>().await.unwrap();
                    println!("login: {body:?}");

                    // let token = body["data"]["token"].clone().as_str().unwrap().to_string();
                    // let req = client.get("http://127.0.0.1:8090/api/hello")
                    //     .header("X-API-Key", token)
                    //     .build()
                    //     .unwrap();
                    // let result = client.execute(req).await.unwrap();
                    // println!("hello: {result:?}")
                } else {
                    let body = result.bytes().await.unwrap();
                    let str = String::from_utf8(body.to_vec()).unwrap();
                    println!("login fail: {str}");
                }


            }
        });
    }

    #[test]
    fn test_controller() {
        env_logger::init();

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let eth_cli = Provider::connect("http://127.0.0.1:8545").await;
            let domain = Authority::from_str("0.0.0.0:8090").unwrap();
            let db = {
                let db = ReDB::new(&PathBuf::from("/tmp/pozk/"), true).unwrap();
                Arc::new(db)
            };
            let docker_manager = DockerManager::new().unwrap();

            let api = ApiService {
                host: "0.0.0.0:8090".to_string(),
                chain_id: 31337,
                eth_cli,
                domain,
                db,
                docker_manager,
            };
            api.run().await.unwrap();

            // new client
            let client = reqwest::Client::new();

            // ** test /controller/new **
            // 1. new controller
            // 2. query controller list
            {
                println!("** test /controller/new **");
                println!("**************************");

                // 1.
                let req = client
                    .post("http://127.0.0.1:8090/api/controller/new")
                    .build()
                    .unwrap();
                let result = client
                    .execute(req)
                    .await
                    .unwrap()
                    .json::<Value>()
                    .await
                    .unwrap();
                println!("** controller new: {result:?}");

                // 2.
                let req = client
                    .get("http://127.0.0.1:8090/api/controller/list")
                    .build()
                    .unwrap();
                let resp = client.execute(req).await.unwrap();
                if resp.status() == StatusCode::OK {
                    let result = resp.json::<Value>().await.unwrap();
                    println!("** controller list: {result:?}");
                } else {
                    let result = resp.bytes().await.unwrap();
                    let result = String::from_utf8(result.to_vec()).unwrap();
                    println!("** controller list: {result:?}");
                };

                println!("**************************");
            }

            // ** test /controller/set **
            // 1. new controller
            // 2. set controller
            // 3. query set controller
            {
                println!("** test /controller/set **");
                println!("**************************");

                // 1.
                let req = client
                    .post("http://127.0.0.1:8090/api/controller/new")
                    .build()
                    .unwrap();
                let result = client
                    .execute(req)
                    .await
                    .unwrap()
                    .json::<Value>()
                    .await
                    .unwrap();
                println!("** controller new: {result:?}");

                // 2.
                let data = {
                    let address = result["data"]["controller"].as_str().unwrap();
                    let req = ControllerSetReq {
                        address: address.to_string(),
                    };
                    serde_json::to_value(req).unwrap()
                };
                let req = client
                    .post("http://127.0.0.1:8090/api/controller/set")
                    .json(&data)
                    .build()
                    .unwrap();
                let result = client.execute(req).await.unwrap();

                if result.status() == StatusCode::OK {
                    println!("** controller set: {result:?}");
                } else {
                    let bytes = result.bytes().await.unwrap();
                    let msg = String::from_utf8(bytes.to_vec()).unwrap();
                    eprintln!("*** controller set error: {msg}");
                    panic!("{msg}");
                }

                // 3.
                let req = client
                    .get("http://127.0.0.1:8090/api/controller/set")
                    .build()
                    .unwrap();
                let result = client.execute(req).await.unwrap();
                println!("** query controller set: {result:?}");

                println!("**************************");
            }

            // ** test /controller/add **
            // 1. new signing key
            // 2. add controller
            // 3. controller list
            {
                println!("** test /controller/add **");
                println!("**************************");

                // 1.
                let sk = SigningKey::random(&mut thread_rng());
                let key = ControllerKey::from(&sk);
                println!("** signing key: {:?}", key.0);

                // 2.
                let data = {
                    let bytes = sk.to_bytes();
                    let hex = encode(bytes.as_slice());
                    let req = ControllerAddReq { signing_key: hex };
                    serde_json::to_value(req).unwrap()
                };
                let req = client
                    .post("http://127.0.0.1:8090/api/controller/add")
                    .json(&data)
                    .build()
                    .unwrap();
                let result = client.execute(req).await.unwrap();
                println!("** add controller: {result:?}");

                // 3.
                let req = client
                    .get("http://127.0.0.1:8090/api/controller/list")
                    .build()
                    .unwrap();
                let resp = client
                    .execute(req)
                    .await
                    .unwrap()
                    .json::<Value>()
                    .await
                    .unwrap();
                println!("** controller list: {resp:?}");

                println!("**************************");
            }
        });
    }

    #[test]
    fn test_prover() {
        env_logger::init();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {

            let eth_cli = Provider::connect("http://127.0.0.1:8545").await;
            let domain = Authority::from_str("localhost:4000").unwrap();
            let db = {
                let db = ReDB::new(&PathBuf::from("/tmp/pozk/"), true).unwrap();
                Arc::new(db)
            };
            let docker_manager = DockerManager::new().unwrap();

            let api = ApiService {
                host: "0.0.0.0:8090".to_string(),
                chain_id: 31337,
                eth_cli,
                domain,
                db,
                docker_manager,
            };
            api.run().await.unwrap();

            let client = reqwest::Client::new();

            let token = {
                let client = reqwest::Client::new();

                let req = client.post("http://127.0.0.1:8090/api/login")
                    .json(&json!({
                            "domain": "localhost:4000",
                            "address": "0x28B9FEAE1f3d76565AAdec86E7401E815377D9Cc",
                            "uri": "http://0.0.0.0:8090/api/login",
                            "version": "1",
                            "chain_id": 31337,
                            "nonce": "00000000",
                            "issued_at": "2024-07-23T11:42:18.807Z",
                            "v": 27,
                            "r": "0xa2a8e125ffb4ded5892d2ccf3535357b6089952ef046073fe26a259e0f2483ac",
                            "s": "0x30a1b932730eb7af9c118fcf69e2c3bde4f035ea48f53d00e936852dc6966614",
                            "statement": "Welcome to Zytron!",
                            "resources": []
                    }))
                    .build().unwrap();
                let result = client.execute(req).await.unwrap().json::<Value>().await.unwrap();
                println!("login: {result:?}");

                let token = result["data"]["token"].clone().as_str().unwrap().to_string();
                token
            };

            // test pull prover
            {
                let req = client.post("http://127.0.0.1:8090/api/prover/pull")
                    .json(&json!({
                        "repository": "docker.registry.cyou/zyphernetwork/0x48a7fb14fd5711cf057bc7392973680231e8aebb",
                        "prover": "0x48a7fb14fd5711cf057bc7392973680231e8aebb",
                        "tag": "v1",
                        "name": "shuffle"
                    }))
                    .header("X-API-Key", token.clone())
                    .build().unwrap();

                let result = client.execute(req).await.unwrap();
                if result.status() == StatusCode::OK {
                    let body = result.json::<Value>().await.unwrap();
                    let str = serde_json::to_string_pretty(&body).unwrap();
                    println!("add prover success result: {str}");
                } else {
                    let str = result.bytes().await.unwrap();
                    println!("add prover fail: {str:?}");
                }
            }

            // test new container to prover
            {
                let req = client.post("http://127.0.0.1:8090/api/prover/new")
                    .json(&json!({
                        "option": {
                            "env": [
                                "INPUT=/data/0x48a7fb14fd5711cf057bc7392973680231e8aebb.input",
                                "OUTPUT=/data/0x48a7fb14fd5711cf057bc7392973680231e8aebb.publics",
                                "PROOF=/data/0x48a7fb14fd5711cf057bc7392973680231e8aebb.proof"
                            ],
                            "volumes": [
                                {
                                    "src_volumes": "/data",
                                    "host_volumes": "/home/cloud/zypher/pozk-shuffle/prover/examples"
                                }
                            ]
                        },
                        "prover": "0x48a7fb14fd5711cf057bc7392973680231e8aebb",
                        "tag": "v1"
                    }))
                    .header("X-API-Key", token.clone())
                    .build().unwrap();

                let result = client.execute(req).await.unwrap();
                if result.status() == StatusCode::OK {
                    let body = result.json::<Value>().await.unwrap();
                    let str = serde_json::to_string_pretty(&body).unwrap();
                    println!("new prover success result: {str}");
                } else {
                    let str = result.bytes().await.unwrap();
                    println!("new prover fail: {str:?}");
                }
            }

            // test image list
            let mut images = None;
            {
                let req = client.get("http://127.0.0.1:8090/api/prover/list")
                    .header("X-API-Key", token.clone())
                    .build().unwrap();
                let result = client.execute(req).await.unwrap();
                if result.status() == StatusCode::OK {
                    let body = result.json::<Value>().await.unwrap();
                    let data = body["data"].clone();
                    let dil = serde_json::from_value::<DockerImageList>(data).unwrap();
                    images.replace(dil);
                    let str = serde_json::to_string_pretty(&body).unwrap();
                    println!("prover list success result: {str}");
                } else {
                    let str = result.bytes().await.unwrap();
                    println!("prover list fail: {str:?}");
                }
            }

            // test container list
            {
                if images.is_some() {
                    let dil = images.unwrap();
                    for di in dil.data {
                        let req = client.get(format!("http://127.0.0.1:8090/api/prover/{}/list", di.prover))
                            .header("X-API-Key", token.clone())
                            .build().unwrap();

                        let result = client.execute(req).await.unwrap();
                        if result.status() == StatusCode::OK {
                            let body = result.json::<Value>().await.unwrap();
                            let str = serde_json::to_string_pretty(&body).unwrap();
                            println!("prover {} list success result: {str}", di.prover);
                        } else {
                            let str = result.bytes().await.unwrap();
                            println!("prover {} list fail: {str:?}", di.prover);
                        }
                    }
                }
            }
        });
    }
}
