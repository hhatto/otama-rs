extern crate otama;
extern crate glob;

fn main() {
    let config_file = ::std::env::args().nth(1).expect("argument error");
    let mut o = otama::Otama::new(config_file.as_str()).expect("otama error");
    println!("otama={:?}", o);

    match o.create_database() {
        Err(e) => println!("create_database() error. e={:?}", e),
        _ => {},
    }

    for entry in glob::glob("./image/*.[jp][pn]g").expect("invalid glob pattern") {
        match entry {
            Ok(path) => {
                match o.insert(path.to_str().unwrap()) {
                    Err(e) => println!("insert() error. e={:?}", e),
                    Ok(id) => println!("insert id={:?} file={:?}. ", id, path),
                }
            },
            Err(e) => println!("error={:?}", e),
        }
    }

    match o.pull() {
        Err(e) => println!("pull() error. e={:?}", e),
        _ => {},
    }

    match o.search(10, "./image/lena.jpg") {
        Err(e) => println!("search() error. e={:?}", e),
        Ok(v) => {
            for ret in v {
                println!("{:?}", ret);
            }
        },
    }
}
