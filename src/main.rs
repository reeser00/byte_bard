use proxy::Proxy;
use tokio;
#[tokio::main]
async fn main() {
    let home_addr = String::from("127.0.0.1:15201");
    let server_addr = String::from("85.17.202.49:15201");
    let mut my_proxy = Proxy::from(home_addr, server_addr).await.unwrap();
    my_proxy.start().await;
    println!("{:#?}", my_proxy);
    loop{}
}
