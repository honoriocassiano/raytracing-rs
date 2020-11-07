use super::Timestamp;

pub struct Interval {
    start: Timestamp,
    end: Timestamp,
}

impl Interval {
    pub fn new(start: Timestamp, end: Timestamp) -> Interval {
        Interval { start, end }
    }

    pub fn start(self) -> Timestamp {
        self.start
    }

    pub fn end(self) -> Timestamp {
        self.end
    }
}
