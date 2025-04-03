mod tuple_demo;
mod display_demo;
mod primitive;
fn main() {
    // 修改這個變數來切換要執行的部分：
    let mode = 3; 

    match mode {
        1 => tuple_demo::run_tuple_demo(),
        2 => display_demo::run_display_demo(),
        3 => {primitive::primitive();},
        4 => {
            println!("This is a placeholder for future functionality.");
        }
        _ => {
            println!("Invalid mode selected.");
        }
    }
}