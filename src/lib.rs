#[macro_use] extern crate iron;
extern crate pulldown_cmark as pulldown;

use pulldown::{Parser, html};
use iron::middleware::{Handler, AfterMiddleware, BeforeMiddleware};
use iron::prelude::*;
use std::fs::File;
use std::str;
use std::io::prelude::*;
use std::collections::HashMap;
use iron::status;

struct Blog;

impl AfterMiddleware for Blog {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        let mut resp_body = Vec::new();
        {
            if let Some(ref mut body) = res.body {
                body.write_body(&mut iron::response::ResponseBody::new(&mut resp_body)).unwrap();
            }
        }
        let body = String::from_utf8_lossy(&resp_body);
        let parser = Parser::new(&body);
        let mut html_body = String::from("<html>");
        html::push_html(&mut html_body, parser);
        html_body.push_str("</html>");
        res.headers.set_raw("content-length", vec![format!("{}", html_body.len()).into_bytes()]);                
        res.body = Some(Box::new(html_body));
        Ok(res)
    }
}

impl Handler for Blog {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let query = parse_query(&req.url);
        let mut buf = Vec::new();
        if let Some(id) = query.get("id") {
            let mut file = File::open(id);
            if let Ok(mut reader) = file {
                reader.read_to_end(&mut buf);
            }
            else {
                return Ok(Response::with(status::NotFound));
            }
        }

        Ok(Response::with((status::Ok, itry!(str::from_utf8(&buf)))))
    }
}

#[no_mangle]
pub fn application() -> Box<Handler> {
    let mut chain = Chain::new(Blog);
    chain.link_after(Blog);
    Box::new(chain)
}

fn parse_query(url: &iron::Url) -> HashMap<&str, &str> {
    let mut hm = HashMap::new();    
    if let Some(ref query) = url.query {
        let args = query.split("&").collect::<Vec<&str>>();
        for arg in args {
            let (key, val) = arg.split_at(arg.find("=").unwrap());
            hm.insert(key, &val[1..]);
        }
    }
    println!("{:?}", hm);

    hm
}
