use std::env;
use std::fs::File;
use std::io::Write;

static TASK_INFO: &str = "/task.rs";
static STAKE_INFO: &str = "/stake.rs";

static GAME_MARKET_INFO: &str = "/prover.rs";

fn main() {
    let dest_path = env::var("OUT_DIR").unwrap();

    let _task_info = {
        let json_content = std::fs::read_to_string("./../../public/ABI/Task.json").unwrap();

        let mut f = File::create(&format!("{dest_path}{TASK_INFO}")).unwrap();
        f.write_all(format!("pub const TASK_CONTRACT_ABI: &str = {json_content:?};\n").as_bytes())
            .unwrap();
    };

    let _stake_info = {
        let json_content = std::fs::read_to_string("./../../public/ABI/Stake.json").unwrap();

        let mut f = File::create(&format!("{dest_path}{STAKE_INFO}")).unwrap();
        f.write_all(format!("pub const STAKE_CONTRACT_ABI: &str = {json_content:?};\n").as_bytes())
            .unwrap();
    };

    let _prover_info = {
        let json_content = std::fs::read_to_string("./../../public/ABI/Prover.json").unwrap();

        let mut f = File::create(&format!("{dest_path}{GAME_MARKET_INFO}")).unwrap();
        f.write_all(
            format!("pub const PROVER_CONTRACT_ABI: &str = {json_content:?};\n").as_bytes(),
        )
        .unwrap();
    };
}
