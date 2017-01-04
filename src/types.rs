use std::default::Default;
use std::collections::HashMap;


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
pub struct Page {
    pub path: String,
    pub url: String,
    pub title: String,
    pub description: String,

    #[serde(default)]
    pub author_name: String,

    #[serde(default)]
    pub author_url: String,

    #[serde(default)]
    pub image_url: String,

    #[serde(default)]
    pub content: Vec<Node>,
    pub views: i32,

    #[serde(default)]
    pub can_edit: bool,
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
pub struct Node {}

#[derive(Serialize, Deserialize, Debug)]
pub struct NodeElement {
    pub tag: String,

    #[serde(default)]
    pub attrs: HashMap<String, String>,

    #[serde(default)]
    pub children: Vec<Node>,
}
