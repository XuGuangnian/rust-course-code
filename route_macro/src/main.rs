use route_macro_attribute::route;

fn main() {
    println!("Hello, world!");
}

#[route(GET, "/")]
fn index() {}
