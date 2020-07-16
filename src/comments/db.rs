use crate::schema::comments::{self, dsl};
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::{self, prelude::*};
use std::ops::Sub;

#[table_name = "comments"]
#[derive(Queryable, Insertable, Debug)]
pub struct InsertComment {
    pub ip_address: Vec<u8>,
    pub author: String,
    pub content: String,
}

#[table_name = "comments"]
#[derive(Queryable, Insertable, Debug)]
pub struct QueryComment {
    pub id: i32,
    pub post_time: NaiveDateTime,
    pub ip_address: Option<Vec<u8>>,
    pub author: String,
    pub content: String,
    pub from_engineer: bool,
}

/// Gets all the posts sent in the last `time_frame` from `address`
pub fn count_recent_from(
    time_frame: Duration,
    ip: &[u8],
    conn: &SqliteConnection,
) -> QueryResult<i64> {
    let cutoff = Utc::now().naive_utc().sub(time_frame);
    dsl::comments
        .filter(dsl::ip_address.eq(ip))
        .filter(dsl::post_time.ge(cutoff))
        .count()
        .get_result(conn)
}

/// Add a post to the database, returns true for success, false for error
pub fn push_comment(comment: InsertComment, conn: &SqliteConnection) -> QueryResult<()> {
    diesel::insert_into(comments::table)
        .values(&comment)
        .execute(conn)
        .map(|_| ())
}

pub fn list_comments(count: i64, conn: &SqliteConnection) -> QueryResult<Vec<QueryComment>> {
    dsl::comments
        .order(dsl::post_time.desc())
        .limit(count)
        .load(conn)
}
