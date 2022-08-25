use log::info;

use pink_sidevm as sidevm;
use sidevm::{logger::Logger, ocall};

#[sidevm::main]
async fn main() {
    Logger::with_max_level(log::LevelFilter::Trace).init();
    ocall::enable_ocall_trace(true).unwrap();

    loop {
        info!("waiting next message...");
        if let Some(message) = sidevm::channel::input_messages().next().await {
            let text_message = String::from_utf8_lossy(&message);
            info!("received message: {}", text_message);
        } else {
            info!("no message received");
            break;
        }
    }
}
