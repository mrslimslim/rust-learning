/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-06 23:55:01
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-08 00:18:11
 */
use std::fmt::Debug;
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

//  生命周期
fn main() {
    let string1 = String::from("longest string");

    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        let str3: &'static str = "I have static lifetime";
    }

    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {:?} bytes at 0x{:X} stored: {:?}",
        length, pointer, message
    );

    let i = "123";

    print_it(&i);
    // print_it1(&i);
}

fn print_it<T: Debug + 'static>(input: &T) {
    println!("'static value passed in is: {:?}", input);
}

// fn print_it1(input: impl Debug + 'static) {
//     println!("'static value passed in is: {:?}", input);
// }

fn get_memory_location() -> (usize, usize) {
    // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
    // 但持有它的变量 `string` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
    let string = "123";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
    // `string` 在这里被 drop 释放
    // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    // 使用裸指针需要 `unsafe{}` 语句块
    unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

fn ret_static_str<'a>() -> &'a str {
    // 对于字符串字面量来说，它直接被打包到二进制文件中，永远不会被 drop，因此它能跟程序活得一样久，自然它的生命周期是 'static
    let str3: &'a str = "I have static lifetime";
    str3
}

// 在存在多个引用时，编译器有时会无法自动推导生命
// 周期，此时就需要我们手动去标注，通过为参数标注合适的生命周期
// 来帮助编译器进行借用检查的分析。

// 我们在定义该函数时，首先无法知道传递给函数的具体值
// 传入引用的具体生命周期也无法知道，因此也不能像之前的例子那样
// 通过分析生命周期来确定引用是否有效。
// 编译器的借用检查也无法推导出返回值的生命周期，因为它不知道 x 和 y 的生命周期跟返回值的生命周期之间的关系是怎样的
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    x
}

// 在此例中，y 完全没有被使用，因此 y 的生命周期与 x 和返回值的生命周期没有任何关系，意味着我们也不必再为 y 标注生命周期，只需要标注 x 参数和返回值即可。
fn longest_a<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

struct Memo<'a> {
    tip: &'a str,
}

// 函数的返回值如果是一个引用类型，那么它的生命周期只会来源于：
// 函数参数的生命周期
// 函数体中某个新建引用的生命周期

// fn longest_b<'a>(x: &str, y: &str) -> &'a str {
//     let str = String::from("vanish leave function");
//     // cannot return reference to local variable `str`
//     // returns a reference to data owned by the current function
//     &str
// }
fn longest_b<'a>(x: &str, y: &str) -> String {
    let str = String::from("vanish leave function");
    // cannot return reference to local variable `str`
    // returns a reference to data owned by the current function
    str
}

fn failed_borrow<'a>() {
    let _x = 21;
    // `_x` does not live long enough
    // borrowed value does not live long enoughrustcE0597
    // let y: &'a i32 = &_x;
}
