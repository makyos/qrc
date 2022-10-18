use std::env;
use qrcode::QrCode;

fn main() {

    let args: Vec<String> = env::args().collect();
    let string = &args[1..].join(" ").to_string();

    let code = QrCode::new(&string).unwrap();
    let qr = code.render::<char>()
        .quiet_zone(true)
        .module_dimensions(2, 1)
        .light_color('░')
        .dark_color('█')
        .build();

    println!("{}", qr);
    println!("{}", &string);

}

