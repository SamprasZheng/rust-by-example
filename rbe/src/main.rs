mod tuple_demo;
mod display_demo;
mod primitive;
mod ownership;
mod reference;
fn main() {
    // 修改這個變數來切換要執行的部分：
    let mode = 5; 

    match mode {
        1 => tuple_demo::run_tuple_demo(),
        2 => display_demo::run_display_demo(),
        3 => {primitive::primitive();},
        4 => {
            ownership::main();
        }
        5 => {
            reference::main();
        }
        6 => {
            println!("This is a placeholder for future functionality.");
        }
        7 => {
            println!("This is a placeholder for future functionality.");
        }
        8 => {
            println!("This is a placeholder for future functionality.");
        }
        9 => {
            println!("This is a placeholder for future functionality.");
        }
        10 => {
            println!("This is a placeholder for future functionality.");
        }
        11 => {
            println!("This is a placeholder for future functionality.");
        }
        12 => {
            println!("This is a placeholder for future functionality.");
        }       
        13 => {
            println!("This is a placeholder for future functionality.");
        }
        14 => {
            println!("This is a placeholder for future functionality.");
        }
        15 => {
            println!("This is a placeholder for future functionality.");
        }
        _ => {
            println!("Invalid mode selected.");
        }
    }
}