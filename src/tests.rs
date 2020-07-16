use super::rocket;
use rocket::http::Status;
use rocket::local::Client;

#[test]
fn test_routes() {
    test_urls(vec![
        "/",
        "/shop",
        "/shop/engines",
        "/shop/cum",
        "/shop/parts",
        "/about",
    ]);
}

fn test_urls(urls: Vec<&str>) {
    let mut client = Client::new(prepare()).unwrap();
    for url in urls {
        test_url(&mut client, url)
    }
}

fn test_url(client: &mut Client, url: &str) {
    assert_eq!(client.get(url).dispatch().status(), Status::Ok);
}
