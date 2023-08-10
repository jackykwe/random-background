use anyhow::{anyhow, Context};
use clap::{command, Parser};
use image::{Rgb, RgbImage};
use std::path::PathBuf;

use crate::{
    config::Config,
    imageops::{choose_one_image, process_image},
};

mod config;
mod countdown;
mod dateutils;
mod imageops;
mod imageutils;

#[derive(Parser, Debug)]
#[command(name = "Random Background", author, version, about, long_about = None)]
struct Args {
    /// Path to directory containing the images
    #[arg(short, long)]
    dir: String,
}

fn ensure_working_dir_exists(parent_dir: &str) -> anyhow::Result<()> {
    let working_directory_path: PathBuf = [parent_dir, "Working"].iter().collect();
    std::fs::create_dir_all(&working_directory_path).with_context(|| {
        format!(
            "Failed to ensure \"{}\" directory exists",
            working_directory_path.to_string_lossy().to_string()
        )
    })?;
    Ok(())
}

fn ensure_blank_background_exists(blank_wallpaper_path: &str) -> anyhow::Result<()> {
    RgbImage::from_pixel(1, 1, Rgb([0, 0, 0]))
        .save(&blank_wallpaper_path)
        .with_context(|| format!("Failed to save blank wallpaper to {}", blank_wallpaper_path))?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args: Args = Args::parse();
    let config = Config::read_from_dir(&args.dir)?;

    ensure_working_dir_exists(&args.dir)?;
    let blank_wallpaper_path: PathBuf = [&args.dir, "Working", "blank.png"].iter().collect();
    let blank_wallpaper_path = blank_wallpaper_path.to_string_lossy().to_string();
    ensure_blank_background_exists(&blank_wallpaper_path)?;

    let chosen_img_path = choose_one_image(&args.dir)?;
    let final_wallpaper_path: PathBuf = [&args.dir, "Working", "current.png"].iter().collect();
    let final_wallpaper_path = final_wallpaper_path.to_string_lossy().to_string();

    process_image(&chosen_img_path, &final_wallpaper_path, &config)?;

    wallpaper::set_mode(wallpaper::Mode::Crop)
        .map_err(|e| anyhow!("Failed to set wallpaper mode: {}", e))?;

    wallpaper::set_from_path(&blank_wallpaper_path)
        .map_err(|e| anyhow!("Failed to set wallpaper: {}", e))?;

    wallpaper::set_from_path(&final_wallpaper_path)
        .map_err(|e| anyhow!("Failed to set wallpaper: {}", e))?;

    Ok(())
}
