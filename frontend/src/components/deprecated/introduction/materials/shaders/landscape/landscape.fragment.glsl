varying vec2 vUv;
uniform sampler2D uDiffuse;
uniform sampler2D uMultispectral;
uniform float uOpacity;
uniform float u_transition_progress;
uniform float u_time;

// Convert RGB to grayscale (luminance)
float luminance(vec3 color) {
    return dot(color, vec3(0.299, 0.587, 0.114));
}

// Generate thermal infrared color based on luminance (heat signature)
vec3 thermalInfraredColor(float intensity) {
    // Thermal IR color palette: cold (dark blue/black) to hot (white/yellow)
    // This mimics real thermal imaging cameras
    
    if (intensity < 0.2) {
        // Very cold - dark blue to purple
        return mix(vec3(0.0, 0.0, 0.1), vec3(0.2, 0.0, 0.4), intensity * 5.0);
    } else if (intensity < 0.4) {
        // Cold - purple to red
        return mix(vec3(0.2, 0.0, 0.4), vec3(0.8, 0.0, 0.2), (intensity - 0.2) * 5.0);
    } else if (intensity < 0.6) {
        // Warm - red to orange
        return mix(vec3(0.8, 0.0, 0.2), vec3(1.0, 0.5, 0.0), (intensity - 0.4) * 5.0);
    } else if (intensity < 0.8) {
        // Hot - orange to yellow
        return mix(vec3(1.0, 0.5, 0.0), vec3(1.0, 1.0, 0.0), (intensity - 0.6) * 5.0);
    } else {
        // Very hot - yellow to white
        return mix(vec3(1.0, 1.0, 0.0), vec3(1.0, 1.0, 1.0), (intensity - 0.8) * 5.0);
    }
}

void main() {
    vec2 uv = vUv;
    
    // Sample the visible light image
    vec4 visibleTexture = texture2D(uDiffuse, uv);
    vec3 visibleColor = visibleTexture.rgb;
    
    // Calculate luminance to determine "heat" levels
    float heat = luminance(visibleColor);
    
    // Add some variation based on UV coordinates to simulate different thermal properties
    // Lighter areas (grass, leaves) tend to be warmer during day
    // Darker areas (shadows, water) tend to be cooler
    heat = heat * 0.7 + 0.3; // Normalize to avoid pure black areas
    
    // Generate thermal infrared representation
    vec3 thermalColor = thermalInfraredColor(heat);
    
    // Simple clean transition between visible and thermal IR
    vec3 finalColor = mix(visibleColor, thermalColor, u_transition_progress);
    
    // Optional: Add subtle "scanning" effect during transition
    if (u_transition_progress > 0.0 && u_transition_progress < 1.0) {
        float scanLine = sin(uv.y * 150.0 + u_time * 3.0) * 0.5 + 0.5;
        float scanIntensity = sin(u_transition_progress * 3.14159) * 0.03; // Peak at 50% transition
        finalColor += vec3(0.0, 0.2, 0.5) * scanLine * scanIntensity;
    }
    
    gl_FragColor = vec4(finalColor, uOpacity);
}
