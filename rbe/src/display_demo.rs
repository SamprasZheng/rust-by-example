// src/display_demo.rs

use std::fmt;

pub fn run_display_demo() {
    #[derive(Debug)]
    struct MinMax(i64, i64);

    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    println!("\n--- Display 範例 ---");

    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}",
             big = big_range, small = small_range);

    let point = Point2D { x: 3.3, y: 7.2 };
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let foo = 3735928559u32;
    println!("十進制: {}", foo);
    println!("十六進制: 0x{:X}", foo);
    println!("十六進制: 0x{:x}", foo);
    println!("八進制: 0o{:o}", foo);
    println!("二進制: {:b}", foo);
    println!("科學記號: {:e}", foo);
    println!("科學記號: {:E}", foo);
    println!("記憶體位址: {:p}", &foo);
}
