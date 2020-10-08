use std::panic;
use std::env;
use std::sync::mpsc::TryRecvError;

use crate::ykre::*;

pub fn find(search: &str, def: &str, data:String) {
    let doc = YamlDocument::load_str(data);
    let rs  = doc.match_docs(&def, &search);
    if !rs.is_empty() {
        for i in rs.iter() {
            println!("{}", doc.dump(*i));
        }
        return
    } 
    panic!(YKRE_ERROR_NOTFOUND);
}

pub fn get_data(buf :&mut String, limit :u32) -> bool {
    let stdin = spawn_stdin();
    let mut attempt = 0;
    let mut has_data :bool = false;
    sleep(YKRE_SLEEP_START);
    while attempt < limit {
        let rs = match stdin.try_recv() {
            Ok(s) => s,
            Err(TryRecvError::Empty) => String::from(""),
            Err(TryRecvError::Disconnected) => String::from(""),
        };
        if !rs.is_empty() {
            *buf += &rs;
            has_data = true;
        }
        else if has_data {
            break;
        }
        attempt += 1;
        sleep(match has_data {
            true => YKRE_SLEEP_DATA,
            _    => YKRE_SLEEP_NODATA,
        });
    }
    return has_data
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", YKRE_HELP);
        return
    }
    else if args.len() > 1 && args.len() <= 3 {

        let mut buf :String = String::from("");
        let search  :&str   = &args[1];
        let def     :&str   = match args.len() {
            3 => &args[2],
            _ => YKRE_DEFAULT_DEF,
        };
        let limit : u32  = env::var("YKRE_LIMIT_ATTEMPT").unwrap_or(format!("{}", YKRE_LIMIT_ATTEMPT).to_owned()).parse::<u32>().unwrap();
        if get_data(&mut buf, limit) {
            return find(search, def, buf);
        }
        panic!(YKRE_ERROR_NOPIPE)
    }
    panic!(YKRE_ERROR_INVALID)
}
