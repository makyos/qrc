use std::env;
extern crate qrcode;
use qrcode::QrCode;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let string = format!("{}{}", "https://www.google.com/search?q=", &args[1..].join("+"));
    let string = format!("{}", &args[1..].join(" "));

    let code = QrCode::new(&string).unwrap();
    let qr = code.render::<char>()
        .quiet_zone(true)
        .module_dimensions(2, 1)
        .light_color(' ')
        .dark_color('█')
        .build();

    println!("{}", qr);
    println!("{}", &string);
}
