extern crate http;

use http::header::HeaderName;

fn main() {
    // let _h = HeaderName::from_static("special! characters!");  // Bomb!
    let _h = HeaderName::from_static("no-special-characters");
}
