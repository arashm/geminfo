use serde::{Deserialize, Serialize, Deserializer};

fn null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    let option = Option::deserialize(deserializer)?;
    Ok(option.unwrap_or_default())
}

#[derive(Serialize, Deserialize)]
pub struct Geminfo {
    pub name: String,
    pub version: String,
    pub info: String,
    #[serde(default, deserialize_with = "null_default")]
    pub source_code_uri: String,
    #[serde(default, deserialize_with = "null_default")]
    pub homepage_uri: String,
    pub version_created_at: String,
    pub licenses: Vec<String>,
}

impl Geminfo {
    pub fn url(&self) -> &String {
        if !self.source_code_uri.is_empty() {
            &self.source_code_uri
        } else {
            &self.homepage_uri
        }
    }
}
