use ethers::prelude::{Address, LocalWallet};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub enum ServiceMessage {
    /// tid, prover, data
    CreateTask(u64, Address, Vec<u8>),
    /// tid, is_me
    AcceptTask(u64, bool),
    /// prover, version, overtime
    ApproveProver(Address, u64, u64),
    /// tid, publics, proof
    UploadProof(u64, Vec<u8>, Vec<u8>),
    /// controller wallet
    ChangeController(LocalWallet),
    /// pull prover, tag, name and overtime
    PullProver(Address, String, String, u64),
    /// remove prover
    RemoveProver(Address),
}

pub fn new_service_channel() -> (
    UnboundedSender<ServiceMessage>,
    UnboundedReceiver<ServiceMessage>,
) {
    unbounded_channel()
}
