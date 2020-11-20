use super::Timestamp;

#[derive(Copy, Clone)]
pub struct Interval {
    start: Timestamp,
    end: Timestamp,
}

impl Interval {
    pub const fn new(start: Timestamp, end: Timestamp) -> Self {
        Self { start, end }
    }

    pub const fn start(&self) -> Timestamp {
        self.start
    }

    pub const fn end(&self) -> Timestamp {
        self.end
    }
}
