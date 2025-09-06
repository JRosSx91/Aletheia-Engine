varying vec2 vUv;
varying vec3 vPositionW;
varying vec4 vPos;
varying vec3 vNormalW;
    
uniform float time;
uniform float fresnelOpacity;
uniform float scanlineSize;
uniform float fresnelAmount;
uniform float signalSpeed;
uniform float hologramBrightness;
uniform float hologramOpacity;
uniform bool blinkFresnelOnly;
uniform bool enableBlinking;
uniform vec3 hologramColor;

float flicker(float amt, float time) { return clamp(fract(cos(time) * 43758.5453123), amt, 1.0); }
    
    void main() {
      vec2 vCoords = vPos.xy / vPos.w * 0.5 + 0.5;
      vec2 myUV = fract(vCoords);
      
      vec4 holoColor = vec4(hologramColor, hologramOpacity);
      
      float scanlines = 10. * sin(time * signalSpeed * 5. - myUV.y * 20. * scanlineSize);
      scanlines *= smoothstep(1.3 * cos(time * signalSpeed + myUV.y * scanlineSize), 0.78, 0.9);
      scanlines *= max(0.25, sin(time * signalSpeed));
      
      holoColor += vec4(scanlines, scanlines, scanlines, 1.0) * 0.1;
      
      vec3 viewDirectionW = normalize(cameraPosition - vPositionW);
      float fresnelEffect = dot(viewDirectionW, vNormalW) * (1.6 - fresnelOpacity / 2.0);
      fresnelEffect = clamp(fresnelAmount - fresnelEffect, 0., fresnelOpacity);
      
      float blinkValue = enableBlinking ? 0.6 - signalSpeed : 1.0;
      float blink = flicker(blinkValue, time * signalSpeed * 0.1);
      
      vec3 finalColor;
      if(blinkFresnelOnly){
        finalColor = holoColor.rgb + fresnelEffect * blink;
      } else {
        finalColor = holoColor.rgb * blink + fresnelEffect;
      }
      gl_FragColor = vec4( finalColor * hologramBrightness, hologramOpacity);
    }