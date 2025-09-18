import { useRef, useMemo } from "react";
import { useFrame, extend } from "@react-three/fiber";
import * as THREE from "three";
import { shaderMaterial } from "@react-three/drei";

const PlasmaMaterial = shaderMaterial(
  {
    uTime: 0,
    uIntensity: 1.0,
    uColorA: new THREE.Color("#00ffff"),
    uColorB: new THREE.Color("#0066ff"),
    uDistortion: 0.5,
  },
  `
    uniform float uTime;
    uniform float uDistortion;
    varying vec3 vNormal;
    
    void main() {
      vNormal = normal;
      vec3 pos = position;
      
      float noise1 = sin(position.x * 3.0 + uTime * 0.5) * 0.1;
      float noise2 = cos(position.y * 4.0 + uTime * 0.7) * 0.1;
      float noise3 = sin(position.z * 2.0 + uTime * 0.3) * 0.1;
      
      pos += normal * (noise1 + noise2 + noise3) * uDistortion;
      
      gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
    }
  `,
  `
    uniform float uTime;
    uniform float uIntensity;
    uniform vec3 uColorA;
    uniform vec3 uColorB;
    varying vec3 vNormal;
    
    void main() {
      float plasma = (sin(vNormal.x * 5.0 + uTime * 1.5) + cos(vNormal.y * 4.0 + uTime * 1.0) + sin(vNormal.z * 6.0 + uTime * 0.8)) / 3.0;
      plasma = (plasma + 1.0) * 0.5;
      
      vec3 color = mix(uColorA, uColorB, plasma);
      
      float fresnel = pow(1.0 - dot(vNormal, vec3(0.0, 0.0, 1.0)), 3.0);
      color += fresnel * 0.5;
      
      float pulse = sin(uTime * 3.0) * 0.2 + 0.8;
      
      gl_FragColor = vec4(color, (0.1 + fresnel * 0.3) * uIntensity * pulse);
    }
  `
);

extend({ PlasmaMaterial });

export function PlasmaLayer({
  speaking,
  thinking,
}: {
  speaking: boolean;
  thinking: boolean;
}) {
  const layer1Ref = useRef<THREE.Mesh>(null!);
  const layer2Ref = useRef<THREE.Mesh>(null!);
  const material1Ref = useRef<any>(null!);
  const material2Ref = useRef<any>(null!);

  const [geometry1, geometry2] = useMemo(() => {
    const geo1 = new THREE.IcosahedronGeometry(0.3, 5);
    const geo2 = new THREE.DodecahedronGeometry(0.4, 3);

    const positions1 = geo1.attributes.position;
    for (let i = 0; i < positions1.count; i++) {
      const noise = Math.random() * 0.1 - 0.05;
      const p = new THREE.Vector3().fromBufferAttribute(positions1, i);
      p.multiplyScalar(1 + noise);
      positions1.setXYZ(i, p.x, p.y, p.z);
    }

    const positions2 = geo2.attributes.position;
    for (let i = 0; i < positions2.count; i++) {
      const noise = Math.random() * 0.15 - 0.075;
      const p = new THREE.Vector3().fromBufferAttribute(positions2, i);
      p.multiplyScalar(1 + noise);
      positions2.setXYZ(i, p.x, p.y, p.z);
    }

    geo1.computeVertexNormals();
    geo2.computeVertexNormals();

    return [geo1, geo2]; // Esto es lo que faltaba en el código de Claude
  }, []);

  useFrame((state) => {
    const time = state.clock.elapsedTime;
    if (layer1Ref.current) {
      layer1Ref.current.rotation.y = time * 0.1;
      layer1Ref.current.rotation.x = time * 0.2;
    }
    if (layer2Ref.current) {
      layer2Ref.current.rotation.y = -time * 0.08;
      layer2Ref.current.rotation.z = time * 0.15;
    }
    if (material1Ref.current) {
      material1Ref.current.uTime = time;
      material1Ref.current.uIntensity = thinking ? 0.8 : 0.4;
      material1Ref.current.uDistortion = speaking ? 1.0 : 0.5;
    }
    if (material2Ref.current) {
      material2Ref.current.uTime = time;
      material2Ref.current.uIntensity = speaking ? 1.0 : 0.5;
      material2Ref.current.uDistortion = thinking ? 0.8 : 0.4;
    }
  });

  return (
    <group>
      <mesh ref={layer1Ref} geometry={geometry1}>
        <plasmaMaterial
          ref={material1Ref}
          side={THREE.DoubleSide}
          transparent
          depthWrite={false}
          blending={THREE.AdditiveBlending}
        />
      </mesh>
      <mesh ref={layer2Ref} geometry={geometry2}>
        <plasmaMaterial
          ref={material2Ref}
          side={THREE.DoubleSide}
          transparent
          depthWrite={false}
          blending={THREE.AdditiveBlending}
        />
      </mesh>
    </group>
  );
}
