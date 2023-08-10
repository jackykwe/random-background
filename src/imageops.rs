use std::path::PathBuf;

use anyhow::{anyhow, bail, Context};
use image::{imageops::FilterType, DynamicImage, Pixel, Rgba, RgbaImage};
use imageproc::drawing::text_size;
use rand::seq::IteratorRandom;
use rusttype::{Font, Scale};

use crate::{
    config::Config,
    countdown::{
        generate_today_string, get_countdown_str, get_font_fill_colour, get_font_stroke_colour,
    },
    imageutils::draw_text_with_border_mut,
};

pub(crate) const SCREEN_WIDTH_PX: u32 = 1920;
pub(crate) const SCREEN_HEIGHT_PX: u32 = 1080;

pub(crate) fn choose_one_image(dir_path: &str) -> anyhow::Result<String> {
    let all_paths = std::fs::read_dir(dir_path)
        .with_context(|| format!("Directory {} does not exist", dir_path))?;
    let image_paths = all_paths
        .filter_map(|path| path.ok())
        .filter(|path| path.metadata().is_ok_and(|metadata| metadata.is_file()));
    image_paths
        .choose(&mut rand::thread_rng())
        .map(|e| e.path().display().to_string())
        .ok_or(anyhow!("Directory {} is empty", dir_path))
}

/// No cropping is done!
/// Numerical operations should be sound: overflows won't happen in the range we're handling.
fn resize_to_contain_screen(img: DynamicImage) -> DynamicImage {
    let width = img.width();
    let height = img.height();
    log::debug!("Image original width={}, height={}", width, height);
    if width as f64 / height as f64 <= SCREEN_WIDTH_PX as f64 / SCREEN_HEIGHT_PX as f64 {
        // in terms of aspect ratio, image too tall; fit width to SCREEN_WIDTH_PX
        log::debug!("Resizing: image too tall, fitting width to SCREEN_WIDTH_PX");
        if width == SCREEN_WIDTH_PX {
            log::debug!("Resizing: Early return");
            return img;
        }
        let scale_ratio = SCREEN_WIDTH_PX as f64 / width as f64;
        img.resize(
            SCREEN_WIDTH_PX,
            ((height as f64) * scale_ratio).ceil() as u32,
            FilterType::CatmullRom,
        )
    } else {
        // in terms of aspect ratio, image too long; fit height to SCREEN_HEIGHT_PX
        log::debug!("Resizing: image too long, fitting width to SCREEN_HEIGHT_PX");
        if height == SCREEN_HEIGHT_PX {
            log::debug!("Resizing: Early return");
            return img;
        }
        let scale_ratio = SCREEN_HEIGHT_PX as f64 / height as f64;
        img.resize(
            ((width as f64) * scale_ratio).ceil() as u32,
            SCREEN_HEIGHT_PX,
            FilterType::CatmullRom,
        )
    }
}

/// Assumes you ran resize_to_contain_screen already
fn crop_to_fit_screen_exactly(mut img: DynamicImage) -> DynamicImage {
    let width = img.width();
    let height = img.height();
    log::debug!(
        "[After resizing, before cropping] width={} (SCREEN_WIDTH_PX={}), height={} (SCREEN_HEIGHT_PX={})",
        width,
        SCREEN_WIDTH_PX,
        height,
        SCREEN_HEIGHT_PX
    );
    if height == SCREEN_HEIGHT_PX && width == SCREEN_WIDTH_PX {
        return img;
    }
    if height == SCREEN_HEIGHT_PX {
        // image too long, grab horizontal center
        img.crop(
            width / 2 - SCREEN_WIDTH_PX / 2,
            0,
            SCREEN_WIDTH_PX,
            SCREEN_HEIGHT_PX,
        )
    } else if width == SCREEN_WIDTH_PX {
        // image too tall, grab vertical center
        img.crop(
            0,
            height / 2 - SCREEN_HEIGHT_PX / 2,
            SCREEN_WIDTH_PX,
            SCREEN_HEIGHT_PX,
        )
    } else {
        panic!("Impossible branch")
    }
}

