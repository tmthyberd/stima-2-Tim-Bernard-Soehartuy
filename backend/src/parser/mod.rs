use std::collections::HashMap;

use crate::models::{DomNode, DomTree, NodeType};

pub enum Token {
    OpenTag(String, HashMap<String, String>),
    CloseTag(String),
    Text(String),
}

pub fn parse_open_tag(content: String) -> Token {
    let parts: Vec<&str> = content.split_whitespace().collect();

    let tag_name = parts[0].to_string();

    let mut attributes = HashMap::new();

    for attr in &parts[1..] {
        if let Some((key, val)) = attr.split_once('=') {
            let clean_val = val.trim_matches('"').trim_matches('\'');
            attributes.insert(key.to_string(), clean_val.to_string());
        }
    }
    Token::OpenTag(tag_name, attributes)
}

pub fn tokenize(html: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = html.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '<' => {
                chars.next(); // Buang '<'
                if chars.peek() == Some(&'/') {
                    chars.next(); // Buang '/'
                    let mut tag_name = String::new();
                    while let Some(&next_c) = chars.peek() {
                        if next_c == '>' {
                            break;
                        }
                        tag_name.push(chars.next().unwrap());
                    }
                    chars.next(); // Buang '>'
                    tokens.push(Token::CloseTag(tag_name.trim().to_string()));
                } else {
                    let mut tag_content = String::new();
                    while let Some(&next_c) = chars.peek() {
                        if next_c == '>' {
                            break;
                        }
                        tag_content.push(chars.next().unwrap());
                    }
                    chars.next(); // Buang '>'
                    tokens.push(parse_open_tag(tag_content));
                }
            }
            _ => {
                // ambil teks secara manual
                let mut text = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c == '<' {
                        break;
                    } // Berhenti tepat sebelum'<'
                    text.push(chars.next().unwrap());
                }

                let trimmed = text.trim();
                if !trimmed.is_empty() {
                    tokens.push(Token::Text(trimmed.to_string()));
                }
            }
        }
    }
    tokens
}

pub fn parse(html: &str) -> DomTree {
    let tokens = tokenize(html);
    let mut tree = DomTree::new();

    let mut stack = vec![0];

    for token in tokens {
        match token {
            Token::OpenTag(name, attrs) => {
                let parent_index = *stack.last().unwrap();

                let new_node = DomNode {
                    node_type: NodeType::Element,
                    tag_name: Some(name),
                    attributes: attrs,
                    children: Vec::new(),
                    parent: Some(parent_index),
                    text_content: None,
                };

                let new_index = tree.add_node(new_node);

                tree.nodes[parent_index].children.push(new_index);

                stack.push(new_index);
            }
            Token::Text(content) => {
                let parent_index = *stack.last().unwrap();

                let text_node = DomNode {
                    node_type: NodeType::Text,
                    tag_name: None,
                    attributes: HashMap::new(),
                    children: Vec::new(),
                    parent: Some(parent_index),
                    text_content: Some(content),
                };

                let text_index = tree.add_node(text_node);
                tree.nodes[parent_index].children.push(text_index);
            }
            Token::CloseTag(_name) => {
                if stack.len() > 1 {
                    stack.pop();
                }
            }
        }
    }

    tree
}
//
//#[test]
//fn test_parse_nested() {
//    let html = r#"<html><body><div class="box"><p id="judul">Selamat</p><p>Datang</p></div></body></html>"#;
//    let tree = parse(html);
//
//    for (i, node) in tree.nodes.iter().enumerate() {
//        println!(
//            "Node {}: {:?} | tag: {:?} | text: {:?} | children: {:?} | attrs: {:?}",
//            i, node.node_type, node.tag_name, node.text_content, node.children, node.attributes
//        );
//    }
//
//    // Berapa node yang seharusnya ada? Hitung dulu sebelum jalankan!
//    assert_eq!(tree.nodes.len(), 8);
//}
