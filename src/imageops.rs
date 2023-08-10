use std::path::PathBuf;

use anyhow::{anyhow, Context};
use image::{imageops::FilterType, DynamicImage};
use rand::seq::IteratorRandom;
use rusttype::Scale;

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
        .with_context(|| format!("Directory \"{}\" does not exist", dir_path))?;
    let image_paths = all_paths
        .filter_map(|path| path.ok())
        .filter(|path| path.metadata().is_ok_and(|metadata| metadata.is_file()));
    image_paths
        .choose(&mut rand::thread_rng())
        .map(|e| e.path().display().to_string())
        .ok_or(anyhow!("Directory \"{}\" is empty", dir_path))
}

/// No cropping is done!
/// Numerical operations should be sound: overflows won't happen in the range we're handling.
pub(crate) fn resize_to_contain_screen(img: DynamicImage) -> DynamicImage {
    let width = img.width();
    let height = img.height();
    if width as f64 / height as f64 <= SCREEN_WIDTH_PX as f64 / SCREEN_HEIGHT_PX as f64 {
        // in terms of aspect ratio, image too tall; fit width to SCREEN_WIDTH_PX
        if width == SCREEN_WIDTH_PX {
            return img;
        }
        let scale_ratio = SCREEN_WIDTH_PX as f64 / width as f64;
        img.resize(
            SCREEN_WIDTH_PX,
            ((height as f64) * scale_ratio).round() as u32,
            FilterType::CatmullRom,
        )
    } else {
        // in terms of aspect ratio, image too long; fit height to SCREEN_HEIGHT_PX
        if height == SCREEN_HEIGHT_PX {
            return img;
        }
        let scale_ratio = SCREEN_HEIGHT_PX as f64 / height as f64;
        img.resize(
            ((width as f64) * scale_ratio).round() as u32,
            SCREEN_HEIGHT_PX,
            FilterType::CatmullRom,
        )
    }
}

/// Assumes you ran resize_to_contain_screen already
pub(crate) fn crop_to_fit_screen_exactly(mut img: DynamicImage) -> DynamicImage {
    let width = img.width();
    let height = img.height();
    println!(
        "width={} ({}), height={} ({})",
        width, SCREEN_WIDTH_PX, height, SCREEN_HEIGHT_PX
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
    println!("PROCESSING: {}", image_path);
    let mut img = image::io::Reader::open(image_path)?.decode()?;
    img = resize_to_contain_screen(img);
    img = crop_to_fit_screen_exactly(img);
    let mut img = img.to_rgba8();

    let font_data = std::fs::read(&config.general.ttf_font_path)?;
    let font = rusttype::Font::try_from_vec(font_data).ok_or(anyhow!(
        "Invalid font provided at \"{}\"",
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
    draw_text_with_border_mut(
        &mut img,
        image::Rgba([255, 255, 255, 255]),
        SCREEN_WIDTH_PX as i32 - 12 - image_name_text_size.0,
        SCREEN_HEIGHT_PX as i32 - 40 - 12 - image_name_text_size.1, // Height of taskbar is 40 px (Latte)
        image_name_font_scale,
        &font,
        &image_name,
        image::Rgba([0, 0, 0, 127]),
        2,
    );

    if let Some(c) = &config.countdown {
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

    img.save_with_format(final_path, image::ImageFormat::Png)
        .with_context(|| format!("Failed to save processed image to {}", final_path))?;
    println!("SAVING... DONE");
    Ok(())
}
