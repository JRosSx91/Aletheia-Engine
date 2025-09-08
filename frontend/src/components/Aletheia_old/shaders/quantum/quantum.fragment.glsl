// src/components/Aletheia/shaders/quantum/quantum.fragment.glsl - CORREGIDO

uniform float uTime;
uniform float uBrightness; 
uniform float uGridScale;
uniform float uCollapseProgress; // Rango 0.0 a 1.0

varying vec2 vUv;

float getQuantumPattern(vec2 uv) {
    vec2 grid_uv = uv * uGridScale;
    vec2 d = fwidth(grid_uv);
    vec2 grid = fract(grid_uv);
    vec2 a = smoothstep(vec2(0.0), d * 1.5, grid);
    vec2 b = smoothstep(vec2(0.0), d * 1.5, 1.0 - grid);
    return 1.0 - (a.x * a.y * b.x * b.y);
}

void main() {
    vec2 baseUv = vUv;
    baseUv.x += uTime * 0.05;
    float lines = getQuantumPattern(baseUv);
    vec3 color = vec3(0.5, 0.8, 1.0) * lines;
    float alpha = pow(lines, 2.0) * uBrightness;

    if (uCollapseProgress > 0.0) {
        vec3 colorOnda1 = vec3(1.0, 0.1, 0.1);
        vec3 colorOnda2 = vec3(0.1, 1.0, 0.1);
        vec3 colorOnda3 = vec3(0.1, 0.1, 1.0);

        float dist = distance(vUv, vec2(0.5));
        
        // CORRECCIÓN: 'loat' a 'float'
        float phaseDuration = 1.0 / 3.0;
        
        float wave1_radius = uCollapseProgress * 0.7;
        if (dist < wave1_radius) {
            color = colorOnda1 * lines;
        }

        float wave2_progress = smoothstep(phaseDuration, phaseDuration * 2.0, uCollapseProgress);
        float wave2_radius = wave2_progress * 0.7;
        if (dist < wave2_radius) {
            color = colorOnda2 * lines;
        }

        float wave3_progress = smoothstep(phaseDuration * 2.0, 1.0, uCollapseProgress);
        float wave3_radius = wave3_progress * 0.7;
        if (dist < wave3_radius) {
            color = colorOnda3 * lines;
        }
    }
    
    gl_FragColor = vec4(color, alpha);
}