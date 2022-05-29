/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-05-28 10:33:13
 * @LastEditors: tengyu
 * @LastEditTime: 2022-05-28 22:34:44
 */
fn main() {
    // println!("Hello, world!");
    // loop

    let str = "Hello World";
    let mut str_word = str.to_string();
    str_word.push_str("!");

    let southern_germany = "Grüß Gott!";
    let chinese = "你好，世界!";
    let japanese = "こんにちは、世界!";
    let regions = [southern_germany, chinese, japanese, &str_word[..]];
    for region in regions.iter() {
        println!("{}", region);
    }
}
