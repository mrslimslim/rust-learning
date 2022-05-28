fn main() {
    // println!("Hello, world!");
    // loop
    let southern_germany = "Grüß Gott!";
    let chinese = "你好，世界!";
    let japanese = "こんにちは、世界!";
    let regions = [southern_germany, chinese, japanese];
    for region in regions.iter() {
        println!("{}", region);
    }
}
