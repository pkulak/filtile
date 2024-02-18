use crate::{
    parse::{parse_command, Command, Operation},
    tile::TileType,
};

#[derive(PartialEq)]
enum ConfigValue {
    Inner(u32),
    Outer(u32),
    Ratio(u32),
    Main(u32),
    Tile(TileType),
    Pad(bool),
    Monocle(bool),
    SmartH(Option<u32>),
    SmartV(Option<u32>),
}

#[derive(PartialEq)]
struct ConfigEntry {
    output: Option<String>,
    tags: Option<u32>,
    value: ConfigValue,
}

pub struct ConfigStorage {
    entries: Vec<ConfigEntry>,
}

impl ConfigStorage {
    pub fn new() -> ConfigStorage {
        ConfigStorage {
            entries: Vec::new(),
        }
    }

    pub fn build(&self, tags: Option<u32>, output: Option<&str>) -> Config {
        let mut config = Config::new();

        for e in &self.entries {
            if (e.output.as_deref() == output || e.output.is_none())
                && (e.tags == tags || e.tags.is_none())
            {
                match e.value {
                    ConfigValue::Inner(v) => config.inner = v,
                    ConfigValue::Outer(v) => config.outer = v,
                    ConfigValue::Ratio(v) => config.ratio = v,
                    ConfigValue::Main(v) => config.main = v,
                    ConfigValue::Tile(v) => config.tile = v,
                    ConfigValue::Pad(v) => config.pad = v,
                    ConfigValue::Monocle(v) => config.monocle = v,
                    ConfigValue::SmartH(v) => config.smart_h = v,
                    ConfigValue::SmartV(v) => config.smart_v = v,
                }
            }
        }

        config
    }

    fn add(&mut self, entry: ConfigEntry) {
        // get rid of any dupes
        self.entries.retain(|e| e != &entry);

        // then add our new value
        self.entries.push(entry);
    }

    fn apply(&mut self, tags: Option<u32>, output: Option<&str>, config: &Config) {
        let existing = self.build(tags, output);

        let make_entry = |value: ConfigValue| ConfigEntry {
            tags,
            output: output.map(|o| o.to_string()),
            value,
        };

        if existing.inner != config.inner {
            self.add(make_entry(ConfigValue::Inner(config.inner)));
        }

        if existing.outer != config.outer {
            self.add(make_entry(ConfigValue::Outer(config.outer)));
        }

        if existing.ratio != config.ratio {
            self.add(make_entry(ConfigValue::Ratio(config.ratio)));
        }

        if existing.main != config.main {
            self.add(make_entry(ConfigValue::Main(config.main)));
        }

        if existing.tile != config.tile {
            self.add(make_entry(ConfigValue::Tile(config.tile)));
        }

        if existing.pad != config.pad {
            self.add(make_entry(ConfigValue::Pad(config.pad)));
        }

        if existing.monocle != config.monocle {
            self.add(make_entry(ConfigValue::Monocle(config.monocle)));
        }

        if existing.smart_h != config.smart_h {
            self.add(make_entry(ConfigValue::SmartH(config.smart_h)));
        }

        if existing.smart_v != config.smart_v {
            self.add(make_entry(ConfigValue::SmartV(config.smart_v)));
        }
    }

    #[cfg(test)]
    fn apply_with(&mut self, tags: Option<u32>, output: Option<&str>, f: impl Fn(&mut Config)) {
        let mut config = self.build(tags, output);
        f(&mut config);
        self.apply(tags, output, &config);
    }

