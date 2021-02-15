#[macro_use]
extern crate clap;
use clap::App;
use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let keys = matches.args.keys();
    for key in keys {
        if key == &"from-dia" || key == &"f" {
            let path = Path::new(matches.value_of(key).unwrap());
            if !path.exists() || !path.is_file() {
                println!("Path {:?} not exists!", path);
            }
        }
        continue;
    }
}
