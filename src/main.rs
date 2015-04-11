extern crate iron;
extern crate rusqlite;

use iron::prelude::*;
use iron::status;
use rusqlite::SqliteConnection;

struct Person {
    name: String
}

fn main() {

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let me = Person {
            name: "Juuuule".to_string()
        };
        let conn = SqliteConnection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE hello (
            name   TEXT NOT NULL
            )", &[]).unwrap();
        conn.execute("INSERT INTO hello (name)
                      VALUES ($1)",
                      &[&me.name]).unwrap();
        let mut result_person = Person {
            name: "".to_string()
        };
        let mut stmt = conn.prepare("SELECT name FROM hello").unwrap();
        for row in stmt.query(&[]).unwrap().map(|row| row.unwrap()) {
            result_person = Person{
                name: row.get(0)
            };
            //result = String::from_utf8(blub).unwrap();
        };
        Ok(Response::with((status::Ok, result_person.name)))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
