import { Background, Controls, type Edge, type Node, ReactFlow } from "@xyflow/react";
import type React from "react";

interface NodalCanvasProps {
  nodes: Node[];
  edges: Edge[];
  onNodeClick: (nodeId: string) => void;
}

export function NodalCanvas({ nodes, edges, onNodeClick }: NodalCanvasProps): React.JSX.Element {
  return (
    <div style={{ width: "100%", height: "100%" }}>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodeClick={(_, node) => onNodeClick(node.id)}
        fitView
      >
        <Background color="#1e293b" gap={16} />
        <Controls />
      </ReactFlow>
    </div>
  );
}
