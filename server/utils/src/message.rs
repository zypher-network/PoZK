use ethers::prelude::{Address, LocalWallet};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    oneshot::Sender,
};

pub enum ServiceMessage {
    /// tid, prover, inputs, publics
    CreateTask(u64, Address, Vec<u8>, Vec<u8>),
    /// tid, is_me
    AcceptTask(u64, bool),
    /// prover, version, overtime
    ApproveProver(Address, u64, u64),
    /// tid, proof
    UploadProof(String, Vec<u8>),
    /// controller wallet and sk bytes
    ChangeController(LocalWallet, Vec<u8>),
    /// pull prover, tag, name and overtime
    PullProver(Address, String, String, u64),
    /// remove prover
    RemoveProver(Address),
    /// test id, prover, overtime, inputs, publics
    MinerTest(u64, Address, u64, Vec<u8>, Vec<u8>),
    /// task from player service
    ApiTask(String, Option<Sender<Vec<u8>>>),
}

pub fn new_service_channel() -> (
    UnboundedSender<ServiceMessage>,
    UnboundedReceiver<ServiceMessage>,
) {
    unbounded_channel()
}
