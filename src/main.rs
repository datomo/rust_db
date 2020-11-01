use mysql::*;
use mysql::prelude::*;
use rand::thread_rng;
use rand::Rng;

use std::time::{Instant};

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}


fn main() {
    let now = Instant::now();
    insert();
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now));

}


fn  insert() {

    let url = "mysql://root:admin@localhost:3306/test";

    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();


    // Let's create payment table.
    // Unwrap just to make sure no error happened.
    conn.query_drop(r"CREATE TABLE rust_table (
                         customer_id int not null,
                         amount int not null,
                         account_name text
                     )").unwrap();

    /*let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: None },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: None },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];*/
    let mut payments = Vec::new();

    let mut rng = thread_rng();

    for i in 0..2000000 {
        payments.push([i as i32, rng.gen_range(0, 20000), 0 ]);
    }

    println!("finished");
    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    conn.exec_batch(r"INSERT INTO rust_table
                                       (customer_id, amount, account_name)
                                   VALUES
                                       (:customer_id, :amount, :account_name)",
                                        payments.iter().map(|p| params! {
                                            "customer_id" => p[0],
                                            "amount" => p[1],
                                            "account_name" => p[2],
                                        })
    ).unwrap();

    println!("Yay!");

}