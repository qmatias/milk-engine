use rocket::request::Form;
use rocket::response::{Debug, Flash, Redirect};
use std::net::SocketAddr;
use crate::DbConn;
use anyhow::Result;
use chrono::{Duration};
use diesel::result::Error as QueryError;
use super::{db, view};

fn validate_submission(comment: &mut Submission) -> bool {
    comment.author = comment.author.trim().chars().take(30).collect();
    comment.content = comment.content.trim().chars().take(1000).collect();
    let has_empty = comment.author.is_empty() || comment.content.is_empty();
    !has_empty
}

#[post("/comments/post", data = "<submission_form>")]
pub fn post(
    address: SocketAddr,
    submission_form: Form<Submission>,
    conn: DbConn,
) -> Result<Flash<Redirect>, Debug<QueryError>> {
    let ip = address.to_string();
    let redirect = Redirect::to(uri!(view::index: _));

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

#[derive(FromForm, Debug)]
pub struct Submission {
    pub author: String,
    pub content: String,
    pub sell_soul: bool,
}


