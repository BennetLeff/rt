

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new() -> Self {
        Self {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }

    pub fn with_values(_min: f32, _max: f32) -> Self {
        Self {
            min: _min,
            max: _max,
        }
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}

pub const EMPTY: Interval = Interval {
    min: f32::INFINITY,
    max: f32::NEG_INFINITY,
};

pub const UNIVERSE: Interval = Interval {
    min: f32::NEG_INFINITY,
    max: f32::INFINITY,
};