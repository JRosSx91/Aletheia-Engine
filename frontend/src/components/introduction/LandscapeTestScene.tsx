import React, { useState } from "react";
import { Canvas } from "@react-three/fiber";
import { LandscapeScene } from "./LandscapeScene";

export const LandscapeTestScene: React.FC = () => {
  const [transitionProgress, setTransitionProgress] = useState(0);

  return (
    <div style={{ width: "100vw", height: "100vh", position: "relative" }}>
      <Canvas camera={{ position: [0, 5, 10], fov: 60 }}>
        <ambientLight intensity={0.5} />
        <pointLight position={[10, 10, 10]} />
        <LandscapeScene transitionProgress={transitionProgress} />
      </Canvas>
      
      {/* Control Panel */}
      <div style={{
        position: "absolute",
        top: "20px",
        left: "20px",
        background: "rgba(0,0,0,0.7)",
        color: "white",
        padding: "20px",
        borderRadius: "10px",
        fontFamily: "monospace"
      }}>
        <h3>Channel Changing Effect Test</h3>
        <div style={{ marginBottom: "10px" }}>
          <label>Transition Progress: {transitionProgress.toFixed(2)}</label>
          <input
            type="range"
            min="0"
            max="1"
            step="0.01"
            value={transitionProgress}
            onChange={(e) => setTransitionProgress(parseFloat(e.target.value))}
            style={{ width: "200px", display: "block", margin: "10px 0" }}
          />
        </div>
        
        <div style={{ fontSize: "12px", marginTop: "15px" }}>
          <p>• At 0.0: Normal landscape texture</p>
          <p>• At 0.5: Maximum glitch/noise effect</p>  
          <p>• At 1.0: Multispectral texture</p>
          <p>• The effect includes:</p>
          <p>&nbsp;&nbsp;- Noise-based transition mask</p>
          <p>&nbsp;&nbsp;- UV distortion</p>
          <p>&nbsp;&nbsp;- Color channel shifting</p>
          <p>&nbsp;&nbsp;- Scan line interference</p>
          <p>&nbsp;&nbsp;- Random glitch blocks</p>
        </div>
        
        <button
          onClick={() => {
            // Animate from 0 to 1 and back
            let progress = 0;
            const animate = () => {
              progress += 0.02;
              if (progress > 2) progress = 0;
              setTransitionProgress(progress > 1 ? 2 - progress : progress);
              if (progress < 2) requestAnimationFrame(animate);
            };
            animate();
          }}
          style={{
            marginTop: "10px",
            padding: "8px 16px",
            background: "#007acc",
            color: "white",
            border: "none",
            borderRadius: "4px",
            cursor: "pointer"
          }}
        >
          Auto Animate
        </button>
      </div>
    </div>
  );
};
