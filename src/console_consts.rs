pub enum Console {
    World = 0,
    Ui = 1,
    Overlays = 2,
}

impl From<Console> for usize {
    fn from(console: Console) -> Self {
        console as usize
    }
}

pub enum Layer {
    Entities = 0,
}

impl From<Layer> for usize {
    fn from(layer: Layer) -> Self {
        layer as usize
    }
}

// pub const CONSOLE_WORLD: usize = 0;
// pub const CONSOLE_UI: usize = 1;

// pub const LAYER_ENTITIES: usize = 0;
