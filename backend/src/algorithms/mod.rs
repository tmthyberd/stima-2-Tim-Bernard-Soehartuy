use crate::models::DomTree;
use crate::selectors::{matches_selector, parse_selector, Selector};
use std::collections::VecDeque;
pub struct SearchResult {
    pub found_indices: Vec<usize>, // Sekarang isinya angka index, bukan bool
    pub traversal_log: Vec<usize>,
}

pub fn bfs(tree: &DomTree, selector_str: &str) -> SearchResult {
    let mut result = SearchResult {
        found_indices: Vec::new(),
        traversal_log: Vec::new(),
    };

    let selector = parse_selector(selector_str);

    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited = vec![false; tree.nodes.len()];

    queue.push_back(0);
    visited[0] = true;

    while let Some(curr) = queue.pop_front() {
        result.traversal_log.push(curr);

        if matches_selector(&tree.nodes[curr], &selector) {
            result.found_indices.push(curr);
        }

        for &child_index in &tree.nodes[curr].children {
            if !visited[child_index] {
                visited[child_index] = true;
                queue.push_back(child_index);
            }
        }
    }

    result
}

pub fn dfs(tree: &DomTree, selector_str: &str) -> SearchResult {
    let mut result = SearchResult {
        found_indices: Vec::new(),
        traversal_log: Vec::new(),
    };

    let selector = parse_selector(selector_str);

    dfs_helper(0, tree, &selector, &mut result);

    result
}

fn dfs_helper(curr: usize, tree: &DomTree, selector: &Selector, result: &mut SearchResult) {
    result.traversal_log.push(curr);

    if matches_selector(&tree.nodes[curr], selector) {
        result.found_indices.push(curr);
    }

    for &child_index in &tree.nodes[curr].children {
        dfs_helper(child_index, tree, selector, result);
    }
}
#[test]
fn test_bfs() {
    use crate::parser::parse;

    let html = r#"<html><body><div><p>Satu</p><p>Dua</p></div></body></html>"#;
    let tree = parse(html);
    let result = bfs(&tree, "p");

    println!("Traversal log: {:?}", result.traversal_log);
    println!("Found at: {:?}", result.found_indices);

    assert_eq!(result.found_indices.len(), 2);
}
//#[test]
//fn test_dfs() {
//    use crate::parser::parse;
//
//    let html = r#"<html><body><div><p>Satu</p><p>Dua</p></div></body></html>"#;
//    let tree = parse(html);
//    let result = dfs(&tree, "p");
//
//    println!("Traversal log: {:?}", result.traversal_log);
//    println!("Found at: {:?}", result.found_indices);
//
//    assert_eq!(result.found_indices.len(), 2);
//}
//
//#[test]
//fn test_selector_class_dan_id() {
//    use crate::parser::parse;
//
//    let html = r#"<html><body><div class="box"><p id="judul">Halo</p></div></body></html>"#;
//    let tree = parse(html);
//
//    // Test class selector
//    let result_class = bfs(&tree, ".box");
//    println!("Class '.box' found at: {:?}", result_class.found_indices);
//    assert_eq!(result_class.found_indices.len(), 1);
//
//    // Test id selector
//    let result_id = dfs(&tree, "#judul");
//    println!("Id '#judul' found at: {:?}", result_id.found_indices);
//    assert_eq!(result_id.found_indices.len(), 1);
//}
