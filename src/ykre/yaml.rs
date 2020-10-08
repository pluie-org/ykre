use yaml_rust::{YamlLoader, Yaml, YamlEmitter};
use std::vec::Vec;

#[derive(Debug)]
pub struct YamlDocument {
    path: String,
    content: String,
    docs: Vec<yaml_rust::Yaml>
}

impl YamlDocument {

    fn loaded_from(content: String, source: Option<String>) -> YamlDocument {
        let docs = YamlLoader::load_from_str(&content).unwrap();
        let path = source.unwrap_or("".to_owned());
        YamlDocument { path, content, docs }
    }

    pub fn load_str(content: String) -> YamlDocument {
        Self::loaded_from(content, None)
    }

    pub fn find_str(&self, path: &str, index: usize) -> Option<&str> {
        let search = path.to_string();
        let v: Vec<&str> = search.split(".").collect();
        let mut node: &Yaml = &self.docs[index];
        for key in &v {
            node = &node[*key];
        }
        node.as_str().clone()
    }

    pub fn dump(&self, index: usize) -> String {
        let doc = &self.docs[index];
        let mut out_str = String::new();
        {
            let mut emitter = YamlEmitter::new(&mut out_str);
            emitter.dump(doc).unwrap();
        }
        out_str
    }

    pub fn match_doc(&self, search: &str, matched: &str) -> Option<usize> {
        let mut index = 0;
        let mut rs: Option<usize> = None;
        while index < self.docs.len() {
            match &self.find_str(search, index) {
                Some(ref s) if s == &matched => rs = Some(index),
                _                            => (),
            }
            index += 1;
        }
        rs
    }

    pub fn match_docs(&self, search: &str, matched: &str) -> Vec<usize> {
        let mut v: Vec<usize> = Vec::new();
        let mut index = 0;
        while index < self.docs.len() {
            match &self.find_str(search, index) {
                Some(ref s) if s == &matched => v.push(index),
                _                            => (),
            }
            index += 1;
        }
        v
    }
}
