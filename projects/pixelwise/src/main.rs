pub struct Pixelwise {
    pub w: usize,
    pub h: usize,
    pub r: fn(u8, u8) -> u8,
    pub g: fn(u8, u8) -> u8,
    pub b: fn(u8, u8) -> u8,
    pub a: fn(u8, u8) -> u8,
}

impl Default for Pixelwise {}
