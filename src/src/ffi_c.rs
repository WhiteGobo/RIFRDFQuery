use std::ptr;
use std::ffi::{c_char, CStr};
use oxigraph::store::Store;
use crate::rifquery::rifquery;

#[unsafe(no_mangle)]
pub extern "C" fn RIFRDFQuery_Graph_query_rif_data(
    data_graph: *mut Store,
    query_graph: *mut Store,
    ) -> i8
{
    let dgraph: &Store = unsafe{&(*data_graph)};
    let qgraph: &Store = unsafe{&(*query_graph)};
    if rifquery(dgraph, qgraph){
        return 0;
    } else {
        return 1;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn RIFRDFQ_Graph_from_data(
    cdata: *const c_char,
    cmedia_type: *const c_char,
    ) -> *mut Store
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
    let mybox = Box::new(store);
    Box::into_raw(mybox)
}

#[unsafe(no_mangle)]
pub extern "C" fn free_RIFRDFQuery_Graph(store: *mut Store){
    unsafe{
        let _ = Box::from_raw(store);
    }
}
