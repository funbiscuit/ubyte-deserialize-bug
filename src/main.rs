use std::fs::File;
use rocket::data::ByteUnit;
use serde::Deserialize;


#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub test: ByteUnit,

}

fn main() {
    let _: AppConfig =
        serde_yaml::from_reader(File::open("config.yaml").unwrap())
            .unwrap();
}
