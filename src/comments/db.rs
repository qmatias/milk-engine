use crate::schema::comments::{self, dsl};
use crate::DbConn;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{self, prelude::*};
use std::ops::Sub;

#[table_name = "comments"]
#[derive(Queryable, Insertable, Debug)]
pub struct InsertComment {
    pub ip_address: String,
    pub author: String,
    pub content: String,
}

#[table_name = "comments"]
#[derive(Queryable, Insertable, Debug)]
pub struct QueryComment {
    pub id: i32,
    pub post_time: NaiveDateTime,
    pub ip_address: Option<String>,
    pub author: String,
    pub content: String,
    pub from_engineer: bool,
}

/// Gets all the posts sent in the last `time_frame` from `address`
pub async fn count_recent_from(
    time_frame: Duration,
    ip: String,
    conn: &DbConn,
) -> QueryResult<i64> {
    let cutoff = Utc::now().naive_utc().sub(time_frame);
    conn.run(move |c| {
        dsl::comments
            .filter(dsl::ip_address.eq(ip))
            .filter(dsl::post_time.ge(cutoff))
            .count()
            .get_result(c)
    })
    .await
}

/// Add a post to the database, returns true for success, false for error
pub async fn push_comment(comment: InsertComment, conn: &DbConn) -> QueryResult<()> {
    conn.run(move |c| {
        diesel::insert_into(comments::table)
            .values(&comment)
            .execute(c)
            .map(|_| ())
    })
    .await
}

pub async fn count_comments(conn: &DbConn) -> QueryResult<i64> {
    conn.run(move |c| dsl::comments.count().get_result(c)).await
}

pub async fn list_comments(
    start: i64,
    count: i64,
    conn: &DbConn,
) -> QueryResult<Vec<QueryComment>> {
    conn.run(move |c| {
        dsl::comments
            .order(dsl::post_time.desc())
            .offset(start)
            .limit(count)
            .load(c)
    })
    .await
}
