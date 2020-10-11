extern crate yaml_rust;
extern crate colored;

pub mod ykre;

fn main() -> std::io::Result<()> {
    ykre::run()
}
