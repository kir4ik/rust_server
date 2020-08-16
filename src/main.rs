extern crate ini;
extern crate postgres;

#[path = "cli/cli.rs"]
mod cli;
mod connection;
mod db;
mod structs;

fn main() {
  let conn = connection::get_connection();
  db::create_table(&conn, true);
  cli::main();
}
