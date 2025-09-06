import { Canvas } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";
// import { GenesisSequence } from "./GenesisSequence";
import { AletheiaAvatar } from "./AletheiaAvatar";

export function AletheiaScene() {
  return (
    <Canvas camera={{ position: [0, 0, 15], fov: 35 }}>
      <color args={["#010101"]} attach="background" />
      <AletheiaAvatar />
      {/* Añadimos OrbitControls para poder mover la cámara */}
      <OrbitControls />
    </Canvas>
  );
}
