use mysql::*;
use mysql::prelude::*;

use std::fs;

#[derive(Debug, PartialEq, Eq)]
struct Item {
    amount: i32,
    name: String,
}


fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = format!("mysql://root:{}@localhost:3306/Terraria", get_mysql_password());
    let pool = Pool::new(url.as_str())?;

    let mut conn = pool.get_conn()?;

    conn.query_drop(
        r"CREATE TABLE TestItem (
            amount int not null,
            name text
        )")?;

    let items = vec![
        Item { amount: 1, name: "test".into() },
    ];

    conn.exec_batch(
        r"INSERT INTO TestItem (amount, name)
          VALUES (:amount, :name)",
        items.iter().map(|p| params! {
            "amount" => p.amount,
            "name" => &p.name,
        })
    )?;

    let selected_payments = conn
        .query_map(
            "SELECT amount, name from TestItem",
            |(amount, name)| {
                Item { amount, name }
            },
        )?;

    println!("{:?}", selected_payments);

    Ok(())
}

fn get_mysql_password() -> String {

    let contents = fs::read_to_string(".env").expect(".env file not found");
    let split_contents = contents.split("=");
    let password = split_contents.collect::<Vec<&str>>().remove(1);

    password.to_string()

}