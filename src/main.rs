extern crate warp;

use warp::Filter;

fn main() {
    let hello = path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030));
}

