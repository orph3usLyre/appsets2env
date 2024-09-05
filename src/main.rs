use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde_json::Value;
use std::{ops::Deref, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input appsettings file
    #[arg(value_name = "APPSETTINGS_FILE")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let raw_json = std::fs::read_to_string(&cli.input).context("Read file")?;
    let json_value = serde_json::from_str(&raw_json).context("Read contents as JSON")?;
    appsettings_json_to_env_recursive(&json_value, None);
    Ok(())
}

fn appsettings_json_to_env_recursive(json_value: &Value, key: Option<&[&str]>) {
    match json_value {
        Value::Object(map) => {
            let mut key_buff: Vec<&str> = match key {
                Some(arr) => arr.into_iter().map(Deref::deref).collect(),
                None => Vec::new(),
            };
            for (key, val) in map {
                // add new key if object
                key_buff.push(key);
                appsettings_json_to_env_recursive(val, Some(&key_buff));
            }
        }
        Value::Array(arr) => {
            for (i, val) in arr.into_iter().enumerate() {
                let mut key_buff: Vec<&str> = match key {
                    Some(arr) => arr.into_iter().map(Deref::deref).collect(),
                    None => Vec::new(),
                };
                let formatted_idx = i.to_string();
                key_buff.push(&formatted_idx);
                appsettings_json_to_env_recursive(val, key);
            }
        }
        _ => {
            if let Some(key_buff) = key {
                // print the value directly if it's a primitive (String, Number, Bool, Null)
                println!("{}={}", capitalize_and_join(&key_buff), json_value);
            }
        }
    }
}

fn capitalize_and_join(arr: &[&str]) -> String {
    arr.iter()
        .enumerate()
        .fold(String::new(), |mut acc, (i, s)| {
            if i > 0 {
                acc.push_str("__"); // Add underscore before all but the first item
            }
            acc.push_str(&s.to_ascii_uppercase());
            acc
        })
}
