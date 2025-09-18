import React, { useState } from "react";
import { Canvas } from "@react-three/fiber";
import { LandscapeScene } from "./LandscapeScene";

export const LandscapeTestScene: React.FC = () => {
  const [transitionProgress, setTransitionProgress] = useState(0);

  return (
    <div style={{ width: "100vw", height: "100vh", position: "relative" }}>
      <Canvas camera={{ position: [0, 5, 10], fov: 60 }}>
        <ambientLight intensity={0.5} />
        <pointLight position={[10, 10, 10]} />
        <LandscapeScene transitionProgress={transitionProgress} />
      </Canvas>
      
