import { useRef, useMemo } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";

const createGridLines = (size = 50, divisions = 10) => {
  const geometry = new THREE.BufferGeometry();
  const vertices = [];
  const step = size / divisions;
  const halfSize = size / 2;

  for (let i = 0; i <= divisions; i++) {
    const pos = -halfSize + i * step;

    // Líneas del plano X-Y
    vertices.push(-halfSize, pos, -halfSize);
    vertices.push(halfSize, pos, -halfSize);
    vertices.push(-halfSize, pos, halfSize);
    vertices.push(halfSize, pos, halfSize);

    // Líneas del plano Y-Z
    vertices.push(pos, -halfSize, -halfSize);
    vertices.push(pos, halfSize, -halfSize);
    vertices.push(pos, -halfSize, halfSize);
    vertices.push(pos, halfSize, halfSize);

    // Líneas del plano X-Z
    vertices.push(-halfSize, -halfSize, pos);
    vertices.push(halfSize, -halfSize, pos);
    vertices.push(-halfSize, halfSize, pos);
    vertices.push(halfSize, halfSize, pos);
  }

  geometry.setAttribute(
    "position",
    new THREE.Float32BufferAttribute(vertices, 3)
  );
  return geometry;
};

export const QuantumGrid = ({
  gridSize,
  divisions,
}: {
  gridSize: number;
  divisions: number;
}) => {
  const gridLinesRef = useRef<any>(null);

  const gridGeometry = useMemo(
    () => createGridLines(gridSize, divisions),
    [gridSize, divisions]
  );

  useFrame((state) => {
    if (gridLinesRef.current) {
      gridLinesRef.current.material.uniforms.uTime.value =
        state.clock.elapsedTime;
    }
  });

  return (
    <lineSegments ref={gridLinesRef} geometry={gridGeometry}>
      <quantumGridMaterial transparent uOpacity={0.3} />
    </lineSegments>
  );
};
