use std::{io, error::Error};

pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub token: String,
    pub time_token_was_made: f64,
}

pub fn load_credentials() -> Result<Credentials, Box<dyn Error>> {
    dotenvy::dotenv_override()?;
    let client_id = dotenvy::var("CLIENT_ID")?;
    let client_secret = dotenvy::var("CLIENT_SECRET")?;
    let token = dotenvy::var("TOKEN")?;
    let time_token_was_made = dotenvy::var("TIME_TOKEN_WAS_MADE")?.parse::<f64>()?;

    Ok(Credentials {
        client_id,
        client_secret,
        token,
        time_token_was_made,
    })
}

pub fn update_env_file(token: &serde_json::Value, time: f64) -> io::Result<()> {
    let contents = std::fs::read_to_string(".env")?;
    let mut new_contents = vec![];

    for line in contents.lines() {
        if line.starts_with("TOKEN") {
            let new_line = format!("TOKEN={}", token);
            new_contents.push(new_line);
        } else if line.starts_with("TIME_TOKEN_WAS_MADE") {
            let new_line = format!("TIME_TOKEN_WAS_MADE={}", time);
            new_contents.push(new_line);
        } else {
            new_contents.push(line.to_string());
        }
    }
    let mut updated_file_contents = String::with_capacity(new_contents.len());
    for line in new_contents {
        updated_file_contents.push_str(&line);
        updated_file_contents.push('\n');
    }
    std::fs::write(".env", updated_file_contents)?;

    Ok(())
}