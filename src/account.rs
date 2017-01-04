extern crate hyper;
extern crate serde_json;

use request;

pub use types::{Account, CreateAccountResponse};


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

    pub fn create_account(&mut self) -> String {
        let url = format!(
            "https://api.telegra.ph/createAccount?short_name={}&author_name={}",
            self.short_name,
            self.author_name
        );
        let res_json = request::get(&url);
        let decoded: CreateAccountResponse = serde_json::from_str(&res_json).unwrap();

        self.set_access_token(&decoded.result.access_token)
            .set_auth_url(&decoded.result.auth_url);

        res_json
    }

    pub fn edit_accout_info(&self, acc: &Account) -> String {
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
        if acc.author_url.len() > 0 {
            let author_url = "author_url=".to_string() + &acc.author_url;
            params.push(author_url);
        }
        if acc.author_name.len() > 0 {
            let author_name = "author_name=".to_string() + &acc.author_name;
            params.push(author_name);
        }
        if acc.auth_url.len() > 0 {
            let auth_url = "auth_url=".to_string() + &acc.auth_url;
            params.push(auth_url);
        }

        let url = url + &params.join("&");

        request::get(&url)
    }

    pub fn get_account_info(access_token: &String, fields_list: &Vec<&str>) -> String {
        let fields = fields_list.iter()
            .map(|&field| "\"".to_string() + field + "\"")
            .collect::<Vec<String>>()
            .join(",");
        let fields = "[".to_string() + &fields + "]";


        let url = format!(
            "https://api.telegra.ph/getAccountInfo?access_token={}&fields={}",
            access_token,
            fields
        );

        request::get(&url)
    }

}
