use egui::Vec2;
//use proxy::Proxy;
use ui::App;


#[tokio::main]
async fn main() {
    /* let home_addr = String::from("127.0.0.1:15201");
    let server_addr = String::from("85.17.202.49:15201");
    let mut my_proxy = Proxy::from(home_addr, server_addr).await.unwrap();
    my_proxy.start().await;
    println!("{:#?}", my_proxy); */

    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder {
            fullscreen: Some(false),
            min_inner_size: Some(Vec2{x: 1920., y: 1080.}),
            ..Default::default()
        },
        follow_system_theme: true,
        persist_window: true,
        ..Default::default()
    };

    run_gui(options).await;


    
}

async fn run_gui(options: eframe::NativeOptions) {
    
    let _ = eframe::run_native(
        "ByteBard",
        options,
        Box::new(move |cc| Box::new(App::new(cc))) // Clone Arc for closure
    );

}
