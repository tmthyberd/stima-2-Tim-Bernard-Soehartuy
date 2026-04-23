use crate::models::DomNode;

pub enum Selector {
    Tag(String),   // "p","div"
    Class(String), // ".box"
    Id(String),    // "#judul"
}

pub fn parse_selector(selector: &str) -> Selector {
    if selector.starts_with('.') {
        Selector::Class(selector[1..].to_string())
    } else if selector.starts_with('#') {
        Selector::Id(selector[1..].to_string())
    } else {
        Selector::Tag(selector.to_string())
    }
}

pub fn matches_selector(node: &DomNode, selector: &Selector) -> bool {
    match selector {
        Selector::Tag(name) => node.tag_name.as_ref() == Some(name),
        Selector::Id(id_value) => node.attributes.get("id") == Some(id_value),
        Selector::Class(class_name) => {
            if let Some(attr_class) = node.attributes.get("class") {
                attr_class.split_whitespace().any(|c| c == class_name)
            } else {
                false
            }
        }
    }
}
