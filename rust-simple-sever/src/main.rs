/*
 * @Description:
 * @Version: 2.0
 * @Autor: tengyu
 * @Date: 2022-06-11 11:55:07
 * @LastEditors: tengyu
 * @LastEditTime: 2022-06-12 21:27:20
 */
// 会提升
// use http::method::Method;
// use http::Request;
use server::Server;

// 引入module
mod http;
mod server;

fn main() {
    let server = Server::new("localhost:8080".to_string());
    server.run();
}

/*
    GET /user?id=10 HTTP/1.1\r\n
    HEADERS \r\n
    BODY;
*/

fn foo<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
