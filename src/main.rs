use proxy::Proxy;
fn main() {
    let home_addr = String::from("127.0.0.1:15201");
    let server_addr = String::from("85.17.202.49:15201");
    let mut my_proxy = Proxy::from(home_addr, server_addr).unwrap();
    my_proxy.start();
    loop{}
}
