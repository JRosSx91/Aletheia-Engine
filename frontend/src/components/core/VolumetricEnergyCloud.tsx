import { useRef, useMemo } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";
import { shaderMaterial } from "@react-three/drei";

export const VolumetricCloudMaterial = shaderMaterial(
  {
    uTime: 0,
    uIntensity: 1.0,
    uDensity: 0.3,
    uSpeed: 0.5,
    uScale: 1.0,
    uColorA: new THREE.Color("#ffffff"),
    uColorB: new THREE.Color("#00ffff"),
    uColorC: new THREE.Color("#0044aa"),
  },
  `
    varying vec2 vUv;
    varying vec3 vPosition;
    varying vec3 vWorldPosition;
    varying vec3 vNormal;
    
    void main() {
      vUv = uv;
      vNormal = normalize(normalMatrix * normal);
      vPosition = position;
      
      vec4 worldPosition = modelMatrix * vec4(position, 1.0);
      vWorldPosition = worldPosition.xyz;
      
      gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
    }
  `,
  `
    varying vec2 vUv;
    varying vec3 vPosition;
    varying vec3 vWorldPosition;
    varying vec3 vNormal;
    
    uniform float uTime;
    uniform float uIntensity;
    uniform float uDensity;
    uniform float uSpeed;
    uniform float uScale;
    uniform vec3 uColorA;
    uniform vec3 uColorB;
    uniform vec3 uColorC;
    
    vec3 random3(vec3 c) {
      float j = 4096.0 * sin(dot(c, vec3(17.0, 59.4, 15.0)));
      vec3 r;
      r.z = fract(512.0 * j);
      j *= .125;
      r.x = fract(512.0 * j);
      j *= .125;
      r.y = fract(512.0 * j);
      return r - 0.5;
    }
    
    float simplex3d(vec3 p) {
      const float F3 = 0.3333333;
      const float G3 = 0.1666667;
      
      vec3 s = floor(p + dot(p, vec3(F3)));
      vec3 x = p - s + dot(s, vec3(G3));
      
      vec3 e = step(vec3(0.0), x - x.yzx);
      vec3 i1 = e * (1.0 - e.zxy);
      vec3 i2 = 1.0 - e.zxy * (1.0 - e);
      
      vec3 x1 = x - i1 + G3;
      vec3 x2 = x - i2 + 2.0 * G3;
      vec3 x3 = x - 1.0 + 3.0 * G3;
      
      vec4 w, d;
      w.x = dot(x, x);
      w.y = dot(x1, x1);
      w.z = dot(x2, x2);
      w.w = dot(x3, x3);
      
      w = max(0.6 - w, 0.0);
      
      d.x = dot(random3(s), x);
      d.y = dot(random3(s + i1), x1);
      d.z = dot(random3(s + i2), x2);
      d.w = dot(random3(s + 1.0), x3);
      
      w *= w;
      w *= w;
      d *= w;
      
      return dot(d, vec4(52.0));
    }
    
    float fbm(vec3 p) {
      float f = 0.0;
      f += 0.5000 * simplex3d(p); p *= 2.01;
      f += 0.2500 * simplex3d(p); p *= 2.02;
      f += 0.1250 * simplex3d(p); p *= 2.03;
      f += 0.0625 * simplex3d(p);
      return f / 0.9375;
    }
    
    void main() {
      vec3 p = vPosition * uScale;
      float time = uTime * uSpeed;
      
      vec3 q = p + vec3(time * 0.1, time * 0.15, time * 0.08);
      float noise1 = fbm(q);
      
      vec3 r = p + vec3(noise1 * 2.0) + vec3(time * 0.2, -time * 0.1, time * 0.12);
      float noise2 = fbm(r);
      
      vec3 s = p + vec3(noise2 * 1.5) + vec3(-time * 0.05, time * 0.25, -time * 0.15);
      float noise3 = fbm(s * 0.5);
      
      float density = noise1 + noise2 * 0.5 + noise3 * 0.25;
      density = smoothstep(-0.8, 0.8, density);
      
      float distanceFromCenter = length(vPosition);
      float radialFade = 1.0 - smoothstep(1.0, 3.0, distanceFromCenter);
      
      float pulse = sin(time * 2.0 + distanceFromCenter) * 0.3 + 0.7;
      
      density *= radialFade * pulse * uDensity;
      
      vec3 color = mix(uColorC, uColorB, density);
      color = mix(color, uColorA, density * density);
      
      float colorVariation = sin(time * 1.5 + distanceFromCenter * 2.0) * 0.3 + 0.7;
      color *= colorVariation;
      
      vec3 viewDir = normalize(vWorldPosition - cameraPosition);
      float fresnel = pow(1.0 - abs(dot(vNormal, viewDir)), 1.5);
      color += fresnel * uColorA * 0.4;
      
      color *= uIntensity;
      density *= uIntensity;
      
      float alpha = density * 0.6;
      alpha = clamp(alpha, 0.0, 0.8);
      
      gl_FragColor = vec4(color, alpha);
    }
  `
);

