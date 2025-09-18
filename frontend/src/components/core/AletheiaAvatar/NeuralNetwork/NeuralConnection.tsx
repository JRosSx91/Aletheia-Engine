import { useMemo, useRef } from "react";
import * as THREE from "three";
import { useFrame, extend, useThree } from "@react-three/fiber";
import { NeuralNode } from "../../../../hooks/useNeuralNetwork";
import { MeshLineGeometry, MeshLineMaterial } from "meshline";

extend({ MeshLineGeometry, MeshLineMaterial });

export function NeuralConnection({
  startNode,
  endNode,
  pulseTime,
  energy,
}: {
  startNode: NeuralNode;
  endNode: NeuralNode;
  pulseTime: number;
  energy: number;
}) {
  const materialRef = useRef<THREE.ShaderMaterial & { dashOffset: number }>(
    null!
  );
  const { size } = useThree();

  const points = useMemo(() => {
    const curve = new THREE.CatmullRomCurve3([
      startNode.position,
      endNode.position,
    ]);
    return curve.getPoints(20);
  }, [startNode, endNode]);

  useFrame((state) => {
    if (materialRef.current) {
      materialRef.current.dashOffset =
        (-((state.clock.elapsedTime - pulseTime) * 0.2) % 2) + 1;
    }
  });

  return (
    <mesh>
      <meshLineGeometry points={points} />
      <meshLineMaterial
        transparent
        blending={THREE.AdditiveBlending}
        depthWrite={false}
        sizeAttenuation={true}
        lineWidth={0.0025 + 0.12 * Math.pow(energy, 2.0)}
        color={"#00aaff"}
        resolution={new THREE.Vector2(size.width, size.height)}
      />
    </mesh>
  );
}
