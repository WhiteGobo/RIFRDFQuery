use oxigraph::store::Store;
use std::collections::HashMap;
pub use crate::c2rust_triple::BNodeMap;
use oxrdf::BlankNode;

pub struct RIFRDFQueryGraph {
    pub store: Store,
    bnodemap: HashMap<String, BlankNode>,
}

impl RIFRDFQueryGraph {
    pub fn new(store: Store) -> Self {
        RIFRDFQueryGraph {
            store: store,
            bnodemap: HashMap::new(),
        }
    }
}

impl BNodeMap for RIFRDFQueryGraph {
    fn get_bnode(&mut self, key: &str) -> BlankNode {
        let m = &mut self.bnodemap;
        match m.get(key){
            Some(bnode) => bnode.clone(),
            None => {
                let bnode = BlankNode::default();
                m.insert(key.to_owned(), bnode.clone());
                bnode
            },
        }
    }
}
