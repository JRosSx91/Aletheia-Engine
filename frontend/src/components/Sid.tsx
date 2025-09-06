import { useRef } from "react";
import type { JSX } from "react";
import { useFrame } from "@react-three/fiber";
import { useGLTF, Float } from "@react-three/drei";
import { useControls } from "leva";
import * as THREE from "three";
import type { ShaderMaterial } from "three";

export function Sid(props: JSX.IntrinsicElements["group"]) {
  const { nodes } = useGLTF("/models/pluto/scene.gltf");
  const plutoNode = nodes["Object_2"] as THREE.Mesh;
  const materialRef = useRef<ShaderMaterial>(null!);

  const holographicControls = useControls("Holograma Pluton", {
    fresnelAmount: { value: 0.45, min: 0.0, max: 1.0 },
    fresnelOpacity: { value: 1.0, min: 0.0, max: 1.0 },
    scanlineSize: { value: 8.0, min: 1.0, max: 25.0 },
    hologramBrightness: { value: 1.2, min: 0.0, max: 2.0 },
    signalSpeed: { value: 0.45, min: 0.0, max: 2.0 },
    hologramColor: { value: "#00ccff" },
    hologramOpacity: { value: 1.0, min: 0.0, max: 1.0 },
    enableBlinking: true,
    blinkFresnelOnly: true,
    enableAdditive: true,
  });

  useFrame((state) => {
    if (materialRef.current) {
      materialRef.current.uniforms.time.value = state.clock.getElapsedTime();
    }
  });

  if (!plutoNode) return null;

  return (
    <group {...props} dispose={null}>
      <Float rotationIntensity={0.4} floatIntensity={2} speed={1.5}>
        <mesh geometry={plutoNode.geometry} scale={0.02}>
          <holographicMaterial
            ref={materialRef}
            key={Math.random()}
            transparent={true}
            side={THREE.DoubleSide}
            blending={
              holographicControls.enableAdditive
                ? THREE.AdditiveBlending
                : THREE.NormalBlending
            }
            {...holographicControls}
          />
        </mesh>
      </Float>
    </group>
  );
}

useGLTF.preload("/models/pluto/scene.gltf");
