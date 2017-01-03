extern crate serde_json;
extern crate hyper;

use self::hyper::Client;
use self::hyper::Url;


pub fn get(url: &str) -> String {
    use::std::io::Read;

    let mut res_json = String::new();
    let client = Client::new();
    let req_url = Url::parse(&url).unwrap();
    let mut res = client.get(req_url).send().unwrap();

    res.read_to_string(&mut res_json).unwrap();

    res_json
}
