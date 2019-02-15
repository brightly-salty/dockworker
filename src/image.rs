use serde::de::{DeserializeOwned, Deserializer};
use serde::Deserialize;
use std::{fmt, result};
use chrono::DateTime;
use chrono::offset::FixedOffset;

fn null_to_default<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned + Default,
{
    let actual: Option<T> = Option::deserialize(de)?;
    Ok(actual.unwrap_or_default())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SummaryImage {
    pub Id: String,
    pub ParentId: String,
    #[serde(deserialize_with = "null_to_default")]
    pub RepoTags: Vec<String>,
    #[serde(deserialize_with = "null_to_default", default = "Vec::default")]
    pub RepoDigests: Vec<String>,
    pub Created: u64,
    pub Size: i64,
    #[serde(default = "i64::default")]
    pub SharedSize: i64,
    pub VirtualSize: i64,
    #[serde(default = "i64::default")]
    pub Containers: i64,
}

/// Type of /images/{}/json api
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Image {
    pub Id: String,
    pub RepoTags: Vec<String>,
    pub RepoDigests: Vec<String>,
    pub Parent: String,
    pub Comment: String,
    #[serde(with = "format::chrono")]
    pub Created: DateTime<FixedOffset>,
    pub Container: String,
    //pub ContainerConfig: ContainerConfig,
    pub DockerVersion: String,
    pub Author: String,
    //pub Config: Config,
    pub Architecture: String,
    pub Os: String,
    pub Size: i64,
    pub VirtualSize: i64,
    //pub GraphDriver: GraphDriver,
    //pub RootFS: RootFS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageStatus {
    pub status: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ImageId {
    id: String,
}

impl From<String> for ImageId {
    fn from(id: String) -> Self {
        Self { id }
    }
}

impl fmt::Display for ImageId {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        self.id.fmt(f)
    }
}

impl ImageId {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

pub mod format {
    pub mod chrono {
        use chrono::DateTime;
        use chrono::offset::FixedOffset;
        use serde::Serializer;
        use serde::de::{self, Deserialize, Deserializer};

        pub fn serialize<S>(dt: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
        {
            let str = dt.to_rfc3339();
            serializer.serialize_str(&str)
        }

        pub fn deserialize<'de, D>(de: D) -> Result<DateTime<FixedOffset>, D::Error>
        where D: Deserializer<'de>,
        {
            let str = String::deserialize(de)?;
            DateTime::parse_from_rfc3339(&str).map_err(de::Error::custom)
        }
    }
}

