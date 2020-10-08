mod yaml;
mod pipin;
mod app;

pub use self::yaml::YamlDocument;
pub use self::pipin::{spawn_stdin, sleep};
pub use self::app::{run};

const YKRE_LIMIT_ATTEMPT   : u64  = 900;
const YKRE_SLEEP_START     : u64  = 50;
const YKRE_SLEEP_DATA      : u64  = 1;
const YKRE_SLEEP_NODATA    : u64  = 1;
const YKRE_DEFAULT_DEF     : &str = "metadata.name";
const YKRE_ERROR_NOTFOUND  : &str = r"
  :: Ykre ::

  error : no match found for specified pattern
";
const YKRE_ERROR_INVALID   : &str = r"
  :: Ykre ::

  error : invalid parameter. type ykre for usage
";
const YKRE_ERROR_NOPIPE    : &str = r"
  :: Ykre ::

  error : you must send data using pipe
";
const YKRE_HELP            : &str = r"
-----------------------------------------------------------------------------------
 :: Ykre :: v0.1                                license : MIT - author : a-Sansara
-----------------------------------------------------------------------------------

 Ykre is a small program  written in `rust` wich purpose is to extract from a list
 of yaml documents, documents matching a specified condition. Ykre goal is to find 
 specific Kubernetes Resource from the kustomize output.
 Ykre stands for Yaml Kubernetes Resources Ectractor.

 Usage :

     ykre SEARCH [DEF]

     you MUST use pipe with ykre command

     SEARCH - value of kubernetes resource name
     DEF    - the yaml node to look for 
              (default : 'metadata.name', dot is the separator for embed nodes)

     ex :
         # display content of the kubernetes resource 'myResourceName'
         cat kubresources.yaml | ykre 'myResourceName'
         # write content of the kubernetes resource 'pv-dump' into a file
         kustomize build ./config/volumes/local | ykre 'pv-dump' > ./tmp.yaml
         # retriew kubernetes pv resources matching 2Gi disk capacity
         kustomize build ./config/volumes/dev | ykre 2Gi spec.capacity.storage
";
