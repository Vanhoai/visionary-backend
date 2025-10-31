use serde::Deserialize;

pub fn deserialize_comma_separated<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(',').map(|item| item.trim().to_string()).filter(|item| !item.is_empty()).collect())
}
