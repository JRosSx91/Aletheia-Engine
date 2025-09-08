import { useState, useEffect } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";

interface Bolt {
  id: number;
  start: THREE.Vector3;
  end: THREE.Vector3;
  opacity: number;
}

export function SynapticLightning({ active }: { active: boolean }) {
  const [bolts, setBolts] = useState<Bolt[]>([]);

  useEffect(() => {
    if (active) {
      const interval = setInterval(() => {
        setBolts((prev) => {
          const newBolts = prev.filter((b) => b.opacity > 0.01).slice(0, 4);
          const startAngle = Math.random() * Math.PI * 2;
          const endAngle = Math.random() * Math.PI * 2;
          const startRadius = 1.5 + Math.random() * 0.5;
          const endRadius = 2.5 + Math.random() * 0.5;

          newBolts.push({
            id: Date.now(),
            start: new THREE.Vector3(
              Math.cos(startAngle) * startRadius,
              (Math.random() - 0.5) * 2,
              Math.sin(startAngle) * startRadius
            ),
            end: new THREE.Vector3(
              Math.cos(endAngle) * endRadius,
              (Math.random() - 0.5) * 2,
              Math.sin(endAngle) * endRadius
            ),
            opacity: 1,
          });
          return newBolts;
        });
      }, 200);
      return () => clearInterval(interval);
    }
  }, [active]);

  useFrame(() => {
    setBolts((prev) =>
      prev.map((bolt) => ({ ...bolt, opacity: bolt.opacity * 0.95 }))
    );
  });

  return (
    <group>
      {bolts.map((bolt) => (
        <line key={bolt.id}>
          <bufferGeometry>
            <bufferAttribute
              attach="attributes-position"
              args={[
                new Float32Array([
                  ...bolt.start.toArray(),
                  ...bolt.end.toArray(),
                ]),
                3,
              ]}
            />
          </bufferGeometry>
          <lineBasicMaterial
            color="#00ffff"
            transparent
            opacity={bolt.opacity}
            blending={THREE.AdditiveBlending}
            linewidth={2}
          />
        </line>
      ))}
    </group>
  );
}
