import { useRef } from "react";
import { Canvas } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";
import { useAletheiaState } from "../../../hooks/useAletheiaState";
import { SimulationViewer } from "../../simulation/SimulationViewer";

export const UltronStatusDisplay = ({
  speaking,
  thinking,
  alert,
  message,
  intensity,
}: {
  speaking: boolean;
  thinking: boolean;
  alert: boolean;
  message: string;
  intensity: number;
}) => (
  <div
    style={{
      position: "absolute",
      top: "20px",
      right: "20px",
      color: "#00ffff",
      fontFamily: '"Courier New", monospace',
      fontSize: "12px",
      textShadow: "0 0 5px #00ffff",
      zIndex: 1000,
      background: "rgba(0,0,0,0.8)",
      padding: "15px",
      borderRadius: "8px",
      border: "1px solid #00ffff40",
      minWidth: "200px",
    }}
  >
    <div
      style={{
        marginBottom: "10px",
        fontSize: "14px",
        fontWeight: "bold",
        color: alert
          ? "#ff4444"
          : speaking
          ? "#00ff44"
          : thinking
          ? "#ffff44"
          : "#44aaff",
      }}
    >
      ◉ ULTRON NEURAL CORE
    </div>
    <div style={{ marginBottom: "8px" }}>
      <span style={{ color: speaking ? "#00ff00" : "#666" }}>
        SPEECH: {speaking ? "[ACTIVE]" : "[IDLE]"}
      </span>
    </div>
    <div style={{ marginBottom: "8px" }}>
      <span style={{ color: thinking ? "#ffaa00" : "#666" }}>
        COGNITION: {thinking ? "[PROCESSING]" : "[STANDBY]"}
      </span>
    </div>
    <div style={{ marginBottom: "8px" }}>
      <span style={{ color: alert ? "#ff0000" : "#666" }}>
        THREAT: {alert ? "[DETECTED]" : "[SECURE]"}
      </span>
    </div>
    <div
      style={{
        marginTop: "10px",
        fontSize: "10px",
        color: "#888",
        borderTop: "1px solid #333",
        paddingTop: "8px",
      }}
    >
      POWER: {Math.round(intensity * 100)}%
    </div>
    <div
      style={{
        marginTop: "5px",
        fontSize: "10px",
        color: "#aaa",
        fontStyle: "italic",
        maxWidth: "180px",
        lineHeight: "1.3",
      }}
    >
      {message}
    </div>
  </div>
);

export const AletheiaAvatar = () => {
  const { speaking, thinking, alert, intensity, message } = useAletheiaState();
  const simulationViewerRef = useRef<{ ws: WebSocket | null }>(null);

  const injectState = () => {
    const ws = simulationViewerRef.current?.ws;
    if (ws && ws.readyState === WebSocket.OPEN) {
      const command = {
        type: "INJECT_STATE",
        payload: {
          x: 0,
          y: 0,
          z: 0,
          state: 1,
        },
      };
      ws.send(JSON.stringify(command));
    } else {
      console.error("WebSocket no está conectado.");
    }
  };

  return (
    <div
      style={{
        width: "100vw",
        height: "100vh",
        background: "black",
        position: "relative",
      }}
    >
      {/* UI Status Display */}
      <UltronStatusDisplay
        speaking={speaking}
        thinking={thinking}
        alert={alert}
        message={message}
        intensity={intensity}
      />

      {/* Nuevo botón de inyección */}
      <button
        onClick={injectState}
        style={{
          position: "absolute",
          bottom: "20px",
          left: "20px",
          zIndex: 1000,
          padding: "10px",
          background: "rgba(0,0,0,0.8)",
          border: "1px solid #00ff00",
          color: "#00ff00",
          cursor: "pointer",
        }}
      >
        INJECT STATE
      </button>

      <Canvas
        camera={{ position: [0, 0, 100], fov: 50 }}
        gl={{ antialias: true, alpha: false }}
      >
        <color attach="background" args={["#000511"]} />
        <ambientLight intensity={0.03} />
        <SimulationViewer ref={simulationViewerRef} />
        <OrbitControls
          enableZoom={true}
          enablePan={false}
          maxDistance={200}
          minDistance={50}
          maxPolarAngle={Math.PI / 1.3}
          minPolarAngle={Math.PI / 4}
          autoRotate={false}
          enableDamping={true}
          dampingFactor={0.05}
        />
      </Canvas>
    </div>
  );
};

export default AletheiaAvatar;
