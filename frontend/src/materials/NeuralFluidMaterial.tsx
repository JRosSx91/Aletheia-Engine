import { shaderMaterial } from "@react-three/drei";
import * as THREE from "three";
import vertexShader from "../shaders/fluid/neuralFluid.vertex.glsl";
import fragmentShader from "../shaders/fluid/neuralFluid.fragment.glsl";

export const NeuralFluidMaterial = shaderMaterial(
  {
    uTime: 0,
    uMouse: new THREE.Vector2(0.5, 0.5),
    uIntensity: 0.01,
    uNoiseScale: 3.0,
    uSpeed: 0.3,
    uColorA: new THREE.Color("#0080ff"),
    uColorB: new THREE.Color("#00ffff"),
    uColorC: new THREE.Color("#ffffff"),
    uDistortion: 0.2,
    uSpeaking: 0.2,
  },
  vertexShader,
  fragmentShader
);
