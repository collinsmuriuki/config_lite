use std::env;

use regex::Regex;
use crate::{ConfigError, ConfigResult};

pub fn parse_json<'a, T: for<'de> serde::Deserialize<'de>>(
    file_content: &str,
    string_path: &str,
) -> ConfigResult<T> {
    let value: serde_json::Value = serde_json::from_str(&file_content)?;
    let string_path_vec = string_path.split(".").collect::<Vec<&str>>();
    let mut current_value = &value;
    for s in string_path_vec {
        current_value = current_value
            .get(s)
            .ok_or_else(|| ConfigError::ValueError)?;
    }
    // check if current value is a string and if the string matches a {{}} regex pattern
    if current_value.is_string() {
        let re = Regex::new(r"\{\{(\w+)\}\}").unwrap();
        let value_string = current_value.as_str().unwrap();
        match re.captures(value_string) {
            Some(c) => {
                let env_var_name = c.get(1).unwrap().as_str();
                let value = env::var(env_var_name).unwrap();
                let value = serde_json::Value::String(value);
                return Ok(serde_json::from_value(value)?);
            },
            None => {/*TODO: throw error*/ unimplemented!()}
        }
    }
    Ok(serde_json::from_value(current_value.to_owned())?)
}
