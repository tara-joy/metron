use crate::models::{Session, MetronError, Result};
use crate::storage::Storage;
use chrono::Utc;
use uuid::Uuid;

pub struct SessionManager<'a> {
    storage: &'a mut Storage,
}

impl<'a> SessionManager<'a> {
    pub fn new(storage: &'a mut Storage) -> Self {
        Self { storage }
    }

    pub fn start_session(&mut self, title: String, category: String, tags: Vec<String>, duration: u32) -> Result<()> {
        let data = self.storage.get_data_mut();
        
        // Validate duration is multiple of 15
        if duration == 0 || duration % 15 != 0 {
            return Err(MetronError::InvalidDuration);
        }

        // Check if category exists
        if !data.categories.iter().any(|c| c.name == category) {
            return Err(MetronError::CategoryNotFound);
        }

        // Check if all tags exist
        for tag in &tags {
            if !data.tags.iter().any(|t| t.name == *tag) {
                return Err(MetronError::TagNotFound);
            }
        }

        let now = Utc::now();
        let end_time = now + chrono::Duration::minutes(duration as i64);
        
        let session = Session {
            id: Uuid::new_v4().to_string(),
            title: title.clone(),
            category: category.clone(),
            tags: tags.clone(),
            start: now,
            end: Some(end_time),
            duration,
        };

        data.sessions.push(session);
        self.storage.save()?;

        println!("✓ Started session '{}' in category '{}' for {} minutes", title, category, duration);
        if !tags.is_empty() {
            println!("  Tags: {}", tags.join(", "));
        }
        println!("  Session will end at: {}", end_time.format("%H:%M:%S"));

        Ok(())
    }

    pub fn end_session(&mut self, id: String) -> Result<()> {
        let data = self.storage.get_data_mut();
        
        let session = data.sessions.iter_mut()
            .find(|s| s.id == id)
            .ok_or(MetronError::SessionNotFound)?;

        let now = Utc::now();
        let actual_duration = now.signed_duration_since(session.start).num_minutes() as u32;
        
        // Round down to nearest 15 minutes
        let rounded_duration = (actual_duration / 15) * 15;
        
        session.end = Some(now);
        session.duration = rounded_duration;
        
        self.storage.save()?;

        if rounded_duration < actual_duration {
            println!("✓ Session ended early. Duration rounded down: {}min → {}min", actual_duration, rounded_duration);
        } else {
            println!("✓ Session completed: {}min", rounded_duration);
        }

        Ok(())
    }

    pub fn list_sessions(&self) -> Result<()> {
        let data = self.storage.get_data();
        
        if data.sessions.is_empty() {
            println!("No sessions found.");
            return Ok(());
        }

        println!("Sessions:");
        println!("{:<8} {:<25} {:<15} {:<10} {:<20} {:<20}", "ID", "Title", "Category", "Duration", "Start", "Tags");
        println!("{}", "-".repeat(120));
        
        for session in &data.sessions {
            let short_id = &session.id[..8];
            let start_time = session.start.format("%Y-%m-%d %H:%M");
            let tags_str = if session.tags.is_empty() { 
                "-".to_string() 
            } else { 
                session.tags.join(", ") 
            };
            
            println!("{:<8} {:<25} {:<15} {:<10}min {:<20} {:<20}", 
                short_id, 
                session.title, 
                session.category, 
                session.duration,
                start_time,
                tags_str
            );
        }

        println!("{}", "-".repeat(120));
        println!("Total sessions: {}", data.sessions.len());

        Ok(())
    }

    pub fn delete_session(&mut self, id: String) -> Result<()> {
        let data = self.storage.get_data_mut();
        
        // Try to find by full ID first, then by partial ID
        let index = data.sessions.iter()
            .position(|s| s.id == id || s.id.starts_with(&id))
            .ok_or(MetronError::SessionNotFound)?;

        let session = &data.sessions[index];
        println!("Deleting session: '{}'", session.title);
        println!("Are you sure? (y/N)");
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Deletion cancelled.");
            return Ok(());
        }

        data.sessions.remove(index);
        self.storage.save()?;

        println!("✓ Session deleted");
        Ok(())
    }
}
