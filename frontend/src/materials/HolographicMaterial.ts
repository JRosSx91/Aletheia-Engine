import { shaderMaterial } from "@react-three/drei";
import { Color } from "three";
import vertexShader from "./shaders/holographic.vertex.glsl?raw";
import fragmentShader from "./shaders/holographic.fragment.glsl?raw";

export const HolographicMaterial = shaderMaterial(
  {
    time: 0,
    fresnelOpacity: 1.0,
    fresnelAmount: 0.45,
    scanlineSize: 8.0,
    hologramBrightness: 1.2,
    signalSpeed: 0.45,
    hologramColor: new Color("#51a4de"),
    enableBlinking: true,
    blinkFresnelOnly: true,
    hologramOpacity: 1.0,
  },
  vertexShader,
  fragmentShader
);
