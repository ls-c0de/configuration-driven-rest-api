use serde::Deserialize;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, event};
use notify::event::{EventKind, ModifyKind};
use notify::*;
use std::{path::Path, fs, time::Duration};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Config {
    field: Vec<Field>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Field {
    name: String,
    datatype: String,
}

fn main() {
    let path = Path::new("configs/shop.toml");

    println!("Watching {:?}", path);
    
    if let Err(error) = watch(path) {
        println!("Error: {error:?}");
    }
}

fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, notify::Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                // 1. Nur echte File-Write Events akzeptieren
                if !matches!(event.kind, EventKind::Modify(ModifyKind::Data(_))) {
                    read_toml();
                    
                };
            },
            Err(error) => println!("Error: {error:?}"),
        }
    }

    Ok(())
}

fn read_toml() {
    let data = fs::read_to_string("configs/shop.toml").expect("");
    let config: Config = toml::from_str(&data).expect("");

    println!("{:?}", config)
}
