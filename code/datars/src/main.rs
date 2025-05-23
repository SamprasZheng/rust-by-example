extern crate rand;
use rand::Rng;

fn count_characters(s: &str, c: char) -> usize {
    s.chars().filter(|x| *x == c).count()
}

fn main() {
    let mut rng = rand::thread_rng();
    let result = if rng.gen_bool(0.5) {
        count_characters("Hello world", 'o')
    } else {
        count_characters("Hello world", 'l')
    };

    println!("Matched characters: {}", result);

    // Uncommenting the following lines will cause a compilation error
    // let x: u32 = -200;
    // println!("x = {}", x);

    

}
