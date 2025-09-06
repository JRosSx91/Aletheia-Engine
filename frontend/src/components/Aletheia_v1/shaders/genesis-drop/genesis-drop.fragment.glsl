// frontend/src/components/Aletheia/shaders/genesis-drop/genesis-drop.fragment.glsl - CORREGIDO

uniform float uTime;
uniform float uDisplacementAmount;
uniform float uOpacity; 
uniform float uDissolve; 

varying vec2 vUv;
varying float vDisplacement;

#include "../../../../lygia/generative/snoise.glsl";

float getQuantumPattern(vec2 uv) {
    vec2 grid_uv = uv * 25.0;
    vec2 d = fwidth(grid_uv);
    vec2 grid = fract(grid_uv);
    vec2 a = smoothstep(vec2(0.0), d, grid);
    vec2 b = smoothstep(vec2(0.0), d, 1.0 - grid);
    return 1.0 - (a.x * a.y * b.x * b.y);
}

void main() {
    float quantumPattern = getQuantumPattern(vUv + vDisplacement * 0.1);

    float dissolveNoise = snoise(vUv * 2.0 + uTime);
    
    // Si el ruido es menor que el umbral de disolución, el alfa es 0
    float dissolveFactor = smoothstep(uDissolve - 0.1, uDissolve, dissolveNoise);

    vec3 quantumColor = vec3(0.5, 0.8, 1.0) * quantumPattern;
    
    // El alfa final se ve afectado por la opacidad general Y por la disolución
    float finalAlpha = quantumPattern * uOpacity * (1.0 - dissolveFactor);

    gl_FragColor = vec4(quantumColor, finalAlpha);
}