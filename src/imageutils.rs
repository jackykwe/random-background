use image::{DynamicImage, Pixel, Rgba, RgbaImage};
use rusttype::{Font, Scale};

/// Courtesy of and inspired from https://github.com/image-rs/imageproc/issues/479#issuecomment-991778692
pub fn draw_text_with_border_mut<'a>(
    canvas: &mut RgbaImage,
    color: Rgba<u8>,
    x: i32,
    y: i32,
    scale: Scale,
    font: &'a Font<'a>,
    text: &str,
    outline_color: Rgba<u8>,
    outline_width: u8,
) {
    let mut t_image = DynamicImage::new_luma8(canvas.width(), canvas.height());
    imageproc::drawing::draw_text_mut(&mut t_image, Rgba([255; 4]), x, y, scale, font, text);
    let mut t_image = t_image.to_luma8();

    imageproc::morphology::dilate_mut(
        &mut t_image,
        imageproc::distance_transform::Norm::LInf,
        outline_width,
    );

    // Add a border to the text.
    for x in 0..t_image.width() {
        for y in 0..t_image.height() {
            let pixval = 255 - t_image.get_pixel(x, y).0[0];
            if pixval != 255 {
                // i.e. image2.get_pixel(x, y).0[0] is not 0 (black)
                canvas.get_pixel_mut(x, y).blend(&outline_color);
            }
        }
    }
    imageproc::drawing::draw_text_mut(canvas, color, x, y, scale, font, text);
}
