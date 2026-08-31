import React, { useState, useCallback, useEffect } from 'react';
import { ReactFlow, Background, Controls, useNodesState, useEdgesState } from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { CustomNode, edgeOptions, mapDomToGraph } from './lib/graphMapper';
import { SearchResponse } from './types';

const nodeTypes = {
  custom: CustomNode,
};

const API_BASE_URL = "http://localhost:8080/api";

export default function ExplorerPage() {
  const [isVisualizing, setIsVisualizing] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [errorMsg, setErrorMsg] = useState('');

  // State Mode A
  const [url, setUrl] = useState('');
  const [rawHtml, setRawHtml] = useState('');
  const [algorithm, setAlgorithm] = useState('BFS');
  const [targetElement, setTargetElement] = useState('');
  const [topN, setTopN] = useState<number | string>(''); // Kosong atau 0 berarti semua

  // State Mode B
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [executionTime, setExecutionTime] = useState(0);
  const [nodesVisitedCount, setNodesVisitedCount] = useState(0);
  const [traversalLog, setTraversalLog] = useState<string[]>([]);

  // State untuk Animasi Traversal
  const [searchResult, setSearchResult] = useState<{ tree: any, found_indices: number[], traversal_log: number[], limit: number, max_depth: number } | null>(null);
  const [playbackStep, setPlaybackStep] = useState(0);

  // Helper Kedalaman Maksimum
  const calculateMaxDepth = useCallback((nodes: any[], rootIndex: number = 0, currentDepth: number = 0): number => {
    if (!nodes || nodes.length === 0) return 0;
    const node = nodes[rootIndex];
    if (!node || !node.children || node.children.length === 0) return currentDepth;

    let maxChildDepth = currentDepth;
    for (const childIdx of node.children) {
      const childDepth = calculateMaxDepth(nodes, childIdx, currentDepth + 1);
      if (childDepth > maxChildDepth) {
        maxChildDepth = childDepth;
      }
    }
    return maxChildDepth;
  }, []);

  const handleStart = async (e: React.FormEvent) => {
    e.preventDefault();
    setErrorMsg('');
    setIsLoading(true);

    try {
      let targetHtml = rawHtml;

      if (!url.trim() && !rawHtml.trim()) {
        throw new Error("Persiapan gagal: Anda harus memasukkan Tujuan (URL) atau Bahan Mentah (Raw HTML).");
      }

      if (url.trim() !== "" && rawHtml.trim() !== "") {
        throw new Error("Persiapan gagal: Harap isi salah satu saja (Tujuan URL ATAU raw HTML)");
      }

      if (!targetElement.trim()) {
        throw new Error("Persiapan gagal: Target Elemen (CSS Selector) tidak boleh kosong.");
      }

      const parsedTopN = parseInt(topN as string, 10);
      if (topN !== '' && isNaN(parsedTopN)) {
        throw new Error("Persiapan gagal: Batas Penemuan (Top N) harus berupa angka yang valid.");
      }
      if (!isNaN(parsedTopN) && parsedTopN < 0) {
        throw new Error("Persiapan gagal: Batas Penemuan (Top N) tidak boleh kurang dari nol.");
      }

      // 1. Jika URL terisi, lakukan Scraping ke Backend
      if (url.trim() !== "") {
        const scrapeRes = await fetch(`${API_BASE_URL}/scrape`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ url: url.trim() })
        });

        if (!scrapeRes.ok) {
          throw new Error(`Gagal melakukan Scraping: Cek kembali URL. (Status: ${scrapeRes.status})`);
        }

        const scrapeData = await scrapeRes.json();
        targetHtml = scrapeData.html;
      }

      // 2. Catat waktu mulai
      const startTime = performance.now();

      const top_n_payload = (!isNaN(parsedTopN) && parsedTopN > 0) ? parsedTopN : 0;

      // 3. Lakukan Search/Traversal ke Backend
      const searchRes = await fetch(`${API_BASE_URL}/search`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          html: targetHtml,
          selector: targetElement,
          algorithm: algorithm.toLowerCase(), // Backend butuh 'bfs' atau 'dfs' lowercase
          top_n: top_n_payload
        })
      });

      if (!searchRes.ok) {
        const errData = await searchRes.json().catch(() => ({}));
        throw new Error(errData.error || `Gagal melakukan penelusuran. (Status: ${searchRes.status})`);
      }

      const data: SearchResponse = await searchRes.json();

      // 4. Catat waktu selesai & hitung durasi
      const endTime = performance.now();
      const durationMs = endTime - startTime;

      // 5. Logika limit 
      const limit = top_n_payload > 0 ? top_n_payload : data.found_indices.length;

      // 6. Simpan Hasil untuk Animasi
      setSearchResult({
        tree: data.tree,
        found_indices: data.found_indices,
        traversal_log: data.traversal_log,
        limit,
        max_depth: calculateMaxDepth(data.tree.nodes, 0)
      });

      // Setup state untuk UI Mode B
      setExecutionTime(Number(durationMs.toFixed(2)));
      setNodesVisitedCount(0); // Akan diupdate oleh animasi
      setTraversalLog([]); // Akan diupdate oleh animasi
      setPlaybackStep(1); // Mulai dari langkah pertama

      setIsVisualizing(true);

    } catch (error: any) {
      console.error("Search failed:", error);
      setErrorMsg(error.message || "Terjadi kesalahan saat memulai penjelajahan.");
    } finally {
      setIsLoading(false);
    }
  };

  // Efek untuk memutar animasi secara bertahap
  useEffect(() => {
    if (!searchResult || !isVisualizing) return;

    if (playbackStep < searchResult.traversal_log.length) {
      const timer = setTimeout(() => {
        setPlaybackStep(prev => prev + 1);
      }, 1); // 1000ms (1 detik) per iterasi agar alur lebih mudah diikuti
      return () => clearTimeout(timer);
    }
  }, [searchResult, playbackStep, isVisualizing]);

  // Efek untuk mengupdate graf ketika langkah bertambah
  useEffect(() => {
    if (!searchResult || !isVisualizing) return;

    const currentTraversal = searchResult.traversal_log.slice(0, playbackStep);
    const { nodes: newNodes, edges: newEdges } = mapDomToGraph(
      searchResult.tree,
      searchResult.found_indices,
      currentTraversal,
      searchResult.limit
    );
    setNodes(newNodes);
    setEdges(newEdges);

    setNodesVisitedCount(currentTraversal.length);

    // Buat log
    const newLogs = currentTraversal.map(idx => {
      const node = searchResult.tree.nodes[idx];
      const isFound = searchResult.found_indices.includes(idx);
      const tag = node.tag_name ? `<${node.tag_name}>` : (node.node_type || "Unknown");
      return isFound ? `Menemukan target di ${tag}!` : `Menyinggahi ${tag}`;
    });
    setTraversalLog(newLogs);

  }, [searchResult, playbackStep, isVisualizing, setNodes, setEdges]);


  // --- MODE A: FORM ---
  if (!isVisualizing) {
    return (
      <div className="flex flex-col items-center justify-center py-20 px-4 relative">
        {/* Loading overlay for Mode A */}
        {isLoading && (
          <div className="absolute inset-0 z-50 bg-[#F4F0E6]/80 flex flex-col items-center justify-center">
            <div className="w-12 h-12 border-4 border-[#728C69] border-t-transparent rounded-full animate-spin mb-4" />
            <p className="font-bold text-[#4A453F] tracking-wide animate-pulse">Menyiapkan Peralatan...</p>
          </div>
        )}

        <h1 className="text-4xl font-black mb-8 text-[#4A453F]">
          Mulai Petualangan DOM
        </h1>

        {errorMsg && (
          <div className="w-full max-w-2xl bg-[#E8B851]/20 border-2 border-[#E8B851] text-[#4A453F] px-5 py-4 rounded-xl mb-6 font-bold flex items-start gap-3 shadow-[4px_4px_0px_0px_rgba(232,184,81,0.5)]">
            <span className="text-xl leading-none mt-0.5">&#9888;</span>
            <span>{errorMsg}</span>
          </div>
        )}

        <form
          onSubmit={handleStart}
          className="w-full max-w-2xl bg-white border-2 border-[#4A453F] rounded-2xl p-8 shadow-[8px_8px_0px_0px_rgba(74,69,63,1)]"
        >
          <div className="space-y-6">
            <div>
              <label className="block font-bold mb-2">Tujuan (URL)</label>
              <input
                type="url"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                placeholder="https://..."
                className="w-full bg-[#F4F0E6] border-2 border-[#4A453F] rounded-xl px-4 py-3 focus:outline-none focus:border-[#728C69] transition-colors placeholder:text-[#4A453F]/40 font-mono"
              />
            </div>

            <div className="flex items-center font-bold text-[#4A453F]/50">
              <div className="flex-1 border-b-2 border-[#4A453F]/20"></div>
              <span className="px-4">atau</span>
              <div className="flex-1 border-b-2 border-[#4A453F]/20"></div>
            </div>

            <div>
              <label className="block font-bold mb-2">Bahan Mentah (Raw HTML)</label>
              <textarea
                value={rawHtml}
                onChange={(e) => setRawHtml(e.target.value)}
                placeholder="<div id='app'>...</div>"
                rows={4}
                className="w-full bg-[#F4F0E6] border-2 border-[#4A453F] rounded-xl px-4 py-3 focus:outline-none focus:border-[#728C69] transition-colors resize-none placeholder:text-[#4A453F]/40 font-mono"
              />
            </div>

            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block font-bold mb-2">Gaya Penjelajahan</label>
                <select
                  value={algorithm}
                  onChange={(e) => setAlgorithm(e.target.value)}
                  className="w-full bg-[#F4F0E6] border-2 border-[#4A453F] rounded-xl px-4 py-3 focus:outline-none focus:border-[#728C69] font-bold appearance-none cursor-pointer"
                >
                  <option value="BFS">Breadth-First</option>
                  <option value="DFS">Depth-First</option>
                </select>
              </div>
              <div>
                <label className="block font-bold mb-2">Target Elemen</label>
                <input
                  type="text"
                  value={targetElement}
                  onChange={(e) => setTargetElement(e.target.value)}
                  placeholder=".class, #id"
                  className="w-full bg-[#F4F0E6] border-2 border-[#4A453F] rounded-xl px-4 py-3 focus:outline-none focus:border-[#728C69] font-mono"
                />
              </div>
            </div>

            <div>
              <label className="block font-bold mb-2">Batas Penemuan (Top N)</label>
              <input
                type="number"
                min="0"
                value={topN}
                onChange={(e) => setTopN(e.target.value)}
                placeholder="0 (Temukan Semua)"
                className="w-full bg-[#F4F0E6] border-2 border-[#4A453F] rounded-xl px-4 py-3 focus:outline-none focus:border-[#728C69] font-mono"
              />
            </div>

            <button
              type="submit"
              disabled={isLoading}
              className="w-full bg-[#728C69] text-[#FCFAF5] rounded-xl font-bold py-4 border-2 border-[#4A453F] shadow-[4px_4px_0px_0px_rgba(74,69,63,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(74,69,63,1)] transition-all mt-4 text-lg disabled:opacity-70"
            >
              Mulai Penjelajahan &rarr;
            </button>
          </div>
        </form>
      </div>
    );
  }

  // --- MODE B: VISUALISASI GRAF ---
  return (
    <div className="flex flex-col h-[calc(100vh-68px)]">
      {/* 70% Canvas */}
      <div className="h-[70%] border-b-2 border-[#4A453F] bg-[#F4F0E6] relative">
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          nodeTypes={nodeTypes}
          fitView
          fitViewOptions={{ maxZoom: 1, padding: 0.2 }}
        >
          <Background color="#4A453F" gap={24} size={2} className="opacity-20" />
          <Controls className="border-2 border-[#4A453F] bg-[#FCFAF5] shadow-[4px_4px_0px_0px_rgba(74,69,63,1)] rounded-xl overflow-hidden [&>button]:border-b-2 [&>button]:border-[#4A453F] [&>button:last-child]:border-0" />
        </ReactFlow>
      </div>

      {/* 30% Panel Bawah */}
      <div className="h-[30%] bg-white p-6 grid grid-cols-1 md:grid-cols-3 gap-6">

        {/* Kolom 1 */}
        <div className="flex items-start">
          <button
            onClick={() => setIsVisualizing(false)}
            className="px-6 py-3 bg-[#FCFAF5] border-2 border-[#4A453F] rounded-xl font-bold shadow-[4px_4px_0px_0px_rgba(74,69,63,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(74,69,63,1)] transition-all"
          >
            &larr; Cari Tempat Lain
          </button>
        </div>

        {/* Kolom 2 */}
        <div className="bg-[#F4F0E6] border-2 border-[#4A453F] rounded-xl p-5 flex flex-col justify-start">
          <h3 className="font-bold mb-4 border-b-2 border-[#4A453F] pb-2 text-lg">Ringkasan Perjalanan</h3>
          <div className="grid grid-cols-3 gap-4">
            <div>
              <p className="text-sm font-bold opacity-80 mb-1">Waktu Ditempuh</p>
              <p className="font-mono font-bold text-2xl">{executionTime} <span className="text-sm font-sans">ms</span></p>
            </div>
            <div>
              <p className="text-sm font-bold opacity-80 mb-1">Disinggahi</p>
              <p className="font-mono font-bold text-2xl">{nodesVisitedCount}</p>
            </div>
            <div>
              <p className="text-sm font-bold opacity-80 mb-1">Kedalaman Maks</p>
              <p className="font-mono font-bold text-2xl">{searchResult?.max_depth || 0}</p>
            </div>
          </div>
        </div>

        {/* Kolom 3 */}
        <div className="bg-[#F4F0E6] border-2 border-[#4A453F] rounded-xl p-4 flex flex-col">
          <h3 className="font-bold mb-3 border-b-2 border-[#4A453F] pb-2 text-lg">Catatan Petualangan</h3>
          <div className="flex-1 overflow-y-auto space-y-2 pr-2 font-mono text-sm bg-white border-2 border-[#4A453F] rounded-lg p-3">
            {traversalLog.map((log, idx) => (
              <div key={idx} className="flex gap-2 items-start">
                <span className="text-[#728C69] mt-0.5">&bull;</span>
                <span>{log}</span>
              </div>
            ))}
            {traversalLog.length === 0 && <span className="opacity-50 italic">Jurnal masih kosong...</span>}
          </div>
        </div>

      </div>
    </div>
  );
}
