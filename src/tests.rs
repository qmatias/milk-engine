use super::rocket;
use crate::build_rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_routes() {
    test_urls(vec![
        "/",
        "/shop",
        "/shop/engines",
        "/shop/milk",
        "/shop/parts",
        "/about",
        "/comments",
    ]);
}

fn test_urls(urls: Vec<&str>) {
    let mut client = Client::new(build_rocket()).unwrap();
    for url in urls {
        test_url(&mut client, url)
    }
}

fn test_url(client: &mut Client, url: &str) {
    assert_eq!(client.get(url).dispatch().status(), Status::Ok);
}
