varying vec2 vUv;
varying vec3 vPosition;
varying vec3 vNormal;
varying vec3 vViewDir;
uniform float uTime;
uniform float uIntensity;
uniform float uNoiseScale;
uniform float uSpeed;
uniform vec3 uColorA;
uniform vec3 uColorB;
uniform vec3 uColorC;
uniform float uSpeaking;

float random(vec3 st) {
  return fract(sin(dot(st, vec3(12.9898, 78.233, 37.719))) * 43758.5453123);
}

float noise(vec3 p) {
  vec3 i = floor(p);
  vec3 f = fract(p);
  vec3 u = f * f * (3.0 - 2.0 * f);

  float n000 = random(i);
  float n100 = random(i + vec3(1.0, 0.0, 0.0));
  float n010 = random(i + vec3(0.0, 1.0, 0.0));
  float n110 = random(i + vec3(1.0, 1.0, 0.0));
  float n001 = random(i + vec3(0.0, 0.0, 1.0));
  float n101 = random(i + vec3(1.0, 0.0, 1.0));
  float n011 = random(i + vec3(0.0, 1.0, 1.0));
  float n111 = random(i + vec3(1.0, 1.0, 1.0));

  float nx00 = mix(n000, n100, u.x);
  float nx10 = mix(n010, n110, u.x);
  float nx01 = mix(n001, n101, u.x);
  float nx11 = mix(n011, n111, u.x);
  float nxy0 = mix(nx00, nx10, u.y);
  float nxy1 = mix(nx01, nx11, u.y);
  return mix(nxy0, nxy1, u.z);
}

float fbm(vec3 st) {
  float value = 0.0;
  float amplitude = 0.5;
  float frequency = 1.0;
  for (int i = 0; i < 5; i++) {
    value += amplitude * noise(st * frequency);
    frequency *= 2.0;
    amplitude *= 0.5;
  }
  return value;
}

void main() {
  vec3 st = vPosition * uNoiseScale;
  st += vec3(uTime * uSpeed * 0.1, uTime * uSpeed * 0.05, uTime * uSpeed * 0.08);

  float warped = fbm(st + vec3(uTime * 0.1));
  warped = fbm(st + warped);
  float noiseValue = fbm(st + warped);

  float pattern = smoothstep(0.3, 0.6, noiseValue);
  pattern += smoothstep(0.5, 0.8, fbm(st * 2.0 + uTime * 0.2)) * 0.5;

  float pulse = sin(uTime * 10.0) * 0.5 + 0.5;
  pattern += uSpeaking * pulse * 0.3;

  vec3 color = mix(uColorA, uColorB, pattern);
  color = mix(color, uColorC, pow(pattern, 2.0) * 0.5);

  vec3 n = normalize(vNormal);
  vec3 viewDir = normalize(vViewDir);
  float ndotv = max(dot(n, viewDir), 0.0);
  float fresnel = pow(1.0 - ndotv, 2.0) * 0.65;
  color += uColorC * fresnel * uIntensity;

  float brightness = pattern * uIntensity + fresnel * 0.5;

  gl_FragColor = vec4(color * brightness, pattern * 0.9 + 0.1);
}
