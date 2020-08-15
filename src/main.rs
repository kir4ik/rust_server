extern crate ini;
extern crate postgres;

use postgres::Connection;

mod connection;

fn main() {
  let conn = connection::get_connection();
  push_fake_data(conn);
}
struct Person {
  id: i32,
  name: String,
  data: Option<Vec<u8>>,
}

fn push_fake_data(conn: Connection) {
  conn
    .execute(
      "CREATE TABLE IF NOT EXISTS person (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR NOT NULL,
    data            BYTEA
  )",
      &[],
    )
    .unwrap();
  let me = Person {
    id: 0,
    name: "Steven".to_owned(),
    data: None,
  };
  conn.execute("DELETE FROM person WHERE id <> $1", &[&1]).unwrap();
  conn
    .execute("INSERT INTO person (name, data) VALUES ($1, $2)", &[&me.name, &me.data])
    .unwrap();

  let stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();

  for row in &stmt.query(&[]).unwrap() {
    let person = Person {
      id: row.get("id"),
      name: row.get("name"),
      data: row.get("data"),
    };
    println!(
      "Found person >>> (id: {}) (name: {}) (data: {:?})",
      person.id, person.name, person.data
    );
  }
}
