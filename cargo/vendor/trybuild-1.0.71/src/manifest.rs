use crate::dependencies::{Dependency, Patch, RegistryPatch, TargetDependencies};
use serde::ser::{SerializeMap, Serializer};
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Serialize, Debug)]
pub struct Manifest {
    pub package: Package,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub features: Map<String, Vec<String>>,
    pub dependencies: Map<String, Dependency>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub target: Map<String, TargetDependencies>,
    #[serde(rename = "bin")]
    pub bins: Vec<Bin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<Workspace>,
    #[serde(
        serialize_with = "serialize_patch",
        skip_serializing_if = "empty_patch"
    )]
    pub patch: Map<String, RegistryPatch>,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub replace: Map<String, Patch>,
}

#[derive(Serialize, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: Edition,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<String>,
    pub publish: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Edition {
    #[serde(rename = "2015")]
    E2015,
    #[serde(rename = "2018")]
    E2018,
    #[serde(rename = "2021")]
    E2021,
}

#[derive(Serialize, Debug)]
pub struct Bin {
    pub name: Name,
    pub path: PathBuf,
}

#[derive(Serialize, Clone, Debug)]
pub struct Name(pub String);

#[derive(Serialize, Debug)]
pub struct Config {
    pub build: Build,
}

#[derive(Serialize, Debug)]
pub struct Build {
    pub rustflags: Vec<&'static str>,
}

#[derive(Serialize, Debug)]
pub struct Workspace {
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub dependencies: Map<String, Dependency>,
}

impl Default for Edition {
    fn default() -> Self {
        Edition::E2018
    }
}

impl AsRef<OsStr> for Name {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

fn serialize_patch<S>(patch: &Map<String, RegistryPatch>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = serializer.serialize_map(None)?;
    for (registry, patch) in patch {
        if !patch.crates.is_empty() {
            map.serialize_entry(registry, patch)?;
        }
    }
    map.end()
}

fn empty_patch(patch: &Map<String, RegistryPatch>) -> bool {
    patch
        .values()
        .all(|registry_patch| registry_patch.crates.is_empty())
}
