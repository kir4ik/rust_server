use crate::structs::Person;
use ::postgres::Connection;

pub fn create_table(conn: &Connection, if_not_exists: bool) {
  let query_string = format!(
    "CREATE TABLE{} person (
    id              SERIAL PRIMARY KEY,
    name            VARCHAR NOT NULL,
    data            BYTEA
    )",
    if if_not_exists { " IF NOT EXISTS" } else { "" }
  );

  conn.execute(&query_string, &[]).unwrap();
}

pub fn insert(conn: &Connection, person: &Person, clear: bool) {
  if clear {
    conn.execute("DELETE FROM person WHERE id <> $1", &[&1]).unwrap();
  }

  conn
    .query(
      "INSERT INTO person (name, data) VALUES ($1, $2)",
      &[&person.name, &person.data],
    )
    .unwrap();
}

pub fn get_all(conn: &Connection) -> Vec<Person> {
  let stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
  let mut persons = vec![];

  for row in &stmt.query(&[]).unwrap() {
    persons.push(Person {
      id: row.get("id"),
      name: row.get("name"),
      data: row.get("data"),
    });
  }

  persons
}

pub fn remove(conn: &Connection, ids: &Vec<i32>) {
  let transaction = conn.transaction().unwrap();

  let stmt = transaction.prepare("DELETE FROM person WHERE id = $1").unwrap();
  for id in ids {
    stmt.execute(&[id]).unwrap();
  }

  transaction.set_commit();
}

pub fn update(conn: &Connection, person: &Person) {
  let stmt = conn
    .prepare("UPDATE person SET name = $2, data = $3 WHERE id = $1")
    .unwrap();
  stmt.query(&[&person.id, &person.name, &person.data]).unwrap();
}

pub fn format(persons: &Vec<Person>) {
  for item in persons {
    println!(
      "Person >>> (id: {}) (name: {}) (data: {:?})",
      item.id, item.name, item.data
    );
  }
}
