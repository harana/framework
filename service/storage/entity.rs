use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait Entity: Serialize + DeserializeOwned + Send + Sync + Clone + Debug {
    fn id(&self) -> &str;
    fn entity_type() -> &'static str;
}
