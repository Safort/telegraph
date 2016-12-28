extern crate telegraph_rs;

use telegraph_rs::account::{Account};

fn main() {
    let mut user = Account::new("short_name", "author_name", "author_url");
    user.create_account();

    println!("{:?}", user);
}
