use super::{db, Message};
use crate::{util, DbConn};
use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::result::Error as QueryError;
use rocket::request::FlashMessage;
use rocket::response::Debug;
use rocket_contrib::templates::Template;
use std::ops::Sub;

fn time_since_posted(now: DateTime<Utc>, posted: NaiveDateTime) -> String {
    let duration = now.naive_utc().sub(posted);
    util::format_duration(duration)
}

#[derive(Serialize, Debug)]
pub struct Comment {
    pub time_passed: String,
    pub author: String,
    pub content: String,
    pub from_engineer: bool,
}

fn get_paging(page: Option<i64>, comment_count: i64) -> (i64, Paging) {
    let mut max = comment_count / PAGE_SIZE;
    if comment_count % PAGE_SIZE != 0 {
        max += 1
    }

    let current = page.unwrap_or(1).clamp(1, max);

    let prev = (1 < current).then_some(current - 1);
    let next = (current < max).then_some(current + 1);

    let first = (2 < current).then_some(1);
    let last = (current < max - 1).then_some(max);

    (
        current - 1,
        Paging {
            first,
            prev,
            current,
            next,
            last,
        },
    )
}

#[derive(Serialize, Debug)]
struct ListingContext {
    title: &'static str,
    desc: &'static str,
    image: &'static str,
    message: Option<Message>,
    comments: Vec<Comment>,
    paging: Paging,
}

#[get("/comments?<page>")]
pub async fn index(
    page: Option<i64>,
    conn: DbConn,
    message: Option<FlashMessage<'_, '_>>,
) -> Result<Template, Debug<QueryError>> {
    let now = Utc::now();

    let comment_count = db::count_comments(&conn).await?;
    let (index, paging) = get_paging(page, comment_count);

    let comments = db::list_comments(index * PAGE_SIZE, PAGE_SIZE, &conn).await?
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
            image: "icon.png",
            message: message.map(|f| Message {
                name: util::to_bulma_class(f.name()),
                msg: f.msg().to_owned(),
            }),
            desc: "Cum Engineers - Message Board",
            paging,
            comments,
        },
    ))
}

const PAGE_SIZE: i64 = 10;

#[derive(Serialize, Debug)]
struct Paging {
    last: Option<i64>,
    prev: Option<i64>,
    current: i64,
    next: Option<i64>,
    first: Option<i64>,
}
