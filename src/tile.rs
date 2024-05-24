use std::cmp;

pub trait Tile {
    fn get_primary_x(&self, params: &Params, index: u32) -> i32;

    fn get_primary_y(&self, params: &Params, index: u32) -> i32;

    fn get_primary_width(&self, params: &Params, index: u32) -> u32;

    fn get_primary_height(&self, params: &Params, index: u32) -> u32;

    fn get_stack_x(&self, params: &Params, index: u32) -> i32;

    fn get_stack_y(&self, params: &Params, index: u32) -> i32;

    fn get_stack_width(&self, params: &Params, index: u32) -> u32;

    fn get_stack_height(&self, params: &Params, index: u32) -> u32;

    fn get_main(&self) -> u32;

    fn get_x(&self, params: &Params, index: u32) -> i32 {
        if index < self.get_main() {
            self.get_primary_x(params, index)
        } else {
            self.get_stack_x(params, index)
        }
    }

    fn get_y(&self, params: &Params, index: u32) -> i32 {
        if index < self.get_main() {
            self.get_primary_y(params, index)
        } else {
            self.get_stack_y(params, index)
        }
    }

    fn get_width(&self, params: &Params, index: u32) -> u32 {
        if index < self.get_main() {
            self.get_primary_width(params, index)
        } else {
            self.get_stack_width(params, index)
        }
    }

