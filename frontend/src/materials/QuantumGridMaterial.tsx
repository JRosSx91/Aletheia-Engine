import { shaderMaterial } from "@react-three/drei";
import * as THREE from "three";
import vertexShader from "../shaders/quantum/grid/quantumGrid.vertex.glsl";
import fragmentShader from "../shaders/quantum/grid/quantumGrid.fragment.glsl";

// Material para las líneas del grid principal
export const QuantumGridMaterial = shaderMaterial(
  {
    uTime: 0,
    uColor: new THREE.Color(0.0, 1.0, 1.0),
    uOpacity: 0.8,
  },
  vertexShader,
  fragmentShader
);
