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

    pub fn set_access_token(&mut self, access_token: &str) -> &mut Account {
        self.access_token = access_token.to_string();
        self
    }

    pub fn create_account(&mut self) -> Account {
        let req_url = format!(
            "https://api.telegra.ph/createAccount?short_name={}&author_name={}",
            self.short_name,
            self.author_name
        );
        let res_json = request::get(&req_url);

        let decoded: CreateAccountResponse = serde_json::from_str(&res_json).unwrap();

        println!("{:?}", decoded);

        self.set_access_token(&decoded.result.access_token)
            .set_auth_url(&decoded.result.auth_url);

        decoded.result
    }
}
