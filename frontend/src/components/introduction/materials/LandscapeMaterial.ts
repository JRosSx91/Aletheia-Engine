import * as THREE from "three";
import vertexShader from "./shaders/landscape/landscape.vertex.glsl";
import fragmentShader from "./shaders/landscape/landscape.fragment.glsl";

export class LandscapeMaterial extends THREE.ShaderMaterial {
  constructor() {
    super({
      uniforms: {
        u_time: { value: 0.0 },
        u_transition_progress: { value: 0.0 },
        uDiffuse: { value: null },
        uMultispectral: { value: null },
        uOpacity: { value: 1.0 },
      },
      vertexShader,
      fragmentShader,
      transparent: true,
    });
  }
}