    pub fn apply_cmd(&mut self, tags: Option<u32>, output: Option<&str>, cmd: &str) {
        let mut config = self.build(tags, output);

        match parse_command(cmd) {
            Command::Single("flip") => match config.tile {
                TileType::Left => config.tile = TileType::Right,
                TileType::Top => config.tile = TileType::Bottom,
                TileType::Right => config.tile = TileType::Left,
                TileType::Bottom => config.tile = TileType::Top,
            },
            Command::Single("pad") => {
                config.pad = !config.pad;
            }
            Command::Single("monocle") => {
                config.monocle = !config.monocle;
            }
            Command::Textual {
                namespace: "main-location",
                value: "left",
            } => config.tile = TileType::Left,
            Command::Textual {
                namespace: "main-location",
                value: "top",
            } => config.tile = TileType::Top,
            Command::Textual {
                namespace: "main-location",
                value: "right",
            } => config.tile = TileType::Right,
            Command::Textual {
                namespace: "main-location",
                value: "bottom",
            } => config.tile = TileType::Bottom,
            Command::Textual {
                namespace: "pad",
                value: "on",
            } => config.pad = true,
            Command::Textual {
                namespace: "pad",
                value: "off",
            } => config.pad = false,
            Command::Textual {
                namespace: "monocle",
                value: "on",
            } => config.monocle = true,
            Command::Textual {
                namespace: "monocle",
                value: "off",
            } => config.monocle = false,
            Command::Textual {
                namespace: "smart-padding",
                value: "off",
            } => {
                config.smart_h = None;
                config.smart_v = None
            }
            Command::Numeric {
                namespace: "view-padding",
                operation,
                value,
            } => match operation {
                Operation::Add => config.inc_inner(value),
                Operation::Subtract => config.dec_inner(value),
                Operation::Set => config.set_inner(value),
            },
            Command::Numeric {
                namespace: "outer-padding",
                operation,
                value,
            } => match operation {
                Operation::Add => config.inc_outer(value),
                Operation::Subtract => config.dec_outer(value),
                Operation::Set => config.set_outer(value),
            },
            Command::Numeric {
                namespace: "smart-padding",
                operation,
                value,
            } => match operation {
                Operation::Add => {
                    config.inc_smart_h(value);
                    config.inc_smart_v(value)
                }
                Operation::Subtract => {
                    config.dec_smart_h(value);
                    config.dec_smart_v(value)
                }
                Operation::Set => {
                    config.set_smart_h(value);
                    config.set_smart_v(value)
                }
            },
            Command::Numeric {
                namespace: "smart-padding-h",
                operation,
                value,
            } => match operation {
                Operation::Add => config.inc_smart_h(value),
                Operation::Subtract => config.dec_smart_h(value),
                Operation::Set => config.set_smart_h(value),
            },
            Command::Numeric {
                namespace: "smart-padding-v",
                operation,
                value,
            } => match operation {
                Operation::Add => config.inc_smart_v(value),
                Operation::Subtract => config.dec_smart_v(value),
                Operation::Set => config.set_smart_v(value),
            },
            Command::Numeric {
                namespace: "main-ratio",
                operation,
                value,
            } => match operation {
                Operation::Add => config.inc_ratio(value),
                Operation::Subtract => config.dec_ratio(value),
                Operation::Set => config.set_ratio(value),
            },
            Command::Numeric {
                namespace: "main-count",
                operation,
                value,
            } => match operation {
                Operation::Add => config.inc_main(value),
                Operation::Subtract => config.dec_main(value),
                Operation::Set => config.set_main(value),
            },
            _ => println!("invalid command {}", cmd),
        };

        self.apply(tags, output, &config)
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub inner: u32,
    pub outer: u32,
    pub ratio: u32,
    pub main: u32,
    pub tile: TileType,
    pub pad: bool,
    pub monocle: bool,
    pub smart_h: Option<u32>,
    pub smart_v: Option<u32>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            inner: 6,
            outer: 6,
            ratio: 55,
            main: 1,
            tile: TileType::Left,
            pad: false,
            monocle: false,
            smart_h: None,
            smart_v: None,
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

    pub fn inc_smart_h(&mut self, value: u32) {
        self.smart_h = match self.smart_h {
            Some(v) => Some(Config::ranged_inc(v, value, 1024)),
            None => Some(Config::ranged_inc(self.inner + self.outer, value, 1024)),
        }
    }

    pub fn inc_smart_v(&mut self, value: u32) {
        self.smart_v = match self.smart_v {
            Some(v) => Some(Config::ranged_inc(v, value, 1024)),
            None => Some(Config::ranged_inc(self.inner + self.outer, value, 1024)),
        }
    }

    pub fn inc_ratio(&mut self, value: u32) {
        self.ratio = Config::ranged_inc(self.ratio, value, 90);
    }

    pub fn inc_main(&mut self, value: u32) {
        self.main = Config::ranged_inc(self.main, value, 16);
    }

    pub fn dec_inner(&mut self, value: u32) {
        self.inner = Config::ranged_dec(self.inner, value, 0);
    }

    pub fn dec_outer(&mut self, value: u32) {
        self.outer = Config::ranged_dec(self.outer, value, 0);
    }

    pub fn dec_smart_h(&mut self, value: u32) {
        self.smart_h = match self.smart_h {
            Some(v) => Some(Config::ranged_dec(v, value, 0)),
            None => Some(Config::ranged_dec(self.inner + self.outer, value, 0)),
        }
    }

    pub fn dec_smart_v(&mut self, value: u32) {
        self.smart_v = match self.smart_v {
            Some(v) => Some(Config::ranged_dec(v, value, 0)),
            None => Some(Config::ranged_dec(self.inner + self.outer, value, 0)),
        }
    }

    pub fn dec_ratio(&mut self, value: u32) {
        self.ratio = Config::ranged_dec(self.ratio, value, 10);
    }

    pub fn dec_main(&mut self, value: u32) {
        self.main = Config::ranged_dec(self.main, value, 1);
    }

    pub fn set_inner(&mut self, value: u32) {
        self.inner = Config::ranged_set(value, 0, 1024);
    }

    pub fn set_outer(&mut self, value: u32) {
        self.outer = Config::ranged_set(value, 0, 1024);
    }

    pub fn set_smart_h(&mut self, value: u32) {
        self.smart_h = Some(Config::ranged_set(value, 0, 1024))
    }

    pub fn set_smart_v(&mut self, value: u32) {
        self.smart_v = Some(Config::ranged_set(value, 0, 1024))
    }

    pub fn set_ratio(&mut self, value: u32) {
        self.ratio = Config::ranged_set(value, 10, 90);
    }

    pub fn set_main(&mut self, value: u32) {
        self.main = Config::ranged_set(value, 1, 16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_applies_configs() {
        let mut storage = ConfigStorage::new();

        // apply a new default
        storage.apply_with(None, None, |c| {
            c.inner = 32;
            c.tile = TileType::Top
        });

        assert_eq!(storage.build(None, None).inner, 32);
        assert_eq!(storage.build(None, None).tile, TileType::Top);

        assert_eq!(storage.build(Some(32), Some("HD-1")).inner, 32);
        assert_eq!(storage.build(Some(32), Some("HD-1")).tile, TileType::Top);

        // now a single tag
        storage.apply_with(Some(32), None, |c| c.inner = 16);

        assert_eq!(storage.build(Some(32), Some("HD-1")).inner, 16);
        assert_eq!(storage.build(Some(32), None).inner, 16);

        // now a single monitor
        storage.apply_with(None, Some("HD-1"), |c| c.inner = 8);

        assert_eq!(storage.build(Some(32), Some("HD-1")).inner, 8);
        assert_eq!(storage.build(None, Some("HD-1")).inner, 8);

        // finally, override everything we monitor-tag combination
        storage.apply_with(Some(32), Some("HD-1"), |c| c.inner = 4);

        assert_eq!(storage.build(Some(32), Some("HD-1")).inner, 4);
        assert_eq!(storage.build(Some(4), Some("HD-1")).inner, 8);
    }
}
