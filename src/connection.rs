use ini::Ini;
use postgres::params::{Builder, ConnectParams, Host};
use postgres::{Connection, TlsMode};
use std::str::FromStr;

pub fn get_connection() -> Connection {
  let (connect_params, tls) = get_params();
  Connection::connect(connect_params, tls).unwrap()
}

pub fn get_params() -> (ConnectParams, TlsMode<'static>) {
  let conf = Ini::load_from_file("conf.ini").unwrap();
  let general = conf.general_section();

  let host = general.get("host").unwrap();
  let port = general.get("port").unwrap();
  let tls_mode = general.get("tls_mode").unwrap();
  let db_name = general.get("db_name").unwrap();
  let user = general.get("user").unwrap();
  let pass = general.get("pass").unwrap();

  let tls_mode = match tls_mode.as_ref() {
    "disable" => TlsMode::None,
    "enable" => unimplemented!("tls_mode = enabled"),
    _ => panic!("Wrong tls_mode"),
  };

  let mut params = Builder::new();
  params
    .port(FromStr::from_str(port).unwrap())
    .user(user.as_ref(), Some(pass))
    .database(db_name.as_ref());

  (params.build(Host::Tcp(host.to_owned())), tls_mode)
}

/*
fn get_connect_params() -> String {
  let user = "postgres";
  let pass = "admin";
  let host = "localhost";
  let db_name = "rust";
  let port = 5432;

  format!(
    "postgresql://{user}:{pass}@{host}:{port}/{db_name}",
    user = user,
    pass = pass,
    host = host,
    port = port,
    db_name = db_name
  )
}
*/
