import type { ShaderMaterial } from "three";
import type { Object3DNode, ThreeElement } from "@react-three/fiber";
import type {
  MeshLineGeometry,
  MeshLineMaterial,
  PointsRepresentation,
} from "meshline";
import { HolographicMaterialProps } from "../components/HolographicMaterial";
declare module "@react-three/fiber" {
  interface ThreeElements {
    meshLineGeometry: Partial<
      Omit<ThreeElement<typeof MeshLineGeometry>, "points">
    > & {
      points?: PointsRepresentation;
    };
    meshLineMaterial: Partial<
      Omit<ThreeElement<typeof MeshLineMaterial>, "sizeAttenuation">
    > & {
      sizeAttenuation?: number | boolean;
    };
    holographicMaterial: Object3DNode<
      ShaderMaterial & HolographicMaterialProps,
      typeof ShaderMaterial
    >;
    particleGalaxyMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    fractalEnergyMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    metamorphosisMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    landscapeMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    neuralFluidMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    plasmaMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    volumetricCloudMaterial: Object3DNode<
      ShaderMaterial,
      typeof ShaderMaterial
    >;
    quantumCellMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
    quantumGridMaterial: Object3DNode<ShaderMaterial, typeof ShaderMaterial>;
  }
}
