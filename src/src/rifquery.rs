use oxigraph::store::Store;
use oxigraph::sparql::{QuerySolution};
use oxigraph::model::{Term, NamedOrBlankNodeRef, NamedOrBlankNode, NamedNodeRef};
use crate::constants::rif;
use crate::formula_container::{MyTerm, Frame, Atom, Exists, Formula};
use crate::rifask::rifask;


enum FormulaType {
    Frame(Term, Term),
    Atom(Term),
    Exists(Term),
}

impl MyTerm {
    pub fn retrieve(graph: &Store, term: Term) -> Option<Self> {
        use oxigraph::model::{NamedOrBlankNodeRef, NamedOrBlankNode, NamedNodeRef, Literal, LiteralRef};
        let tmp_t = NamedOrBlankNode::try_from(term).ok()?;
        let t = Some(tmp_t.as_ref());
        let mut const_iri: Option<Literal> = None;
        let mut value: Option<Literal> = None;
        let mut items: Option<NamedOrBlankNode> = None;
        let mut lang: Option<Literal> = None;
        let mut valuetype: Option<Literal> = None;
        let mut var: Option<Literal> = None;
        for x in graph.quads_for_pattern(t, None, None, None){
            let xx = x.ok()?;
            let pred: NamedNodeRef = xx.predicate.as_ref();
            match pred {
                rif::CONSTIRI => {
                    let iri = Literal::try_from(xx.object).ok()?;
                    const_iri = Some(iri);
                },
                rif::VALUE => {
                    let val = Literal::try_from(xx.object).ok()?;
                    value = Some(val);
                },
                rif::ITEMS => {
                    let root = NamedOrBlankNode::try_from(xx.object).ok()?;
                    items = Some(root);
                }
                rif::VARNAME => {
                    let qvar = Literal::try_from(xx.object).ok()?;
                    var = Some(qvar);
                }
                _ => {},
            }
        }
        match (const_iri, value, valuetype, lang, var, items) {
            (Some(x), None, None, None, None, None)
                => Some(MyTerm::RIFIri(x)),
            (None, Some(x), None, None, None, None)
                => Some(MyTerm::RIFSimpleLiteral(x)),
            (None, Some(x), Some(y), None, None, None)
                => Some(MyTerm::RIFTypedLiteral(x, y)),
            (None, Some(x), None, Some(y), None, None)
                => Some(MyTerm::RIFLangLiteral(x, y)),
            (None, None, None, None, Some(x), None)
                => Some(MyTerm::RIFVariable(x)),
            (None, None, None, None, None, Some(x))
                => match rdflist_to_vec(graph, x.as_ref()) {
                    Some(x) => Some(MyTerm::RIFList(x)),
                    None => None,
                },
            _ => None,
        }
    }

}

impl Formula {
    pub fn retrieve_data_from(
        graph: &Store, term_type: FormulaType,
    ) -> Option<Self>
    {
        use FormulaType as ft;
        Some(match term_type {
            ft::Frame(n, s) => {
                Self::Frame(Frame::retrieve(graph, n, s)?)
            },
            ft::Atom(n) => {
                eprintln!("asdf2");
                Self::Atom(Atom::retrieve(graph, n)?)
            },
            ft::Exists(n) => {
                Self::Exists(Exists::retrieve(graph, n)?)
            },
            _ => {return None;},
        })
    }
}

impl Exists {
    fn retrieve(
        graph: &Store, root: Term,
    ) -> Option<Self>
    {
        let exists = NamedOrBlankNode::try_from(root).ok()?;

        let f = match graph.quads_for_pattern(Some(exists.as_ref()), Some(rif::FORMULA), None, None).next()?
        {
            Ok(x) => NamedOrBlankNode::try_from(x.object).ok()?,
            Err(_) => {return None;},
        };

        let formulatype = match FormulaType::retrieve(graph, f.as_ref()) {
            Some(x) => x,
            None => {
                eprintln!("Couldnt retrieve formula for rif:Exists");
                return None;},
        };
        let mut targets = Vec::new();
        for x in formulatype {
            let tmp = match Formula::retrieve_data_from(graph, x) {
                Some(y) => y,
                None => {
                    eprintln!("Couldnt transform formulas in rif:Exists");
                    return None;
                },
            };
            targets.push(tmp);
        }
        Some(Exists {
            formula: targets,
        })
    }
}

