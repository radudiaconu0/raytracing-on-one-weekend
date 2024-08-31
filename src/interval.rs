pub struct Interval {
    pub min: f64,
    pub max: f64,
}

const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: -f64::INFINITY,
};

const UNIVERSE: Interval = Interval {
    min: -f64::INFINITY,
    max: f64::INFINITY,
};

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn default() -> Self {
        Self {
            min: -f64::INFINITY,
            max: f64::INFINITY,
        }
    }
    pub fn size(&self) -> f64 {
        self.max - self.min
    }
    pub fn expand(&mut self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
