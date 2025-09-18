varying vec2 vUv;
varying vec3 vPositionW;
varying vec4 vPos;
varying vec3 vNormalW;

void main() {
    vUv = uv;
    vPos = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
    vPositionW = vec3(modelMatrix * vec4(position, 1.0));
    vNormalW = normalize(vec3(modelMatrix * vec4(normal, 0.0)));
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
}