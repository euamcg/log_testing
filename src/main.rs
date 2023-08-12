use std::time::Duration;

use tracing::{
    dispatcher::{self, Dispatch},
    error, info,
};
use tracing_logfmt;
use tracing_subscriber::{layer::SubscriberExt, Registry};

fn main() {
    let sub = Registry::default().with(tracing_logfmt::layer());
    dispatcher::set_global_default(Dispatch::new(sub)).expect("Global logger already set");
    loop {
        info!("Starting something");
        std::thread::sleep(Duration::from_millis(3000));
        error!("Does this work?");
        std::thread::sleep(Duration::from_millis(2000))
    }
}
