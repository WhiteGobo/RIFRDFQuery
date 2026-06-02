use oxigraph::model::Literal;

#[derive(Debug)]
pub enum MyTerm {
    RIFLangLiteral(Literal, Literal),
    RIFTypedLiteral(Literal, Literal),
    RIFSimpleLiteral(Literal),
    RIFVariable(Literal),
    RIFIri(Literal),
    RIFList(Vec<MyTerm>),
}

#[derive(Debug)]
pub struct Frame {
    pub object: MyTerm,
    pub slotkey: MyTerm,
    pub slotvalue: MyTerm,
}

#[derive(Debug)]
pub struct Atom {
    pub op: MyTerm,
    pub args: Vec<MyTerm>,
}

#[derive(Debug)]
pub struct Exists {
    pub formula: Vec<Formula>,
}

#[derive(Debug)]
pub enum Formula {
    Frame(Frame),
    Atom(Atom),
    Exists(Exists),
}
