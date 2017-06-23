extern crate dotenv;
extern crate iron_blog;
use iron_blog::*;
use schema::posts::dsl::*;


fn main() {
    let url = dotenv::var("DATABASE_URL").unwrap();
    let mut blog = Blog::new(&url).unwrap();
    let post = PostBuilder::new().title("My first post").filename("foo.rs").author("Shibe").build();
    blog.publish(post);
    println!("{:?}", posts.first::<Post>(&*blog).unwrap());
    println!("{} posts", posts.count().get_result::<i64>(&*blog).unwrap());
}
