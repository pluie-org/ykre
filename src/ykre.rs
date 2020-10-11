mod yaml;
mod pipin;
mod app;
mod echo;
mod color;

pub use self::yaml::YamlDocument;
pub use self::pipin::*;
pub use self::echo::*;
pub use self::color::*;
pub use self::app::run;

const BG                   : Rgb   = Rgb { r: 30, g: 85, b:125 };
const CSEP                 : Rgb   = Rgb { r: 30, g: 85, b:125 };
const CTITLE               : Rgb   = Rgb { r:255, g:220, b:190 };
const CTITLESEP            : Rgb   = Rgb { r:255, g:175, b: 35 };
const CKEY1                : Rgb   = Rgb { r: 23, g:141, b: 84 };
const CVAL1                : Rgb   = Rgb { r:149, g:197, b:157 };
const CKEY2                : Rgb   = Rgb { r:237, g:214, b:140 };
const CVAL2                : Rgb   = Rgb { r:238, g:194, b: 55 };
const CTEXT                : Rgb   = Rgb { r:190, g:162, b:216 };
const CPROG                : Rgb   = Rgb { r:255, g:255, b:255 };
const COPTSEP              : Rgb   = Rgb { r:255, g:151, b: 75 };
//~ const COPT                 : Rgb   = Rgb { r: 23, g:141, b: 84 };
const CARG                 : Rgb   = Rgb { r: 90, g:205, b:255 };
const CUSAGE               : Rgb   = Rgb { r:238, g:194, b: 55 };
const CSTR                 : Rgb   = Rgb { r: 66, g:135, b: 99 };
const CCOM                 : Rgb   = Rgb { r:105, g:103, b:149 };
const CERR                 : Rgb   = Rgb { r:175, g: 37, b: 21 };
const CERRMSG              : Rgb   = Rgb { r:210, g: 99, b: 86 };

const WIDTH                : usize = 82;
const PANIC_BYPASS_LINE    : u32   = 9;
const PANIC_BYPASS_FILE    : &str  = "src/ykre/pipin.rs";
const YKRE_AUTHOR          : &str  = "a-Sansara";
const YKRE_VERSION         : &str  = "0.2";
const YKRE_LICENSE         : &str  = "MIT";
const YKRE_NAME            : &str  = "Ykre";
const YKRE_DEFAULT_DEF     : &str  = "metadata.name";
const YKRE_LIMIT_ATTEMPT   : u64   = 1500;
const YKRE_SLEEP_START     : u64   = 50;
const YKRE_SLEEP_DATA      : u64   = 1;
const YKRE_SLEEP_NODATA    : u64   = 1;
const YKRE_ERROR_NOTFOUND  : &str  = "no match found for specified pattern";
const YKRE_ERROR_INVALID   : &str  = "invalid parameter. Type ykre for help";
const YKRE_ERROR_NOPIPE    : &str  = "you must send data using pipe. Type ykre for help";
const YKRE_DESCRIPTION     : &str  = r"
 Ykre is a small program  written in `rust` wich purpose is to extract from a list
 of yaml documents, documents matching a specified condition. Ykre goal is to find 
 specific Kubernetes Resource from the kustomize output.
 Ykre stands for Yaml Kubernetes Resources Extractor.
";
