import { useMemo, useRef } from "react";
import * as THREE from "three";
import { useFrame } from "@react-three/fiber";
import { shaderMaterial } from "@react-three/drei";
import { useControls } from "leva";
import vertexShader from "./shaders/galaxy/galaxy.vertex.glsl?raw";
import fragmentShader from "./shaders/galaxy/galaxy.fragment.glsl?raw";

export const ParticleGalaxyMaterial = shaderMaterial(
  { uTime: 0, uSize: 30.0, uCurlStrength: 4.0 },
  vertexShader,
  fragmentShader
);

export function ParticleGalaxy() {
  const materialRef = useRef<THREE.ShaderMaterial>(null!);

  const galaxyControls = useControls("Galaxia", {
    curlStrength: { value: 4.0, min: 0, max: 20, step: 0.1, label: "Caos" },
    size: { value: 30.0, min: 1, max: 100, label: "Tamaño Puntos" },
  });

  const particlePositions = useMemo(() => {
    const count = 50000;
    const positions = new Float32Array(count * 3);
    for (let i = 0; i < count; i++) {
      const theta = Math.acos(2.0 * Math.random() - 1.0);
      const phi = Math.random() * Math.PI * 2.0;
      const radius = 3 + Math.random() * 2;
      positions[i * 3] = Math.cos(phi) * Math.sin(theta) * radius;
      positions[i * 3 + 1] = Math.sin(phi) * Math.sin(theta) * radius;
      positions[i * 3 + 2] = Math.cos(theta) * radius;
    }
    return positions;
  }, []);

  useFrame((state) => {
    if (materialRef.current) {
      materialRef.current.uniforms.uTime.value = state.clock.getElapsedTime();
      materialRef.current.uniforms.uCurlStrength.value =
        galaxyControls.curlStrength;
      materialRef.current.uniforms.uSize.value = galaxyControls.size;
    }
  });

  return (
    <points>
      <bufferGeometry>
        <bufferAttribute
          attach="attributes-position"
          count={particlePositions.length / 3}
          args={[particlePositions, 3]}
        />
      </bufferGeometry>
      {/* Usamos nuestro material de shader personalizado */}
      <particleGalaxyMaterial
        ref={materialRef}
        key={ParticleGalaxyMaterial.key}
        transparent={true}
        blending={THREE.AdditiveBlending}
        depthWrite={false}
      />
    </points>
  );
}
