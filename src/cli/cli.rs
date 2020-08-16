use crate::connection::get_connection;
use crate::db;
use crate::structs::Person;

const HELP: &'static str = "Usage: phonebook COMMAND [ARG]...
Commands:
    add NAME PHONE - create new record;
    del ID1 ID2... - delete record;
    edit ID        - edit record;
    show           - display all records;
    show STRING    - display records which contain a given substring in the name;
    help           - display this help.";

pub fn main() {
  let args: Vec<String> = std::env::args().collect();
  let conn = get_connection();

  match args.get(1) {
    Some(text) => match text.as_ref() {
      "add" => {
        if args.len() != 4 {
          panic!("Usage: phonebook add NAME PHONE");
        }

        db::insert(
          &conn,
          &Person {
            id: 0,
            name: args[2].to_string(),
            // data: args[3],
            data: None,
          },
          false,
        );
      }
      "del" => {
        if args.len() < 3 {
          panic!("Usage: phonebook del ID...");
        }

        let ids: Vec<i32> = args[2..].iter().map(|s| s.parse().unwrap()).collect();
        db::remove(&conn, &ids);
      }
      "edit" => {
        if args.len() != 5 {
          panic!("Usage: phonebook edit ID NAME PHONE");
        }

        let id = args[2].parse().unwrap();
        db::update(
          &conn,
          &Person {
            id,
            name: args[3].to_string(),
            data: None,
          },
        );
      }
      "show" => {
        if args.len() > 3 {
          panic!("Usage: phonebook show [SUBSTRING]");
        }

        if args.len() == 3 {
          // like =
          unimplemented!();
        }

        let persons = db::get_all(&conn);
        db::format(&persons);
      }
      "help" => {
        println!("{}", HELP);
      }
      command @ _ => panic!(format!("Invalid command: {}", command)),
    },
    None => panic!("No command supplied"),
  }
}
