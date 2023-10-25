extern crate csv;
extern crate rusqlite;

use csv::Reader;
use rusqlite::{Connection, Result};
use std::env;
use std::error::Error;
use std::fs::File;
use std::process;
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the SQLite database file path and CSV file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <database_file> <csv_file>", args[0]);
        process::exit(1);
    }

    let db_path = &args[1];
    let csv_file_path = &args[2];

    // Open the SQLite database connection
    let conn = Connection::open(db_path)?;

    // Create a table to store the CSV data
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data (column1 TEXT, column2 TEXT, column3 TEXT)",
        [],
    )?;

    // Open the CSV file and insert its data into the SQLite database
    let file = File::open(csv_file_path)?;
    let mut rdr = Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        conn.execute(
            "INSERT INTO data (column1, column2, column3) VALUES (?1, ?2, ?3)",
            &[&record[0], &record[1], &record[2]],
        )?;
    }

    println!("CSV data imported into the SQLite database.");

    // Allow SQL queries from the terminal
    loop {
        let mut input = String::new();
        print!("Enter an SQL query (or 'exit' to quit): ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin().read_line(&mut input)?;

        let query = input.trim();
        if query == "exit" {
            break;
        }

        match conn.execute_batch(query) {
            Ok(_) => println!("Query executed successfully."),
            Err(err) => eprintln!("Error executing query: {}", err),
        }
    }

    Ok(())
}
