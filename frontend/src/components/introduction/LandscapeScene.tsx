import React from "react";
import * as THREE from "three";
import { useFrame } from "@react-three/fiber";
import type { RootState } from "@react-three/fiber";
import { useTexture } from "@react-three/drei";
import { LandscapeMaterial } from "./materials/LandscapeMaterial";

interface LandscapeSceneProps {
  transitionProgress?: number;
}

export const LandscapeScene = React.forwardRef<THREE.Mesh, LandscapeSceneProps>(
  ({ transitionProgress = 0 }, ref) => {
  const diffuseTexture = useTexture("/textures/landscape-diffuse.jpg");
  // Load the same texture for testing purposes
  const multispectralTexture = useTexture("/textures/landscape-diffuse.jpg");

  const material = React.useMemo(() => new LandscapeMaterial(), []);

  React.useEffect(() => {
    if (material) {
      material.uniforms.uDiffuse.value = diffuseTexture;
      material.uniforms.uMultispectral.value = multispectralTexture;
    }
  }, [material, diffuseTexture, multispectralTexture]);

  // Update transition progress uniform
  React.useEffect(() => {
    if (material) {
      material.uniforms.u_transition_progress.value = transitionProgress;
    }
  }, [material, transitionProgress]);

  // Animation loop for time uniform
  useFrame((state: RootState) => {
    if (material) {
      material.uniforms.u_time.value = state.clock.elapsedTime;
    }
  });

  return (
    <mesh ref={ref} rotation={[-Math.PI / 2, 0, 0]}>
      <planeGeometry args={[16, 9]} />
      <primitive object={material} attach="material" />
    </mesh>
  );
});

LandscapeScene.displayName = "LandscapeScene";
