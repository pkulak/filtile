use crate::{
    parse::{parse_command, Command, Operation},
    tile::TileType,
};

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

static ALL: &str = "all";

pub struct ConfigStorage {
    tag_list: Vec<ConfigEntry>,
    default: Config,
}

#[derive(Debug)]
struct ConfigEntry {
    tags: u32,
    output: String,
    config: Config,
}

impl ConfigEntry {
    fn matches_exact(&self, tags: u32, output: &str) -> bool {
        self.tags == tags && self.output == output
    }

    fn matches_output(&self, output: &str) -> bool {
        self.output == output && self.tags == 0
    }

    fn matches_tags(&self, tags: u32) -> bool {
        self.tags == tags && self.output == ALL
    }

    fn matches_any(&self) -> bool {
        self.output == ALL && self.tags == 0
    }
}

impl ConfigStorage {
    pub fn new(default: Config) -> ConfigStorage {
        ConfigStorage {
            tag_list: Vec::new(),
            default,
        }
    }

    pub fn retrieve(&self, tags: u32, output: &str) -> &Config {
        // narrow it down
        let filtered: Vec<&ConfigEntry> = self
            .tag_list
            .iter()
            .filter(|e| {
                e.matches_exact(tags, output)
                    || e.matches_output(output)
                    || e.matches_tags(tags)
                    || e.matches_any()
            })
            .collect();

        if let Some(entry) = filtered.iter().find(|e| e.matches_exact(tags, output)) {
            return &entry.config;
        }

        if let Some(entry) = filtered.iter().find(|e| e.matches_output(output)) {
            return &entry.config;
        }

        if let Some(entry) = filtered.iter().find(|e| e.matches_tags(tags)) {
            return &entry.config;
        }

        if let Some(entry) = filtered.iter().find(|e| e.matches_any()) {
            return &entry.config;
        }

        &self.default
    }

    fn store(&mut self, tags: u32, output: &str, config: Config) {
        // remove if exists
        self.tag_list.retain(|e| !e.matches_exact(tags, output));

        // and store it
        self.tag_list.push(ConfigEntry {
            tags,
            output: output.to_string(),
            config,
        });
    }

    pub fn apply(&mut self, tags: u32, output: &str, f: impl Fn(&mut Config)) {
        let filtered: Vec<&mut ConfigEntry> = self
            .tag_list
            .iter_mut()
            .filter(|e| {
                if tags == 0 && output == ALL {
                    true
                } else if tags == 0 {
                    e.output == output
                } else if output == ALL {
                    e.tags == tags
                } else {
                    false
                }
            })
            .collect();

        // apply to everything existing
        for entry in filtered {
            f(&mut entry.config);
        }

        // make a whole new config and apply there as well
        let mut config = self.retrieve(tags, output).clone();
        f(&mut config);
        self.store(tags, output, config);
    }

    pub fn apply_cmd(&mut self, tags: u32, output: &str, cmd: &str) {
        self.apply(tags, output, |config| match parse_command(cmd) {
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
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::ALL, tile::TileType};

    use super::{Config, ConfigStorage};

    #[test]
    fn it_applies_configs() {
        let mut storage = ConfigStorage::new(Config::new());

        // apply something very specific
        storage.apply(32, "HD-1", |c| c.ratio = 9);

        assert_eq!(1, storage.tag_list.len());
        assert_eq!(9, storage.tag_list[0].config.ratio);

        // now apply for tag 32, and all outputs, which should modify our
        // existing config
        storage.apply(32, ALL, |c| c.inner = 42);

        assert_eq!(2, storage.tag_list.len());
        assert_eq!(9, storage.tag_list[0].config.ratio);
        assert_eq!(42, storage.tag_list[0].config.inner);
        assert_eq!(42, storage.tag_list[1].config.inner);

        // set a new default (which will also modify everything else)
        storage.apply(0, ALL, |c| c.ratio = 8);

        assert_eq!(3, storage.tag_list.len());
        assert_eq!(8, storage.tag_list[0].config.ratio);
        assert_eq!(8, storage.tag_list[1].config.ratio);
        assert_eq!(8, storage.tag_list[2].config.ratio);
    }

    #[test]
    fn it_combines_configs() {
        let mut storage = ConfigStorage::new(Config::new());

        // use padding on the whole monitor
        storage.apply(0, "HD-1", |c| c.pad = true);

        // but change a couple things on tag 1
        storage.apply(1, "HD-1", |c| c.ratio = 80);
        storage.apply(1, "HD-1", |c| c.tile = TileType::Right);

        // grab a config on tag 6
        let config = storage.retrieve(32, "HD-1");

        assert!(config.pad);
        assert_eq!(config.ratio, 55);
        assert_eq!(config.tile, TileType::Left);

        // and then 1
        let config = storage.retrieve(1, "HD-1");

        assert!(config.pad);
        assert_eq!(config.ratio, 80);
        assert_eq!(config.tile, TileType::Right);
    }
}
