use crate::models::DomTree;
use crate::selectors::{matches_selector, parse_selector, Selector};
use std::collections::VecDeque;
pub struct SearchResult {
    pub found_indices: Vec<usize>,
    pub traversal_log: Vec<usize>,
}

pub fn bfs(tree: &DomTree, selector_str: &str, top_n: usize) -> Result<SearchResult, String> {
    let mut result = SearchResult {
        found_indices: Vec::new(),
        traversal_log: Vec::new(),
    };

    let selector = parse_selector(selector_str)?;

    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited = vec![false; tree.nodes.len()];

    queue.push_back(0);
    visited[0] = true;

    while let Some(curr) = queue.pop_front() {
        result.traversal_log.push(curr);

        if matches_selector(tree, curr, &selector) {
            result.found_indices.push(curr);
            if top_n > 0 && result.found_indices.len() >= top_n {
                break;
            }
        }

        for &child_index in &tree.nodes[curr].children {
            if !visited[child_index] {
                visited[child_index] = true;
                queue.push_back(child_index);
            }
        }
    }

    Ok(result)
}

pub fn dfs(tree: &DomTree, selector_str: &str, top_n: usize) -> Result<SearchResult, String> {
    let mut result = SearchResult {
        found_indices: Vec::new(),
        traversal_log: Vec::new(),
    };

    let selector = parse_selector(selector_str)?;

    dfs_helper(0, tree, &selector, &mut result, top_n);

    Ok(result)
}

fn dfs_helper(
    curr: usize,
    tree: &DomTree,
    selector: &Selector,
    result: &mut SearchResult,
    top_n: usize,
) -> bool {
    result.traversal_log.push(curr);

    if matches_selector(tree, curr, selector) {
        result.found_indices.push(curr);
        if top_n > 0 && result.found_indices.len() >= top_n {
            return true;
        }
    }

    for &child_index in &tree.nodes[curr].children {
        if dfs_helper(child_index, tree, selector, result, top_n) {
            return true;
        }
    }
    false
}
// #[test]
// fn test_bfs() {
//     use crate::parser::parse;
//
//     let html = r#"<html><body><div><p>Satu</p><p>Dua</p></div></body></html>"#;
//     let tree = parse(html);
//     let result = bfs(&tree, "p", 0).unwrap();
//
//     println!("Traversal log: {:?}", result.traversal_log);
//     println!("Found at: {:?}", result.found_indices);
//
//     assert_eq!(result.found_indices.len(), 2);
// }
//
// #[test]
// fn test_combinator_selectors() {
//     use crate::parser::parse;
//
//     let html = r#"
//         <html>
//             <body>
//                 <main>
//                     <section class="card"><p id="inside">Inside</p></section>
//                     <p id="after-section">After</p>
//                     <article><p id="nested">Nested</p></article>
//                     <h2>Title</h2>
//                     <p id="after-heading">Lead</p>
//                     <p id="last">Last</p>
//                 </main>
//             </body>
//         </html>
//     "#;
//     let tree = parse(html);
//
//     assert_eq!(
//         ids_for(&tree, bfs(&tree, "main p", 0).unwrap()),
//         vec!["after-section", "after-heading", "last", "inside", "nested",]
//     );
//     assert_eq!(
//         ids_for(&tree, bfs(&tree, "main > p", 0).unwrap()),
//         vec!["after-section", "after-heading", "last"]
//     );
//     assert_eq!(
//         ids_for(&tree, bfs(&tree, "section + p", 0).unwrap()),
//         vec!["after-section"]
//     );
//     assert_eq!(
//         ids_for(&tree, bfs(&tree, "section ~ p", 0).unwrap()),
//         vec!["after-section", "after-heading", "last"]
//     );
// }
//
// #[cfg(test)]
// fn ids_for(tree: &DomTree, result: SearchResult) -> Vec<String> {
//     result
//         .found_indices
//         .iter()
//         .filter_map(|&index| tree.nodes[index].attributes.get("id").cloned())
//         .collect()
// }
