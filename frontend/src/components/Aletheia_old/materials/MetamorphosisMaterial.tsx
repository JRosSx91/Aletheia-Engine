import { shaderMaterial } from "@react-three/drei";
import vertexShader from "../shaders/genesis-drop/genesis-drop.vertex.glsl";
import fragmentShader from "../shaders/genesis-drop/genesis-drop.fragment.glsl";

export const MetamorphosisMaterial = shaderMaterial(
  {
    uTime: 0,
    uDisplacementAmount: 1.0,
    uOpacity: 1.0,
    uTransitionProgress: 0.0,
    uColorTransition: 0.0,
    uAlphaMap: null,
    uEmissiveIntensity: 0.0,
  },
  vertexShader,
  fragmentShader
);