export function VolumetricEnergyCloud({
  active,
  intensity = 1.0,
  speaking = false,
  thinking = false,
}: {
  active: boolean;
  intensity?: number;
  speaking?: boolean;
  thinking?: boolean;
}) {
  const groupRef = useRef<THREE.Group>(null!);
  const cloudRefs = [
    useRef<THREE.Mesh>(null!),
    useRef<THREE.Mesh>(null!),
    useRef<THREE.Mesh>(null!),
  ];
  const materialRefs = [
    useRef<any>(null!),
    useRef<any>(null!),
    useRef<any>(null!),
  ];

  const geometries = useMemo(() => {
    const cloud1 = new THREE.SphereGeometry(2.5, 16, 12);
    const cloud2 = new THREE.IcosahedronGeometry(3.2, 1);
    const cloud3 = new THREE.DodecahedronGeometry(4.0, 0);

    return [cloud1, cloud2, cloud3];
  }, []);

  useFrame((state) => {
    const time = state.clock.elapsedTime;

    if (active) {
      const baseSpeed = speaking ? 1.2 : thinking ? 0.8 : 0.5;
      const baseDensity = speaking ? 0.4 : thinking ? 0.6 : 0.3;
      const baseScale = speaking ? 0.8 : thinking ? 1.2 : 1.0;

      materialRefs.forEach((materialRef, index) => {
        if (materialRef.current) {
          materialRef.current.uTime = time;
          materialRef.current.uIntensity = intensity * (1.0 - index * 0.15);
          materialRef.current.uSpeed = baseSpeed * (1.0 + index * 0.3);
          materialRef.current.uDensity = baseDensity * (1.0 - index * 0.1);
          materialRef.current.uScale = baseScale * (0.8 + index * 0.4);
        }
      });

      cloudRefs.forEach((cloudRef, index) => {
        if (cloudRef.current) {
          const rotationSpeed = 0.05 + index * 0.02;
          cloudRef.current.rotation.x =
            Math.sin(time * rotationSpeed * 0.7) * 0.3;
          cloudRef.current.rotation.y = time * rotationSpeed;
          cloudRef.current.rotation.z =
            Math.cos(time * rotationSpeed * 0.5) * 0.2;

          const scale = 1.0 + Math.sin(time * (1 + index * 0.3)) * 0.1;
          cloudRef.current.scale.setScalar(scale);
        }
      });
    }

    if (groupRef.current) {
      groupRef.current.rotation.y = time * 0.02;
    }
  });

  if (!active) return null;

  return (
    <group ref={groupRef}>
      <mesh ref={cloudRefs[1]} geometry={geometries[1]}>
        <volumetricCloudMaterial
          ref={materialRefs[1]}
          transparent
          side={THREE.DoubleSide}
          blending={THREE.AdditiveBlending}
          depthWrite={false}
          uColorA={new THREE.Color("#00ddff")}
          uColorB={new THREE.Color("#0099cc")}
          uColorC={new THREE.Color("#004488")}
        />
      </mesh>
    </group>
  );
}
