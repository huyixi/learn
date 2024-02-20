use chrono::prelude::*;

impl DateTime<Utc> {
    fn to_string(&self) -> String {
        self.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
