/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-04 23:06:18
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-04 23:35:24
 */

use std::{ops::Add, process::Output};

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
    z: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let point_a = Point {
        x: 1 as f64,
        y: 2 as f64,
        z: 3 as f64,
    };
    let point_b = Point {
        x: 2.0,
        y: 3.0,
        z: 5.0,
    };

    let new_point = add(point_a, point_b);

    println!("{:?}", new_point);
}
