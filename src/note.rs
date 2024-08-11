use std::fs;
use std::collections::HashMap;
use serde_yaml;
use chrono::DateTime;
use chrono;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Note {
    frontmatter: HashMap<String, serde_yaml::Value>,
    content: String,
    path: PathBuf,
}

impl Note {
    pub fn file_summary(&self) -> String {
        format!("
            File path: {},
            Timestamp: {:?},
            Tags: {},
            Word Count: {}
        ", self.path.display(), self.get_timestamp().unwrap(), self.get_tags().unwrap().join(", "), self.word_count())
        
    }
    fn get_timestamp(&self) -> Option<DateTime<chrono::Utc>> {
        match self.frontmatter.get("unix") {
            Some(serde_yaml::Value::Number(n)) => DateTime::from_timestamp(n.as_i64().unwrap(), 0),
            _ => None
        }
    }

    fn write_to_file(&self) {
        let text = format!("---\n{}\n---\n{}", serde_yaml::to_string(&self.frontmatter).unwrap(), self.content);
        fs::write(&self.path, text).unwrap();
    }

    fn get_tags(&self) -> Option<Vec<String>> {
        match self.frontmatter.get("tags") {
            Some(serde_yaml::Value::Sequence(arr)) => {
            Some(arr.iter().map(|v| v.as_str().unwrap().to_string()).collect())
            },
            _ => None
        }
    }

    fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }
}

pub fn parse_note(path: PathBuf) -> Note {
    let text = match fs::read_to_string(&path){
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

