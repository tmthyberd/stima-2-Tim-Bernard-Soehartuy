# API Contracts

Base URL: `http://localhost:8080`

---

## Data Types

### DomNode
```json
{
  "node_type": "Document" | "Element" | "Text",
  "tag_name": string | null,
  "attributes": { [key: string]: string },
  "children": number[],
  "parent": number | null,
  "text_content": string | null
}


### DomTree
{
  "nodes": DomNode[]
}


### End Points

1. Parse HTML
Menerima string HTML dan mengembalikan DOM Tree.

Request
POST /api/parse
Content-Type: application/json

{
  "html": "<div id='root'><p>Hello</p></div>"
}

Response
{
  "tree": {
    "nodes": [
      {
        "node_type": "Document",
        "tag_name": null,
        "attributes": {},
        "children": [1],
        "parent": null,
        "text_content": null
      },
      {
        "node_type": "Element",
        "tag_name": "div",
        "attributes": { "id": "root" },
        "children": [2],
        "parent": 0,
        "text_content": null
      },
      {
        "node_type": "Element",
        "tag_name": "p",
        "attributes": {},
        "children": [3],
        "parent": 1,
        "text_content": null
      },
      {
        "node_type": "Text",
        "tag_name": null,
        "attributes": {},
        "children": [],
        "parent": 2,
        "text_content": "Hello"
      }
    ]
  },
  "node_count": 4
}

2. Scrape URL
Menerima URL dan mengembalikan HTML string dari halaman tersebut.

Request
POST /api/scrape
Content-Type: application/json\
JSON
{
  "url": "[https://example.com](https://example.com)"
}
RESPONSE
{
  "html": "<!DOCTYPE html><html>...</html>"
}


3. Search
Menerima HTML, selector, dan algoritma — mengembalikan hasil pencarian beserta DOM Tree dan traversal log.

Request
POST /api/search
Content-Type: application/json
JSON
{
  "html": "<div><p class='box'>Halo</p><p>Dunia</p></div>",
  "selector": ".box",
  "algorithm": "bfs"
}
Response
{
  "found_indices": [2],
  "traversal_log": [0, 1, 2, 3, 4],
  "tree": {
    "nodes": [ ... ]
  }
}

Error Response
{
  "error": "Pesan error di sini"
}

---

### 2. File: `src/models/mod.rs`
(Ganti semua isi file `mod.rs` di dalam folder `models` dengan ini)

```rust
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum NodeType {
    Document,
    Element,
    Text,
}

#[derive(Debug, Serialize, Clone)]
pub struct DomNode {
    pub node_type: NodeType,
    pub tag_name: Option<String>,
    pub attributes: HashMap<String, String>,
    pub children: Vec<usize>,
    pub parent: Option<usize>,
    pub text_content: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DomTree {
    pub nodes: Vec<DomNode>,
}

impl DomTree {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: DomNode) -> usize {
        let index = self.nodes.len();
        self.nodes.push(node);
        index
    }
}

File : Cargo.toml
[package]
name = "backend"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
# Tambahkan ini jika belum ada untuk server nanti
axum = "0.7" 
tower-http = { version = "0.5", features = ["cors"] }