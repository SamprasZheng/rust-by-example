pub fn hello() {
    println!("Hello from foo!");
}


// fn mystery_function(input: &str) -> i32 {
//     let mysteries = "aeiouAEIOU";
//     input.chars().filter(|c| mysteries.contains(*c)).count() as i32
//     // let mystery_count = mystery_function(&input_string);
//     println!("The number of mysteries in '{}' is {}", input_string, input.chars().filter(|c| mysteries.contains(*c)).count() as i32);
// }

pub fn mystery_function(input: &str) -> i32 {
    let mysteries = "aeiouAEIOU";
    let count = input.chars().filter(|c| mysteries.contains(*c)).count() as i32;
    println!("The number of mysteries in '{}' is {}", input, count);
    count
}


