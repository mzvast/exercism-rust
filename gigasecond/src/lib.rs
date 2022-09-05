use time::{ext::NumericalDuration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // unimplemented!("What time is a gigasecond later than {}", start);
    // 先转成seconds
    // 然后增加1^9s
    // 再转回去

    start.saturating_add(1e9.seconds())
}
