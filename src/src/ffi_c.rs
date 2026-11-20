use std::ptr;
use std::ffi::{c_char, CStr, c_void};
use oxigraph::store::Store;
use crate::rifquery::rifquery;
use crate::rifrdfQueryGraph::RIFRDFQueryGraph;

#[unsafe(no_mangle)]
pub extern "C" fn RIFRDFQuery_Graph_query_rif_data(
    data_graph: *mut RIFRDFQueryGraph,
    query_graph: *mut RIFRDFQueryGraph,
    ) -> i8
{
    if data_graph.is_null() {
        eprintln!("RIFRDFQuery_Graph_query_rif_data got NULL as  data_graph");
        return -2;
    }
    if query_graph.is_null() {
        eprintln!("RIFRDFQuery_Graph_query_rif_data got NULL as query_graph");
        return -3;
    }
    let dgraph: &Store = unsafe{&(*data_graph).store};
    let qgraph: &Store = unsafe{&(*query_graph).store};
    //TODO: This will produce an error output, on local values but will still
    //return as expected a normal 1 as for not found.
    //This is strange and should be overthought.
    if rifquery(dgraph, qgraph){
        return 0;
    } else {
        return 1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFRDFQuery_Graph_new() -> *mut RIFRDFQueryGraph
{
    use oxrdfio::{RdfFormat, RdfParser, ReaderQuadParser};
    use oxrdf::Quad;
    use oxigraph::store::{Store};
    let store = match Store::new() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Internal RIFRDFQuery Error. Init of store failed {}", e);
            return ptr::null_mut();
        },
    };
    let mybox = Box::new(RIFRDFQueryGraph::new(store));
    Box::into_raw(mybox)
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFRDFQuery_Graph_add(
        subject: *const c_char, subject_type: u8,
        predicate: *const c_char,
        object: *const c_char, object_suffix: *const c_char,
        object_type: u8,
        _graph_id: *const c_char, _graph_type: u8,
        graph: *mut RIFRDFQueryGraph,
        ) -> i64
{
    use oxrdf::{Quad, GraphName};
    use crate::c2rust_triple::BNodeMap; //Make translate_ctriple available
    if graph.is_null() {return -1;}
    unsafe {
        let (s, p, o) = match (*graph).translate_ctriple(
                                        subject, subject_type, predicate,
                                        object, object_suffix, object_type)
        {
            Ok(x) => x,
            Err(_) =>{
                eprintln!("Couldnt translate quad in RIFRDFQueryGraph_add");
                return -2;
            }
        };
        let quad = Quad::new(s, p, o, GraphName::DefaultGraph);
        match (*graph).store.insert(&quad) {
            Ok(_) => {return 0;},
            Err(_) => {
                eprintln!("Couldnt add quad to store of RIFRDFQueryGraph");
                return -3;
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFRDFQ_Graph_from_data(
    cdata: *const c_char,
    cmedia_type: *const c_char,
    ) -> *mut RIFRDFQueryGraph
{
    use oxrdfio::{RdfFormat, RdfParser, ReaderQuadParser};
    use oxrdf::Quad;
    use oxigraph::store::{Store};
    if cmedia_type.is_null(){return ptr::null_mut();}
    if cdata.is_null(){return ptr::null_mut();}
    let media_type: &str = match unsafe {CStr::from_ptr(cmedia_type)}.to_str(){
        Ok(x) => x,
        Err(_) => {
            eprintln!("Failed to read media_type");
            return ptr::null_mut();
        },
    };
    let format = match RdfFormat::from_media_type(media_type) {
        Some(x) => x,
        None => match RdfFormat::from_extension(media_type) {
            Some(y) => y,
            None => {
                eprintln!("Couldnt identify RDF format {}", media_type);
                return ptr::null_mut();
            },
        },
    };
    let data = unsafe {CStr::from_ptr(cdata)}.to_bytes();

    let parser = RdfParser::from_format(format);
    let quadparser = parser.for_reader(data);
    let store = match Store::new() {
        Ok(x) => x,
        Err(e) => {
            eprintln!("brubru {}", e);
            return ptr::null_mut();
        },
    };
    for equad in quadparser{
        match equad {
            Ok(q) => {store.insert(&q);},
            Err(e) => {
                eprintln!("Failed with error {}", e);
                eprintln!("Failed to parse input {:?}", data);
                return ptr::null_mut();
            },
        }
    }
    let mybox = Box::new(RIFRDFQueryGraph::new(store));
    Box::into_raw(mybox)
}

#[unsafe(no_mangle)]
pub extern "C" fn free_RIFRDFQuery_Graph(store: *mut RIFRDFQueryGraph){
    unsafe{
        let _ = Box::from_raw(store);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn fprintf_RIFRDFQuery_Graph(_f: *mut c_void, store: *mut RIFRDFQueryGraph){
    eprintln!("RIFRDFQueryGraph(");
    unsafe{
        for x in &(*store).store {
            match x {
                Ok(quad) => {eprintln!("{}", quad);},
                Err(_e) => {},
            }
        }
    }
    eprintln!(")");
}
