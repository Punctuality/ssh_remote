use crate::model::Config;
use crate::model::Host;
use std::io;
use std::rc::Rc;
use yaml_rust::Yaml;

fn read_number(limit: usize, retries: usize) -> Result<usize, String> {
    if retries > 0 {
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => {
                buf = buf.trim().to_string();
                let parsed = str::parse::<usize>(buf.as_str());
                match parsed {
                    Ok(res) if res <= limit && res >= 1 => Ok(res),
                    Ok(res) => {
                        eprintln!("Input choice is too big: {}, maximum: {}", res, limit);
                        read_number(limit, retries - 1)
                    }
                    Err(err) => {
                        eprintln!("Failed to parse: {}, reason: {}", buf, err.to_string());
                        read_number(limit, retries - 1)
                    }
                }
            }
            Err(err) => Err(err.to_string()),
        }
    } else {
        Err("Wasn't able to pick correct option".to_string())
    }
}

fn pick_from_vars(from_name: String, vars: Vec<Rc<&Yaml>>) -> Result<Rc<&Yaml>, String> {
    let vars_repr = vars
        .iter()
        .enumerate()
        .fold("".to_string(), |prev, (idx, next)| {
            prev + "\n" + format!("{}. {}", idx + 1, next.as_str().unwrap_or("<unknown>")).as_str()
        });
    println!("Pick {} from: {}", from_name, vars_repr);

    let idx = read_number(vars.len(), 3)?;
    Ok(vars[idx - 1].clone())
}

pub fn select_by_config(config: &Config) -> Result<Host, String> {
    let keys: Vec<Rc<&Yaml>> = config
        .keys()
        .filter_map(|yaml| match yaml {
            str @ Yaml::String(_) => Some(Rc::new(str)),
            _ => None,
        })
        .collect();

    let key_idx = *(pick_from_vars("option".to_string(), keys)?);

    match config.get(key_idx) {
        Some(Yaml::Array(hosts)) if hosts.len() == 1 => {
            return match &hosts[0] {
                Yaml::String(host) => Ok(host.clone()),
                other => Err(format!(
                    "Encountered unexpected value in array: {}",
                    other.as_str().unwrap_or("<unknown>")
                )),
            };
        }
        Some(Yaml::Array(hosts)) => {
            let hosts_vars = hosts.to_vec();
            let host_res = pick_from_vars(
                "hosts".to_string(),
                hosts_vars.iter().map(|y| Rc::new(y)).collect(),
            )?;
            return match *host_res {
                Yaml::String(host) => Ok(host.clone()),
                other => Err(format!(
                    "Encountered unexpected value in array: {}",
                    other.as_str().unwrap_or("<unknown>")
                )),
            };
        }
        Some(Yaml::Hash(sub_config)) => select_by_config(sub_config),
        Some(other) => Err(format!(
            "Encountered unexpected value while: {} (on key {})",
            other.as_str().unwrap_or("<unknown>"),
            key_idx.as_str().unwrap_or("<unknown>")
        )),
        None => Err(format!(
            "Found no value on key {}",
            key_idx.as_str().unwrap_or("<unknown>")
        )),
    }
}
