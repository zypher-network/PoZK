use std::time::Duration;
use crate::{Config, Resp, RespData};
use anyhow::{anyhow, Result};
use ethers::prelude::{Http, Middleware, Provider, ProviderExt};
use jwt::SignWithKey;
use once_cell::sync::OnceCell;
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem::error::InternalServerError;
use poem_openapi::{OpenApi, OpenApiService, Tags};
use poem_openapi::payload::Json;
use serde_json::json;
use tokio::{spawn};
use uuid::Uuid;
use crate::poem::{ApiAuth, LoginReq, SERVER_KEY, User};

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
    Logout
}

pub struct ApiService {
    host: String,
    chain_id: u64,
    eth_cli: Provider<Http>
}

#[OpenApi]
impl ApiService {
    pub async fn new(cfg: &Config) -> Self {
        let host = format!("{}:{}", cfg.host, cfg.port);
        let eth_cli = Provider::<Http>::connect(&cfg.endpoint).await;

        Self {
            host,
            chain_id: cfg.chain_id,
            eth_cli,
        }
    }

    pub async fn run(self) -> Result<()> {
        let host = self.host.clone();
        let server_config = format!("http://{}/api", &host);
        let api_service = OpenApiService::new(self, "miner", "").server(server_config);
        let swagger_ui = api_service.swagger_ui();

        spawn(async move {
            log::info!("1");
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
            log::info!("2");
        });

        tokio::time::sleep(Duration::from_secs(10)).await;

        log::info!(">>>>> api server run <<<<<");
        Ok(())
    }

    #[oai(path = "/hello", method = "get")]
    pub async fn hello(&self, auth: ApiAuth) -> poem::Result<Resp> {
        Ok(Resp::Ok(Json(RespData::new(&auth.0.address.to_string()))))
    }

    #[oai(path = "/login", method = "post", tag = "ApiTags::Login")]
    pub async fn login(
        &self,
        req: Json<LoginReq>
    ) -> poem::Result<Resp> {
        let uid = Uuid::new_v4().to_string();
        log::info!("[login] uid: [{uid}], req: [{req:?}]");

        // to param
        let param = req.to_param().map_err(|e|{
            log::error!("[login] uid: {:?}", e.backtrace());
            e
        })?;

        // get nonce
        let nonce = self.eth_cli.get_transaction_count(param.address, None).await.map_err(|e|anyhow!("{e:?}"))?;

        // check hash
        param.check(nonce.as_u64(), self.chain_id)?;

        let token = User { address: param.address }
            .sign_with_key(&SERVER_KEY.clone())
            .map_err(|e|{
                log::error!("[login] uid: {e:?}");
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

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let eth_cli = Provider::connect("http://127.0.0.1:8545").await;
        let api = ApiService{ host: "0.0.0.0:8090".to_string(), chain_id: 31337, eth_cli,  };
        api.run().await.unwrap();
        tokio::time::sleep(Duration::from_secs(500000)).await;
    });
}
