uniform float uTime;
uniform float uAmplitude;
uniform float uFrequency;
uniform float uGenesisAmplitude;
uniform float uGenesisWidth; 
uniform float uFieldTurbulence;

varying vec2 vUv;

#include "../../../../lygia/generative/fbm.glsl";

void main() {
    vUv = uv;
    vec3 displacedPosition = position;
    
    float turbulence = fbm(vec3(position.xy * uFrequency, uTime * 0.3));
    displacedPosition.z += turbulence * uAmplitude * uFieldTurbulence;

    float dist = distance(position.xy, vec2(0.0));
    float waveProfile = exp(-pow(dist, 2.0) / uGenesisWidth);
    float genesisWave = waveProfile * uGenesisAmplitude;
    displacedPosition.z += genesisWave;

    gl_Position = projectionMatrix * modelViewMatrix * vec4(displacedPosition, 1.0);
}