import { useRef } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";

interface QuantumCellProps {
  position: [number, number, number];
  color: THREE.Color;
  active: boolean;
}

export const QuantumCell = ({ position, color, active }: QuantumCellProps) => {
  const meshRef = useRef<THREE.Mesh>(null);
  const materialRef = useRef<any>(null);

  useFrame((state) => {
    if (materialRef.current) {
      materialRef.current.uniforms.uTime.value = state.clock.elapsedTime;
      materialRef.current.uniforms.uActive.value = active ? 1.0 : 0.0;
      materialRef.current.uniforms.uCellColor.value = color;
    }
  });

  return (
    <mesh ref={meshRef} position={position}>
      <boxGeometry args={[1.8, 1.8, 1.8]} />
      <quantumCellMaterial
        ref={materialRef}
        transparent
        blending={THREE.AdditiveBlending}
        depthWrite={false}
        uCellColor={color}
        uOpacity={0.75}
        uActive={active ? 1.0 : 0.0}
      />
    </mesh>
  );
};
