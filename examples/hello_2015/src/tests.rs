use rocket::{self, routes, local::Client};

#[test]
fn hello_world() {
    let rocket = rocket::ignite().mount("/", routes![super::hello]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Hello, Rust 2015!".into()));
}

// Tests unrelated to the example.
mod scoped_uri_tests {
    use rocket::{get, routes};

    mod inner {
        use rocket::uri;

        #[rocket::get("/")]
        pub fn hello() -> String {
            format!("Hello! Try {}.", uri!(super::hello_name: "Rust 2015"))
        }
    }

    #[get("/<name>")]
    fn hello_name(name: String) -> String {
        format!("Hello, {}! This is {}.", name, rocket::uri!(hello_name: &name))
    }

    fn rocket() -> rocket::Rocket {
        rocket::ignite()
            .mount("/", routes![hello_name])
            .mount("/", rocket::routes![inner::hello])
    }

    use rocket::local::Client;

    #[test]
    fn test_inner_hello() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/").dispatch();
        assert_eq!(response.body_string(), Some("Hello! Try /Rust%202015.".into()));
    }

    #[test]
    fn test_hello_name() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/Rust%202015").dispatch();
        assert_eq!(response.body_string().unwrap(), "Hello, Rust 2015! This is /Rust%202015.");
    }
}
