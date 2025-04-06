pub fn main() {
    println!("for integer, copy trait is used");
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    
    // in rust s2=s1 is not shallow copy
    // s1 is moved to s2
    println!("for string, move is used in s2=s1");

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    // 哪些类型实现了 Copy trait 



    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 移给 s1
  
    let s2 = String::from("hello");     // s2 进入作用域
  
    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3
   // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
    // 所以什么也不会发生。s1 移出作用域并被丢弃

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
    // 这里 s1 已经被移动到 calculate_length 函数中



}
    
    
    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() 返回字符串的长度
    
        (s, length)
    }
    
  
  fn gives_ownership() -> String {           // gives_ownership 将返回值移动给
                                             // 调用它的函数
  
    let some_string = String::from("yours"); // some_string 进入作用域
  
    some_string                              // 返回 some_string 并移出给调用的函数
  }
  
  // takes_and_gives_back 将传入字符串并返回该值
  fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
  
    a_string  // 返回 a_string 并移出给调用的函数
  }
  