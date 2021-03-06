extern crate hyper;
extern crate serde_json;

use request;
use serde_json::{from_str, from_value};

pub use types::*;


impl Account {
    pub fn new(short_name: &str, author_name: &str, author_url: &str) -> Account {
        Account {
            short_name: short_name.to_string(),
            author_name: author_name.to_string(),
            author_url: author_url.to_string(),
            access_token: "".to_string(),
            auth_url: "".to_string(),
            page_count: 0,
        }
    }

    pub fn set_short_name(&mut self, short_name: &str) -> &mut Account {
        self.short_name = short_name.to_string();
        self
    }

    pub fn set_author_name(&mut self, author_name: &str) -> &mut Account {
        self.author_name = author_name.to_string();
        self
    }

    pub fn set_author_url(&mut self, author_url: &str) -> &mut Account {
        self.author_url = author_url.to_string();
        self
    }

    pub fn set_auth_url(&mut self, auth_url: &str) -> &mut Account {
        self.auth_url = auth_url.to_string();
        self
    }

    pub fn set_page_count(&mut self, page_count: &i32) -> &mut Account {
        self.page_count = page_count.clone();
        self
    }

    pub fn set_access_token(&mut self, access_token: &str) -> &mut Account {
        self.access_token = access_token.to_string();
        self
    }

    fn update(&mut self, new_acc: &Account) -> &mut Account {
        self.set_short_name(&new_acc.short_name);
        self.set_author_name(&new_acc.author_name);
        self.set_author_url(&new_acc.author_url);

        self
    }

    pub fn create_account(&mut self) -> Result<CreateAccountResponse, &str> {
        let url = format!(
            "https://api.telegra.ph/createAccount?short_name={}&author_name={}",
            self.short_name,
            self.author_name
        );
        let res_json = request::get(&url);
        let decoded: CreateAccountResponse = serde_json::from_str(&res_json).unwrap();

        self.set_access_token(&decoded.result.access_token)
            .set_auth_url(&decoded.result.auth_url);

        Ok(decoded)
    }

    pub fn edit_account_info(&mut self, acc: &Account) -> Result<EditAccountResponse, &str> {
        let url = String::from("https://api.telegra.ph/editAccountInfo?");
        let mut params: Vec<String> = vec![];

        let access_token = "access_token=".to_string() + &if acc.access_token.len() == 0 {
                self.access_token.clone()
            } else {
                acc.access_token.clone()
            };
        params.push(access_token);

        if acc.short_name.len() > 0 {
            let short_name = "short_name=".to_string() + &acc.short_name;
            params.push(short_name);
        }
        if acc.author_name.len() > 0 {
            let author_name = "author_name=".to_string() + &acc.author_name;
            params.push(author_name);
        }
        if acc.author_url.len() > 0 {
            let author_url = "author_url=".to_string() + &acc.author_url;
            params.push(author_url);
        }

        let url = url + &params.join("&");
        let res_json = request::get(&url);
        let decoded: EditAccountResponse = serde_json::from_str(&res_json).unwrap();

        self.update(&decoded.result);

        Ok(decoded)
    }

    pub fn get_account_info<'a>(&self, fields_list: &Vec<&str>) -> Result<EditAccountResponse, &'a str> {
        let fields = fields_list.iter()
            .map(|&field| "\"".to_string() + field + "\"")
            .collect::<Vec<String>>()
            .join(",");
        let fields = "[".to_string() + &fields + "]";
        let url = format!(
            "https://api.telegra.ph/getAccountInfo?access_token={}&fields={}",
            self.access_token,
            fields
        );
        let res_json = request::get(&url);
        let decoded: EditAccountResponse = serde_json::from_str(&res_json).unwrap();

        Ok(decoded)
    }

    pub fn revoke_access_token(&mut self) -> Result<EditAccountResponse, &str> {
        let url = format!(
            "https://api.telegra.ph/revokeAccessToken?access_token={}",
            self.access_token
        );
        let res_json = request::get(&url);
        let decoded: EditAccountResponse = serde_json::from_str(&res_json).unwrap();

        self.set_access_token(&decoded.result.access_token);

        Ok(decoded)
    }

    pub fn create_page(&mut self, page: &Page, return_content: bool) -> Result<CreatePageResponse, &str> {
        let mut url = format!("https://api.telegra.ph/createPage?");
        let mut params: Vec<String> = vec![];

        params.push("access_token=".to_string() + &self.access_token);
        params.push("title=".to_string() + &page.title);
        params.push("content=".to_string() + &page.content.to_string());
        params.push("return_content=".to_string() + &return_content.to_string());

        url = url + &params.join("&");

        let res_json = request::get(&url);
        let decoded: CreatePageResponse = serde_json::from_str(&res_json).unwrap();

        Ok(decoded)
    }

    pub fn edit_page(&mut self, page: &Page, return_content: bool) -> Result<CreatePageResponse, &str> {
        let mut url = format!("https://api.telegra.ph/editPage?");
        let mut params: Vec<String> = vec![];

        params.push("access_token=".to_string() + &self.access_token);
        params.push("title=".to_string() + &page.title);
        params.push("content=".to_string() + &page.content.to_string());
        params.push("path=".to_string() + &page.path);
        params.push("return_content=".to_string() + &return_content.to_string());

        url = url + &params.join("&");

        let res_json = request::get(&url);
        let decoded: CreatePageResponse = serde_json::from_str(&res_json).unwrap();

        Ok(decoded)
    }

    pub fn get_page<'a>(path: String, return_content: bool) -> Result<GetPageResponse, &'a str> {
        let mut url = format!("https://api.telegra.ph/getPage/{}?", path);

        if return_content == true {
            url = url + "return_content=true";
        }

        let res_json = request::get(&url);
        let decoded: GetPageResponse = serde_json::from_str(&res_json).unwrap();

        Ok(decoded)
    }


    pub fn get_views<'a>(path: String, year: i32, month: i32, day: i32, hour: i32) -> Result<PageViewsResponse, &'a str> {
        let mut url = format!("https://api.telegra.ph/getViews/{}?", path);
        let mut params: Vec<String> = vec![];

        if year > 0 {
            params.push("year=".to_string() + &year.to_string());
        }
        if month > 0 {
            params.push("month=".to_string() + &month.to_string());
        }
        if day > 0 {
            params.push("day=".to_string() + &day.to_string());
        }
        if hour > -1 {
            params.push("hour=".to_string() + &hour.to_string());
        }

        url = url + &params.join("&");

        let res_json = request::get(&url);
        let decoded: PageViewsResponse = serde_json::from_str(&res_json).unwrap();

        Ok(decoded)
    }
}
