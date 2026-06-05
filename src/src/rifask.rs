use oxigraph::store::Store;
use crate::formula_container::{MyTerm, Frame, Atom, Exists, Formula, Subclass, Member, Equal};
use std::io::Error;

const PREFIXES: &str = "
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rif: <http://www.w3.org/2007/rif#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
ASK WHERE {
";

struct MyQuery {
    parts: Vec<String>,
    i: u32,
}


pub fn rifask(
    data_graph: &Store,
    formula: Vec<Formula>,
) -> Result<bool, ()>
{
    //let query = match combine_as_sparqlquery(&formula) {
    let mut query = MyQuery::new();
    if !query.combine_formulas(&formula) {
        eprintln!("internal error combine as sparqlquery");
        return Err(());
    }
    query.execute(data_graph)
}

impl MyQuery {
    pub fn new() -> Self {
        MyQuery {
            parts: Vec::new(),
            i: 0,
        }
    }

    fn new_var(&mut self, name: &str) -> String {
        let ret = format!("?{}{}", name, self.i);
        self.i = self.i + 1;
        ret
    }

    fn generate_termcheck(&mut self, name: &str, node: &MyTerm) -> String {
        use MyTerm as mt;
        match node {
            mt::RIFLangLiteral(value, lang) => {
                format!("\t{} rif:value {}.\n", name, value)
            },
            mt::RIFTypedLiteral(value, vtype) => {
                format!("\t{} rif:value {}.\n", name, value)
            },
            mt::RIFSimpleLiteral(value) => {
                format!("\t{} rif:value {}.\n", name, value)
            },
            mt::RIFVariable(var) => {
                "".to_string()
            },
            mt::RIFIri(value) => {
                format!("\t{} rif:constIRI {}.\n", name, value)
            },
            mt::RIFList(valuelist) => {
                self.generate_riflist_check(name, valuelist)
            },
        }
    }

    fn generate_riflist_check(&mut self, name: &str, items: &Vec<MyTerm>) -> String {
        use std::iter::zip;
        let mut itemnames = Vec::new();
        for node in items.iter() {
            itemnames.push(self.new_var("item"));
        }
        let mut ret = format!("\t{} rif:items ({}).\n",name, itemnames.join(" "));
        for (rname, rnode) in zip(itemnames.iter(), items.iter()) {
            ret += &self.generate_termcheck(rname, rnode);
        }
        ret

    }

    fn generate_querypart_subclass(&mut self, subclass: &Subclass
        ) -> Result<String, Error>
    {
        let subclassnode = self.new_var("subclass");
        let sub_node = self.new_var("sub");
        let super_node = self.new_var("super");
        let mut ret = format!("\t{} rif:sub {} ; rif:super {} .\n",
                                subclassnode, sub_node, super_node);
        for (name, node) in [
            (&sub_node, &subclass.sub),
            (&super_node, &subclass.super_)
        ] {
            ret += &self.generate_termcheck(name, node);
        }
        return Ok(ret)
    }

    fn generate_querypart_member(&mut self, member: &Member,
        ) -> Result<String, Error>
    {
        let membernode = self.new_var("member");
        let instance_node = self.new_var("instance");
        let class_node = self.new_var("class");
        let mut ret = format!("\t{} rif:instance {} ; rif:class {} .\n",
                                membernode, instance_node, class_node);
        for (name, node) in [
            (&instance_node, &member.instance),
            (&class_node, &member.class)
        ] {
            ret += &self.generate_termcheck(name, node);
        }
        return Ok(ret)
    }

    fn generate_querypart_equal(&mut self, equal: &Equal,
        ) -> Result<String, Error>
    {
        let membernode = self.new_var("equal");
        let left_node = self.new_var("left");
        let right_node = self.new_var("right");
        let mut ret
             = format!("\t{} rif:left | rif:right {} ; rif:right | rif:left {} .\n",
                        membernode, left_node, right_node);
        for (name, node) in [
            (&left_node, &equal.left),
            (&right_node, &equal.right)
        ] {
            ret += &self.generate_termcheck(name, node);
        }
        return Ok(ret)
    }

    fn generate_querypart_frame(&mut self, frame: &Frame
        ) -> Result<String, Error>
    {
        let framenode = self.new_var("frame");
        let objectnode = self.new_var("obj");
        let slotnode = self.new_var("slot");
        let slotkey = self.new_var("key");
        let slotvalue = self.new_var("value");
        let mut ret
            = format!("\t{} rif:slots / rdf:rest* / rdf:first {};\n",
                framenode, slotnode)
            + &format!("\t\trif:object {}.\n", objectnode)
            + &format!("\t{} rif:slotkey {}; rif:slotvalue {}.\n",
                slotnode, slotkey, slotvalue);
        for (name, node) in [
            (&objectnode, &frame.object),
            (&slotkey, &frame.slotkey),
            (&slotvalue, &frame.slotvalue)
        ] {
            ret += &self.generate_termcheck(name, node);
        }
        return Ok(ret);
    }

    fn generate_querypart_atom(&mut self, atom: &Atom
        ) -> Result<String, Error>
    {
        use std::iter::zip;
        let atomnode = self.new_var("atom");
        let opnode = self.new_var("op");
        let mut argnames = Vec::new();
        if atom.args.is_empty(){
            let filternode = self.new_var("args");
            let mut ret = format!("\t{} rif:op {}.\n",
                                    atomnode, opnode);
            ret += format!("\tOPTIONAL {{{} rif:args {}}}\n", atomnode, filternode).as_ref();

            ret += format!("\tFILTER(!bound({}) || ({} = rdf:nil))\n", filternode, filternode).as_ref();
            ret += &self.generate_termcheck(&opnode, &atom.op);
            return Ok(ret);
        } else {
            for node in atom.args.iter() {
                argnames.push(self.new_var("arg"));
            }
            let mut ret = format!("\t{} rif:op {};\n\t\trif:args ({}).\n",
                                    atomnode, opnode, argnames.join(" "));
            ret += &self.generate_termcheck(&opnode, &atom.op);

            for (rname, rnode) in zip(argnames.iter(), atom.args.iter()) {
                ret += &self.generate_termcheck(rname, rnode);
            }
            return Ok(ret);
        }
    }

    fn generate_querypart_exists(&mut self, exists: &Exists
        ) -> Result<String, Error>
    {
        match self.combine_formulas(&exists.formula) {
            true => Ok("".to_owned()),
            false => Err(Error::other("exists")),
        }
    }

    fn combine_formulas(&mut self, formulas: &Vec<Formula>) -> bool
    {
        for x in formulas {
            let new = match x {
                Formula::Frame(f) => self.generate_querypart_frame(f),
                Formula::Atom(a) => self.generate_querypart_atom(a),
                Formula::Exists(x) => self.generate_querypart_exists(x),
                Formula::Subclass(x) => self.generate_querypart_subclass(x),
                Formula::Member(x) => self.generate_querypart_member(x),
                Formula::Equal(x) => self.generate_querypart_equal(x),
            };
            match new {
                Ok(x) => self.parts.push(x),
                Err(_) => {eprintln!("internal error3"); return false;},
            }
        }
        true
    }
}



impl MyQuery {
    fn execute(&self, graph: &Store) -> Result<bool, ()> {
        use oxigraph::sparql::{QueryResults, SparqlEvaluator, QuerySolutionIter};
        use spargebra::SparqlParser;
        let mut querystring = PREFIXES.to_owned();
        for s in &self.parts {
            querystring += &s
        }
        querystring += "}";

        eprintln!("query: {}", querystring);
        let query = match SparqlParser::new().parse_query(&querystring)
        {
            Ok(q) => q,
            Err(e) => {
                eprintln!("Internal Error. {} {}", e, querystring);
                return Err(());
            },
        };

        let prepared_query = SparqlEvaluator::new().for_query(query);

        match prepared_query.on_store(&graph).execute() {
            Ok(QueryResults::Boolean(b)) => {
                eprintln!("result is {}", b);
                Ok(b)
            }, //result of ASK
            Err(e) => {
                eprintln!("Error during executing sparql query: {}", e);
                Err(())
            },
            Ok(_) => {
                eprintln!("Internal Err. Expected ASK but");
                Err(())
            },
        }
    }
}


/*
impl Formula {

    fn query_in(
        &self, graph: &Store,
    ) -> Result<bool, ()>
    {
        match self {
            Formula::Frame(f) => f.query_in(graph),
            Formula::Atom(x) => x.query_in(graph),
            Formula::Exists(e) => e.query_in(graph),
        }
    }
}

impl Exists {
    pub fn generate_querystring(&self, variables: Option<&mut Vec<()>>)
        -> Result<String, ()>
    {
        Err(())
    }

    pub fn query_in(
        &self, graph: &Store,
    ) -> Result<bool, ()>
    {
        eprintln!("check Exists");
        Err(())
    }
}

impl Atom {
    pub fn generate_querystring(&self, variables: Option<&mut Vec<()>>)
        -> Result<String, ()>
    {
        Err(())
    }

    pub fn query_in(
        &self, graph: &Store,
    ) -> Result<bool, ()>
    {
        eprintln!("check Atom");
        Err(())
    }
}

impl Frame {
    pub fn generate_querystring(&self, variables: Option<&mut Vec<()>>)
        -> Result<String, ()>
    {
        let framenode = "?frame".to_string();
        let objectnode = "?obj".to_string();
        let slotnode = "?slot".to_string();
        let slotkey = "?key".to_string();
        let slotvalue = "?value".to_string();
        let mut ret = format!("{} rif:slots / rdf:rest* / rdf:first {};
rif:object {}.
{} rif:slotkey {}; rif:slotvalue {}.",
            framenode, slotnode, objectnode, slotnode, slotkey, slotvalue);
        {
            let q: Option<&mut Vec<()>> = variables;
        match self.object.generate_querystring(&objectnode, q) {
            Ok(x) => ret += &x,
            _ => {return Err(());},
        };
        }
            let q: Option<&mut Vec<()>> = variables;
        match self.slotkey.generate_querystring(&slotkey, q) {
            Ok(x) => ret += &x,
            _ => {return Err(());},
        };
        match self.slotvalue.generate_querystring(&slotvalue, variables) {
            Ok(x) => ret += &x,
            _ => {return Err(());},
        };
        return Ok(ret);
    }

    pub fn query_in(
        &self, graph: &Store,
    ) -> Result<bool, ()>
    {
        eprintln!("check Frame");
        execute_ask_query(graph, "")
    }
}


impl MyTerm {
    pub fn generate_querystring(&self, root: &str, variables: Option<&mut Vec<()>>) -> Result<String, ()>
    {
        use MyTerm as mt;
        match self {
            mt::RIFLangLiteral(x, y) => Err(()),
            mt::RIFTypedLiteral(x, y) => Err(()),
            mt::RIFSimpleLiteral(x) => Err(()),
            mt::RIFVariable(x) => Err(()),
            mt::RIFIri(x) => Err(()),
        }
    }
}

/*
"
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rif: <http://www.w3.org/2007/rif#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

SELECT ?frame ?slotkey ?slotvalue WHERE {
    ?frame rif:slots / rdf:rest* / rdf:first [
        rif:slotkey ?slotkey
        rif:slotvalue ?slotvalue
    ] .
    ?slotkey rif:constIRI \"http://my.example/iri\" .
    ?slotvalue rif:value \"myvalue\" .
}
"

"
PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
PREFIX rif: <http://www.w3.org/2007/rif#>
PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

SELECT ?atom ?op ?arg1 ?arg2 WHERE {
    ?atom rif:op ?op.
    ?atom rif:args [
        rdf:first ?arg1;
        rdf:rest [
            rdf:first ?arg2;
            rif:rest rdf:nil;
        ]
    ] .
    ?op rif:constIRI \"http://my.example/iri\" .
    ?arg1 rif:value \"myvalue\" .
    ?arg2 rif:value \"myvalue\" .
}
"
*/

fn execute_ask_query(graph: &Store, querystring: &str) -> Result<bool, ()> {
    use oxigraph::sparql::{QueryResults, SparqlEvaluator, QuerySolutionIter};
    use spargebra::SparqlParser;
    let query = match SparqlParser::new().parse_query(querystring)
    {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Internal Error. {} {}", e, querystring);
            return Err(());
        },
    };

    let prepared_query = SparqlEvaluator::new().for_query(query);

    match prepared_query.on_store(&graph).execute() {
        Ok(QueryResults::Boolean(b)) => {
            eprintln!("result is {}", b);
            Ok(b)
        }, //result of ASK
        Err(e) => {
            eprintln!("Error during executing sparql query: {}", e);
            Err(())
        },
        Ok(_) => {
            eprintln!("Internal Err. Expected ASK but");
            Err(())
        },
    }
}

fn combine_as_sparqlquery(
    formulas: &Vec<Formula>,
) -> Option<String>
{
    let mut ret: String = PREFIXES.to_owned();
    let mut terms: Vec<MyTerm> = Vec::new();
    for x in formulas {
        let new = match x {
            Formula::Frame(f) => f.generate_querystring(None),
            Formula::Atom(a) => a.generate_querystring(None),
            Formula::Exists(x) => x.generate_querystring(None),
        };
        match new {
            Ok(x) => ret += &x,
            Err(_) => {eprintln!("internal error3");return None;},
        }
    }
    ret += "\n}";
    Some(ret)
}

*/
