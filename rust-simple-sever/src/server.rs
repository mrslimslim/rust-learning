/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-11 15:03:52
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-11 21:08:55
 */
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;
pub struct Server {
    addr: String,
}

impl Server {
    // 两种不同的写法
    // fn new(addr: String) -> Server {
    //     Server { addr }
    // }
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(&self) {
        println!("Listening on {}", self.addr);
        // 返回了一个Result Enum
        // TODO学习错误处理
        let listenerResult = TcpListener::bind(&self.addr);
        let listener = match listenerResult {
            Ok(i) => i,
            Err(err) => {
                panic!("Error is {}", err)
            }
        };

        loop {
            // 退出
            // break;
            // let result = listener.accept();
            // Bad
            // if result.is_err() {
            //     continue;
            // }
            // let (stream, addr) = result.unwrap();

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf: [u8; 1024] = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Receive a request: {}", String::from_utf8_lossy(&buf));
                            match Request::try_from(&buf as &[u8]) {
                                Ok(r) => {}
                                Err(e) => {}
                            }
                        }
                        Err(err) => println!("Failed to read {}", err),
                    }
                }
                Err(err) => println!("Failed to establish a connection: {}", err),
                // default situation 情况
                // _ => {}
            }
        }
    }
}
