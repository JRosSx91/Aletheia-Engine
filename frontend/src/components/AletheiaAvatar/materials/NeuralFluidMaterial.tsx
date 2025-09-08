import { shaderMaterial } from "@react-three/drei";
import * as THREE from "three";
import vertexShader from "./shaders/neuralFluid.vertex.glsl";
import fragmentShader from "./shaders/neuralFluid.fragment.glsl";

export const NeuralFluidMaterial = shaderMaterial(
  {
    uTime: 0,
    uMouse: new THREE.Vector2(0.5, 0.5),
    uIntensity: 1.0,
    uNoiseScale: 3.0,
    uSpeed: 0.5,
    uColorA: new THREE.Color("#0080ff"),
    uColorB: new THREE.Color("#00ffff"),
    uColorC: new THREE.Color("#ffffff"),
    uDistortion: 0.5,
    uSpeaking: 0.0,
  },
  vertexShader,
  fragmentShader
);
