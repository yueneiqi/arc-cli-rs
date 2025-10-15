use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    #[serde(rename = "windowId")]
    pub window_id: i32,
    #[serde(rename = "tabId")]
    pub tab_id: i32,
    pub url: String,
    pub title: String,
    pub location: TabLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TabLocation {
    TopApp,
    Pinned,
    Unpinned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    pub id: i32,
    pub title: String,
}
