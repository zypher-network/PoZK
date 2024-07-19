use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Write;

static TASK_MARKET_INFO: &str = "/task_market.rs";
static STAKE_INFO: &str = "/stake.rs";

static GAME_MARKET_INFO: &str = "/game_market.rs";

fn main() {
    let dest_path = env::var("OUT_DIR").unwrap();

    let _task_market_info = {
        let json_content = std::fs::read_to_string("./../../public/ABI/TaskMarket.json").unwrap();

        let mut f = File::create(&format!("{dest_path}{TASK_MARKET_INFO}")).unwrap();
        f.write_all(
            format!("pub const TASK_MARKET_CONTRACT_ABI: &str = {json_content:?};\n").as_bytes(),
        )
        .unwrap();
    };

    let _stake_info = {
        let json_content = std::fs::read_to_string("./../../public/ABI/Stake.json").unwrap();

        let mut f = File::create(&format!("{dest_path}{STAKE_INFO}")).unwrap();
        f.write_all(format!("pub const STAKE_CONTRACT_ABI: &str = {json_content:?};\n").as_bytes())
            .unwrap();
    };

    let _game_market_info = {
        let json_content = std::fs::read_to_string("./../../public/ABI/GameMarket.json").unwrap();

        let mut f = File::create(&format!("{dest_path}{GAME_MARKET_INFO}")).unwrap();
        f.write_all(
            format!("pub const GAME_MARKET_CONTRACT_ABI: &str = {json_content:?};\n").as_bytes(),
        )
        .unwrap();
    };
}
