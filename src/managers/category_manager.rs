use crate::models::{Category, MetronError, Result};
use crate::storage::Storage;

pub struct CategoryManager<'a> {
    storage: &'a mut Storage,
}

impl<'a> CategoryManager<'a> {
    pub fn new(storage: &'a mut Storage) -> Self {
        Self { storage }
    }

    pub fn create_category(&mut self, name: String, quota: u32) -> Result<()> {
        let data = self.storage.get_data_mut();
        
        // Check if category already exists
        if data.categories.iter().any(|c| c.name == name) {
            return Err(MetronError::DuplicateName);
        }

        // Check if adding this quota would exceed total weekly quota
        if let Some(total_quota) = data.total_weekly_quota {
            let current_total: u32 = data.categories.iter().map(|c| c.category_weekly_quota).sum();
            if current_total + quota > total_quota {
                return Err(MetronError::QuotaExceeded);
            }
        }

        let category = Category {
            name: name.clone(),
            category_weekly_quota: quota,
        };

        data.categories.push(category);
        self.storage.save()?;

        println!("✓ Created category '{}' with {}h/week quota", name, quota);
        Ok(())
    }

    pub fn list_categories(&self) -> Result<()> {
        let data = self.storage.get_data();
        
        if data.categories.is_empty() {
            println!("No categories found.");
            return Ok(());
        }

        println!("Categories:");
        println!("{:<20} {:<15}", "Name", "Weekly Quota");
        println!("{}", "-".repeat(35));
        
        for category in &data.categories {
            println!("{:<20} {:<15}h", category.name, category.category_weekly_quota);
        }

        // Show total quota info
        let total_used: u32 = data.categories.iter().map(|c| c.category_weekly_quota).sum();
        if let Some(total_quota) = data.total_weekly_quota {
            println!("{}", "-".repeat(35));
            println!("Total used: {}h / {}h", total_used, total_quota);
        } else {
            println!("{}", "-".repeat(35));
            println!("Total used: {}h (no total quota set)", total_used);
        }

        Ok(())
    }

    pub fn update_category(&mut self, name: String, quota: u32) -> Result<()> {
        let data = self.storage.get_data_mut();
        
        // Check quota limits first
        if let Some(total_quota) = data.total_weekly_quota {
            let other_quotas: u32 = data.categories.iter()
                .filter(|c| c.name != name)
                .map(|c| c.category_weekly_quota)
                .sum();
            
            if other_quotas + quota > total_quota {
                return Err(MetronError::QuotaExceeded);
            }
        }

        // Find and update the category
        let category = data.categories.iter_mut()
            .find(|c| c.name == name)
            .ok_or(MetronError::CategoryNotFound)?;

        let old_quota = category.category_weekly_quota;
        category.category_weekly_quota = quota;
        
        self.storage.save()?;

        println!("✓ Updated category '{}' quota: {}h → {}h", name, old_quota, quota);
        Ok(())
    }

    pub fn delete_category(&mut self, name: String) -> Result<()> {
        let data = self.storage.get_data_mut();
        
        let index = data.categories.iter()
            .position(|c| c.name == name)
            .ok_or(MetronError::CategoryNotFound)?;

        // Check if any sessions use this category
        let sessions_using_category = data.sessions.iter()
            .any(|s| s.category == name);
        
        if sessions_using_category {
            println!("Warning: Category '{}' is used by existing sessions.", name);
            println!("Delete anyway? (y/N)");
            
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            
            if !input.trim().to_lowercase().starts_with('y') {
                println!("Deletion cancelled.");
                return Ok(());
            }
        }

        data.categories.remove(index);
        self.storage.save()?;

        println!("✓ Deleted category '{}'", name);
        Ok(())
    }
}
