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
pub struct Subclass {
    pub sub: MyTerm,
    pub super_: MyTerm,
}

#[derive(Debug)]
pub struct Member {
    pub class: MyTerm,
    pub instance: MyTerm,
}

#[derive(Debug)]
pub struct Equal{
    pub left: MyTerm,
    pub right: MyTerm,
}

#[derive(Debug)]
pub enum Formula {
    Frame(Frame),
    Atom(Atom),
    Exists(Exists),
    Subclass(Subclass),
    Member(Member),
    Equal(Equal),
}
