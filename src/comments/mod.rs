use crate::{util, DbConn};
use anyhow::Result;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;
use std::net::SocketAddr;
use std::ops::Sub;

mod db;

#[derive(Serialize, Debug)]
struct Message {
    name: &'static str,
    msg: String,
}

#[derive(Serialize, Debug)]
struct ListingContext {
    title: &'static str,
    desc: &'static str,
    image: &'static str,
    message: Option<Message>,
    comments: Vec<Comment>,
}

#[derive(FromForm, Debug)]
pub struct Submission {
    pub author: String,
    pub content: String,
    pub sell_soul: bool,
}

#[derive(Serialize, Debug)]
pub struct Comment {
    pub time_passed: String,
    pub author: String,
    pub content: String,
    pub from_engineer: bool,
}

fn validate_submission(comment: &mut Submission) -> bool {
    comment.author = comment.author.trim().chars().take(30).collect();
    comment.content = comment.content.trim().chars().take(1000).collect();
    !comment.author.is_empty() || !comment.content.is_empty()
}

#[post("/comments/post", data = "<submission_form>")]
pub fn post(
    address: SocketAddr,
    submission_form: Form<Submission>,
    conn: DbConn,
) -> Result<Flash<Redirect>> {
    let ip = util::convert_ip(address);
    let redirect = Redirect::to(uri!(index));

    // Check that the IP doesn't have > 3 posts in the last minute
    if 3 <= db::count_recent_from(Duration::minutes(1), &ip, &conn)? {
        return Ok(Flash::error(redirect, "Slow down, cowboy."));
    }

    let mut submission = submission_form.into_inner();
    if !validate_submission(&mut submission) {
        return Ok(Flash::error(
            redirect,
            "Make sure your message and name aren't empty.",
        ));
    }

    let comment = db::InsertComment {
        ip_address: ip,
        author: submission.author,
        content: submission.content,
    };
    db::push_comment(comment, &conn)?;

    if submission.sell_soul {
        Ok(Flash::success(
            redirect,
            "Thanks for the message. And your kidney.",
        ))
    } else {
        Ok(Flash::success(
            redirect,
            "Thanks for the message. I really do hope you come around on the whole kidney thing.",
        ))
    }
}

fn time_since_posted(now: DateTime<Utc>, posted: NaiveDateTime) -> String {
    let duration = now.naive_utc().sub(posted);
    util::format_duration(duration)
}

#[get("/comments")]
pub fn index(conn: DbConn, message: Option<FlashMessage>) -> Result<Template> {
    let now = Utc::now();
    let comments = db::list_comments(10, &conn)?
        .into_iter()
        .map(|c| Comment {
            time_passed: time_since_posted(now, c.post_time),
            author: c.author,
            content: c.content,
            from_engineer: c.from_engineer,
        })
        .collect();

    Ok(Template::render(
        "comments",
        ListingContext {
            title: "Message Board",
            image: "logo.png",
            message: message.map(|f| Message {
                name: util::to_bulma_class(f.name()),
                msg: f.msg().to_owned(),
            }),
            desc: "Cum Engineers - Message Board",
            comments,
        },
    ))
}
