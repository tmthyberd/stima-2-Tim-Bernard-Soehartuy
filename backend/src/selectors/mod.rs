use crate::models::DomNode;
use crate::models::DomTree;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Combinator {
    Descendant,
    Child,
    AdjacentSibling,
    GeneralSibling,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub universal: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectorPart {
    pub combinator: Option<Combinator>,
    pub selector: SimpleSelector,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selector {
    pub parts: Vec<SelectorPart>,
}

pub fn parse_selector(selector: &str) -> Result<Selector, String> {
    let trimmed = selector.trim();

    if trimmed.is_empty() {
        return Err("Selector tidak boleh kosong.".to_string());
    }

    let chars: Vec<char> = trimmed.chars().collect();
    let mut pos = 0;
    let mut parts = Vec::new();
    let mut pending_combinator = None;

    while pos < chars.len() {
        let had_space = consume_whitespace(&chars, &mut pos);

        if pos >= chars.len() {
            break;
        }

        if let Some(combinator) = read_combinator(chars[pos]) {
            if parts.is_empty() {
                return Err("Combinator membutuhkan selector di sebelah kiri.".to_string());
            }
            if pending_combinator.is_some() {
                return Err("Combinator tidak boleh ditulis berurutan.".to_string());
            }

            pending_combinator = Some(combinator);
            pos += 1;
            continue;
        }

        if !parts.is_empty() && pending_combinator.is_none() {
            if had_space {
                pending_combinator = Some(Combinator::Descendant);
            } else {
                return Err("Selector majemuk membutuhkan combinator.".to_string());
            }
        }

        let combinator = if parts.is_empty() {
            None
        } else {
            Some(
                pending_combinator
                    .take()
                    .ok_or_else(|| "Selector majemuk membutuhkan combinator.".to_string())?,
            )
        };

        let simple_selector = parse_simple_selector(&chars, &mut pos)?;
        parts.push(SelectorPart {
            combinator,
            selector: simple_selector,
        });
    }

    if pending_combinator.is_some() {
        return Err("Selector tidak boleh diakhiri dengan combinator.".to_string());
    }

    if parts.is_empty() {
        return Err("Selector tidak boleh kosong.".to_string());
    }

    Ok(Selector { parts })
}

pub fn matches_selector(tree: &DomTree, node_index: usize, selector: &Selector) -> bool {
    if selector.parts.is_empty() || node_index >= tree.nodes.len() {
        return false;
    }

    matches_selector_part(tree, node_index, selector, selector.parts.len() - 1)
}

fn consume_whitespace(chars: &[char], pos: &mut usize) -> bool {
    let start = *pos;

    while *pos < chars.len() && chars[*pos].is_whitespace() {
        *pos += 1;
    }

    *pos > start
}

fn read_combinator(c: char) -> Option<Combinator> {
    match c {
        '>' => Some(Combinator::Child),
        '+' => Some(Combinator::AdjacentSibling),
        '~' => Some(Combinator::GeneralSibling),
        _ => None,
    }
}

fn parse_simple_selector(chars: &[char], pos: &mut usize) -> Result<SimpleSelector, String> {
    let mut selector = SimpleSelector {
        tag_name: None,
        id: None,
        classes: Vec::new(),
        universal: false,
    };
    let mut has_token = false;

    while *pos < chars.len() {
        let c = chars[*pos];

        if c.is_whitespace() || read_combinator(c).is_some() {
            break;
        }

        match c {
            '*' => {
                if selector.universal || selector.tag_name.is_some() {
                    return Err("Universal selector tidak valid di posisi ini.".to_string());
                }

                selector.universal = true;
                has_token = true;
                *pos += 1;
            }
            '.' => {
                *pos += 1;
                let class_name =
                    parse_identifier(chars, pos, "Class selector harus memiliki nama.")?;
                selector.classes.push(class_name);
                has_token = true;
            }
            '#' => {
                *pos += 1;
                let id = parse_identifier(chars, pos, "Id selector harus memiliki nama.")?;

                if selector.id.is_some() {
                    return Err("Id selector hanya boleh muncul satu kali.".to_string());
                }

                selector.id = Some(id);
                has_token = true;
            }
            _ if is_name_char(c) => {
                if selector.universal || selector.tag_name.is_some() {
                    return Err("Tag selector tidak valid di posisi ini.".to_string());
                }

                selector.tag_name = Some(parse_identifier(
                    chars,
                    pos,
                    "Tag selector harus memiliki nama.",
                )?);
                has_token = true;
            }
            _ => {
                return Err(format!("Karakter '{}' belum didukung pada selector.", c));
            }
        }
    }

    if !has_token {
        return Err("Selector tidak boleh kosong.".to_string());
    }

    Ok(selector)
}

fn parse_identifier(
    chars: &[char],
    pos: &mut usize,
    error_message: &str,
) -> Result<String, String> {
    let start = *pos;

    while *pos < chars.len() && is_name_char(chars[*pos]) {
        *pos += 1;
    }

    if *pos == start {
        return Err(error_message.to_string());
    }

    Ok(chars[start..*pos].iter().collect())
}

fn is_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | ':')
}

