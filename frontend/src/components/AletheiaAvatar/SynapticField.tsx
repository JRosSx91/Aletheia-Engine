import { useRef, useMemo } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";

export function SynapticField({
  count = 5000,
  speaking,
  thinking,
}: {
  count?: number;
  speaking: boolean;
  thinking: boolean;
}) {
  const meshRef = useRef<THREE.Points>(null!);

  const particles = useMemo(() => {
    const positions = new Float32Array(count * 3);
    for (let i = 0; i < count; i++) {
      const theta = Math.random() * Math.PI * 2;
      const phi = Math.acos(Math.random() * 2 - 1);
      const radius = 2 + Math.random() * 1.5;

      positions.set(
        [
          Math.sin(phi) * Math.cos(theta) * radius,
          Math.sin(phi) * Math.sin(theta) * radius,
          Math.cos(phi) * radius,
        ],
        i * 3
      );
    }
    return { positions };
  }, [count]);

  useFrame((state) => {
    if (meshRef.current) {
      const time = state.clock.elapsedTime;
      const posAttr = meshRef.current.geometry.attributes
        .position as THREE.BufferAttribute;

      for (let i = 0; i < count; i++) {
        const i3 = i * 3;
        const x = particles.positions[i3];
        const y = particles.positions[i3 + 1];
        const z = particles.positions[i3 + 2];

        const wave1 = Math.sin(time * 0.5 + x * 0.5) * 0.1;
        const wave2 = Math.cos(time * 0.7 + y * 0.5) * 0.1;
        const wave3 = Math.sin(time * 0.3 + z * 0.5) * 0.1;

        posAttr.setXYZ(
          i,
          x + wave1 * (speaking ? 2 : 1),
          y + wave2 * (speaking ? 2 : 1),
          z + wave3 * (thinking ? 1.5 : 1)
        );
      }

      posAttr.needsUpdate = true;
      meshRef.current.rotation.y = time * 0.05;
    }
  });

  return (
    <points ref={meshRef}>
      <bufferGeometry>
        <bufferAttribute
          attach="attributes-position"
          args={[particles.positions, 3]}
        />
      </bufferGeometry>
      <pointsMaterial
        size={0.03}
        color="#00ddff"
        blending={THREE.AdditiveBlending}
        transparent
        opacity={0.6}
        sizeAttenuation={true}
      />
    </points>
  );
}
