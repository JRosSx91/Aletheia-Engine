import { useFrame, useThree } from "@react-three/fiber";

export function useOptimizedRendering() {
  const { gl, scene, camera } = useThree();

  useFrame(() => {
    scene.traverse((object) => {
      if (object.userData.particleSystem) {
        const distance = object.position.distanceTo(camera.position);
        object.visible = distance < 10;
      }
    });

    const pixelRatio = Math.min(window.devicePixelRatio, 2);
    gl.setPixelRatio(pixelRatio);
  });
}
