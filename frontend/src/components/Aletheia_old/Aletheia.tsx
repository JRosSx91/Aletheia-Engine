import { ConnectionBeams } from "./ConnectionBeams";
import { EnergyContainer } from "./EnergyContainer";

function Singularity() {
  return (
    <mesh>
      <sphereGeometry args={[0.1, 32, 32]} />
      <meshBasicMaterial color="#FFFFFF" toneMapped={false} />
    </mesh>
  );
}

export function Aletheia() {
  return (
    <group>
      <EnergyContainer />
      <Singularity />
      <ConnectionBeams radius={1.5} />
    </group>
  );
}
