mod clean;
mod dirty;

use clean::*;
use dirty::*;
fn main() {
     // This was the cleanest way I could think of to run both versions of the code
     // Following the Rust way of doing things, but not the most beautiful.
     clean();

    // And this was my functional wanna be way of doing things.
    // Just an implementation of a self cloning and ordering List struct
    // ! This is bad for memory and performance, but it's was a good exercise...
     dirty();
}
