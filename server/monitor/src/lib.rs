mod config;
mod event;
mod service;
mod tx;

pub use config::MonitorConfig;

pub use service::Monitor;
pub use tx::TxService;

include!(concat!(env!("OUT_DIR"), "/task_market.rs"));
include!(concat!(env!("OUT_DIR"), "/prover_market.rs"));
include!(concat!(env!("OUT_DIR"), "/stake.rs"));

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
