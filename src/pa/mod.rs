mod common;
mod sync_loop;
mod async_loop;
mod pa_actions;
mod callbacks;
mod monitor;

use common::*;
use lazy_static::lazy_static;
use tokio::sync::mpsc;
use state::Storage;

pub use async_loop::start_async;
pub use sync_loop::start;

pub type Monitors = HashMap<u32, (Rc<RefCell<Stream>>, Option<u32>, cb_channel::Sender<u32>)>;

pub enum PAInternal {
    Tick,
    Command(Letter),
    AskInfo(EntryIdentifier),
}

lazy_static! {
    pub static ref INFO_SX: Storage<mpsc::UnboundedSender<EntryIdentifier>> = Storage::new();
    pub static ref SPEC: pulse::sample::Spec = pulse::sample::Spec {
        format: pulse::sample::SAMPLE_FLOAT32,
        channels: 1,
        rate: 15,
    };
}