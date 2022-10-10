mod cli;
mod model;

extern crate yaml_rust;

use crate::cli::config_display::display_config;
use crate::cli::selector::select_by_config;
use crate::model::Config;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashSet;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::{env, fs};
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

fn load_config(location: &Path) -> Result<Config, String> {
    let raw = fs::read_to_string(location).map_err(|e| e.to_string())?;
    let yaml = YamlLoader::load_from_str(raw.as_str())
        .map(|v| v[0].clone())
        .map_err(|e| e.to_string())?;
    match yaml {
        Yaml::Hash(hash) => Ok(hash),
        _ => Err("Root YAML is not map (hash)".to_string()),
    }
}

fn app() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();

    let is_verbose = args.iter().find(|arg| *arg == "--verbose").is_some();
    let is_dry = args.iter().find(|arg| *arg == "--dry").is_some();

    let path = Path::new("hostnames.yaml");
    let config = load_config(path)?;

    if is_verbose {
        println!("{}", display_config(&config));
    }

    let res_host = select_by_config(&config)?;

    let shell_command = format!("ssh {}", res_host);

    if !is_dry {
        let mut ctx = ClipboardContext::new().map_err(|err| err.to_string())?;

        ctx.set_contents(shell_command.clone())
            .map_err(|err| err.to_string())?
    }

    Ok(println!("\n{}\n{}", "-".repeat(10), shell_command))
}

fn main() {
    app().unwrap_or_else(|err| panic!("{}", err))
}
