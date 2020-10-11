use crate::ykre::*;
use colored::*;
use std::borrow::Cow;

pub fn title(label :&str, version :&str, license: &str, author :&str) {
    let space     = 42+version.len()+label.len()+license.len()+author.len();
    let sep       = format!("{}", CSEP.fg(&"―".repeat(WIDTH+1)));
    let titlesep  = CTITLESEP.fg(" :: ");
    let ptitle    = BG.bg(&format!("{}{}{}", titlesep, CTITLE.fg(label), titlesep)).bold();
    println!("{}"    , sep);
    print!  (" {} "  , ptitle);
    print!  ("{}"    , keyval("version", version, CKEY1, CVAL1).bold());
    print!  ("{}"    , " ".repeat(WIDTH-space));
    print!  ("{}   " , keyval("license", license, CKEY1, CVAL1).bold());
    println!("{}"    , keyval("author", author, CKEY2, CVAL2).bold());
    println!("{}"    , sep);
}

pub fn keyval (key :&str, val :&str, ck : Rgb, cv : Rgb) -> Cow<'static, str> {
    format!("{} {}", ck.fg(&format!("{} :", key)), cv.fg(val)).into()
}

pub fn description(description :&str) {
    println!(" {}", CTEXT.fg(description));
}

pub fn usage() {
    let space_arg : usize = 11;
    let arg1 : &str = "SEARCH";
    let arg2 : &str = "DEF";
    println!(" {} :", CUSAGE.fg("Usage").bold());
    println!("");
    println!(" {}", format!("{} {} {}{}{}", 
        CPROG.fg(&YKRE_NAME.to_lowercase()),
        CARG.fg(&arg1),
        COPTSEP.fg("["),
        CARG.fg(&arg2),
        COPTSEP.fg("]")
    ).bold());
    println!("");
    println!("     {}", CTEXT.fg("you MUST use pipe with ykre command"));
    println!("");
    print!  ("{}{} : ", " ".repeat(space_arg-arg1.len()), CARG.fg(arg1).bold());
    println!("{}"     , CTEXT.fg("value of matching yaml node (kubernetes resource name)"));
    println!("");
    print!  ("{}{} : ", " ".repeat(space_arg-arg2.len()), CARG.fg(arg2).bold());
    println!("{}"     , CTEXT.fg("the yaml node to look for"));
    println!("{}{}"   , " ".repeat(space_arg+3), CTEXT.fg("(default : 'metadata.name', dot is the separator for embed nodes)"));
}

pub fn example() {
    println!("");
    println!(" {} :" , CUSAGE.fg("Examples").bold());
    println!("");
 
    println!("    {}", CCOM.fg("# display content of the kubernetes resource 'myResourceName'"));
    print!  ("    {}", format!("{} {} {}", CPROG.fg("cat"), CARG.fg("'./kubresources.yaml'"), COPTSEP.fg("|")).bold());
    println!(" {}"   , format!("{} {}"   , CPROG.fg("ykre"), CARG.fg("'myResourceName'")).bold());
    println!("");

    println!("    {}", CCOM.fg("# write content of the kubernetes resource 'pv-dump' into a file"));
    print!  ("    {}", format!("{} {} {} {}", CPROG.fg("kustomize"), CARG.fg("build"), CARG.fg("'./config/volumes/local'"), COPTSEP.fg("|")).bold());
    println!(" {}"   , format!("{} {} {} {}", CPROG.fg("ykre"), CARG.fg("'pv-dump'"), COPTSEP.fg(">"), CSTR.fg("./tmp.yaml")).bold());
    println!("");

    println!("    {}", CCOM.fg("# retriew kubernetes pv resources matching 2Gi disk capacity"));
    print!  ("    {}", format!("{} {} {} {}", CPROG.fg("kustomize"), CARG.fg("build"), CARG.fg("'./config/volumes/dev'"), COPTSEP.fg("|")).bold());
    println!(" {}"   , format!("{} {} {}"   , CPROG.fg("ykre"), CARG.fg("'2Gi'"), CARG.fg("'spec.capacity.storage'")).bold());
    println!("");
}

pub fn perror(msg: &str) {
    println!("");
    println!(" {} : {}", format!("{}", CERR.fg("error")).bold(), CERRMSG.fg(msg));
    println!("");
}
