import { useRef } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";

export function EnergyRings({ speaking }: { speaking: boolean }) {
  const ring1Ref = useRef<THREE.Mesh>(null!);
  const ring2Ref = useRef<THREE.Mesh>(null!);
  const ring3Ref = useRef<THREE.Mesh>(null!);

  useFrame((state) => {
    const time = state.clock.elapsedTime;
    if (ring1Ref.current) {
      ring1Ref.current.rotation.x = time * 0.5;
      ring1Ref.current.rotation.z = time * 0.3;
    }
    if (ring2Ref.current) {
      ring2Ref.current.rotation.y = time * 0.4;
      ring2Ref.current.rotation.x = time * 0.2;
    }
    if (ring3Ref.current) {
      ring3Ref.current.rotation.z = time * 0.3;
      ring3Ref.current.rotation.y = time * 0.6;
    }
  });

  return (
    <group>
      <mesh ref={ring1Ref}>
        <torusGeometry args={[1.8, 0.02, 16, 100]} />
        <meshBasicMaterial
          color="#00aaff"
          transparent
          opacity={speaking ? 0.8 : 0.3}
          blending={THREE.AdditiveBlending}
        />
      </mesh>
      <mesh ref={ring2Ref}>
        <torusGeometry args={[2.2, 0.02, 16, 100]} />
        <meshBasicMaterial
          color="#0088ff"
          transparent
          opacity={speaking ? 0.6 : 0.2}
          blending={THREE.AdditiveBlending}
        />
      </mesh>
      <mesh ref={ring3Ref}>
        <torusGeometry args={[2.5, 0.02, 16, 100]} />
        <meshBasicMaterial
          color="#0066ff"
          transparent
          opacity={speaking ? 0.4 : 0.1}
          blending={THREE.AdditiveBlending}
        />
      </mesh>
    </group>
  );
}