fn matches_selector_part(
    tree: &DomTree,
    node_index: usize,
    selector: &Selector,
    part_index: usize,
) -> bool {
    if !matches_simple_selector(
        &tree.nodes[node_index],
        &selector.parts[part_index].selector,
    ) {
        return false;
    }

    if part_index == 0 {
        return true;
    }

    match selector.parts[part_index].combinator {
        Some(Combinator::Descendant) => matches_descendant(tree, node_index, selector, part_index),
        Some(Combinator::Child) => matches_child(tree, node_index, selector, part_index),
        Some(Combinator::AdjacentSibling) => {
            matches_adjacent_sibling(tree, node_index, selector, part_index)
        }
        Some(Combinator::GeneralSibling) => {
            matches_general_sibling(tree, node_index, selector, part_index)
        }
        None => false,
    }
}

fn matches_simple_selector(node: &DomNode, selector: &SimpleSelector) -> bool {
    let Some(node_tag) = node.tag_name.as_ref() else {
        return false;
    };

    if let Some(tag_name) = selector.tag_name.as_ref() {
        if !node_tag.eq_ignore_ascii_case(tag_name) {
            return false;
        }
    }

    if let Some(id) = selector.id.as_ref() {
        if node.attributes.get("id") != Some(id) {
            return false;
        }
    }

    if selector.classes.is_empty() {
        return selector.universal || selector.tag_name.is_some() || selector.id.is_some();
    }

    let Some(attr_class) = node.attributes.get("class") else {
        return false;
    };

    selector
        .classes
        .iter()
        .all(|class_name| attr_class.split_whitespace().any(|c| c == class_name))
}

fn matches_descendant(
    tree: &DomTree,
    node_index: usize,
    selector: &Selector,
    part_index: usize,
) -> bool {
    let mut parent = tree.nodes[node_index].parent;

    while let Some(parent_index) = parent {
        if matches_selector_part(tree, parent_index, selector, part_index - 1) {
            return true;
        }

        parent = tree.nodes[parent_index].parent;
    }

    false
}

fn matches_child(
    tree: &DomTree,
    node_index: usize,
    selector: &Selector,
    part_index: usize,
) -> bool {
    tree.nodes[node_index].parent.is_some_and(|parent_index| {
        matches_selector_part(tree, parent_index, selector, part_index - 1)
    })
}

fn matches_adjacent_sibling(
    tree: &DomTree,
    node_index: usize,
    selector: &Selector,
    part_index: usize,
) -> bool {
    let Some((siblings, position)) = sibling_position(tree, node_index) else {
        return false;
    };

    if position == 0 {
        return false;
    }

    matches_selector_part(tree, siblings[position - 1], selector, part_index - 1)
}

fn matches_general_sibling(
    tree: &DomTree,
    node_index: usize,
    selector: &Selector,
    part_index: usize,
) -> bool {
    let Some((siblings, position)) = sibling_position(tree, node_index) else {
        return false;
    };

    siblings[..position]
        .iter()
        .rev()
        .any(|&sibling| matches_selector_part(tree, sibling, selector, part_index - 1))
}

fn sibling_position(tree: &DomTree, node_index: usize) -> Option<(&[usize], usize)> {
    let parent_index = tree.nodes[node_index].parent?;
    let siblings = tree.nodes[parent_index].children.as_slice();
    let position = siblings.iter().position(|&sibling| sibling == node_index)?;

    Some((siblings, position))
}
