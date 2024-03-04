mod config;
mod parse;
mod tile;

use config::ConfigStorage;
use parse::{parse_output, parse_tags, split_commands};
use river_layout_toolkit::{run, GeneratedLayout, Layout, Rectangle};
use std::{convert::Infallible, env, iter};
use tile::{flip, rotate, LeftPrimary, Monocle, Padded, Params, Tile, TileType};

fn main() {
    let mut layout = FilTile {
        tag_log: TagLog::new(),
        configs: ConfigStorage::new(),
    };

    let all_args: Vec<String> = env::args().collect();
    let call_string = all_args[1..].join(" ").trim().to_string();

    if !call_string.is_empty() {
        let _ = layout.user_cmd(call_string, None, "all");
    }

    run(layout).unwrap();
}

struct FilTile {
    tag_log: TagLog,
    configs: ConfigStorage,
}

impl Layout for FilTile {
    type Error = Infallible;

    const NAMESPACE: &'static str = "filtile";

    fn user_cmd(
        &mut self,
        cmd: String,
        tags: Option<u32>,
        output: &str,
    ) -> Result<(), Self::Error> {
        if let Some(t) = tags {
            self.tag_log.record_tags(t);
        }

        let output = match output {
            "all" => None,
            _ => Some(output),
        };

        let (cmd, cdr) = split_commands(&cmd);

        let tags = match parse_tags(cmd) {
            Some(t) => Some(t),
            None => self.tag_log.last_tag,
        };

        let output = match parse_output(cmd) {
            Some(o) => Some(o),
            None => output,
        };

        self.configs.apply_cmd(tags, output, cmd);

        if let Some(remaining) = cdr {
            let _ = self.user_cmd(remaining.to_string(), None, output.unwrap_or("all"));
        }

        Ok(())
    }

    fn generate_layout(
        &mut self,
        view_count: u32,
        usable_width: u32,
        usable_height: u32,
        tags: u32,
        output: &str,
    ) -> Result<GeneratedLayout, Self::Error> {
        self.tag_log.record_tags(tags);

        let config = self.configs.build(self.tag_log.last_tag, Some(output));

        let params = Params {
            view_count,
            usable_width,
            usable_height,
        };

        let base: Box<dyn Tile> = Box::new(LeftPrimary::new(
            config.inner,
            config.outer,
            config.ratio,
            config.main,
        ));

        let mut name = match config.tile {
            TileType::Left => "←",
            TileType::Top => "↑",
            TileType::Right => "→",
            TileType::Bottom => "↓",
        };

        let mut tile = match config.tile {
            TileType::Left => base,
            TileType::Top => rotate(base),
            TileType::Right => flip(base),
            TileType::Bottom => rotate(flip(base)),
        };

        // monocle
        if config.monocle {
            tile = Box::new(Monocle::new(tile));
            name = "M";

        // or single-stack centering
        } else if config.pad && view_count <= config.main {
            if config.tile == TileType::Left || config.tile == TileType::Right {
                let center = (usable_width * config.ratio) / 100;
                let pad = (usable_width - center) / 2;

                tile = Box::new(Padded::new(tile, pad as i32, 0));
            } else {
                let center = (usable_height * config.ratio) / 100;
                let pad = (usable_height - center) / 2;

                tile = Box::new(Padded::new(tile, 0, pad as i32));
            }

        // or, smart gaps
        } else if view_count == 1 && (config.smart_h.is_some() || config.smart_v.is_some()) {
            let existing = (config.inner + config.outer) as i32;

            let transform = |i: Option<u32>| i.map(|i| i as i32).unwrap_or(existing) - existing;

            let h = transform(config.smart_h);
            let v = transform(config.smart_v);

            tile = Box::new(Padded::new(tile, h, v));
        }

        let mut layout = GeneratedLayout {
            layout_name: name.to_string(),
            views: Vec::with_capacity(view_count as usize),
        };

        for index in 0..view_count {
            layout.views.push(Rectangle {
                x: tile.get_x(&params, index),
                y: tile.get_y(&params, index),
                width: tile.get_width(&params, index),
                height: tile.get_height(&params, index),
            });
        }

        Ok(layout)
    }
}

// Keep track of the last "single" tag we see, so that we can store and
// recall configs not based on combinations.
struct TagLog {
    pub last_tag: Option<u32>,
    single_tags: Vec<u32>,
}

impl TagLog {
    pub fn new() -> TagLog {
        TagLog {
            last_tag: None,
            single_tags: (0..31).map(|i| 1 << i).chain(iter::once(0)).collect(),
        }
    }

    pub fn record_tags(&mut self, tag: u32) {
        if self.single_tags.contains(&tag) {
            self.last_tag = Some(tag);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TagLog;

    #[test]
    fn it_logs_single_tags() {
        let mut log = TagLog::new();

        log.record_tags(512);
        log.record_tags(14);
        log.record_tags(12);

        assert_eq!(Some(512), log.last_tag);
    }
}
