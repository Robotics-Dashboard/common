use crate::errors::NotSetError;

use anyhow::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;

#[async_trait]
pub trait Fetchable: Serialize + DeserializeOwned {
    async fn fetch(&self) -> Result<Self>
    where
        Self: Sized;
}

pub trait Storeable {
    fn load(file_path: &Path) -> Result<Self>
    where
        Self: Sized;
    fn save(&self) -> Result<()>;
}

pub trait Validatable {
    fn validate(&self) -> Result<&Self, NotSetError>;
}

pub trait Parsable {
    fn parse(&self, path: &Path) -> Result<Self>
    where
        Self: Sized;
}
