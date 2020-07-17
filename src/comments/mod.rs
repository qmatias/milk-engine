mod db;
pub mod view;
pub mod submit;

#[derive(Serialize, Debug)]
struct Message {
    name: &'static str,
    msg: String,
}