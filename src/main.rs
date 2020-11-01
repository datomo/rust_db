#[macro_use]
extern crate mysql;
// ...

use mysql as my;
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


fn insert() {
    // See docs on the `OptsBuilder`'s methods for the list of options available via URL.
    let pool = my::Pool::new("mysql://root:admin@localhost:3306/test").unwrap();

    // Let's create payment table.
    // Unwrap just to make sure no error happened.
    pool.prep_exec(r"CREATE TABLE rust_table (
                         customer_id int not null,
                         amount int not null,
                         account_name text
                     )", ()).unwrap();

    /*let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: None },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: None },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];*/
    let mut payments = Vec::new();

    for i in 0..20000 {
        payments.push(Payment { customer_id: i as i32, amount: 2, account_name: None });
    }

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"INSERT INTO rust_table
                                       (customer_id, amount, account_name)
                                   VALUES
                                       (:customer_id, :amount, :account_name)").into_iter() {
        for p in payments.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            }).unwrap();
        }
    }

    println!("Yay!");
}