uniform float uTime;
    uniform vec3 uColor;
    uniform float uOpacity;
    
    void main() {
      float pulse = sin(uTime * 2.0) * 0.3 + 0.7;
      gl_FragColor = vec4(uColor * pulse, uOpacity);
    }