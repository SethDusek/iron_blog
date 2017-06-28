extern crate dotenv;
extern crate iron_blog;
extern crate clap;
extern crate time as libtime;
use iron_blog::*;
use schema::posts;
use std::io::stdin;
use clap::Arg;
use clap::SubCommand;

fn time() -> i64 {
    libtime::get_time().sec
}

fn publish(blog: &mut Blog) -> Result<Post, Box<std::error::Error>> {
    println!("Post name:");
    let stdin = stdin();
    let mut title = String::new();
    stdin.read_line(&mut title)?;
    println!("Author?:");
    let mut author = String::new();
    stdin.read_line(&mut author)?;
    println!("Blogfile location?");
    let mut filename = String::new();
    stdin.read_line(&mut filename)?;
    let post = PostBuilder::new().title(&title).author(&author).filename(&filename).time(time()).build();
    Ok(blog.publish(post)?)
}

fn main() { 
    let url = dotenv::var("DATABASE_URL").unwrap();
    let mut blog = Blog::new(&url).expect("Failed to connect to the blog");
    let args = clap::App::new("Iron Blog")
        .version("0.0")
        .about("A blogging application")
        .subcommand(SubCommand::with_name("publish")
             .help("Publishes a blogpost")
             )
        .get_matches();
    if args.is_present("publish") {
        publish(&mut blog);
    }
}
