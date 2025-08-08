use crate::models::{Tag, MetronError, Result};
use crate::storage::Storage;

pub struct TagManager<'a> {
    storage: &'a mut Storage,
}

impl<'a> TagManager<'a> {
    pub fn new(storage: &'a mut Storage) -> Self {
        Self { storage }
    }

    pub fn create_tag(&mut self, name: String) -> Result<()> {
        let data = self.storage.get_data_mut();
        
        // Check if tag already exists
        if data.tags.iter().any(|t| t.name == name) {
            return Err(MetronError::DuplicateName);
        }

        // Check tag limit (max 7 tags)
        if data.tags.len() >= 7 {
            return Err(MetronError::TagLimitExceeded);
        }

        let tag = Tag {
            name: name.clone(),
        };

        data.tags.push(tag);
        self.storage.save()?;

        println!("✓ Created tag '{}'", name);
        Ok(())
    }

    pub fn list_tags(&self) -> Result<()> {
        let data = self.storage.get_data();
        
        if data.tags.is_empty() {
            println!("No tags found.");
            return Ok(());
        }

        println!("Tags ({}/7):", data.tags.len());
        for (i, tag) in data.tags.iter().enumerate() {
            println!("{}. {}", i + 1, tag.name);
        }

        Ok(())
    }

    pub fn delete_tag(&mut self, name: String) -> Result<()> {
        let data = self.storage.get_data_mut();
        
        let index = data.tags.iter()
            .position(|t| t.name == name)
            .ok_or(MetronError::TagNotFound)?;

        // Check if any sessions use this tag
        let sessions_using_tag = data.sessions.iter()
            .any(|s| s.tags.contains(&name));
        
        if sessions_using_tag {
            println!("Warning: Tag '{}' is used by existing sessions.", name);
            println!("Delete anyway? (y/N)");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            
            if !input.trim().to_lowercase().starts_with('y') {
                println!("Deletion cancelled.");
                return Ok(());
            }
        }

        data.tags.remove(index);
        self.storage.save()?;

        println!("✓ Deleted tag '{}'", name);
        Ok(())
    }
}
