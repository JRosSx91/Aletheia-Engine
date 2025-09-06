uniform float uTime;
uniform sampler2D uAlphaMap;
uniform float uTransitionProgress;
uniform float uAlphaTest; 

varying vec2 vUv;

float getQuantumPattern(vec2 uv) {
    vec2 grid_uv = uv * 50.0;
    vec2 d = fwidth(grid_uv);
    vec2 grid = fract(grid_uv);
    vec2 a = smoothstep(vec2(0.0), d * 1.5, grid);
    vec2 b = smoothstep(vec2(0.0), d * 1.5, 1.0 - grid);
    return 1.0 - (a.x * a.y * b.x * b.y);
}

void main() {
    float quantumPattern = getQuantumPattern(vUv);
    vec4 quantumState = vec4(vec3(0.5, 0.8, 1.0) * quantumPattern, pow(quantumPattern, 2.0));

    float fireAlpha = texture2D(uAlphaMap, vUv).r;
    if (fireAlpha < uAlphaTest) {
        fireAlpha = 0.0; 
    }
    
    vec3 red = vec3(1.0, 0.1, 0.0);
    vec3 yellow = vec3(1.0, 0.9, 0.2);
    vec3 blue = vec3(0.8, 0.9, 1.0);
    vec3 fireColor = mix(red, yellow, smoothstep(0.0, 0.5, uTransitionProgress));
    fireColor = mix(fireColor, blue, smoothstep(0.5, 1.0, uTransitionProgress));

    vec4 fireState = vec4(fireColor, fireAlpha);

    gl_FragColor = mix(quantumState, fireState, uTransitionProgress);
}