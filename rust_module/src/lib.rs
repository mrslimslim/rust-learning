// 引入animate 
mod utils;
mod animate;

pub use crate::animate::animation;

mod local_math {
    pub mod add {
        pub fn add(a: i32, b: i32) -> i32 {
          let c =  a + b;
          println!("{}", c);
          c
        }
    }

    // pub mod sub {
    //     pub fn sub(a: i32, b: i32) -> i32 {
    //         a - b
    //     }
    // }
    
    // pub mod times {
    //     pub fn times(a: i32, b: i32) -> i32 {
    //         a * b
    //     }
    // }

    // pub mod div {
    //     pub fn div(a: i32, b: i32) -> i32 {
    //         a / b
    //     }
    // }

    pub fn get_number() -> i32 {
        let number = 5;
        super::output_number() + number
    }
}

fn output_number() -> i32 {
    let c = 1;
    c
}

pub fn do_math() {
    // 相对路径
    local_math::add::add(1, 2);

    // 绝对路径
    crate::local_math::add::add(1, 2);

    crate::local_math::get_number();

    self::local_math::add::add(1, 2);

    animation::move_position();
}