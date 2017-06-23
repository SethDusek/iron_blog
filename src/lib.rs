#![feature(custom_derive)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate pulldown_cmark as pulldown;
pub mod schema;
pub use diesel::prelude::*;
pub use diesel::pg::PgConnection;
use schema::posts;
#[derive(Queryable,Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub filename: String,
    pub author: String,
    pub time: i64
}
#[derive(Insertable)]
#[table_name="posts"]
struct Poster<'a> {
    title: &'a str,
    filename: &'a str,
    author: &'a str,
    time: i64
}

impl Post {
    fn inserter<'a>(&'a self) -> Poster<'a> { //generates a Poster type
        Poster {
            title: &self.title,
            filename: &self.filename,
            author: &self.author,
            time: self.time
        }
    }
}

pub struct PostBuilder {
    id: i64,
    title: String,
    filename: String,
    author: String,
    time: i64
}

impl PostBuilder {
    pub fn new() -> PostBuilder {
        PostBuilder {
            id: 0,
            title: String::new(),
            filename: String::new(),
            author: String::new(),
            time: 0
        }
    }
    pub fn id(mut self, val: i64) -> Self {
        self.id = val;
        self
    }
    pub fn title(mut self, val: &str) -> Self {
        self.title.push_str(val);
        self
    }
    pub fn filename(mut self, val: &str) -> Self {
        self.filename.push_str(val);
        self
    }
    pub fn author(mut self, val: &str) -> Self {
        self.author = val.to_owned();
        self
    }
    pub fn time<T: Into<i64>>(mut self, val: T) -> Self {
        self.time = val.into();
        self
    }
    pub fn build(self) -> Post {
        Post {
            id: self.id,
            title: self.title,
            filename: self.filename,
            author: self.author,
            time: self.time
        }
    }

}

        

pub struct Blog {
    connection: PgConnection
}


impl Blog {
    pub fn new(url: &str) -> ConnectionResult<Self> {
        Ok(Blog { connection: PgConnection::establish(url)? })
    }
    pub fn publish(&mut self, post: Post) -> Result<Post, diesel::result::Error> {
        diesel::insert(&post.inserter()).into(posts::table).get_result(&self.connection)
    }
    pub fn connection(self) -> PgConnection {
        self.connection
    }
}

impl std::ops::Deref for Blog {
    type Target = PgConnection;
    fn deref(&self) -> &PgConnection {
        &self.connection
    }
}
