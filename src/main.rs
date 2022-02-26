// use libc::size_t;
//
// #[link(name="snappy")]
// extern {
//     fn snappy_max_compressed_length(source_length: size_t) -> size_t;
// }
//
// fn main() {
//     let x = unsafe {
//         snappy_max_compressed_length(100)
//     };
//     println!("Hello, world! {}", x);
// }

use libc::size_t;

#[link(name="pcre2")]
extern {
}

fn main() {
    println!("Hello, world!");
}