impl Atom {
    fn retrieve(
        graph: &Store, term: Term,
    ) -> Option<Self>
    {
        use oxigraph::model::{NamedOrBlankNodeRef, NamedOrBlankNode};
        let tmp_t = NamedOrBlankNode::try_from(term).ok()?;

        let t = Some(tmp_t.as_ref());
        let op = {
            let q = match graph.quads_for_pattern(t, Some(rif::OP), None, None).next(){
                Some(x) => x,
                None => {eprintln!("atom expected but no rif:op");return None;},
            };
            q.ok()?.object
        };
        let args = {
            let q = match graph.quads_for_pattern(t, Some(rif::ARGS), None, None).next(){
                Some(x) => x,
                None => {
                    eprintln!("atom expected but no rif:args");
                    return None;
                },
            };
            let args_term = q.ok()?.object;
            NamedOrBlankNode::try_from(args_term).ok()?
        };
        let ret_op = match MyTerm::retrieve(graph, op){
            Some(x) => x,
            None => {
                eprintln!("Missing op in atom");
                return None;
            }
        };
        let ret_args = match rdflist_to_vec(graph, args.as_ref()){
            Some(x) => x,
            None => {
                eprintln!("Failed to convert args in atom");
                return None;
            }
        };
        Some(Atom {
            op: ret_op,
            args: ret_args,
        })
    }
}

fn rdflist_to_vec(graph: &Store, root: NamedOrBlankNodeRef) -> Option<Vec<MyTerm>> {
    use oxigraph::model::vocab::rdf;
    let mut ret1 = Vec::new();
    let mut tmp: NamedOrBlankNode = root.into();
    let rdf_nil: NamedOrBlankNode = rdf::NIL.into();
    while tmp != rdf_nil {
        let s: Option<NamedOrBlankNodeRef> = Some(tmp.as_ref());
        let first = match graph.quads_for_pattern(s, Some(rdf::FIRST), None, None).next(){
            Some(x) => match x {
                Ok(x) => x.object,
                Err(_) => {return None;},
            },
            None => {return None;},
        };
        ret1.push(first);
        let rest = match graph.quads_for_pattern(s, Some(rdf::REST), None, None).next(){
            Some(x) => match x {
                Ok(x) => match NamedOrBlankNode::try_from(x.object) {
                    Ok(x) => x,
                    Err(_) => {return None;},
                }
                Err(_) => {return None;},
            },
            None => {return None;},
        };
        tmp = rest;
    }
    {
        let mut ret2 = Vec::new();
        for x in ret1 {
            ret2.push(MyTerm::retrieve(graph, x)?);
        }
        Some(ret2)
    }
}

impl Frame {
    fn retrieve(
        graph: &Store, term: Term, slot: Term,
    ) -> Option<Self>
    {
        let tmp_t = NamedOrBlankNode::try_from(term).ok()?;
        let t = Some(tmp_t.as_ref());
        let tmp_s = NamedOrBlankNode::try_from(slot).ok()?;
        let s = Some(tmp_s.as_ref());
        let obj = {
            let q = match graph.quads_for_pattern(t, Some(rif::OBJECT), None, None).next(){
                Some(x) => x,
                None => {return None;},
            };
            q.ok()?.object
        };
        let slotkey = {
            let q = match graph.quads_for_pattern(s, Some(rif::SLOTKEY), None, None).next(){
                Some(x) => x,
                None => {eprintln!("broken frame1");return None;},
            };
            q.ok()?.object
        };
        let slotvalue = {
            let q = match graph.quads_for_pattern(s, Some(rif::SLOTVALUE), None, None).next(){
                Some(x) => x,
                None => {eprintln!("broken frame2");return None;},
            };
            q.ok()?.object
        };
        Some(Frame {
            object: MyTerm::retrieve(graph, obj)?,
            slotkey: MyTerm::retrieve(graph, slotkey)?,
            slotvalue: MyTerm::retrieve(graph, slotvalue)?,
        })
    }
}

impl FormulaType {
    pub fn retrieve(graph: &Store, root: NamedOrBlankNodeRef) -> Option<Vec<Self>> {
        let mut op = None;
        let mut args = None;
        let mut object = None;
        let mut slots = None;
        for x in graph.quads_for_pattern(Some(root), None, None, None){
            let xx = x.ok()?;
            let pred: NamedNodeRef = xx.predicate.as_ref();
            match pred {
                rif::OP => {
                    let tmp = NamedOrBlankNode::try_from(xx.object).ok()?;
                    op = Some(tmp);
                },
                rif::ARGS => {
                    let tmp = NamedOrBlankNode::try_from(xx.object).ok()?;
                    args = Some(tmp);
                },
                rif::OBJECT => {
                    let tmp = NamedOrBlankNode::try_from(xx.object).ok()?;
                    object = Some(tmp);
                },
                rif::SLOTS => {
                    let tmp = NamedOrBlankNode::try_from(xx.object).ok()?;
                    slots = Some(tmp);
                },
                _ => {},
            }
        }
        let mut ret = Vec::new();
        match (op, args, object, slots) {
            (Some(x), Some(y), None, None) => {
                ret.push(FormulaType::Atom(root.into()));
            },
            (None, None, Some(x), Some(y)) => {
                eprintln!("retrieve FormulaType for frame not implemented");
                return None;
            },
            _ => {eprintln!("qwer");return None;},
        };
        Some(ret)
    }

