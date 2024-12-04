use ethers::prelude::{Address, LocalWallet};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub enum ServiceMessage {
    /// tid, prover, inputs, publics
    CreateTask(u64, Address, Vec<u8>, Vec<u8>),
    /// tid, overtime, is_me
    AcceptTask(u64, i64, bool),
    /// prover, version, overtime, need url
    ApproveProver(Address, u64, u64, bool),
    /// tid, proof
    UploadProof(String, Vec<u8>),
    /// controller wallet and sk bytes
    ChangeController(LocalWallet, Vec<u8>),
    /// pull prover, tag, name and overtime, need url
    PullProver(Address, String, String, u64, bool),
    /// remove prover
    RemoveProver(Address),
    /// test id, prover, overtime, inputs, publics
    MinerTest(u64, Address, i64, Vec<u8>, Vec<u8>),
    /// task from player service
    ApiTask(String, i64),
    /// Heartbeat for cleanup task
    TaskHeartbeat,
}

pub fn new_service_channel() -> (
    UnboundedSender<ServiceMessage>,
    UnboundedReceiver<ServiceMessage>,
) {
    unbounded_channel()
}
