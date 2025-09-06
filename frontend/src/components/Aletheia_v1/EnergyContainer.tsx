import { useRef } from "react";
import * as THREE from "three";
import { useFrame } from "@react-three/fiber";
import { shaderMaterial } from "@react-three/drei";
import { useControls, folder } from "leva";
import vertexShader from "./shaders/sphere/fractal.vertex.glsl?raw";
import fragmentShader from "./shaders/sphere/fractal.fragment.glsl?raw";

export const FractalEnergyMaterial = shaderMaterial(
  {
    uTime: 0,
    uColor: new THREE.Color("#87CEFA"),
    uStrength: 2.5,
    uCoreColor: new THREE.Color("#FFFFFF"),
    uCorePower: 5.0,
    uWarpFrequency: 2.0,
    uWarpAmplitude: 0.3,
    uFbmFrequency: 3.0,
    uFbmAmplitude: 1.0,
    uFbmOctaves: 5,
  },
  vertexShader,
  fragmentShader
);

export function EnergyContainer() {
  const materialRef = useRef<THREE.ShaderMaterial>(null!);

  const controls = useControls("Núcleo de Energía", {
    Fractal: folder({
      color: { value: "#87CEFA", label: "Color" },
      strength: { value: 2.5, min: 0.0, max: 10.0, label: "Intensidad" },
      fbmFrequency: {
        value: 3.0,
        min: 0.1,
        max: 10.0,
        label: "Frecuencia Fractal",
      },
      fbmAmplitude: {
        value: 1.0,
        min: 0.1,
        max: 5.0,
        label: "Amplitud Fractal",
      },
      fbmOctaves: { value: 5, min: 1, max: 8, step: 1, label: "Octavas" },
    }),
    Turbulencia: folder({
      warpFrequency: {
        value: 2.0,
        min: 0.1,
        max: 10.0,
        label: "Frecuencia Warp",
      },
      warpAmplitude: { value: 0.3, min: 0.0, max: 1.0, label: "Amplitud Warp" },
    }),
    Nucleo: folder({
      coreColor: { value: "#FFFFFF", label: "Color Núcleo" },
      corePower: { value: 5.0, min: 1.0, max: 20.0, label: "Potencia Núcleo" },
    }),
  });

  useFrame((state) => {
    if (materialRef.current) {
      materialRef.current.uniforms.uTime.value = state.clock.getElapsedTime();
      materialRef.current.uniforms.uColor.value.set(controls.color);
      materialRef.current.uniforms.uStrength.value = controls.strength;
      materialRef.current.uniforms.uCoreColor.value.set(controls.coreColor);
      materialRef.current.uniforms.uCorePower.value = controls.corePower;
      materialRef.current.uniforms.uWarpFrequency.value =
        controls.warpFrequency;
      materialRef.current.uniforms.uWarpAmplitude.value =
        controls.warpAmplitude;
      materialRef.current.uniforms.uFbmFrequency.value = controls.fbmFrequency;
      materialRef.current.uniforms.uFbmAmplitude.value = controls.fbmAmplitude;
      materialRef.current.uniforms.uFbmOctaves.value = controls.fbmOctaves;
    }
  });

  return (
    <mesh>
      <sphereGeometry args={[1.5, 64, 64]} />
      <fractalEnergyMaterial
        ref={materialRef}
        key={FractalEnergyMaterial.key}
        transparent={true}
        blending={THREE.AdditiveBlending}
        depthWrite={false}
      />
    </mesh>
  );
}
