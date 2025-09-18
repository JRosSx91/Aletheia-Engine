import { useRef } from "react";
import * as THREE from "three";
import { useFrame } from "@react-three/fiber";
import { useNeuralNetwork } from "../../../../hooks/useNeuralNetwork";
import { NeuralNodes } from "./NeuralNodes";
import { NeuralConnection } from "./NeuralConnection";
import { FluidCore } from "../FluidCore";

export function NeuralNetwork({
  speaking,
  thinking,
}: {
  speaking: boolean;
  thinking: boolean;
}) {
  const groupRef = useRef<THREE.Group>(null!);
  const nodesRef = useRef<THREE.Group>(null!); // Ref para los nodos

  const { nodes, connections, config } = useNeuralNetwork();

  useFrame((state) => {
    const time = state.clock.elapsedTime;

    if (nodesRef.current) {
      (nodesRef.current as any).globalTime = time;
    }
  });

  return (
    <group ref={groupRef}>
      <FluidCore
        speaking={speaking}
        thinking={thinking}
        scale={config.fluidCoreRadius}
        isStatic={false}
      />

      <NeuralNodes ref={nodesRef} nodes={nodes} globalTime={0} />

      {connections.map((conn) => (
        <NeuralConnection
          key={conn.id}
          startNode={conn.start}
          endNode={conn.end}
          pulseTime={conn.pulseTime}
          energy={conn.energy}
        />
      ))}
    </group>
  );
}
