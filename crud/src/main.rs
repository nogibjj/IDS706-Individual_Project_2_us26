extern crate rusqlite;
use rusqlite::{Connection, Result};
use std::env;
use std::io;
use std::process;

fn main() -> Result<()> {
    // Get the SQLite database file path from a command-line argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <database_file>", args[0]);
        process::exit(1);
    }

    let db_path = &args[1];
    let conn = Connection::open(db_path)?;

    loop {
        // Read and execute SQL queries from the command line
        let mut input = String::new();
        println!("Enter an SQL query (or 'exit' to quit):");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        if input.trim() == "exit" {
            break;
        }

        match conn.execute_batch(&input) {
            Ok(_) => {
                // Print the result of the query
                if input.to_lowercase().contains("select") {
                    let mut stmt = conn.prepare(&input)?;
                    let rows = stmt.query_map([], |row| {
                        let result: String = row.get(0)?;
                        Ok(result)
                    })?;

                    for row in rows {
                        if let Ok(result) = row {
                            println!("Result: {}", result);
                        }
                    }
                }
                println!("Query executed successfully.");
            }
            Err(err) => {
                eprintln!("Error executing query: {:?}", err);
            }
        }
    }

    Ok(())
}
