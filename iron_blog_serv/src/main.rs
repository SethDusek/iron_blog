extern crate dotenv;
extern crate iron_blog;
extern crate clap;
extern crate time as libtime;
extern crate termion;
use iron_blog::*;
use std::io::stdin;
use clap::SubCommand;
use std::str::FromStr;
use termion::terminal_size;

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

fn print_post(post: &Post) {
    println!("Title: {}
Id: {}
By: {}
On: {}
", post.title, post.id, post.author, post.time);
}

fn list(blog: &mut Blog) -> Result<(), Box<std::error::Error>> {
    let list = blog.list()?;
    let columns = terminal_size()?.0;
    let mut line = String::with_capacity(columns as usize);
    for _ in 0..columns {
        line.push('-');
    }
    for i in &list {
        print_post(i);
        println!("{}", line);
    }
    Ok(())
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
        .subcommand(SubCommand::with_name("list")
                    .help("Lists all the posts in the table")
                    )
        .get_matches();
    if args.is_present("publish") {
        publish(&mut blog).expect("Failed to publish");
    }
    else if args.is_present("list") {
        list(&mut blog).expect("Failed to list");
    }
}
