use std::time::Duration;

use tracing::{
    debug,
    dispatcher::{self, Dispatch},
    error, info, trace, warn,
};
use tracing_logfmt;
use tracing_subscriber::{layer::SubscriberExt, Registry};

fn main() {
    let sub = Registry::default().with(tracing_logfmt::layer());
    dispatcher::set_global_default(Dispatch::new(sub)).expect("Global logger already set");
    loop {
        info!("Does this work?");
        std::thread::sleep(Duration::from_millis(3000));
        error!("Does this work?");
        std::thread::sleep(Duration::from_millis(3000));
        trace!("Does this work?");
        std::thread::sleep(Duration::from_millis(3000));
        debug!("Does this work?");
        std::thread::sleep(Duration::from_millis(3000));
        warn!("Does this work?");
    }
}
