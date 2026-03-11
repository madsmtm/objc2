#![no_main] // The entry point is specified with `-e _NSExtensionMain` instead.

use objc2::ClassType;
use tracing::{error, trace};
use tracing_oslog::OsLogger;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod filesystem;
mod item;
mod util;
mod volume;

/// Initialize our application.
///
/// We must do this in a constructor, to make it run before `NSExtensionMain`.
#[ctor::ctor]
fn setup() {
    // Initialize tracing-oslog. We use this over stderr logging, because
    // stderr is swallowed up by FSKit when spawning our process.
    //
    // View all logs with:
    // /usr/bin/log stream --predicate 'subsystem = "fskit-example"' --style compact --level debug
    let logger = OsLogger::new("fskit-example", "default");
    tracing_subscriber::registry().with(logger).init();

    // Install a panic hook that logs panics via tracing (and thus OSLog)
    // before the default handler runs.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        error!(%info, "panic");
        default_hook(info);
    }));

    // Register `FileSystem` class with the Objective-C runtime.
    let _ = filesystem::FileSystem::class();
    trace!("setup finished");
}
