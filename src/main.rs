extern crate yaml_rust;
pub mod ykre;

use std::io;
use std::panic;

const PANIC_BYPASS_FILE    : &str = "src/ykre/pipin.rs";
const PANIC_BYPASS_LINE    : u32  = 9;

fn main() -> io::Result<()> {

    panic::set_hook(Box::new(|_info| {
        let mut bypass :bool = false;
        if let Some(location) = &_info.location() {
            bypass = location.file() == PANIC_BYPASS_FILE && location.line() == PANIC_BYPASS_LINE;
        }
        if !bypass {
            //~ println!("{:?}", &_info);
            if let Some(s) = &_info.payload().downcast_ref::<&str>() {
                if s.len() > 1 {
                    println!("{}", s);
                }
            }
        }
    }));

    ykre::run();
    Ok(())
}
