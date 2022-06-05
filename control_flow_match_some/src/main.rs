use std::{cmp::Ordering, io::stdin};

/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-04 23:41:27
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-05 00:12:36
 */
use rand::{self, Rng};
use std::io;
fn main() {
    let target_number = rand::thread_rng().gen_range(0..100);
    println!("target_number is {}", &target_number);
    loop {
        let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("i need a number");
        let guess: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&target_number) {
            Ordering::Equal => {
                println!("Got it");
                break;
            }
            Ordering::Greater => {
                println!("Too Big");
            }
            Ordering::Less => {
                println!("Too Small");
            }
        }
    }
}
