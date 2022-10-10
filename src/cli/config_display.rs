use crate::model::Config;
use std::fmt::format;
use yaml_rust::Yaml;

fn offset_str(offset: usize) -> String {
    "\t".repeat(offset).to_string()
}

fn config_structure(config: &Config, offset: usize) -> String {
    let mut output = Vec::with_capacity(config.len());
    for (id, (key, value)) in config.iter().enumerate() {
        match (key, value) {
            (Yaml::String(key), Yaml::Hash(sub_config)) => {
                output.push(format!(
                    "{}{}{} ↴\n",
                    offset_str(offset),
                    if offset == 0 {
                        "".to_string()
                    } else {
                        format!("{} ", id)
                    },
                    key
                ));
                output.push(config_structure(sub_config, offset + 1));
            }
            (Yaml::String(key), Yaml::Array(hosts)) => output.push(format!(
                "{}{} {}: {} hosts\n",
                offset_str(offset),
                id,
                key,
                hosts.len()
            )),
            _ => {}
        }
    }

    output.iter().fold("".to_string(), |prev, next| prev + next)
}

pub fn display_config(config: &Config) -> String {
    config_structure(config, 0)
}
