use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserTool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserArgs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserOutput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserError {
    #[cfg(feature = "browser-automation")]
    Automation(rustpot::Error),
    #[cfg(not(feature = "browser-automation"))]
    NotAvailable(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BrowserAction {
    Navigate,
    Click,
    Type,
    Screenshot,
    Evaluate,
    Scroll,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabInfo {
    pub id: String,
    pub url: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementSummary {
    pub role: String,
    pub name: Option<String>,
    pub value: Option<String>,
    pub children: Vec<ElementSummary>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActKind {
    Click,
    Type,
    Navigate,
    Scroll,
    Screenshot,
    Wait,
    KeyPress,
}
