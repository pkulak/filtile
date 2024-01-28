pub trait Tile {
    fn get_primary_x(&self, params: &Params) -> i32;

    fn get_primary_y(&self, params: &Params) -> i32;

    fn get_primary_width(&self, params: &Params) -> u32;

    fn get_primary_height(&self, params: &Params) -> u32;

    fn get_stack_x(&self, params: &Params, index: u32) -> i32;

    fn get_stack_y(&self, params: &Params, index: u32) -> i32;

    fn get_stack_width(&self, params: &Params, index: u32) -> u32;

    fn get_stack_height(&self, params: &Params, index: u32) -> u32;
}

#[derive(PartialEq, Clone)]
pub enum TileType {
    LeftPrimary,
    RightPrimary,
}

pub struct Params {
    pub view_count: u32,
    pub usable_width: u32,
    pub usable_height: u32,
}

pub struct LeftPrimary {
    inner: u32,
    outer: u32,
    ratio: u32,
}

impl LeftPrimary {
    pub fn new(inner: u32, outer: u32, ratio: u32) -> LeftPrimary {
        LeftPrimary {
            inner,
            outer,
            ratio,
        }
    }

    fn get_center(&self, usable_width: u32) -> u32 {
        (usable_width * self.ratio) / 100
    }
}

impl Tile for LeftPrimary {
    fn get_primary_x(&self, _params: &Params) -> i32 {
        (self.outer + self.inner) as i32
    }

    fn get_primary_y(&self, _params: &Params) -> i32 {
        (self.inner + self.outer) as i32
    }

    fn get_primary_width(&self, params: &Params) -> u32 {
        if params.view_count == 1 {
            return params.usable_width - self.inner * 2 - self.outer * 2;
        }

        self.get_center(params.usable_width) - self.inner * 2 - self.outer
    }

    fn get_primary_height(&self, params: &Params) -> u32 {
        params.usable_height - self.inner * 2 - self.outer * 2
    }

    fn get_stack_x(&self, params: &Params, _index: u32) -> i32 {
        (self.get_center(params.usable_width) + self.inner) as i32
    }

    fn get_stack_y(&self, params: &Params, index: u32) -> i32 {
        assert!(index > 0);

        let height = self.get_stack_height(params, index);
        let y = self.outer + self.inner + (index - 1) * (self.inner * 2 + height);

        y as i32
    }

    fn get_stack_width(&self, params: &Params, _index: u32) -> u32 {
        (params.usable_width - self.get_center(params.usable_width)) - self.inner * 2 - self.outer
    }

    fn get_stack_height(&self, params: &Params, _index: u32) -> u32 {
        let stack_count = params.view_count - 1;
        let minus_gaps = params.usable_height - (self.inner * stack_count * 2) - self.outer * 2;
        minus_gaps / stack_count
    }
}

pub struct RightPrimary {
    wrapped: LeftPrimary,
}

impl RightPrimary {
    pub fn new(inner: u32, outer: u32, ratio: u32) -> RightPrimary {
        RightPrimary {
            wrapped: LeftPrimary::new(inner, outer, ratio),
        }
    }

    fn get_center(&self, usable_width: u32) -> u32 {
        (usable_width * (100 - self.wrapped.ratio)) / 100
    }
}

impl Tile for RightPrimary {
    fn get_primary_x(&self, params: &Params) -> i32 {
        if params.view_count == 1 {
            return self.wrapped.get_primary_x(params);
        }

        (self.get_center(params.usable_width) + self.wrapped.inner) as i32
    }

    fn get_primary_y(&self, params: &Params) -> i32 {
        self.wrapped.get_primary_y(params)
    }

    fn get_primary_width(&self, params: &Params) -> u32 {
        self.wrapped.get_primary_width(params)
    }

    fn get_primary_height(&self, params: &Params) -> u32 {
        self.wrapped.get_primary_height(params)
    }

    fn get_stack_x(&self, params: &Params, _index: u32) -> i32 {
        self.wrapped.get_primary_x(params)
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

pub struct PaddedPrimary {
    wrapped: Box<dyn Tile>,
}

impl PaddedPrimary {
    pub fn new(wrapped: Box<dyn Tile>) -> PaddedPrimary {
        PaddedPrimary { wrapped }
    }

    // the primary width, pretending that there is more than one view
    fn primary_width(&self, params: &Params) -> u32 {
        let fake_params = Params {
            view_count: 2,
            ..*params
        };

        self.wrapped.get_primary_width(&fake_params)
    }
}

impl Tile for PaddedPrimary {
    fn get_primary_x(&self, params: &Params) -> i32 {
        if params.view_count == 1 {
            return ((params.usable_width - self.primary_width(params)) / 2) as i32;
        }

        self.wrapped.get_primary_x(params)
    }

    fn get_primary_y(&self, params: &Params) -> i32 {
        self.wrapped.get_primary_y(params)
    }

    fn get_primary_width(&self, params: &Params) -> u32 {
        if params.view_count == 1 {
            return self.primary_width(params);
        }

        self.wrapped.get_primary_width(params)
    }

    fn get_primary_height(&self, params: &Params) -> u32 {
        self.wrapped.get_primary_height(params)
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
