#![feature(custom_derive)]
#![feature(plugin)]
#![feature(box_syntax)]
#![plugin(serde_macros)]

#[macro_use] extern crate iron;
#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate serde_yaml;
extern crate pulldown_cmark as pulldown;
extern crate mustache;
extern crate regex;
extern crate time;
//extern crate serde;
//extern crate serde_yaml;

use pulldown::{Parser, html};
use time::{Tm, Timespec};
use std::str::FromStr;
use iron::middleware::{Handler, AfterMiddleware};
use iron::prelude::*;
use std::fs::File;
use std::str;
use std::io::prelude::*;
use mustache::MapBuilder;
use std::collections::HashMap;
use iron::status;
use regex::Regex;

lazy_static! {
    static ref TITLE_REGEX: Regex = Regex::new(r"^# *").unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct Post {
    name: String,
    title: String,
    time: Option<usize>
}

struct Blog;

impl AfterMiddleware for Blog {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        let mut resp_body = String::new();
        if let Some(ref mut body) = res.body {
            body.write_body(&mut iron::response::ResponseBody::new(unsafe { resp_body.as_mut_vec() })).unwrap();
        }
        get_info(&resp_body);
        let post: Post = serde_yaml::from_str(&resp_body).unwrap();
        let mut post_info: Vec<&str> = resp_body.lines().collect();
        //let time = i64::from_str(post_info.remove(0)).unwrap_or(0i64);
        let title = TITLE_REGEX.replace(post_info.remove(0), "");
        let resp_body = post_info.join("\n");        
        let parser = Parser::new(&resp_body);
        let mut html_body = String::new();
        html::push_html(&mut html_body, parser);
        let mut resp_body = Vec::new();
        let template = mustache::compile_path("blog.hbs");                                
        if let Ok(temp) = template {
            let blog_data = MapBuilder::new()
                .insert_str("body", &html_body)
                .insert_str("title", title)
                .insert_str("time", "ay")
                .build();
            temp.render_data(&mut resp_body, &blog_data);
        }
        else { return Ok(Response::with((status::InternalServerError, "Could not load blog template"))); }
        res.headers.set_raw("content-length", vec![format!("{}", resp_body.len()).into_bytes()]);                
        res.body = Some(Box::new(resp_body));
        Ok(res)
    }
}

impl Handler for Blog {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let query = parse_query(&req.url);
        let mut buf = Vec::new();
        if let Some(id) = query.get("id") {
            let file = File::open(id);
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
pub extern fn application() -> Box<Handler> {
    let mut chain = Chain::new(Blog);
    chain.link_after(Blog);
    box chain
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
    hm
}

/// Converts number of seconds since epoch into a date. Assumes seconds given is UTC
fn to_timestamp(time: i64) -> String {
    let timespec = Timespec::new(time, 0);
    let tm = time::at_utc(timespec);
    time::strftime("%A %B %Y", &tm).unwrap()
}

fn get_info(post: &str) -> String {
    let mut lines: Vec<&str> = post.lines().collect();
    let mut yaml_lines = Vec::new();
    if lines.remove(0).starts_with("--") {
        for line in lines {
            if ! line.starts_with("--") {
                yaml_lines.push(line);
            }
            else { break; }
        }
    }
    println!("{:?}", yaml_lines);
    String::from("hi")
}
