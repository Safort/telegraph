extern crate serde_json;

use std::default::Default;
use serde_json::value::Value;


pub type NodeElement = Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum Node {
    Text(String),
    Element(NodeElement),
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    #[serde(default)]
    pub short_name: String,

    #[serde(default)]
    pub author_name: String,

    #[serde(default)]
    pub author_url: String,

    #[serde(default)]
    pub access_token: String,

    #[serde(default)]
    pub auth_url: String,

    #[serde(default)]
    pub page_count: i32,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            short_name: String::new(),
            author_name: String::new(),
            author_url: String::new(),
            access_token: String::new(),
            auth_url: String::new(),
            page_count: 0,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    #[serde(default)]
    pub path: String,

    #[serde(default)]
    pub url: String,

    #[serde(default)]
    pub title: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub author_name: String,

    #[serde(default)]
    pub author_url: String,

    #[serde(default)]
    pub image_url: String,

    pub content: serde_json::Value,

    #[serde(default)]
    pub views: i32,

    #[serde(default)]
    pub can_edit: bool,
}

impl Default for Page {
    fn default() -> Self {
        Page {
            path: String::new(),
            url: String::new(),
            title: String::new(),
            description: String::new(),
            author_name: String::new(),
            author_url: String::new(),
            image_url: String::new(),
            content: json!({}),
            views: 0,
            can_edit: false,
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PageList {
    pub total_count: i32,
    pub pages: Vec<Page>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageViews {
    pub views: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAccountResponse {
    pub ok: bool,
    pub result: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditAccountResponse {
    pub ok: bool,
    pub result: Account,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePageResponse {
    pub ok: bool,
    pub result: Page,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetPageResponse {
    pub ok: bool,
    pub result: Page,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageViewsResponse {
    pub ok: bool,
    pub result: PageViews,
}
