use notify::{RecommendedWatcher, RecursiveMode, Watcher, event};
use notify::event::{EventKind, ModifyKind};
use std::{path::Path, fs, time::Duration};

#[cfg(feature = "tracker")]
fn track_changes_to_configs() {
    let path = Path::new("configs/api");

    println!("Watching {:?}", path);
    
    if let Err(error) = watch(path) {
        println!("Error: {error:?}");
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = PollWatcher::new(
        tx, 
        notify::Config::default().with_poll_interval(Duration::from_secs(1)))?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("Event: {event:?}");
             track_toml();
            },
            Err(error) => println!("Error: {error:?}"),
        }
    }

    Ok(())
}