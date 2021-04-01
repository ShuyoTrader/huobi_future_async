#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]
mod client;
pub mod error;
pub mod models;
mod transport;
pub use crate::client::{websocket::HuobiWebsocket, HuobiFuture};
pub use crate::error::*;
pub use crate::models::*;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref NATS_CONN: nats::Connection = {
        let nats_url = "nats://127.0.0.1:4222";
        let nc = nats::Options::with_user_pass("admin", "admin")
            .connect(&nats_url)
            .expect("nats connection fail!");
        print!("connect to nats server: {}", &nats_url);
        return nc;
    };
}
