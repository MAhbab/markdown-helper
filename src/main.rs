use clap::Parser;
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
use serde_yaml;
use chrono::DateTime;
use chrono;

#[derive(Parser)]
struct Args {
    #[arg()]
    file: PathBuf,
}

#[derive(Debug)]
struct Note {
    frontmatter: HashMap<String, serde_yaml::Value>,
    content: String,
    path: PathBuf,
}

impl Note {
    fn timestamp(&self) -> Option<DateTime<chrono::Utc>> {
        match self.frontmatter.get("unix") {
            Some(serde_yaml::Value::Number(n)) => DateTime::from_timestamp(n.as_i64().unwrap(), 0),
            _ => None
        }
    }
}

fn parse_note(path: PathBuf) -> Note {
    let file = Args::parse().file;
    let text = match fs::read_to_string(&file){
        Ok(text) => text,
        Err(e) => {
            eprintln!("File not found {}: {}", path.display(), e); 
            std::process::exit(1);
        }
    };

    let mut text_components = text.splitn(3, "---");
    text_components.next();
    let frontmatter_str = text_components.next().unwrap();
    let content = text_components.next().unwrap();
    let frontmatter: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&frontmatter_str).unwrap();
    Note {
        frontmatter,
        content: content.to_string(),
        path
    }
    
}

fn main() {
    let note = parse_note(Args::parse().file);
    let ts = note.timestamp().unwrap();

    println!("{:?}", ts);
}
