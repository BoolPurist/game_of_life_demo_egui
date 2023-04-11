use std::{fmt::Display, time::Duration};

#[derive(Clone, Copy, Debug)]
pub enum TimeUnit {
    Seconds(u32),
    MsSeconds(u32),
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seconds(secs) | Self::MsSeconds(secs) => write!(f, "{}", secs),
        }
    }
}

impl From<TimeUnit> for Duration {
    fn from(value: TimeUnit) -> Self {
        match value {
            TimeUnit::Seconds(secs) => Duration::from_secs(secs as u64),
            TimeUnit::MsSeconds(ms_secs) => Duration::from_millis(ms_secs as u64),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedTime {
    Seconds,
    MsSeconds,
}
impl Default for SelectedTime {
    fn default() -> Self {
        Self::Seconds
    }
}
impl Display for SelectedTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seconds => write!(f, "Seconds"),
            Self::MsSeconds => write!(f, "Mili seconds"),
        }
    }
}
