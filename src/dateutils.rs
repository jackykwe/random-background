use anyhow::anyhow;
use chrono::{DateTime, Local, NaiveDate, TimeZone};
use toml::value::Datetime;

pub(crate) fn toml_to_chrono(datetime: &Datetime) -> anyhow::Result<DateTime<Local>> {
    Ok(Local
        .from_local_datetime(
            &NaiveDate::from_ymd_opt(
                datetime.date.ok_or(anyhow!("missing year"))?.year as i32,
                datetime.date.ok_or(anyhow!("missing month"))?.month as u32,
                datetime.date.ok_or(anyhow!("missing day"))?.day as u32,
            )
            .ok_or(anyhow!("out-of-range date, invalid month and/or day"))?
            .and_hms_opt(
                datetime.time.map(|t| t.hour).unwrap_or(0) as u32,
                datetime.time.map(|t| t.minute).unwrap_or(0) as u32,
                datetime.time.map(|t| t.second).unwrap_or(0) as u32
            )
            .ok_or(anyhow!("invalid hour, minute and/or second"))?,
        )
        .single()
        .ok_or(anyhow!("Unable to parse chrono datetime unambiguously: possibly due to negative timezone transition?"))?)
}
