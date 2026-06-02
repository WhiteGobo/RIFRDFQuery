pub mod rif {
    use oxigraph::model::NamedNodeRef;
    pub const OBJECT: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#object");
    pub const FRAME: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#Frame");
    pub const SLOTS: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#slots");
    pub const SLOT: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#Slot");
    pub const SLOTKEY: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#slotkey");
    pub const SLOTVALUE: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#slotvalue");
    pub const CONST: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#Const");
    pub const LANG: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#lang");
    pub const CONSTIRI: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#constIRI");
    pub const VALUE: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#value");
    pub const ATOM: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#Atom");
    pub const OP: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#op");
    pub const ARGS: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#args");
    pub const ITEMS: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#items");
    pub const FORMULA: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#formula");
    pub const VARNAME: NamedNodeRef = NamedNodeRef::new_unchecked("http://www.w3.org/2007/rif#varname");

}
