export interface Node {
  id: string;
  title: string;
  description?: string;
}

export interface Connection {
  id: string;
  type: string;
  fromNode: Node;
  toNode: Node;
}

export interface GraphData {
  nodes: Node[];
  connections: Connection[];
}
