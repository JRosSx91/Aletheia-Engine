import { useRef } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";

export function FluidCore({
  speaking,
  thinking,
}: {
  speaking: boolean;
  thinking: boolean;
}) {
  const sphereRef = useRef<THREE.Mesh>(null!);
  const materialRef = useRef<any>(null!);

  useFrame((state) => {
    const time = state.clock.elapsedTime;

    if (materialRef.current) {
      materialRef.current.uTime = time;
      materialRef.current.uSpeaking = speaking ? 1.0 : 0.0;
      materialRef.current.uIntensity = thinking ? 1.5 : 1.0;
      materialRef.current.uDistortion = speaking ? 0.8 : 0.5;
    }

    if (sphereRef.current) {
      sphereRef.current.rotation.y = time * 0.1;
      const scale = 1 + Math.sin(time * 2) * 0.05;
      sphereRef.current.scale.setScalar(scale);
    }
  });

  return (
    <mesh ref={sphereRef}>
      <sphereGeometry args={[1, 128, 128]} />
      <neuralFluidMaterial
        ref={materialRef}
        transparent={true}
        depthWrite={false}
        blending={THREE.AdditiveBlending}
      />
    </mesh>
  );
}
