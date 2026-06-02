use oxigraph::store::Store;
use oxigraph::model::NamedOrBlankNodeRef;

pub fn print_node_info(graph: &Store, node: NamedOrBlankNodeRef) {
    eprintln!("print node");

    for q in graph.quads_for_pattern(Some(node), None, None, None){
        match q {
            Ok(qok) => {eprintln!("{:?}", qok);},
            Err(e) => {eprintln!("{:?}", e);},
        }
    }
}
