use mysql::prelude::*;
use mysql::*;
use rand::thread_rng;
use rand::Rng;

use std::time::Instant;

use std::fs::{OpenOptions};
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

fn main() {
    let now = Instant::now();
    //insert(1000000);
    //open_write().unwrap();
    //insert_once(10000000, 4000000);
    //open_write_batch(100000).unwrap();
    insert_infile(10000);
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now));
}

fn open_write() -> io::Result<()> {
    let path = "../arcos_all_washpost.tsv";

    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("../arcos_all_washpost.tsv")?;
    let mut out = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("../out.csv")?;
    let reader = BufReader::new(file);


    count_lines(path);


    for (i, line) in reader.lines().enumerate() {
        out.write(
            line?
                .split("\t")
                .collect::<Vec<&str>>()
                .join(",")
                .as_bytes(),
        )?;

        if i % 50000000 == 0 && i != 0 {
            println!("at line: {}", i)
        }
    }

    Ok(())
}

fn count_lines(path: &str) {
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path).unwrap();

    let reader = BufReader::new(file);

    println!("len is: {}", reader.lines().count());
}

fn open_write_batch(chunk: i32) -> io::Result<()> {
    let path = "arcos_all_washpost.tsv";

    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)?;
    let mut out = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("out.csv")?;

    let reader = BufReader::new(file);


    count_lines(path);

    let mut write = "".to_string();
    let mut executed = false;

    for (i, line) in reader.lines().enumerate() {
        executed = false;

        write.push_str(&(line?.split("\t").collect::<Vec<&str>>().join(",") + "\n"));
        //out.sync_all()?;
        if i % chunk as usize == 0 && i != 0 {
            println!("at line: {}", i);

            out.write(write.as_bytes())?;
            write = "".to_string();

            executed = true;
        }
    }

    if !executed {
        out.write(write.as_bytes())?;
    }

    Ok(())
}

fn insert_once(amount: i32, chunk: i32) {
    let url = "mysql://root:admin@localhost:3306/test";

    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    conn.query_drop(
        r"CREATE TABLE rust_table (
                         customer_id int not null,
                         amount int not null,
                         account_name text
                     )",
    ).unwrap();

    conn.query_drop(r"SET GLOBAL max_allowed_packet=107374182400000000")
        .unwrap();

    let mut rng = thread_rng();

    let mut payments = "INSERT INTO rust_table VALUES".to_string();
    let mut executed = false;

    for i in 0..amount {
        if i != 0 && !executed {
            payments.push_str(&format!(
                ",({}, {}, {})",
                i as i32,
                rng.gen_range(0, 20000),
                0
            ))
        } else {
            payments.push_str(&format!(
                "({}, {}, {})",
                i as i32,
                rng.gen_range(0, 20000),
                0
            ))
        }

        executed = false;

        if i % chunk == 0 {
            conn.query_drop(payments).unwrap();
            println!("at line: {}\n executed", i);
            payments = "INSERT INTO rust_table VALUES".to_string();
            executed = true;
        }
    }
    if !executed {
        conn.query_drop(payments).unwrap();
    }
}

fn insert_infile(amount: i32) -> Result<()> {

    let path = "arcos_all_washpost.tsv";

    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)?;
    let mut out = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("out1.csv")?;

    let reader = BufReader::new(file);

    for (i,line) in reader.lines().enumerate() {
        out.write((line.unwrap() + "\n").as_bytes());
        if i == 90000000 {
            out = OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open("out2.csv")?;
        }
    }

    println!("finished parsing...");

    let mut conn = generate_connection();

    conn.query_drop(r"SET GLOBAL local_infile = 'ON'");

    conn.query_drop(r"DROP TABLE IF EXISTS rust_final_table").unwrap();

    conn.query_drop(
        r"CREATE TABLE rust_final_table (
                         REPORTER_DEA_NO VARCHAR(50),
                         REPORTER_BUS_ACT int,
                         REPORTER_NAME VARCHAR(50),
                         REPORTER_ADDL_CO_INFO VARCHAR(50),
                         REPORTER_ADDRESS1 VARCHAR(50),
                         REPORTER_ADDRESS2 VARCHAR(50),
                         REPORTER_CITY VARCHAR(50),
                         REPORTER_STATE VARCHAR(50),
                         REPORTER_ZIP int,
                         REPORTER_COUNTY VARCHAR(50),
                         BUYER_DEA_NO VARCHAR(50),
                         BUYER_BUS_ACT VARCHAR(50),
                         BUYER_NAME VARCHAR(50),
                         BUYER_ADDL_CO_INFO VARCHAR(50),
                         BUYER_ADDRESS1 VARCHAR(50),
                         BUYER_ADDRESS2 VARCHAR(50),
                         BUYER_CITY VARCHAR(50),
                         BUYER_STATE VARCHAR(50),
                         BUYER_ZIP int,
                         BUYER_COUNTY VARCHAR(50),
                         TRANSACTION_CODE VARCHAR(50),
                         DRUG_CODE int,
                         NDC_NO VARCHAR(50),
                         DRUG_NAME VARCHAR(50),
                         QUANTITY VARCHAR(50),
                         UNIT VARCHAR(50),
                         ACTION_INDICATOR VARCHAR(50),
                         ORDER_FORM_NO VARCHAR(50),
                         CORRECTION_NO VARCHAR(50),
                         STRENGTH VARCHAR(50),
                         TRANSACTION_DATE VARCHAR(50),
                         CALC_BASE_WT_IN_GM VARCHAR(50),
                         DOSAGE_UNIT VARCHAR(50),
                         TRANSACTION_ID VARCHAR(50),
                         Product_Name VARCHAR(50),
                         Ingredient_Name VARCHAR(50),
                         Measure VARCHAR(50),
                         MME_Conversion_Factor VARCHAR(50),
                         Combined_Labeler_Name VARCHAR(50),
                         Revised_Company_Name VARCHAR(50),
                         Reporter_family VARCHAR(50),
                         dos_str VARCHAR(50)
                     )",
    ).unwrap();

    println!("created table...");

    conn.query_drop("LOAD DATA LOCAL INFILE 'C:/Users/davel/Desktop/git/rust_db/out.csv' INTO TABLE test.rust_final_table FIELDS TERMINATED BY ',' LINES TERMINATED BY '\n'");


    Ok(())
}

fn generate_connection() -> PooledConn {
    let url = "mysql://root:admin@localhost:3306/test";

    let pool = Pool::new(url).unwrap();

    pool.get_conn().unwrap()
}

fn insert(amount: i32) {
    let url = "mysql://root:admin@localhost:3306/test";

    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    conn.query_drop(
        r"CREATE TABLE rust_table (
                         customer_id int not null,
                         amount int not null,
                         account_name text
                     )",
    ).unwrap();

    let mut payments = Vec::new();

    let mut rng = thread_rng();

    for i in 0..amount {
        payments.push([i as i32, rng.gen_range(0, 20000), 0]);
    }

    println!("finished");

    "INSERT INTO rust_table (customer_id, amount, account_name) VALUES (?, ?, ?)"
        .with(payments.iter().map(|p| (p[0], p[1], p[2])))
        .batch(&mut conn)
        .unwrap();

    println!("Yay!");
}
