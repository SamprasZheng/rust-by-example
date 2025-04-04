// mod intro; // 引入 foo.rs
// // mod bar; // 引入 bar.rs

// fn main() {
//     // intro::hello();
//     intro::mystery_function("Rustacean");
//     // bar::greet("Rustacean");
// }
extern crate rand;
use rand::Rng;

fn count_characters(s: &str, c: char) -> usize {
    s.chars().filter(|x| *x == c).count()
}

fn main() {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        count_characters("Hello world", 'o');
    } else {
        count_characters("Hello world", 'c');
    }
}