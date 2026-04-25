import dagre from 'dagre';
import { Node, Edge, Handle, Position, NodeProps } from '@xyflow/react';
import { DomTree } from '../types';

const nodeWidth = 160;
const nodeHeight = 90;

// Komponen Custom Node
export function CustomNode({ data }: NodeProps) {
    let bgColor = 'bg-[#FCFAF5]'; // Default Node (Kertas)
    let textColor = 'text-[#4A453F]';
    let ringEffect = '';

    if (data.isFound) {
        bgColor = 'bg-[#728C69]'; // Hijau (hanya untuk target yang ketemu)
        textColor = 'text-white';
    } else if (data.isDeadEnd) {
        bgColor = 'bg-[#E8E3D8]'; // Warna kusam untuk dead end (jalan buntu)
    }

    if (data.isActive) {
        ringEffect = 'ring-4 ring-[#E8B851] ring-offset-2 ring-offset-[#F4F0E6]'; // Efek tebal emas
    }

    return (
        <div className={`px-4 py-3 rounded-xl border-2 border-[#4A453F] ${bgColor} w-[150px] flex flex-col items-center transition-all duration-300 ${ringEffect}`}>
            <Handle type="target" position={Position.Top} className="w-3 h-3 border-2 border-[#4A453F] bg-[#F4F0E6]" />
            
            <div className={`font-black text-sm mb-1.5 w-full border-b-2 border-current pb-1 text-center ${textColor}`}>
                {data.tagName ? `<${data.tagName as string}>` : data.nodeType as string}
            </div>
            
            <div className={`text-[10px] w-full flex flex-col items-start gap-0.5 font-mono opacity-90 ${textColor}`}>
                {data.idAttr ? (
                    <div className="flex gap-1 w-full"><span className="opacity-60 shrink-0">id:</span> <span className="truncate" title={`#${data.idAttr}`}>#{data.idAttr as string}</span></div>
                ) : null}
                {data.classNameAttr ? (
                    <div className="flex gap-1 w-full"><span className="opacity-60 shrink-0">cls:</span> <span className="truncate" title={`.${data.classNameAttr}`}>.{data.classNameAttr as string}</span></div>
                ) : null}
                {!data.idAttr && !data.classNameAttr && !data.textContent ? (
                    <div className="italic opacity-50 w-full text-center mt-1">-</div>
                ) : null}
                
                {/* Tampilkan text_content langsung di dalam node elemen */}
                {data.textContent && (data.textContent as string).trim() !== "" ? (
                    <div className="w-full text-center italic truncate px-1 mt-1 font-sans border-t border-current/20 pt-1" title={data.textContent as string}>
                        "{(data.textContent as string).trim()}"
                    </div>
                ) : null}
            </div>

            <Handle type="source" position={Position.Bottom} className="w-3 h-3 border-2 border-[#4A453F] bg-[#F4F0E6]" />
        </div>
    );
}

// Opsi garis (Edges) penghubung
export const edgeOptions = {
    style: { stroke: '#728C69', strokeWidth: 2 },
};

export const getLayoutedElements = (nodes: Node[], edges: Edge[], direction = 'TB') => {
    const dagreGraph = new dagre.graphlib.Graph();
    dagreGraph.setDefaultEdgeLabel(() => ({}));

    const isHorizontal = direction === 'LR';
    dagreGraph.setGraph({ 
        rankdir: direction,
        nodesep: 80, // Jarak horizontal antar node
        ranksep: 100, // Jarak vertikal antar level
        marginx: 40,
        marginy: 40,
        ranker: 'tight-tree' // Memaksa node dengan depth yang sama berada di level/Y-coordinate yang persis sama
    });

    nodes.forEach((node) => {
        dagreGraph.setNode(node.id, { width: nodeWidth, height: nodeHeight });
    });

    edges.forEach((edge) => {
        dagreGraph.setEdge(edge.source, edge.target);
    });

    dagre.layout(dagreGraph);

    const newNodes = nodes.map((node) => {
        const nodeWithPosition = dagreGraph.node(node.id);
        return {
            ...node,
            targetPosition: isHorizontal ? Position.Left : Position.Top,
            sourcePosition: isHorizontal ? Position.Right : Position.Bottom,
            position: {
                x: nodeWithPosition.x - nodeWidth / 2,
                y: nodeWithPosition.y - nodeHeight / 2,
            },
        };
    });

    return { nodes: newNodes, edges };
};

export const mapDomToGraph = (
    tree: DomTree,
    foundIndices: number[],
    traversalLog: number[],
    topN: number
) => {
    const nodes: Node[] = [];
    const edges: Edge[] = [];

    const visibleFoundIndices = new Set(foundIndices.slice(0, topN));
    const visitedIndices = new Set(traversalLog);
    const activeIndex = traversalLog.length > 0 ? traversalLog[traversalLog.length - 1] : -1;

    tree.nodes.forEach((node, index) => {
        const isVisited = visitedIndices.has(index);
        
        // HANYA RENDER JIKA SUDAH DIVISIT
        if (!isVisited) return;

        const label = node.tag_name || node.node_type;
        const isFound = visibleFoundIndices.has(index);
        const isActive = (index === activeIndex);
        const isDeadEnd = (!isFound && node.children.length === 0);

        nodes.push({
            id: index.toString(),
            type: 'custom',
            data: {
                label,
                isFound,
                isVisited,
                isActive,
                isDeadEnd,
                nodeType: node.node_type,
                tagName: node.tag_name,
                classNameAttr: node.attributes?.class || node.attributes?.className,
                idAttr: node.attributes?.id,
                textContent: node.text_content,
            },
            position: { x: 0, y: 0 },
        });

        node.children.forEach((childIndex) => {
            const isChildVisited = visitedIndices.has(childIndex);
            
            // HANYA RENDER EDGE JIKA CHILD JUGA DIVISIT
            if (isChildVisited) {
                const isChildFound = visibleFoundIndices.has(childIndex);
                
                edges.push({
                    id: `e${index}-${childIndex}`,
                    source: index.toString(),
                    target: childIndex.toString(),
                    type: 'smoothstep',
                    animated: !isChildFound, // Marching ants effect untuk rute pencarian biasa
                    style: {
                        stroke: isChildFound ? '#728C69' : '#A8A49C', // Hijau tebal jika target, Abu-abu jika numpang lewat
                        strokeWidth: isChildFound ? 3 : 2,
                        strokeDasharray: isChildFound ? 'none' : '5,5', // Putus-putus jika sekadar lewat
                    },
                });
            }
        });
    });

    return getLayoutedElements(nodes, edges);
};
