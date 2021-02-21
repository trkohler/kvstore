use env_logger;
use log;
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let mut db = Database::new().unwrap();

    db.insert(&key, &value);
    // let contents = format!("{}\t{}\n", key, value);
    // std::fs::write("kv.db", contents);
    println!("Key: {}, value: {}", key, value);

    Ok(db.close()?)
}

struct Database {
    inner: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let contents = match std::fs::read_to_string("kv.db") {
            Ok(content) => content,
            Err(_) => {
                log::info!("Can't found path, creating new kv.db");
                String::new()
            }
        };
        let mut inner = HashMap::new();

        if !contents.is_empty() {
            for line in contents.lines() {
                let chunks: Vec<&str> = line.split("\t").collect();
                if chunks.len() != 2 {
                    log::warn!("Database might be corrupted.")
                }
                let key = chunks[0];
                let value = chunks[1];
                inner.insert(key.to_owned(), value.to_owned());
            }
        }

        Ok(Database { inner })
    }

    fn insert(&mut self, k: &str, v: &str) {
        self.inner.insert(k.to_owned(), v.to_owned());
    }

    fn close(&self) -> Result<(), std::io::Error> {
        let mut s = String::new();
        for (k, v) in &self.inner {
            s = s + k + "\t" + v + "\n";
        }
        std::fs::write("kv.db", s)?;
        Ok(())
    }
}
