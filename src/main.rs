#[macro_use]
extern crate nickel;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, MediaType};
use std::collections::HashMap;

// this function add header to response for example now we add application/json
fn content_type<'a>(_: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
        //MediaType can be any valid type for reference see
        res.content_type(MediaType::Json);
        res.send( "{'foo':'bar'}")
}

fn tmpl_handler<'a> (_: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
    let mut data = HashMap::<&str, &str>::new();
    // add data for render
    // name = {{ name }} in template
    // page_title = {{ page_title }}
    data.insert("name", "Jule");// change "Alex" to your name )
    data.insert("page_title", "lesson 2");
    res.render("app/views/index.tpl", &data)
}

fn main() {
    let mut server = Nickel::new();

    //middleware function logs each request to console
    server.utilize(middleware! { |request|
        println!("logging request: {:?}", request.origin.uri);
    });

    // start using router
    let mut router = Nickel::router();

    //works only on route http://localhost:8080
    router.get("/", tmpl_handler);

    // go to http://localhost:8080/content-type to see this route in action
    router.get("/content-type", content_type);

    server.utilize(router);
    // you can change 8080 to any port
    server.listen("127.0.0.1:8080");
}
