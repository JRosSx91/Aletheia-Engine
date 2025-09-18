import { Canvas } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";
import { AletheiaAvatar } from "./AletheiaAvatar";

export function AletheiaScene() {
  return (
    <Canvas camera={{ position: [0, 0, 15], fov: 35 }}>
      <color args={["#010101"]} attach="background" />
      <AletheiaAvatar />
