mod controller;
mod login;
mod prover;
pub use prover::{ProverNewReq, ProverPullReq};

pub use controller::ControllerAddReq;

pub use login::LoginReq;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub page_count: usize,
    pub page_size: usize,
}

impl Pagination {
    pub fn parse(&self) -> (usize, usize) {
        let page_count = if self.page_count < 1 {
            1
        } else {
            self.page_count
        };

        let page_size = if self.page_size < 1 {
            10
        } else if self.page_size > 100 {
            100
        } else {
            self.page_size
        };

        (page_count, page_size)
    }

    pub fn begin_and_take(&self) -> (usize, usize) {
        let (page_count, page_size) = self.parse();

        let begin = if page_count == 1 {
            0
        } else {
            page_count * page_size
        };

        (begin, page_size)
    }
}
