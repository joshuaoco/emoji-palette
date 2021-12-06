use image::{imageops, DynamicImage, Rgb, RgbImage};
use imageproc::drawing::{draw_filled_circle_mut, draw_hollow_circle_mut};

// The main palette drawing function, returns a square canvas with the pallete image drawn
pub fn draw_palette(
    height: u32,
    color_1: &EmojiColor,
    color_2: &EmojiColor,
    color_3: &EmojiColor,
) -> RgbImage {
    let mut canvas = RgbImage::from_pixel(height, height, Rgb([255, 255, 255]));

    let (c1, c2, c3) = palette_circle_positions(height as i32);

    // Draw circles
    draw_filled_circle_mut(&mut canvas, (c1.x, c1.y), c1.radius, color_1.color);
    draw_filled_circle_mut(&mut canvas, (c2.x, c2.y), c2.radius, color_2.color);
    draw_filled_circle_mut(&mut canvas, (c3.x, c3.y), c3.radius, color_3.color);

    // Create blended colors
    let blend_0_1 = blend(&color_1.color, &color_2.color);
    let blend_0_2 = blend(&color_1.color, &color_3.color);
    let blend_1_2 = blend(&color_2.color, &color_3.color);
    let blend_0_1_2 = blend2(&color_1.color, &color_2.color, &color_3.color);

    // Draw colors in the intersects of each color pair
    for (x, y, pixel) in canvas.enumerate_pixels_mut() {
        if c1.contains(x, y) && c2.contains(x, y) && c3.contains(x, y) {
            *pixel = blend_0_1_2;
        } else if c1.contains(x, y) && c2.contains(x, y) {
            *pixel = blend_0_1;
            // color mix of c1, c2
        } else if c1.contains(x, y) && c3.contains(x, y) {
            // color mix of c1, c3
            *pixel = blend_0_2;
        } else if c2.contains(x, y) && c3.contains(x, y) {
            // color mix of c2, c3
            *pixel = blend_1_2;
        }
    }

    // Draw borders
    let black = Rgb([0, 0, 0]);
    draw_hollow_circle_mut(&mut canvas, (c1.x, c1.y), c1.radius, black);
    draw_hollow_circle_mut(&mut canvas, (c2.x, c2.y), c2.radius, black);
    draw_hollow_circle_mut(&mut canvas, (c3.x, c3.y), c3.radius, black);

    canvas
}

// Some simple geometry used to choose how to colour the palette
struct CircleLocation {
    x: i32,
    y: i32,
    radius: i32,
}

impl CircleLocation {
    fn contains(&self, x: u32, y: u32) -> bool {
        // Returns true if x/y lies within circle
        (x as i32 - self.x).pow(2) + (y as i32 - self.y).pow(2) < self.radius.pow(2)
        // from equation of a circle, see Euclid 3
    }
}

fn palette_circle_positions(height: i32) -> (CircleLocation, CircleLocation, CircleLocation) {
    // Assumes a square image
    // Overlapping circles are 60%, this is purely what looks natural rather than anything formal
    let radius = (height as f32 * 0.3) as i32;

    (
        CircleLocation {
            x: radius,
            y: radius,
            radius,
        },
        CircleLocation {
            x: height - radius,
            y: radius,
            radius,
        },
        CircleLocation {
            x: height / 2,
            y: height - radius,
            radius,
        },
    )
}

#[derive(Debug)]
pub struct EmojiColor {
    image: DynamicImage,
    color: Rgb<u8>,
}

impl From<DynamicImage> for EmojiColor {
    fn from(emoji_image: DynamicImage) -> Self {
        let image = emoji_image.resize(1, 1, imageops::FilterType::CatmullRom);
        let color = image.to_rgb8().get_pixel(0, 0).to_owned();

        EmojiColor { image, color }
    }
}

fn blend(c1: &Rgb<u8>, c2: &Rgb<u8>) -> Rgb<u8> {
    let r = (c1.0[0] as u32 + c2.0[0] as u32) / 2;
    let g = (c1.0[1] as u32 + c2.0[1] as u32) / 2;
    let b = (c1.0[2] as u32 + c2.0[2] as u32) / 2;

    Rgb([r as u8, g as u8, b as u8])
}

fn blend2(c1: &Rgb<u8>, c2: &Rgb<u8>, c3: &Rgb<u8>) -> Rgb<u8> {
    let c1_2 = blend(c1, c2);
    blend(&c1_2, c3)
}
