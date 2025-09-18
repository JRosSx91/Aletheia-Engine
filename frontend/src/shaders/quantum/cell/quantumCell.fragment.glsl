uniform float uTime;
    uniform vec3 uCellColor;
    uniform float uOpacity;
    uniform float uActive;
    
    varying vec3 vWorldPosition;
    
    void main() {
      vec3 finalColor = uCellColor;
      
      gl_FragColor = vec4(finalColor, uOpacity);
    }