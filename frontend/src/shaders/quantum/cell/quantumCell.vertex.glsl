uniform float uTime;
    uniform float uActive;
    varying vec3 vWorldPosition;
    
    void main() {
      vWorldPosition = (modelMatrix * vec4(position, 1.0)).xyz;
      
      vec3 pos = position;
      
      if (uActive > 0.0) {
        float expansion = sin(uTime * 4.0) * 0.02 * uActive;
        pos += normal * expansion;
      }
      
      gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
    }