    pub fn from_solution(solution: QuerySolution, typekey: &str, nodekey: &str, suffixkey: &str) -> Option<Self> {
        use crate::rifquery::FormulaType::{Frame, Atom, Exists};
        use oxsdatatypes::Integer;
        let t: &Term = solution.get(typekey)?;
        let id: String = match t {
            Term::NamedNode(x) => x.to_string(),
            _ => {return None;},
        };
        let node = match solution.get(nodekey) {
            Some(x) => x.clone(),
            None => {return None;}
        };
        let second = match solution.get(suffixkey) {
            Some(x) => Some(x.clone()),
            None => None,
        };
        match id.as_str() {
            "<http://www.w3.org/2007/rif#Frame>" => match second {
                Some(x) => Some(Frame(node, x)),
                None => {eprintln!("internal error: {}", id); return None;},
            }
            "<http://www.w3.org/2007/rif#Atom>" => Some(Atom(node)),
            "<http://www.w3.org/2007/rif#Exists>" => Some(Exists(node)),
            _ => {eprintln!("internal error: {}", id); return None;},
        }
    }
}

pub fn rifquery(
    data_graph: &Store,
    query_graph: &Store,
) -> bool
{
    let q = match find_base_information(query_graph) {
        Ok(q) => q,
        Err(_e) => {
            eprintln!("Problem during querying query_graph for formulas");
            return false;
        },
    };
    match rifask(data_graph, q) {
        Ok(x) => x,
        Err(_) => false,
    }
}


const _FIND_BASE_INFORMATION_QUERY: &str = "
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rif: <http://www.w3.org/2007/rif#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
SELECT ?formula ?t ?slot WHERE {
    ?formula a ?t .
    FILTER( ?t IN (rif:Frame, rif:Atom, rif:Exists))
    FILTER Not Exists {
        ?x ?y ?formula .
    }
    OPTIONAL {
        ?formula rif:slots / rdf:rest* / rdf:first ?slot .
    }
}";
fn find_base_information(
    query_graph: &Store,
) -> Result<Vec<Formula>, ()>
{
    use oxigraph::model::*;
    use oxigraph::sparql::{QueryResults, SparqlEvaluator, QuerySolutionIter};
    use oxigraph::store::Store;
    use spargebra::SparqlParser;

    let query = match SparqlParser::new().parse_query(
                                            _FIND_BASE_INFORMATION_QUERY)
    {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Internal Error. {} {}", e, _FIND_BASE_INFORMATION_QUERY);
            return Err(());
        },
    };

    let prepared_query = SparqlEvaluator::new().for_query(query);

    let results = match prepared_query.on_store(&query_graph).execute() {
        Ok(QueryResults::Solutions(r)) => r, //result of SELECT
        Err(e) => {
            eprintln!("Error during executing sparql query: {}", e);
            return Err(());
        },
        Ok(_) => {
            eprintln!("Internal Err in find_base_information. Expected SELECT but");
            return Err(());
        },
    };
    use oxigraph::model::Term;
    let mut ret_terms = Vec::<FormulaType>::new();
    for solution in results {
        let s = match solution {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error during sparql query: {}", e);
                return Err(());
            },
        };
        let t = match FormulaType::from_solution(s, "t", "formula", "slot") {
            Some(x) => x,
            None => {return Err(());}
        };
        ret_terms.push(t);
    }
    let mut ret = Vec::new();
    eprintln!("brubru1");
    for term_type in ret_terms {
        match Formula::retrieve_data_from(query_graph, term_type) {
            Some(x) => ret.push(x),
            None => {
                eprintln!("Graph broken.");
                return Err(());
            }
        }
    }
    eprintln!("brubru2");
    {
        let q = &ret;
        for x in q.into_iter() {
            eprintln!("qq {:?}", x);
        }
    }
    Ok(ret)
}

