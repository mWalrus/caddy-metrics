use std::{
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
    path::Path,
    sync::{Arc, Mutex},
    thread,
};

use notify::{Config, RecommendedWatcher, Watcher};
use serde::Deserialize;

pub struct ReadState {
    cursor: u64,
}

#[derive(Deserialize, Debug)]
pub struct LogEntry {
    pub request: RequestData,
    pub duration: f64,
    pub status: u16,
    pub size: u64,
}

#[derive(Deserialize, Debug)]
pub struct RequestData {
    pub method: String,
    pub host: String,
    pub uri: String,
}

pub type LogQueue = Arc<Mutex<Vec<LogEntry>>>;

static LOG_PATH: &str = "/var/log/caddy/requests.log";

pub fn watch() -> anyhow::Result<LogQueue> {
    // drain the queue when processing events
    let log_queue = Arc::new(Mutex::new(Vec::<LogEntry>::new()));
    let queue_clone = Arc::clone(&log_queue);

    thread::spawn(move || -> anyhow::Result<()> {
        let (tx, rx) = crossbeam::channel::bounded(1);
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        watcher.watch(Path::new(LOG_PATH), notify::RecursiveMode::NonRecursive)?;

        let mut read_state = ReadState { cursor: 0 };
        loop {
            match rx.recv() {
                Ok(Ok(_e)) => match File::open(LOG_PATH) {
                    Ok(mut file) => {
                        let size = file.metadata()?.len();
                        // skip iteration if size has not changed
                        if size == read_state.cursor {
                            continue;
                        }

                        // if the size is less than the cursor the file was probably recreated
                        if size < read_state.cursor {
                            read_state.cursor = 0;
                        }

                        file.seek(SeekFrom::Start(read_state.cursor))?;

                        read_state.cursor = size;

                        let reader = BufReader::new(&file);
                        if let Ok(queue) = &mut queue_clone.lock() {
                            for line in reader.lines() {
                                let entry: LogEntry = serde_json::from_str(&line?)?;
                                queue.push(entry)
                            }
                        }
                    }
                    Err(e) => eprintln!("ERROR: Failed to open file: {e:#?}"),
                },
                e => eprintln!("Something went wrong with the watcher: {e:#?}"),
            }
        }
    });
    Ok(log_queue)
}
