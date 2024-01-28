use crate::tile::TileType;

#[derive(Clone)]
pub struct Config {
    pub inner: u32,
    pub outer: u32,
    pub ratio: u32,
    pub tile: TileType,
    pub pad: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            inner: 6,
            outer: 6,
            ratio: 55,
            tile: TileType::LeftPrimary,
            pad: false,
        }
    }

    fn ranged_inc(existing: u32, value: u32, max: u32) -> u32 {
        if existing + value > max {
            return max;
        }
        existing + value
    }

    fn ranged_dec(existing: u32, value: u32, min: u32) -> u32 {
        if value > existing - min {
            return min;
        }
        existing - value
    }

    fn ranged_set(value: u32, min: u32, max: u32) -> u32 {
        if value < min {
            return min;
        }

        if value > max {
            return max;
        }

        value
    }

    pub fn inc_inner(&mut self, value: u32) {
        self.inner = Config::ranged_inc(self.inner, value, 1024);
    }

    pub fn inc_outer(&mut self, value: u32) {
        self.outer = Config::ranged_inc(self.outer, value, 1024);
    }

    pub fn inc_ratio(&mut self, value: u32) {
        self.ratio = Config::ranged_inc(self.ratio, value, 90);
    }

    pub fn dec_inner(&mut self, value: u32) {
        self.inner = Config::ranged_dec(self.inner, value, 0);
    }

    pub fn dec_outer(&mut self, value: u32) {
        self.outer = Config::ranged_dec(self.outer, value, 0);
    }

    pub fn dec_ratio(&mut self, value: u32) {
        self.ratio = Config::ranged_dec(self.ratio, value, 10);
    }

    pub fn set_inner(&mut self, value: u32) {
        self.inner = Config::ranged_set(value, 0, 1024);
    }

    pub fn set_outer(&mut self, value: u32) {
        self.outer = Config::ranged_set(value, 0, 1024);
    }

    pub fn set_ratio(&mut self, value: u32) {
        self.ratio = Config::ranged_set(value, 10, 90);
    }
}
