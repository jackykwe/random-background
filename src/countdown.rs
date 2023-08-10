use anyhow::Context;
use chrono::{DateTime, Datelike, Local};
use divrem::DivCeil;
use image::Rgba;
use now::DateTimeNow;

use crate::{config::ConfigCountdown, dateutils::toml_to_chrono};

pub(crate) fn generate_today_string(config: &ConfigCountdown) -> anyhow::Result<String> {
    let now = Local::now();
    let last_paper_end_time = toml_to_chrono(&config.last_paper_end_time)
        .with_context(|| "Failed to parse last_paper_end_time")?;

    let hours_left = DivCeil::div_ceil((last_paper_end_time - now).num_seconds(), 3600);
    Ok(format!(
        "Calculated on {} {} {} (<{}h left)",
        now.format("%a"),
        now.day(),
        now.format("%b"),
        hours_left
    ))
}

pub(crate) fn get_countdown_str(config: &ConfigCountdown) -> anyhow::Result<String> {
    let now = Local::now();
    let start_of_today = now.beginning_of_day();
    let term_start = toml_to_chrono(&config.term_start)
        .with_context(|| "Failed to parse term_start")?
        .beginning_of_day();
    let first_paper = toml_to_chrono(&config.first_paper)
        .with_context(|| "Failed to parse first_paper")?
        .beginning_of_day();
    let last_paper_end_time = toml_to_chrono(&config.last_paper_end_time)
        .with_context(|| "Failed to parse last_paper_end_time")?;

    if start_of_today < term_start {
        Ok(String::from("S"))
    } else if start_of_today < first_paper {
        Ok((first_paper - start_of_today).num_days().to_string())
    } else {
        if now < last_paper_end_time {
            Ok(format!(
                "D{}",
                (start_of_today - first_paper).num_days() + 1
            ))
        } else {
            Ok(String::from("E"))
        }
    }
}

pub(crate) fn get_font_fill_colour(config: &ConfigCountdown) -> anyhow::Result<Rgba<u8>> {
    let now: DateTime<Local> = Local::now();
    let start_of_today = now.beginning_of_day();
    let term_start = toml_to_chrono(&config.term_start)
        .with_context(|| "Failed to parse term_start")?
        .beginning_of_day();
    let term_last_lecture = toml_to_chrono(&config.term_last_lecture)
        .with_context(|| "Failed to parse term_last_lecture")?
        .beginning_of_day();
    let first_paper = toml_to_chrono(&config.first_paper)
        .with_context(|| "Failed to parse first_paper")?
        .beginning_of_day();
    let last_paper_end_time = toml_to_chrono(&config.last_paper_end_time)
        .with_context(|| "Failed to parse last_paper_end_time")?;

    if start_of_today < term_start || now >= last_paper_end_time {
        // Before term start or after papers
        Ok(Rgba([0, 255, 0, 255]))
    } else if start_of_today >= first_paper {
        // During examination period
        Ok(Rgba([255; 4]))
    } else {
        if start_of_today <= term_last_lecture {
            // during term time; white -> orange -> red
            let half_term_length =
                DivCeil::div_ceil((term_last_lecture - term_start).num_days(), 2)
                    .clamp(1, i64::MAX);
            let term_remaining = (term_last_lecture - start_of_today).num_days();
            if term_remaining > half_term_length {
                Ok(Rgba([
                    255,
                    255,
                    ((term_remaining - half_term_length) * 255 / half_term_length)
                        .try_into()
                        .with_context(|| {
                            "Number of days left in countdown exceeds 255 [branch 1]"
                        })?,
                    255,
                ]))
            } else {
                Ok(Rgba([
                    255,
                    (term_remaining * 255 / half_term_length)
                        .try_into()
                        .with_context(|| {
                            "Number of days left in countdown exceeds 255 [branch 2]"
                        })?,
                    0,
                    255,
                ]))
            }
        } else {
            // final sprint; red -> black
            let final_dash_length = (first_paper - term_last_lecture).num_days();
            let final_dash_remaining = (first_paper - start_of_today).num_days();
            Ok(Rgba([
                (final_dash_remaining * 255 / final_dash_length)
                    .try_into()
                    .with_context(|| "Number of days left in countdown exceeds 255 [branch 3]")?,
                0,
                0,
                255,
            ]))
        }
    }
}

pub(crate) fn get_font_stroke_colour(config: &ConfigCountdown) -> anyhow::Result<Rgba<u8>> {
    let now: DateTime<Local> = Local::now();
    let start_of_today = now.beginning_of_day();
    let term_start = toml_to_chrono(&config.term_start)
        .with_context(|| "Failed to parse term_start")?
        .beginning_of_day();
    let term_last_lecture = toml_to_chrono(&config.term_last_lecture)
        .with_context(|| "Failed to parse term_last_lecture")?
        .beginning_of_day();
    let first_paper = toml_to_chrono(&config.first_paper)
        .with_context(|| "Failed to parse first_paper")?
        .beginning_of_day();
    let last_paper_end_time = toml_to_chrono(&config.last_paper_end_time)
        .with_context(|| "Failed to parse last_paper_end_time")?;

    if start_of_today < term_start || now >= last_paper_end_time {
        // Before term start or after papers
        Ok(Rgba([0, 0, 0, 127]))
    } else if start_of_today >= first_paper {
        // During examination period
        Ok(Rgba([255, 0, 0, 255]))
    } else {
        if start_of_today <= term_last_lecture {
            // during term time; white -> orange -> red
            Ok(Rgba([0, 0, 0, 127]))
        } else {
            // final sprint; red -> black
            Ok(Rgba([255; 4]))
        }
    }
}
