use crate::tile::TileType;

#[derive(Clone, Debug)]
pub struct Config {
    pub inner: u32,
    pub outer: u32,
    pub ratio: u32,
    pub tile: TileType,
    pub pad: bool,
    pub monocle: bool,
}

impl Config {
    pub fn new() -> Config {
        Config {
            inner: 6,
            outer: 6,
            ratio: 55,
            tile: TileType::LeftPrimary,
            pad: false,
            monocle: false,
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

static ALL: &str = "all";

// Is it a bit silly to do this instead of throwing them in a map and calling
// it a day? Yup! But, in my defense, storing complex objects in a map causes
// all kinds of issues that this avoids.
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
        storage.apply(1, "HD-1", |c| c.tile = TileType::RightPrimary);

        // grab a config on tag 6
        let config = storage.retrieve(32, "HD-1");

        assert!(config.pad);
        assert_eq!(config.ratio, 55);
        assert_eq!(config.tile, TileType::LeftPrimary);

        // and then 1
        let config = storage.retrieve(1, "HD-1");

        assert!(config.pad);
        assert_eq!(config.ratio, 80);
        assert_eq!(config.tile, TileType::RightPrimary);
    }
}
