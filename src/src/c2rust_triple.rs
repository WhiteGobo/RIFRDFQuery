use std::ffi::{c_char, CStr};
use oxrdf::{NamedNodeRef, BlankNode, NamedOrBlankNode, Term, Literal, NamedNode};

/* Example implementation:
use std::collections::HashMap;
use crate::c2rust_triple::BNodeMap;
use oxrdf::BlankNode;

pub struct TripleLoader {
    bnodemap: HashMap<String, BlankNode>,
}

impl BNodeMap for TripleLoader {
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
*/

pub trait BNodeMap {
    fn get_bnode(&mut self, key: &str) -> BlankNode;


    fn translate_ctriple(
        &mut self,
        subject: *const c_char, subject_type: u8,
        predicate: *const c_char,
        object: *const c_char, object_suffix: *const c_char,
        object_type: u8,
    ) -> Result<(NamedOrBlankNode, NamedNode, Term), ()>
    {
        let subj = match self.generate_IdentifiedNode(subject, subject_type)
        {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Failed to translate subject");
                return Err(());
            },
        };
        let pred = match generate_IRI(predicate) {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Failed to translate predicate");
                return Err(());
            },
        };
        let obj = match self.generate_Term(object, object_suffix, object_type)
        {
            Ok(x) => x,
            Err(_) => {
                eprintln!("Failed to translate object with:");
                return Err(());
            },
        };
        return Ok((subj, pred, obj));
    }

    fn get_bnode_from_cchar(&mut self, key: *const c_char) -> Result<BlankNode, ()> {
        if key.is_null(){
            return Err(());
        }
        let x = match unsafe{ CStr::from_ptr(key) }.to_str() {
            Ok(x) => x,
            Err(_) => {return Err(());},
        };
        Ok(self.get_bnode(x))
    }


    fn generate_IdentifiedNode(
        &mut self,
        value: *const c_char, value_type: u8,
        ) -> Result<NamedOrBlankNode, ()>
    {
        match value_type {
            0 => match generate_IRI(value) {
                Ok(x) => Ok(x.into()),
                Err(_) => Err(()),
            },
            1 => match self.get_bnode_from_cchar(value) {
                Ok(x) => Ok(x.into()),
                Err(_) => Err(()),
            },
            _ => Err(())
        }
    }

    fn generate_Term(
        &mut self,
        value: *const c_char, value_suffix: *const c_char,
        value_type: u8,
        ) -> Result<Term, ()>
    {
        match value_type {
            0 => match generate_IRI(value) {
                Ok(x) => Ok(x.into()),
                Err(_) => Err(()),
            },
            1 => match self.get_bnode_from_cchar(value) {
                Ok(x) => Ok(x.into()),
                Err(_) => Err(()),
            },
            2 => match generate_typed_literal(value, value_suffix) {
                Ok(x) => Ok(x.into()),
                Err(_) => Err(()),
            },
            3 => match generate_lang_literal(value, value_suffix) {
                Ok(x) => Ok(x.into()),
                Err(_) => Err(()),
            },
            _ => Err(())
        }
    }

}

fn generate_lang_literal(
    value: *const c_char, value_suffix: *const c_char,
) -> Result<Literal, ()> {
    if value.is_null(){
        return Err(());
    }
    let obj = match unsafe{ CStr::from_ptr(value) }.to_str() {
        Ok(x) => x,
        Err(_) => {return Err(());},
    };
    if value_suffix.is_null(){
        Ok(Literal::new_language_tagged_literal_unchecked(obj, "").into())
    } else {
        let suf_c = unsafe{ CStr::from_ptr(value_suffix) };
        let suf = match suf_c.to_str() {
            Ok(x) => x,
            Err(_) => {return Err(());},
        };
        Ok(Literal::new_language_tagged_literal_unchecked(obj, suf).into())
    }
}


fn generate_typed_literal(
    value: *const c_char, value_suffix: *const c_char,
) -> Result<Literal, ()> {
    if value.is_null(){
        return Err(());
    }
    let obj = match unsafe{ CStr::from_ptr(value) }.to_str() {
        Ok(x) => x,
        Err(_) => {return Err(());},
    };
    if value_suffix.is_null(){
        Ok(Literal::new_simple_literal(obj).into())
    } else {
        let suf_c = unsafe{ CStr::from_ptr(value_suffix) };
        let suf_iri = match suf_c.to_str() {
            Ok(suf) => match NamedNode::new(suf) {
                Ok(x) => x,
                Err(_) => {return Err(());},
            },
            Err(_) => {return Err(());},
        };
        Ok(Literal::new_typed_literal(obj, suf_iri).into())
    }
}

fn generate_IRI(value: *const c_char) -> Result<NamedNode, ()>
{
    if value.is_null() {
        return Err(());
    }
    let x: &str = match unsafe {CStr::from_ptr(value)}.to_str(){
        Ok(x) => x,
        Err(_) => {return Err(());},
    };
    match NamedNode::new(x) {
        Ok(x) => Ok(x),
        Err(_) => Err(()),
    }
}
