use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Collection<T> {
    Array(Vec<T>),
    Object(Vec<(String, T)>),
}
