use image::RgbaImage;

pub struct Pixelwise16 {
    pub w: u32,
    pub h: u32,
    pub r: fn(isize, isize) -> isize,
    pub g: fn(isize, isize) -> isize,
    pub b: fn(isize, isize) -> isize,
    pub a: fn(isize, isize) -> isize,
}

impl Default for Pixelwise16 {
    fn default() -> Self {
        Pixelwise16 {
            w: 1024,
            h: 1024,
            r: |_, _| 255,
            g: |_, _| 255,
            b: |_, _| 255,
            a: |_, _| 255,
        }
    }
}

impl Pixelwise16 {
    pub fn gray(gray: fn(isize, isize) -> isize) -> Self {
        Pixelwise16 {
            r: gray,
            g: gray,
            b: gray,
            ..Default::default()
        }
    }
    pub fn rgb(r: fn(isize, isize) -> isize, g: fn(isize, isize) -> isize, b: fn(isize, isize) -> isize) -> Self {
        Pixelwise16 {
            r,
            g,
            b,
            ..Default::default()
        }
    }
    pub fn render(&self) -> RgbaImage {
        let mut pixels = RgbaImage::new(self.w as u32, self.h as u32);
        for x in 0..self.w {
            for y in 0..self.h {
                let r = (self.r)(x, y) as u8;
                let g = (self.g)(x, y) as u8;
                let b = (self.b)(x, y) as u8;
                let a = (self.a)(x, y) as u8;
                pixels.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
            }
        }
        pixels
    }
}
