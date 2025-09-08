/* eslint-disable @typescript-eslint/no-explicit-any */
import { useRef } from "react";
import { useFrame } from "@react-three/fiber";
import { shaderMaterial } from "@react-three/drei";
import * as THREE from "three";
import { useControls, folder } from "leva";
import vertexShader from "./shaders/quantum/quantum.vertex.glsl";
import fragmentShader from "./shaders/quantum/quantum.fragment.glsl";

export const QuantumFieldMaterial = shaderMaterial(
  {
    uTime: 0,
    uAmplitude: 1.0,
    uFrequency: 0.5,
    uGridScale: 50.0,
    uBrightness: 0.5,
    uGenesisAmplitude: 0.0,
    uGenesisWidth: 2.5,
    uCollapseProgress: 0.0,
    uFieldTurbulence: 1.0,
  },
  vertexShader,
  fragmentShader
);

interface QuantumFieldProps {
  collapseProgress?: number;
  genesisAmplitude?: number;
  fieldTurbulence?: number;
}

export function QuantumField({
  collapseProgress = 0.0,
  genesisAmplitude = 0.0,
  fieldTurbulence = 1.0,
}: QuantumFieldProps) {
  const materialRef = useRef<THREE.ShaderMaterial>(null!);
  const levaControls = useControls("Campo Cuántico", {
    Deformación: folder({
      amplitude: { value: 1.0, min: 0, max: 5, label: "Amplitud Base" },
      frequency: { value: 0.5, min: 0.1, max: 2.0, label: "Frecuencia Base" },
    }),
    Red: folder({
      gridScale: { value: 400.0, min: 5.0, max: 600.0, label: "Escala" },
      brightness: { value: 0.5, min: 0.0, max: 1.0, label: "Brillo" },
    }),
  });

  useFrame((_state, delta) => {
    if (materialRef.current) {
      const uniforms = (materialRef.current as any).uniforms;
      uniforms.uTime.value += delta;

      // Prioridad a las props: si se recibe una prop, se usa; si no, se usa el valor de Leva.
      uniforms.uAmplitude.value = levaControls.amplitude;
      uniforms.uFrequency.value = levaControls.frequency;
      uniforms.uGridScale.value = levaControls.gridScale;
      uniforms.uBrightness.value = levaControls.brightness;

      // Los valores de la secuencia siempre tienen prioridad cuando se proporcionan.
      // El operador '??' (Nullish Coalescing) es perfecto para esto.
      uniforms.uCollapseProgress.value = collapseProgress ?? 0.0;
      uniforms.uGenesisAmplitude.value = genesisAmplitude ?? 0.0;
      uniforms.uFieldTurbulence.value = fieldTurbulence;
    }
  });

  return (
    <mesh rotation={[-Math.PI / 2, 0, 0]}>
      <planeGeometry args={[50, 50, 128, 128]} />
      <quantumFieldMaterial
        ref={materialRef}
        transparent={true}
        blending={THREE.AdditiveBlending}
        depthWrite={false}
      />
    </mesh>
  );
}
QuantumField.displayName = "QuantumField";
