#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello from my custom application!");
    
    // 计算斐波那契数列的前10个数字
    let mut a = 0;
    let mut b = 1;
    
    println!("Fibonacci sequence:");
    println!("{}", a);
    println!("{}", b);
    
    for _ in 2..10 {
        let temp = a + b;
        println!("{}", temp);
        a = b;
        b = temp;
    }
    
    println!("Custom application completed successfully!");
    0  // 返回成功退出码
}

