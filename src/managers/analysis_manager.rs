use crate::models::Result;
use crate::storage::Storage;
use chrono::{DateTime, Utc, Duration, Datelike};
use std::collections::HashMap;

pub struct AnalysisManager<'a> {
    storage: &'a Storage,
}

impl<'a> AnalysisManager<'a> {
    pub fn new(storage: &'a Storage) -> Self {
        Self { storage }
    }

    pub fn generate_analysis(&self, period: String, category_filter: Option<String>) -> Result<()> {
        let data = self.storage.get_data();
        
        if data.sessions.is_empty() {
            println!("No sessions found for analysis.");
            return Ok(());
        }

        let now = Utc::now();
        let filtered_sessions: Vec<_> = data.sessions.iter()
            .filter(|s| self.is_in_period(s.start, &period, now))
            .filter(|s| {
                if let Some(ref cat_filter) = category_filter {
                    &s.category == cat_filter
                } else {
                    true
                }
            })
            .collect();

        if filtered_sessions.is_empty() {
            println!("No sessions found for the specified period and filter.");
            return Ok(());
        }

        // Group sessions by category
        let mut by_category: HashMap<String, Vec<_>> = HashMap::new();
        for session in &filtered_sessions {
            by_category.entry(session.category.clone()).or_default().push(session);
        }

        println!("📊 Analysis Report - {}", period.to_uppercase());
        if let Some(ref cat) = category_filter {
            println!("   Category: {}", cat);
        }
        println!("{}", "=".repeat(60));

        let mut total_work_time = 0u32;
        let mut total_overtime = 0u32;

        for (category_name, sessions) in &by_category {
            let category = data.categories.iter().find(|c| &c.name == category_name);
            let weekly_quota = category.map(|c| c.category_weekly_quota).unwrap_or(0);
            
            let total_minutes: u32 = sessions.iter().map(|s| s.duration).sum();
            let total_hours = total_minutes as f64 / 60.0;
            
            // Calculate work time vs overtime based on weekly quota
            let quota_minutes = weekly_quota * 60;
            let work_time_minutes = if quota_minutes > 0 {
                std::cmp::min(total_minutes, quota_minutes)
            } else {
                total_minutes
            };
            let overtime_minutes = if total_minutes > quota_minutes && quota_minutes > 0 {
                total_minutes - quota_minutes
            } else {
                0
            };

            total_work_time += work_time_minutes;
            total_overtime += overtime_minutes;

            println!("\n📁 Category: {}", category_name);
            println!("   Sessions: {}", sessions.len());
            println!("   Total Time: {:.1}h ({} minutes)", total_hours, total_minutes);
            if quota_minutes > 0 {
                println!("   Weekly Quota: {}h", weekly_quota);
                println!("   Work Time: {:.1}h ({} minutes)", work_time_minutes as f64 / 60.0, work_time_minutes);
                if overtime_minutes > 0 {
                    println!("   Overtime: {:.1}h ({} minutes)", overtime_minutes as f64 / 60.0, overtime_minutes);
                }
            }
            
            // Show tag breakdown
            let mut tag_minutes: HashMap<String, u32> = HashMap::new();
            for session in sessions {
                for tag in &session.tags {
                    *tag_minutes.entry(tag.clone()).or_default() += session.duration;
                }
            }
            
            if !tag_minutes.is_empty() {
                println!("   Tags:");
                for (tag, minutes) in &tag_minutes {
                    println!("     - {}: {:.1}h", tag, *minutes as f64 / 60.0);
                }
            }
        }

        println!("\n{}", "=".repeat(60));
        println!("📈 SUMMARY");
        println!("   Total Worktime: {:.1}h ({} minutes)", total_worktime as f64 / 60.0, total_worktime);
        if total_overtime > 0 {
            println!("   Total Overtime: {:.1}h ({} minutes)", total_overtime as f64 / 60.0, total_overtime);
        }
        println!("   Grand Total: {:.1}h ({} minutes)", (total_worktime + total_overtime) as f64 / 60.0, total_worktime + total_overtime);
        println!("   Sessions: {}", filtered_sessions.len());

        Ok(())
    }

    fn is_in_period(&self, session_start: DateTime<Utc>, period: &str, now: DateTime<Utc>) -> bool {
        match period.to_lowercase().as_str() {
            "day" | "daily" => {
                session_start.date_naive() == now.date_naive()
            },
            "week" | "weekly" => {
                let week_start = now - Duration::days(now.weekday().num_days_from_monday() as i64);
                session_start >= week_start && session_start <= now
            },
            "month" | "monthly" => {
                session_start.year() == now.year() && session_start.month() == now.month()
            },
            "year" | "yearly" => {
                session_start.year() == now.year()
            },
            _ => {
                eprintln!("Unknown period '{}', using weekly", period);
                let week_start = now - Duration::days(now.weekday().num_days_from_monday() as i64);
                session_start >= week_start && session_start <= now
            }
        }
    }
}
