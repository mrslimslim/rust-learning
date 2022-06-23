/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-19 11:58:29
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-19 22:20:04
 */
use std::collections::HashMap;

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    Mutiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        let mut a = s.split("&");
        for slice_str in a {
            let mut key = slice_str;
            let mut val = "";
            if let Some(i) = slice_str.find("-") {
                key = &slice_str[..i];
                val = &slice_str[i + 1..];
            }
            data.entry(key)
                .and_modify(|exsiting| match exsiting {
                    Value::Single(prev_value) => {
                        *exsiting = Value::Mutiple(vec![prev_value, val]);
                    }
                    Value::Mutiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }
        QueryString { data }
    }
}