pub(crate) fn process_image(
    image_path: &str,
    final_path: &str,
    config: &Config,
) -> anyhow::Result<()> {
    log::info!("Processing (reading) image: {}", image_path);
    let mut img = image::io::Reader::open(image_path)?.decode()?;
    log::info!("Resizing image");
    img = resize_to_contain_screen(img);
    log::info!("Cropping image");
    img = crop_to_fit_screen_exactly(img);
    let mut img = img.to_rgba8();

    log::info!("Loading font");
    let font_data = std::fs::read(&config.general.ttf_font_path)?;
    let font = rusttype::Font::try_from_vec(font_data).ok_or(anyhow!(
        "Invalid font provided at {}",
        config.general.ttf_font_path
    ))?;
    let image_name = PathBuf::from(image_path)
        .file_stem()
        .with_context(|| "Unable to get file stem of image")?
        .to_string_lossy()
        .to_string();

    let image_name_font_scale = Scale::uniform(20.0);
    let image_name_text_size =
        imageproc::drawing::text_size(image_name_font_scale, &font, &image_name);
    log::info!("Drawing image name");
    draw_text_with_border_mut(
        &mut img,
        Rgba([255, 255, 255, 255]),
        SCREEN_WIDTH_PX as i32 - 12 - image_name_text_size.0,
        SCREEN_HEIGHT_PX as i32 - 40 - 12 - image_name_text_size.1, // Height of taskbar is 40 px (Latte)
        image_name_font_scale,
        &font,
        &image_name,
        Rgba([0, 0, 0, 127]),
        2,
    );

    if let Some(c) = &config.countdown {
        log::info!("Processing countdown");
        let today_string = generate_today_string(c)?;
        let today_string_font_scale = Scale::uniform(20.0);
        let today_string_text_size: (i32, i32) =
            imageproc::drawing::text_size(today_string_font_scale, &font, &today_string);

        let countdown_str = get_countdown_str(c)?;
        let countdown_str_font_scale = Scale::uniform(200.0);
        let countdown_str_text_size =
            imageproc::drawing::text_size(countdown_str_font_scale, &font, &countdown_str);

        let font_fill_colour = get_font_fill_colour(c)?;
        let font_stroke_colour = get_font_stroke_colour(c)?;
        log::info!("Drawing countdown");
        draw_text_with_border_mut(
            &mut img,
            font_fill_colour,
            12,
            SCREEN_HEIGHT_PX as i32 - 40 - 12 - today_string_text_size.1, // Height of taskbar is 40 px (Latte)
            today_string_font_scale,
            &font,
            &today_string,
            font_stroke_colour,
            2,
        );
        draw_text_with_border_mut(
            &mut img,
            font_fill_colour,
            12,
            SCREEN_HEIGHT_PX as i32
                - 40
                - 12
                - today_string_text_size.1
                - 12
                - countdown_str_text_size.1, // Height of taskbar is 40 px (Latte)
            countdown_str_font_scale,
            &font,
            &countdown_str,
            font_stroke_colour,
            6,
        );
    }

    if let Some(c) = &config.overlay {
        log::info!("Processing overlay");
        let pixel_sum = img.pixels().fold(0u64, |acc, e| {
            acc + e.0[0] as u64 + e.0[1] as u64 + e.0[2] as u64
        });
        let pixel_mean = (pixel_sum / (img.width() * img.height()) as u64) as u8;
        let base_overlay_rgba = if pixel_mean <= 127 {
            // Original background is more dark than bright; use white overlay
            // [0, 127] pixel_mean |-> alpha [64, 191]
            //                   0 |-> 48   (severely dark; use weak white overlay) [gentler gradient with "gamma encoding"]
            //                 127 |-> 127  (mildly dark; use strong white overlay) [steeper gradient with "gamma encoding"]
            Rgba([
                255,
                255,
                255,
                // 48 + pixel_mean,
                48 + ((pixel_mean as f64 / 127.0) * (127.0 - 48.0)) as u8,
                // 48 + ((pixel_mean as f64 / 127.0).powf(2.2) * (127.0 - 48.0)) as u8,
            ])
        } else {
            // Original background is more bright than dark; use black overlay
            // [128, 255] pixel_mean |-> alpha [64, 191]
            //                   128 |-> 127  (mildly bright; use strong black overlay) [steeper gradient with "gamma encoding"]
            //                   255 |-> 48   (severely bright; use weak black overlay) [gentler gradient with "gamma encoding"]
            Rgba([
                0,
                0,
                0,
                // 48 + (255 - pixel_mean),
                48 + (((255 - pixel_mean) as f64 / 127.0) * (127.0 - 48.0)) as u8,
                // 48 + (((255 - pixel_mean) as f64 / 127.0).powf(2.2) * (127.0 - 48.0)) as u8,
            ])
        };
        log::debug!("pixel_mean = {}", pixel_mean);
        log::debug!("base_overlay_rgba = {:?}", base_overlay_rgba);

        log::info!("Preparing overlay");
        let mut t_image = RgbaImage::from_pixel(img.width(), img.height(), base_overlay_rgba);
        let (overlay_text_scale, overlay_text_size) =
            calculate_overlay_text_scale_and_size(&font, &c.text)?;
        imageproc::drawing::draw_text_mut(
            &mut t_image,
            Rgba([0; 4]),
            (SCREEN_WIDTH_PX as i32 - overlay_text_size.0) / 2,
            // shift upwards by 10, so that the distance from text to top of screen is (x + 20) and
            // the distance from text to bottom of screen is (x + 40), where the header bar is 20 px // tall and Latte is 40 px tall.
            (SCREEN_HEIGHT_PX as i32 - overlay_text_size.1) / 2 - 10,
            overlay_text_scale,
            &font,
            &c.text,
        );

        log::info!("Applying overlay");
        for x in 0..t_image.width() {
            for y in 0..t_image.height() {
                img.get_pixel_mut(x, y).blend(t_image.get_pixel(x, y));
            }
        }
    }

    log::info!("Saving processed image");
    img.save_with_format(final_path, image::ImageFormat::Png)
        .with_context(|| format!("Failed to save processed image to {}", final_path))?;
    log::info!("Done processing image");
    Ok(())
}

