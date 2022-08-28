pub enum Console {
    World = 0,
    Ui = 1,
}

impl Into<usize> for Console {
    fn into(self) -> usize {
        self as usize
    }
}

pub enum Layer {
    Entities = 0,
}

impl Into<usize> for Layer {
    fn into(self) -> usize {
        self as usize
    }
}

// pub const CONSOLE_WORLD: usize = 0;
// pub const CONSOLE_UI: usize = 1;

// pub const LAYER_ENTITIES: usize = 0;
