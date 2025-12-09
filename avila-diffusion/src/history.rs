//! Persistent storage for generation history entries.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: u64,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_prompt: Option<String>,
    pub width: usize,
    pub height: usize,
    pub steps: usize,
    pub guidance: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    pub time_taken: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    pub timestamp_ms: u64,
    pub image_data: String,
    pub mode: String,
}

#[derive(Debug)]
pub struct HistoryStore {
    entries: Vec<HistoryEntry>,
    capacity: usize,
    path: PathBuf,
}

impl HistoryStore {
    pub fn load<P: Into<PathBuf>>(path: P, capacity: usize) -> Self {
        let capacity = capacity.max(1);
        let path = path.into();

        let mut entries: Vec<HistoryEntry> = fs::read_to_string(&path)
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default();

        if entries.len() > capacity {
            entries.truncate(capacity);
        }

        Self {
            entries,
            capacity,
            path,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn add_entry(&mut self, entry: HistoryEntry) -> bool {
        self.entries.insert(0, entry);
        let trimmed = self.entries.len() > self.capacity;
        if trimmed {
            self.entries.truncate(self.capacity);
        }

        if let Err(err) = self.persist() {
            eprintln!("⚠️  Failed to persist history: {}", err);
        }

        trimmed
    }

    pub fn clear(&mut self) -> bool {
        if self.entries.is_empty() {
            return false;
        }

        self.entries.clear();
        if let Err(err) = self.persist() {
            eprintln!("⚠️  Failed to persist history: {}", err);
        }

        true
    }

    fn persist(&self) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(&self.entries)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string()))?;

        fs::write(&self.path, json)
    }
}
