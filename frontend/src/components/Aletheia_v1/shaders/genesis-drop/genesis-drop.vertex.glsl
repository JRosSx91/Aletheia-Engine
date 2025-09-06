uniform float uTime;
uniform float uDisplacementAmount;

varying vec2 vUv;
varying vec3 vNormal;
varying float vDisplacement; // <-- 1. DECLARAMOS LA VARYING

#include "../../../../lygia/generative/snoise.glsl";

void main() {
    vUv = uv;
    vNormal = normalize(normalMatrix * normal);
    vec3 pos = position;

    float displacement = snoise(pos * 5.0 + uTime * 0.5);
    
    // 2. LE ASIGNAMOS EL VALOR CALCULADO
    vDisplacement = displacement * uDisplacementAmount;

    pos += normal * vDisplacement * 0.2;

    gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
}