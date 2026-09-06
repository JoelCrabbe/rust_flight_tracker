use crate::prelude::*;
use anyhow::Result;

pub fn update_env_file(token_manager: &TokenManager) -> Result<()> {
    let contents =
        std::fs::read_to_string(".env").context("problem reading `.env` file to string")?;

    let mut new_contents = Vec::with_capacity(4);
    for line in contents.lines() {
        if line.starts_with("TOKEN") {
            let new_line = format!("TOKEN={}", token_manager.token);
            new_contents.push(new_line);
        } else if line.starts_with("TIME_TOKEN_WAS_MADE") {
            let new_line = format!("TIME_TOKEN_WAS_MADE={}", token_manager.time_token_was_made);
            new_contents.push(new_line);
        } else {
            new_contents.push(format!("{line}"));
        }
    }
    let updated_file_contents = new_contents.join("\n");
    std::fs::write(".env", updated_file_contents)
        .context("problem writing updated contents to `.env` file")?;

    println!("Updated `TOKEN` and `TIME_TOKEN_WAS_MADE` environment variables in `.env`");

    Ok(())
}

pub fn save_data_to_file(data: &AircraftData, filename: &str) -> Result<()> {
    let json_string =
        to_string_pretty(data).context("problem serializing `AircraftData` struct")?;

    std::fs::write(filename, json_string)
        .with_context(|| format!("problem writing json_string to {filename}"))?;

    println!("Wrote data to {filename}");

    Ok(())
}

pub fn assert_lat(lat: f64) {
    assert!(
        (-90.0..=90.0).contains(&lat),
        "latitude must be between -90 and 90 degrees. {lat} does not fit these requirements",
    );
}

pub fn assert_long(long: f64) {
    assert!(
        (-180.0..=180.0).contains(&long),
        "longitude must be between -180 and 180 degrees. {long} does not fit these requirements",
    );
}

