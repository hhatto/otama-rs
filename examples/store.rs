extern crate otama;

fn main() {
    let config_file = ::std::env::args().nth(1).expect("argument error");
    let o = otama::Otama::new(config_file.as_str());
    println!("otama={:?}", o);

    o.create_database();
}
