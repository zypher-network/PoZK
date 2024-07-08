use crate::poem::{ApiAuth, LoginReq, User, SERVER_KEY};
use crate::{Config, Resp, RespData};
use anyhow::{anyhow, Result};
use ethers::prelude::{Http, Middleware, Provider, ProviderExt};
use ethers::types::Address;
use jwt::SignWithKey;
use once_cell::sync::OnceCell;
use poem::error::InternalServerError;
use poem::http::uri::Authority;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_openapi::payload::Json;
use poem_openapi::{OpenApi, OpenApiService, Tags};
use serde_json::json;
use std::str::FromStr;
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

    ///
    Logout,
}

pub struct ApiService {
    host: String,
    chain_id: u64,
    eth_cli: Provider<Http>,
    domain: Authority,
}

#[OpenApi]
impl ApiService {
    pub async fn new(cfg: &Config) -> Result<Self> {
        let host = format!("{}:{}", cfg.host, cfg.port);
        let eth_cli = Provider::<Http>::connect(&cfg.endpoint).await;
        let domain = Authority::from_str(&cfg.domain)?;

        Ok(Self {
            host,
            chain_id: cfg.chain_id,
            eth_cli,
            domain,
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
            log::error!("[login] uid: [{uid}], err: {:?}", e.backtrace());
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
                log::error!("[login] uid: [{uid}], err: {:?}", e.backtrace());
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
}

#[test]
fn test_api() {
    env_logger::init();

    /*
    test data:
    eip4361 message:

    0.0.0.0:8090 wants you to sign in with your Ethereum account:
    0xaa6321F2A813c720F0fa19f13789932d05c11e25


    URI: http://0.0.0.0:8090/api/login
    Version: 1
    Chain ID: 31337
    Nonce: 00000000 // this is block number
    Issued At: 2024-07-08T11:42:18.807Z

    req data:
    {
        "domain": "0.0.0.0:8090",
        "address": "0xaa6321F2A813c720F0fa19f13789932d05c11e25",
        "uri": "http://0.0.0.0:8090/api/login",
        "version": "1",
        "chain_id": 31337,
        "nonce": "00000000",
        "issued_at": "2024-07-08T11:42:18.807Z",
        "v": 27,
        "r": "0x953391bcbad53d9c770728471840dfd57ce7c1622616a11e9e5385afd998f883",
        "s": "0x69e42b5f14e193d591b94d614fa41995b22f8ed00ca2309deea3753481f86ad0",
        "resources": []
    }
    */

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let eth_cli = Provider::connect("http://127.0.0.1:8545").await;
        let domain = Authority::from_str("0.0.0.0:8090").unwrap();
        let api = ApiService {
            host: "0.0.0.0:8090".to_string(),
            chain_id: 31337,
            eth_cli,
            domain,
        };
        api.run().await.unwrap();

        tokio::time::sleep(StdDuration::from_secs(500000)).await;
    });
}
