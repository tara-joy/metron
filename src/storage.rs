use std::fs;
use std::path::Path;
use crate::models::{MetronData, MetronError, Result};

pub struct Storage {
    file_path: String,
    data: MetronData,
}

impl Storage {
    pub fn new(file_path: &str) -> Result<Self> {
        let data = if Path::new(file_path).exists() {
            let contents = fs::read_to_string(file_path)
                .map_err(|e| MetronError::StorageError(e.to_string()))?;
            
            serde_json::from_str(&contents)
                .map_err(|e| MetronError::StorageError(e.to_string()))?
        } else {
            MetronData::new()
        };

        Ok(Self {
            file_path: file_path.to_string(),
            data,
        })
    }

    pub fn get_data(&self) -> &MetronData {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut MetronData {
        &mut self.data
    }

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.data)
            .map_err(|e| MetronError::StorageError(e.to_string()))?;
        
        fs::write(&self.file_path, json)
            .map_err(|e| MetronError::StorageError(e.to_string()))?;
        
        Ok(())
    }
}
