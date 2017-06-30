#![feature(custom_derive)]
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate pulldown_cmark as pulldown;
pub mod schema;
pub use diesel::prelude::*;
pub use diesel::pg::PgConnection;
use schema::posts;
///The post type. This can be retrieved from the Blog type and also used for insertion of new posts
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
struct Poster<'a> { //this poster type is not made public as it is only temporarily created and you dont need to concern yourself with this
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
///The builder type for Post. Use this to initialize the Post struct instead of doing it yourself
pub struct PostBuilder {
    id: i64,
    title: String,
    filename: String,
    author: String,
    time: i64
}

impl PostBuilder {
    ///Create a new PostBuilder with empty/default values
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

        
///The blog type contains the database connection, and can be used to publish or retrieve posts. It
///implements Deref and can be used to access the PgConnection inside
///#Example
/// ```
///let blog = Blog::new(url).unwrap();
///posts.count().get_result::<i64>(&*blog).unwrap()
/// ```
/// 
/// 
pub struct Blog {
    connection: PgConnection
}


impl Blog {
    pub fn new(url: &str) -> ConnectionResult<Self> {
        Ok(Blog { connection: PgConnection::establish(url)? })
    }
    /// Publishes the Post, inserting it into the database. This ignores the id of the Post,
    /// generating its own and returning a new struct with the created id
    pub fn publish(&mut self, post: Post) -> Result<Post, diesel::result::Error> {
        diesel::insert(&post.inserter()).into(posts::table).get_result(&self.connection)
    }
    /// Finds a post within the database that matches the id. Returns None if it cannot find a post with a matching id.
    pub fn find_id(&mut self, id_find: i64) -> Option<Post> {
        posts::table.filter(posts::id.eq(id_find)).first(&self.connection).ok()
    }
    /// Lists all of the posts present in the table at that time 
    pub fn list(&mut self) -> Result<Vec<Post>, diesel::result::Error> {
        posts::table.load::<Post>(&self.connection)
    }
    /// Consumes the Blog to return the PgConnection inside
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
