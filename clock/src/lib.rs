const DAY: i32 = 24 * 60; // 一天的分钟数

/**
 * 将时间规范化到一天中[0,DAY)
 */
fn get_normalized_minutes(mut minutes: i32) -> i32 {
    ((minutes % DAY) + DAY) % DAY
}

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minutes: get_normalized_minutes(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minutes: get_normalized_minutes(self.minutes + minutes),
        }
    }
}

// to_string()
impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / 60, self.minutes % 60)
    }
}
