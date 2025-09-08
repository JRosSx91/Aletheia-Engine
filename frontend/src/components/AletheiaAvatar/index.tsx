import { Canvas } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";
import { useAletheiaState } from "./hooks/useAletheiaState";
import { FluidCore } from "./FluidCore";
import { SynapticField } from "./SynapticField";
import { EnergyRings } from "./EnergyRings";
import { SynapticLightning } from "./SynapticLightning";

// import { UIOverlay } from './UIOverlay'; // La UI 2D se puede añadir aquí después

export const AletheiaAvatar = () => {
  const { speaking, thinking } = useAletheiaState();

  return (
    <div style={{ width: "100vw", height: "100vh", background: "black" }}>
      <Canvas camera={{ position: [0, 0, 6], fov: 50 }}>
        <color attach="background" args={["#000511"]} />
        <ambientLight intensity={0.2} />
        <pointLight position={[0, 0, 0]} intensity={2} color="#00aaff" />
        <FluidCore speaking={speaking} thinking={thinking} />
        <SynapticField count={5000} speaking={speaking} thinking={thinking} />
        <EnergyRings speaking={speaking} />
        <SynapticLightning active={speaking || thinking} />

        <OrbitControls />
      </Canvas>

      {/* <UIOverlay message={message} ... /> */}
    </div>
  );
};

export default AletheiaAvatar;
