varying vec2 vUv;
varying vec3 vPosition;
varying vec3 vNormal;
uniform float uTime;
uniform float uIntensity;
uniform float uNoiseScale;
uniform float uSpeed;
uniform vec3 uColorA;
uniform vec3 uColorB;
uniform vec3 uColorC;
uniform float uSpeaking;

float random(vec2 st) {
  return fract(sin(dot(st.xy, vec2(12.9898,78.233))) * 43758.5453123);
}

float noise(vec2 st) {
  vec2 i = floor(st);
  vec2 f = fract(st);
  float a = random(i);
  float b = random(i + vec2(1.0, 0.0));
  float c = random(i + vec2(0.0, 1.0));
  float d = random(i + vec2(1.0, 1.0));
  vec2 u = f * f * (3.0 - 2.0 * f);
  return mix(a, b, u.x) + (c - a)* u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

float fbm(vec2 st) {
  float value = 0.0;
  float amplitude = 0.5;
  float frequency = 1.0;
  for (int i = 0; i < 5; i++) {
    value += amplitude * noise(st * frequency);
    st *= 2.0;
    frequency *= 2.0;
    amplitude *= 0.5;
  }
  return value;
}

void main() {
  vec2 st = vUv * uNoiseScale;
  st.x += uTime * uSpeed * 0.1;
  st.y += uTime * uSpeed * 0.05;
  
  float noiseValue = fbm(st + fbm(st + fbm(st + uTime * 0.1)));
  
  float pattern = smoothstep(0.3, 0.6, noiseValue);
  pattern += smoothstep(0.5, 0.8, fbm(st * 2.0 + uTime * 0.2)) * 0.5;
  
  float pulse = sin(uTime * 10.0) * 0.5 + 0.5;
  pattern += uSpeaking * pulse * 0.3;
  
  vec3 color = mix(uColorA, uColorB, pattern);
  color = mix(color, uColorC, pow(pattern, 2.0) * 0.5);
  
  float fresnel = pow(1.0 - dot(vNormal, vec3(0.0, 0.0, 1.0)), 2.0);
  color += uColorC * fresnel * uIntensity;
  
  float brightness = pattern * uIntensity + fresnel * 0.5;
  
  gl_FragColor = vec4(color * brightness, pattern * 0.9 + 0.1);
}