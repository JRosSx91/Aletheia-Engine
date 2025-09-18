import { useMemo } from "react";
import * as THREE from "three";

export class NeuralNode {
  position: THREE.Vector3;
  connections: NeuralNode[] = [];
  level: number;
  id: string;
  energy: number;

  constructor(
    position: THREE.Vector3,
    level: number,
    id: string,
    energy: number
  ) {
    this.position = position;
    this.level = level;
    this.id = id;
    this.energy = energy;
  }

  addConnection(node: NeuralNode) {
    if (!this.connections.includes(node)) {
      this.connections.push(node);
      node.connections.push(this);
    }
  }
}

// --- ALGORITMO DE GENERACIÓN SIMPLE Y RÁPIDO ---
// En hooks/useNeuralNetwork.ts

function generateSimpleNetwork(config: any): NeuralNode[] {
  const nodes: NeuralNode[] = [];
  let idCounter = 0;

  const rootNode = new NeuralNode(
    new THREE.Vector3(0, 0, 0),
    0,
    `node-root`,
    config.initialEnergy
  );
  nodes.push(rootNode);

  let activeBranches: NeuralNode[] = [rootNode];

  for (let level = 1; level <= config.maxLevels; level++) {
    const newBranches: NeuralNode[] = [];
    for (const parentNode of activeBranches) {
      const numChildren = Math.floor(
        Math.random() * (config.maxChildrenPerNode + 1)
      );

      for (let i = 0; i < numChildren; i++) {
        if (nodes.length > config.maxTotalNodes) break;

        // --- INICIO DE LA MODIFICACIÓN 2D ---
        // Calculamos una dirección 2D aleatoria y la normalizamos.
        const randomAngle = Math.random() * Math.PI * 2;
        const randomDirection = new THREE.Vector3(
          Math.cos(randomAngle),
          Math.sin(randomAngle),
          0
        );

        // Si el padre tiene una dirección, nos desviamos ligeramente de ella.
        let direction = randomDirection;
        if (parentNode.position.length() > 0) {
          const parentDirection = parentNode.position.clone().normalize();
          direction = parentDirection.lerp(randomDirection, 0.4).normalize();
        }
        // --- FIN DE LA MODIFICACIÓN 2D ---

        const newPosition = parentNode.position
          .clone()
          .add(direction.multiplyScalar(config.segmentLength));

        if (newPosition.length() > config.maxRadius) continue;

        const newNode = new NeuralNode(
          newPosition,
          level,
          `node-${idCounter++}`,
          parentNode.energy * 0.85
        );

        parentNode.addConnection(newNode);
        nodes.push(newNode);
        newBranches.push(newNode);
      }
    }
    activeBranches = newBranches;
    if (activeBranches.length === 0 || nodes.length > config.maxTotalNodes)
      break;
  }

  return nodes;
}

export const useNeuralNetwork = () => {
  const networkData = useMemo(() => {
    // Parámetros muy conservadores para garantizar el rendimiento
    const config = {
      maxLevels: 6,
      maxChildrenPerNode: 3,
      maxRadius: 4.0,
      segmentLength: 0.5,
      initialEnergy: 1.0,
      fluidCoreRadius: 1.5, // Mantenemos este dato para el componente
      maxTotalNodes: 800, // Límite de seguridad para evitar cuelgues
    };

    const nodes = generateSimpleNetwork(config);

    const connections: Array<{
      start: NeuralNode;
      end: NeuralNode;
      pulseTime: number;
      id: string;
      energy: number;
    }> = [];
    const addedConnections = new Set<string>();
    nodes.forEach((node) => {
      node.connections.forEach((connectedNode) => {
        const id1 = node.id;
        const id2 = connectedNode.id;
        const forwardId = `${id1}-${id2}`;
        const backwardId = `${id2}-${id1}`;
        if (
          !addedConnections.has(forwardId) &&
          !addedConnections.has(backwardId)
        ) {
          connections.push({
            start: node,
            end: connectedNode,
            pulseTime: Math.random() * 5,
            id: forwardId,
            energy: Math.min(node.energy, connectedNode.energy),
          });
          addedConnections.add(forwardId);
        }
      });
    });

    return { nodes, connections, config };
  }, []);

  return networkData;
};
