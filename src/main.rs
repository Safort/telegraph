extern crate telegraph;

use telegraph::account::{Account};

fn main() {
    let mut user = Account::new("short_name", "author_name", "author_url");
    user.create_account();

    println!("{:?}", user);
}
