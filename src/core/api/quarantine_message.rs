#![allow(dead_code)]
use serde::{Deserialize, Serialize};

// Request
#[derive(Debug, Serialize)]
pub(crate) struct QuarantinedComponentRequestList {
    pub(crate) page: Option<usize>,
    pub(crate) page_size: Option<usize>,
    pub(crate) sort_by: Option<QuarantineOrderBy>,
}

#[derive(Debug, Serialize)]
pub(crate) enum QuarantineOrderBy {
    Threat(OrderBy),
    PolicyName(OrderBy),
    QuarantineTime(OrderBy),
    Component(OrderBy),
    Reository(OrderBy),
}

#[derive(Debug, Serialize)]
pub(crate) enum OrderBy {
    Ascending,
    Descending,
}

// Response
#[derive(Deserialize)]
pub(crate) struct QuarantinedComponentResponse {
    pub total: usize,
    pub page: usize,
    #[serde(rename = "pageSize")]
    pub page_size: usize,
    #[serde(rename = "pageCount")]
    pub page_count: usize,
    pub results: Vec<QuarantinedComponentData>,
}

#[derive(Deserialize)]
pub(crate) struct QuarantinedComponentData {
    #[serde(rename = "threatLevel")]
    pub threat_level: u8,

    #[serde(rename = "policyName")]
    pub policy_name: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "repositoryName")]
    pub repository_name: String,
    #[serde(rename = "quarantineDate")]
    pub quarantine_date: String,
    pub quarantined: bool,
}

#[derive(Deserialize)]
pub(crate) struct ComponentIdentifier {
    format: String,
    coordinates: Coordinates,
}

#[derive(Deserialize)]
pub(crate) struct Coordinates {
    #[serde(rename = "packageIf")]
    package_id: String,
    version: String,
}
