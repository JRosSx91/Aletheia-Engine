import * as THREE from "three";
import React, { useImperativeHandle } from "react";
import { NeuralNode } from "../../../../hooks/useNeuralNetwork";

export const NeuralNodes = React.forwardRef(
  (
    {
      nodes,
      globalTime: initialTime,
    }: { nodes: NeuralNode[]; globalTime: number },
    ref
  ) => {
    const groupRef = React.useRef<THREE.Group>(null!);

    useImperativeHandle(ref, () => ({
      get globalTime() {
        return (groupRef.current as any).globalTime;
      },
      set globalTime(value) {
        (groupRef.current as any).globalTime = value;
      },
    }));

    (groupRef.current as any) = {
      ...groupRef.current,
      globalTime: initialTime,
    };

    return (
      <group ref={groupRef}>
        {nodes.map((node) => {
          if (node.connections.length !== 1 || node.level === 0) {
            return null;
          }

          const time = (groupRef.current as any)?.globalTime || 0;
          const activity =
            Math.sin(time * 0.5 + node.position.length()) * 0.5 + 0.5;
          const size = 0.003 + activity * 0.002;
          const intensity = 0.3 + activity * 0.4;

          return (
            <mesh key={node.id} position={node.position}>
              <sphereGeometry args={[size, 6, 6]} />
              <meshBasicMaterial
                color={"#00aaff"}
                transparent
                opacity={intensity}
                blending={THREE.AdditiveBlending}
                depthWrite={false}
              />
            </mesh>
          );
        })}
      </group>
    );
  }
);