    fn get_height(&self, params: &Params, index: u32) -> u32 {
        if index < self.get_main() {
            self.get_primary_height(params, index)
        } else {
            self.get_stack_height(params, index)
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TileType {
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Clone, Debug)]
pub struct Params {
    pub view_count: u32,
    pub usable_width: u32,
    pub usable_height: u32,
}

impl Params {
    pub fn with_view_count(&self, view_count: u32) -> Params {
        Params {
            view_count,
            ..*self
        }
    }
}

pub struct LeftPrimary {
    inner: u32,
    outer: u32,
    ratio: u32,
    main: u32,
    dim: i32,
}

impl LeftPrimary {
    pub fn new(inner: u32, outer: u32, ratio: u32, main: u32, dim: i32) -> LeftPrimary {
        LeftPrimary {
            inner,
            outer,
            ratio,
            main,
            dim,
        }
    }

    fn get_center(&self, usable_width: u32) -> u32 {
        (usable_width * self.ratio) / 100
    }

    fn subtract_gaps(&self, count: u32, params: &Params) -> u32 {
        params.usable_height - (self.inner * count * 2) - self.outer * 2
    }

    fn get_height(&self, count: u32, params: &Params) -> u32 {
        self.subtract_gaps(count, params) / count
    }

    fn get_diminished_height(&self, index: u32, count: u32, params: &Params) -> u32 {
        if self.dim == 0 || count == 1 {
            return self.get_height(count, params);
        };

        let total = self.subtract_gaps(count, params);
        let stolen = (total * self.dim.unsigned_abs()) / 100;
        let base = (total - stolen) / count;

        if self.dim > 0 {
            base + LeftPrimary::diminish(stolen, count - index - 1, count)
        } else {
            base + LeftPrimary::diminish(stolen, index, count)
        }
    }

    // find the nth term in 1x + 4x + 16x + 64x ... = 1
    fn diminish(size: u32, index: u32, total: u32) -> u32 {
        // there can be some rounding error in here; compensate in the last element
        if index == total - 1 {
            let prev_sum: u32 = (0..total - 1).map(|i| Self::diminish(size, i, total)).sum();
            return size - prev_sum;
        }

        let total_parts = ((4_u32.pow(total) - 1) / (4 - 1)) as f32; // how many Xs are there?
        let x = 1_f32 / total_parts;
        let n = 4_u32.pow(index) as f32;
        let ratio = n * x;

        ((size as f32) * ratio).round() as u32
    }
}

impl Tile for LeftPrimary {
    fn get_main(&self) -> u32 {
        self.main
    }

    fn get_primary_x(&self, _params: &Params, _index: u32) -> i32 {
        (self.outer + self.inner) as i32
    }

    fn get_primary_y(&self, params: &Params, index: u32) -> i32 {
        let height = self.get_primary_height(params, index);
        let y = self.outer + self.inner + index * (self.inner * 2 + height);

        y as i32
    }

    fn get_primary_width(&self, params: &Params, _index: u32) -> u32 {
        if params.view_count <= self.main {
            return params.usable_width - self.inner * 2 - self.outer * 2;
        }

        self.get_center(params.usable_width) - self.inner * 2 - self.outer
    }

    fn get_primary_height(&self, params: &Params, _index: u32) -> u32 {
        self.get_height(cmp::min(params.view_count, self.main), params)
    }

    fn get_stack_x(&self, params: &Params, _index: u32) -> i32 {
        (self.get_center(params.usable_width) + self.inner) as i32
    }

    fn get_stack_y(&self, params: &Params, index: u32) -> i32 {
        let mut y = self.outer + self.inner;

        for i in self.main..index {
            y += self.get_stack_height(params, i);
            y += self.inner * 2;
        }

        y as i32
    }

    fn get_stack_width(&self, params: &Params, _index: u32) -> u32 {
        (params.usable_width - self.get_center(params.usable_width)) - self.inner * 2 - self.outer
    }

    fn get_stack_height(&self, params: &Params, index: u32) -> u32 {
        self.get_diminished_height(index - self.main, params.view_count - self.main, params)
    }
}

pub struct Flipped {
    wrapped: Box<dyn Tile>,
}

pub fn flip(wrapped: Box<dyn Tile>) -> Box<dyn Tile> {
    Box::new(Flipped::new(wrapped))
}

impl Flipped {
    pub fn new(wrapped: Box<dyn Tile>) -> Flipped {
        Flipped { wrapped }
    }
}

impl Tile for Flipped {
    fn get_main(&self) -> u32 {
        self.wrapped.get_main()
    }

    fn get_primary_x(&self, params: &Params, index: u32) -> i32 {
        params.usable_width as i32
            - self.wrapped.get_primary_x(params, index)
            - self.wrapped.get_primary_width(params, index) as i32
    }

    fn get_primary_y(&self, params: &Params, index: u32) -> i32 {
        self.wrapped.get_primary_y(params, index)
    }

    fn get_primary_width(&self, params: &Params, index: u32) -> u32 {
        self.wrapped.get_primary_width(params, index)
    }

    fn get_primary_height(&self, params: &Params, index: u32) -> u32 {
        self.wrapped.get_primary_height(params, index)
    }

    fn get_stack_x(&self, params: &Params, _index: u32) -> i32 {
        self.wrapped.get_primary_x(params, 0)
    }

    fn get_stack_y(&self, params: &Params, index: u32) -> i32 {
        self.wrapped.get_stack_y(params, index)
    }

    fn get_stack_width(&self, params: &Params, index: u32) -> u32 {
        self.wrapped.get_stack_width(params, index)
    }

    fn get_stack_height(&self, params: &Params, index: u32) -> u32 {
        self.wrapped.get_stack_height(params, index)
    }
}

pub struct Rotated {
    wrapped: Box<dyn Tile>,
}

pub fn rotate(wrapped: Box<dyn Tile>) -> Box<dyn Tile> {
    Box::new(Rotated::new(wrapped))
}

impl Rotated {
    pub fn new(wrapped: Box<dyn Tile>) -> Rotated {
        Rotated { wrapped }
    }

    fn translate(params: &Params) -> Params {
        Params {
            view_count: params.view_count,
            usable_width: params.usable_height,
            usable_height: params.usable_width,
        }
    }
}

impl Tile for Rotated {
    fn get_main(&self) -> u32 {
        self.wrapped.get_main()
    }

    fn get_primary_x(&self, params: &Params, index: u32) -> i32 {
        self.wrapped
            .get_primary_y(&Rotated::translate(params), index)
    }

    fn get_primary_y(&self, params: &Params, index: u32) -> i32 {
        self.wrapped
            .get_primary_x(&Rotated::translate(params), index)
    }

    fn get_primary_width(&self, params: &Params, index: u32) -> u32 {
        self.wrapped
            .get_primary_height(&Rotated::translate(params), index)
    }

    fn get_primary_height(&self, params: &Params, index: u32) -> u32 {
        self.wrapped
            .get_primary_width(&Rotated::translate(params), index)
    }

    fn get_stack_x(&self, params: &Params, index: u32) -> i32 {
        self.wrapped.get_stack_y(&Rotated::translate(params), index)
    }

    fn get_stack_y(&self, params: &Params, index: u32) -> i32 {
        self.wrapped.get_stack_x(&Rotated::translate(params), index)
    }

    fn get_stack_width(&self, params: &Params, index: u32) -> u32 {
        self.wrapped
            .get_stack_height(&Rotated::translate(params), index)
    }

    fn get_stack_height(&self, params: &Params, index: u32) -> u32 {
        self.wrapped
            .get_stack_width(&Rotated::translate(params), index)
    }
}

pub struct Padded {
    wrapped: Box<dyn Tile>,
    h_pad: i32,
    v_pad: i32,
}

impl Padded {
    pub fn new(wrapped: Box<dyn Tile>, h_pad: i32, v_pad: i32) -> Padded {
        Padded {
            wrapped,
            h_pad,
            v_pad,
        }
    }

    fn translate(&self, params: &Params) -> Params {
        // what are we subtracting from the width and height? (can be negative)
        let sub_w = self.h_pad * 2;
        let sub_h = self.v_pad * 2;

        // turn useable width and height into i32s
        let w = params.usable_width as i32;
        let h = params.usable_height as i32;

        // do the check, and do nothing if invalid
        if sub_w >= w || sub_h >= h {
            return params.clone();
        }

        // now we know we can survive the cast back to u32
        Params {
            view_count: params.view_count,
            usable_width: (w - sub_w) as u32,
            usable_height: (h - sub_h) as u32,
        }
    }
}

impl Tile for Padded {
    fn get_main(&self) -> u32 {
        self.wrapped.get_main()
    }

    fn get_primary_x(&self, params: &Params, index: u32) -> i32 {
        self.wrapped.get_primary_x(&self.translate(params), index) + self.h_pad
    }

    fn get_primary_y(&self, params: &Params, index: u32) -> i32 {
        self.wrapped.get_primary_y(&self.translate(params), index) + self.v_pad
    }

    fn get_primary_width(&self, params: &Params, index: u32) -> u32 {
        self.wrapped
            .get_primary_width(&self.translate(params), index)
    }

    fn get_primary_height(&self, params: &Params, index: u32) -> u32 {
        self.wrapped
            .get_primary_height(&self.translate(params), index)
    }

    fn get_stack_x(&self, params: &Params, index: u32) -> i32 {
        self.wrapped.get_stack_x(&self.translate(params), index) + self.h_pad
    }

    fn get_stack_y(&self, params: &Params, index: u32) -> i32 {
        self.wrapped.get_stack_y(&self.translate(params), index) + self.v_pad
    }

    fn get_stack_width(&self, params: &Params, index: u32) -> u32 {
        self.wrapped.get_stack_width(&self.translate(params), index)
    }

    fn get_stack_height(&self, params: &Params, index: u32) -> u32 {
        self.wrapped
            .get_stack_height(&self.translate(params), index)
    }
}

pub struct Monocle {
    wrapped: Box<dyn Tile>,
}

impl Monocle {
    pub fn new(wrapped: Box<dyn Tile>) -> Monocle {
        Monocle { wrapped }
    }
}

impl Tile for Monocle {
    fn get_main(&self) -> u32 {
        self.wrapped.get_main()
    }

    fn get_primary_x(&self, params: &Params, _index: u32) -> i32 {
        self.wrapped.get_primary_x(&params.with_view_count(1), 0)
    }

    fn get_primary_y(&self, params: &Params, _index: u32) -> i32 {
        self.wrapped.get_primary_y(&params.with_view_count(1), 0)
    }

    fn get_primary_width(&self, params: &Params, _index: u32) -> u32 {
        self.wrapped
            .get_primary_width(&params.with_view_count(1), 0)
    }

    fn get_primary_height(&self, params: &Params, _index: u32) -> u32 {
        self.wrapped
            .get_primary_height(&params.with_view_count(1), 0)
    }

    fn get_stack_x(&self, params: &Params, _: u32) -> i32 {
        self.wrapped.get_primary_x(&params.with_view_count(1), 0)
    }

    fn get_stack_y(&self, params: &Params, _: u32) -> i32 {
        self.wrapped.get_primary_y(&params.with_view_count(1), 0)
    }

    fn get_stack_width(&self, params: &Params, _: u32) -> u32 {
        self.wrapped
            .get_primary_width(&params.with_view_count(1), 0)
    }

    fn get_stack_height(&self, params: &Params, _: u32) -> u32 {
        self.wrapped
            .get_primary_height(&params.with_view_count(1), 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_diminishes() {
        assert_eq!(
            (0..5)
                .map(|i| LeftPrimary::diminish(1000, i, 5))
                .sum::<u32>(),
            1000
        );
    }
}
