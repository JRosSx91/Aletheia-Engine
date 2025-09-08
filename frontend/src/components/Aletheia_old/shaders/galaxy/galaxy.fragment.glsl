void main() {
    float distanceToCenter = length(gl_PointCoord - vec2(0.5));
    float strength = 1.0 - step(0.5, distanceToCenter);

    strength += pow(1.0 - distanceToCenter * 2.0, 4.0) * 0.5;
    strength = clamp(strength, 0.0, 1.0);

    gl_FragColor = vec4(0.5, 0.7, 1.0, strength);
}