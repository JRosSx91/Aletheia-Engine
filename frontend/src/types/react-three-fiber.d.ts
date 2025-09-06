import type { ShaderMaterial } from "three";
import type { Object3DNode } from "@react-three/fiber";
import { HolographicMaterialProps } from "../components/HolographicMaterial";

declare module "@react-three/fiber" {
  interface ThreeElements {
    holographicMaterial: Object3DNode<
      ShaderMaterial & HolographicMaterialProps,
      typeof ShaderMaterial
    >;
    particleGalaxyMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    fractalEnergyMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    quantumFieldMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    metamorphosisMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    landscapeMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
  }
}