fn calculate_overlay_text_scale_and_size(
    font: &Font,
    text: &String,
) -> anyhow::Result<(Scale, (i32, i32))> {
    let mut t_scale = Scale::uniform(SCREEN_HEIGHT_PX as f32);
    let mut text_size_tup = text_size(t_scale, font, text);
    let text_size_bounds = (
        SCREEN_WIDTH_PX as i32 - 24,
        SCREEN_HEIGHT_PX as i32 - 40 - 20 - 24,
    );

    if text_size_tup.0 > text_size_bounds.0 {
        // Need scale down
        log::debug!("[calculate_overlay_text_scale_and_size] Scale down");
        let scale_factor: f32 = text_size_bounds.0 as f32 / text_size_tup.0 as f32;
        t_scale = Scale::uniform(SCREEN_HEIGHT_PX as f32 * scale_factor);
        text_size_tup = text_size(t_scale, font, text);
    } else if text_size_tup.1 > text_size_bounds.1 {
        // Won't happen if you don't scale up carelessly
        bail!("Landed in an unexpected case");
    } else {
        // Need scale up
        if text_size_tup.0 as f64 / text_size_tup.1 as f64
            <= text_size_bounds.0 as f64 / text_size_bounds.1 as f64
        {
            log::debug!("[calculate_overlay_text_scale_and_size] Scale up (Case 1)");
            // in terms of aspect ratio, text_size too tall; >> fit HEIGHT to text_size_bounds
            let scale_factor = text_size_bounds.1 as f32 / text_size_tup.1 as f32;
            t_scale = Scale::uniform(SCREEN_HEIGHT_PX as f32 * scale_factor);
            text_size_tup = text_size(t_scale, font, text);
        } else {
            log::debug!("[calculate_overlay_text_scale_and_size] Scale up (Case 2)");
            // in terms of aspect ratio, text_size too long; fit WIDTH to text_size_bounds
            let scale_factor = text_size_bounds.0 as f32 / text_size_tup.0 as f32;
            t_scale = Scale::uniform(SCREEN_HEIGHT_PX as f32 * scale_factor);
            text_size_tup = text_size(t_scale, font, text);
        }
    }
    log::debug!(
        "[calculate_overlay_text_scale_and_size] returning t_scale={:?}, text_size_tup={:?}",
        t_scale,
        text_size_tup
    );
    Ok((t_scale, text_size_tup))
}
