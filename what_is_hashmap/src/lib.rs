/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-03 22:10:15
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-04 11:10:27
 */
use std::collections::HashMap;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::new();
    for (index, item) in nums.iter().enumerate() {
        match hash_map.get(item) {
            Some(&i) => return vec![i, index as i32],
            None => hash_map.insert(target - item, index as i32),
        };
    }
    vec![]
}
