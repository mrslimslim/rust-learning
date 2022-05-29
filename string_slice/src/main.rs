/*
 * @Description: ;
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-05-29 20:51:25
 * @LastEditors: tengyu
 * @LastEditTime: 2022-05-29 21:01:21
 */
fn main() {
    let str = String::from("Hello, world!");
    let char_arr = str.as_bytes();
    for &char_str in char_arr.iter() {
        println!("{}", char_str as char);
    }
    // let s = String::from("Hello, world!");

    // let bytes = s.as_bytes();

    // for (i, &item) in bytes.iter().enumerate() {
    //     println!("{} {}", i, item as char);
    // }
}
