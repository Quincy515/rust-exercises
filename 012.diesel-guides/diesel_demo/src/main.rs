use diesel::{prelude::*, sql_query};
use diesel_demo::*;
use crate::models::*;


fn main() {
    let connection = establish_connection();
    let users: Vec<Database> = sql_query("SELECT * FROM users ORDER BY id")
        .load(&connection)
        .unwrap();
    println!("{:?}", users);
}
