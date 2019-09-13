use std::env;
extern crate qrcode;
use qrcode::QrCode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let url = format!("{}{}", "https://www.google.com/search?q=", &args[1..].join("+"));
    let code = QrCode::new(url).unwrap();
    let string = code.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);
}
