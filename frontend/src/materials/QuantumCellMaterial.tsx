import { shaderMaterial } from "@react-three/drei";
import * as THREE from "three";
import vertexShader from "../shaders/quantum/cell/quantumCell.vertex.glsl";
import fragmentShader from "../shaders/quantum/cell/quantumCell.fragment.glsl";

export const QuantumCellMaterial = shaderMaterial(
  {
    uTime: 0,
    uCellColor: new THREE.Color(0.2, 0.6, 1.0),
    uOpacity: 0.75,
    uActive: 0.0,
  },
  vertexShader,
  fragmentShader
);
