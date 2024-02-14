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

#[derive(PartialEq, Clone, Debug)]
pub enum TileType {
    Left,
    Top,
    Right,
    Bottom,
}

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
}

impl LeftPrimary {
    pub fn new(inner: u32, outer: u32, ratio: u32, main: u32) -> LeftPrimary {
        LeftPrimary {
            inner,
            outer,
            ratio,
            main,
        }
    }

    fn get_center(&self, usable_width: u32) -> u32 {
        (usable_width * self.ratio) / 100
    }

    fn get_height(&self, count: u32, params: &Params) -> u32 {
        let minus_gaps = params.usable_height - (self.inner * count * 2) - self.outer * 2;
        minus_gaps / count
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
        let height = self.get_stack_height(params, index);
        let y = self.outer + self.inner + (index - self.main) * (self.inner * 2 + height);

        y as i32
    }

    fn get_stack_width(&self, params: &Params, _index: u32) -> u32 {
        (params.usable_width - self.get_center(params.usable_width)) - self.inner * 2 - self.outer
    }

    fn get_stack_height(&self, params: &Params, _index: u32) -> u32 {
        self.get_height(params.view_count - self.main, params)
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
    h_pad: u32,
    v_pad: u32,
}

impl Padded {
    pub fn new(wrapped: Box<dyn Tile>, h_pad: u32, v_pad: u32) -> Padded {
        Padded {
            wrapped,
            h_pad,
            v_pad,
        }
    }

    fn translate(&self, params: &Params) -> Params {
        Params {
            view_count: params.view_count,
            usable_width: params.usable_width - self.h_pad * 2,
            usable_height: params.usable_height - self.v_pad * 2,
        }
    }
}

impl Tile for Padded {
    fn get_main(&self) -> u32 {
        self.wrapped.get_main()
    }

    fn get_primary_x(&self, params: &Params, index: u32) -> i32 {
        if params.view_count <= self.wrapped.get_main() {
            self.wrapped.get_primary_x(&self.translate(params), index) + self.h_pad as i32
        } else {
            self.wrapped.get_primary_x(params, index)
        }
    }

    fn get_primary_y(&self, params: &Params, index: u32) -> i32 {
        if params.view_count <= self.wrapped.get_main() {
            self.wrapped.get_primary_y(&self.translate(params), index) + self.v_pad as i32
        } else {
            self.wrapped.get_primary_y(params, index)
        }
    }

    fn get_primary_width(&self, params: &Params, index: u32) -> u32 {
        if params.view_count <= self.wrapped.get_main() {
            self.wrapped
                .get_primary_width(&self.translate(params), index)
        } else {
            self.wrapped.get_primary_width(params, index)
        }
    }

    fn get_primary_height(&self, params: &Params, index: u32) -> u32 {
        if params.view_count <= self.wrapped.get_main() {
            self.wrapped
                .get_primary_height(&self.translate(params), index)
        } else {
            self.wrapped.get_primary_height(params, index)
        }
    }

    fn get_stack_x(&self, params: &Params, index: u32) -> i32 {
        self.wrapped.get_stack_x(params, index)
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
