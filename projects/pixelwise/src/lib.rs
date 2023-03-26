use crate::pixel16::Pixelwise16;

mod pixel16;

fn trig(x: isize, y: isize) -> isize {
    if x != 0 && y != 0 {
        return (x % y) & (y % x);
    };
    0
}

#[test]
fn test() {
    let renderer = Pixelwise16::gray(
        |i, j| trig(),
    );
    renderer.render().save("test1.png").unwrap();
    let renderer = Pixelwise16::gray(
        |i, j| if i != 0 && j != 0 {
            (i % j) & (j % i)
        } else {
            0
        },
    );
    renderer.render().save("test2.png").unwrap();

    let black = Pixelwise16::gray(|i, j| {
        let s = 3.0 / (j + 99) as f32;
        (((i + 1024) as f32 * s + j as f32 * s).floor() as i32 % 2 + ((1024 * 2 - i) as f32 * s + j as f32 * s).floor() as i32 % 2 * 127) as u16
    });
    black.render().save("black.png").unwrap();
}