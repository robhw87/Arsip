pub mod error;
pub use error::ClockError;
use time::macros::format_description;
use time::{Date, Duration, OffsetDateTime, PrimitiveDateTime, UtcOffset};

#[derive(Clone, Debug)]
pub struct AppClock {
    offset: UtcOffset,
}

impl Default for AppClock {
    fn default() -> Self {
        Self {
            offset: UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
        }
    }
}

impl AppClock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn now(&self) -> OffsetDateTime {
        OffsetDateTime::now_utc().to_offset(self.offset)
    }

    pub fn curdate(&self) -> Date {
        self.now().date()
    }

    pub fn today(&self) -> String {
        self.date_str(0)
    }
    pub fn timestamp(&self) -> String {
        self.ts_str(0)
    }
    pub fn parse_local_str(&self, date_str: &str) -> Result<OffsetDateTime, ClockError> {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let primitive = PrimitiveDateTime::parse(date_str, &format)?;
        Ok(primitive.assume_offset(self.offset))
    }

    pub fn date_str(&self, delta: i64) -> String {
        let format = format_description!("[year]-[month]-[day]");
        let date_stringify = self.curdate() + Duration::days(delta);
        date_stringify
            .format(&format)
            .unwrap_or_else(|_| "1970-01-01".to_string())
    }
    pub fn ts_str(&self, delta: i64) -> String {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");
        let date_stringify = self.now() + Duration::days(delta);
        date_stringify
            .format(&format)
            .unwrap_or_else(|_| "1970-01-01T00:00:00".to_string())
    }
}
