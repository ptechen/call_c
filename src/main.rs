// use libc::size_t;

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

// #[link(name="pcre")]
// extern {
// }

/// export LIBRARY_PATH="$LIBRARY_PATH:/opt/homebrew/Cellar/pcre2/10.39/lib"
/// 编译安装也无法找到pcre2
/// 目前无法找到pcre2
#[link(name="pcre2")]
extern {
}

fn main() {
    println!("Hello, world!");
}

