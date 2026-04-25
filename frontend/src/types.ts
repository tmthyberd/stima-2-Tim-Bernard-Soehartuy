export type NodeType = "Document" | "Element" | "Text";

export interface DomNode {
    node_type: NodeType;
    tag_name: string | null;
    attributes: Record<string, string>;
    children: number[];
    parent: number | null;
    text_content: string | null;
}

export interface DomTree {
    nodes: DomNode[];
}

export interface SearchResponse {
    found_indices: number[];
    traversal_log: number[];
    tree: DomTree;
}

export type Algorithm = "BFS" | "DFS";

export interface SearchParams {
    url: string;
    html: string;
    algorithm: Algorithm;
    selector: string;
    topN: number;
}

export interface SearchMetrics {
    executionTime: number;
    nodesVisited: number;
    traversalList: string[];
}
