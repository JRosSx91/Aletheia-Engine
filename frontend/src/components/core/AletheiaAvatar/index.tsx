import { useRef, useState, useCallback } from "react";
import { Canvas } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";
import { useAletheiaState } from "../../../hooks/useAletheiaState";
import { SimulationViewer } from "../../simulation/SimulationViewer";
import { EXPERIMENTS } from "../../../content/experiments";

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
    <div style={{ marginTop: "10px", fontSize: "10px", color: "#888", borderTop: "1px solid #333", paddingTop: "8px" }}>
      POWER: {Math.round(intensity * 100)}%
    </div>
    <div style={{ marginTop: "5px", fontSize: "10px", color: "#aaa", fontStyle: "italic", maxWidth: "180px", lineHeight: "1.3" }}>
      {message}
    </div>
  </div>
);

const btnStyle = (accent: string): React.CSSProperties => ({
  padding: "10px 14px",
  background: "rgba(0,0,0,0.8)",
  border: `1px solid ${accent}`,
  color: accent,
  cursor: "pointer",
  fontFamily: '"Courier New", monospace',
  fontSize: "12px",
  letterSpacing: "1px",
  minWidth: "140px",
  textAlign: "left",
});

export const AletheiaAvatar = () => {
  const { speaking, thinking, alert, intensity, message } = useAletheiaState();
  const simulationViewerRef = useRef<{ ws: WebSocket | null }>(null);

  const [running, setRunning] = useState(false);
  // id del experimento activo, reportado por el backend al conectar (MODE msg).
  const [modeId, setModeId] = useState<string | null>(null);
  const [infoOpen, setInfoOpen] = useState(true);
  const activeExperiment = modeId ? EXPERIMENTS[modeId] : null;

  const sendCommand = useCallback((command: object) => {
    const ws = simulationViewerRef.current?.ws;
    if (ws && ws.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(command));
    } else {
      console.warn("⚠️ WebSocket no conectado todavía:", command);
    }
  }, []);

  const handleInject = useCallback(() => {
    sendCommand({ type: "INJECT_STATE", payload: { x: 0, y: 0, z: 0, state: 1 } });
  }, [sendCommand]);

  const handlePlayPause = useCallback(() => {
    const next = !running;
    setRunning(next);
    sendCommand({ type: "SET_RUNNING", payload: { running: next } });
  }, [running, sendCommand]);

  const handleStep = useCallback(() => {
    sendCommand({ type: "STEP", payload: null });
  }, [sendCommand]);

  const handleReset = useCallback(() => {
    setRunning(false);
    sendCommand({ type: "SET_RUNNING", payload: { running: false } });
    sendCommand({ type: "RESET", payload: null });
  }, [sendCommand]);

  // Un único par frontal en el eje X, chocando en el centro.
  const handleHeadOn = useCallback(() => {
    sendCommand({ type: "INJECT_DIRECTED", payload: { x: -8, y: 0, z: 0, dir: "+x" } });
    sendCommand({ type: "INJECT_DIRECTED", payload: { x: 8, y: 0, z: 0, dir: "-x" } });
  }, [sendCommand]);

  // Rejilla 5x5 de pares frontales: un "frente" de colisiones simultáneas.
  const handleMultiHeadOn = useCallback(() => {
    for (let y = -6; y <= 6; y += 3) {
      for (let z = -6; z <= 6; z += 3) {
        sendCommand({ type: "INJECT_DIRECTED", payload: { x: -8, y, z, dir: "+x" } });
        sendCommand({ type: "INJECT_DIRECTED", payload: { x: 8, y, z, dir: "-x" } });
      }
    }
  }, [sendCommand]);

  // Nube de partículas en direcciones aleatorias: relajación térmica.
  const handleCloud = useCallback(() => {
    const dirs = ["+x", "-x", "+y", "-y", "+z", "-z"];
    for (let i = 0; i < 200; i++) {
      const x = Math.floor(Math.random() * 16) - 8;
      const y = Math.floor(Math.random() * 16) - 8;
      const z = Math.floor(Math.random() * 16) - 8;
      const dir = dirs[Math.floor(Math.random() * 6)];
      sendCommand({ type: "INJECT_DIRECTED", payload: { x, y, z, dir } });
    }
  }, [sendCommand]);

  return (
    <div style={{ width: "100vw", height: "100vh", background: "black", position: "relative" }}>
      <UltronStatusDisplay speaking={speaking} thinking={thinking} alert={alert} message={message} intensity={intensity} />

      {/* Panel explicativo del experimento activo — arriba a la izquierda */}
      {activeExperiment && (
        <div
          style={{
            position: "absolute",
            top: "20px",
            left: "20px",
            zIndex: 1000,
            maxWidth: "360px",
            fontFamily: '"Courier New", monospace',
            background: "rgba(0,5,10,.88)",
            border: "1px solid #00ffff55",
            borderRadius: "8px",
            color: "#cfeff0",
            backdropFilter: "blur(4px)",
          }}
        >
          <div
            onClick={() => setInfoOpen((v) => !v)}
            style={{
              padding: "12px 16px",
              cursor: "pointer",
              display: "flex",
              justifyContent: "space-between",
              alignItems: "center",
              borderBottom: infoOpen ? "1px solid #00ffff30" : "none",
            }}
          >
            <span style={{ fontSize: "10px", letterSpacing: "1px", color: "#00ffff" }}>{activeExperiment.badge}</span>
            <span style={{ fontSize: "11px", color: "#7fa3a4" }}>{infoOpen ? "▾" : "▸"}</span>
          </div>

          {infoOpen && (
            <div style={{ padding: "14px 16px 18px", fontSize: "12.5px", lineHeight: 1.6, maxHeight: "52vh", overflowY: "auto" }}>
              <h3 style={{ margin: "0 0 10px", fontSize: "17px", color: "#eafdff" }}>{activeExperiment.title}</h3>
              <p style={{ color: "#9fc4c6", margin: "0 0 12px" }}>
                <strong style={{ color: "#00ffff" }}>Hipótesis: </strong>
                {activeExperiment.hypothesis}
              </p>
              <p style={{ color: "#8aa3a4", margin: "0 0 6px", fontSize: "10.5px", letterSpacing: "1px" }}>MECANISMO</p>
              <ul style={{ margin: "0 0 12px", paddingLeft: "18px", color: "#a8cccd" }}>
                {activeExperiment.mechanism.map((step, i) => (
                  <li key={i} style={{ marginBottom: "4px" }}>{step}</li>
                ))}
              </ul>
              <p style={{ color: "#8aa3a4", margin: "0 0 6px", fontSize: "10.5px", letterSpacing: "1px" }}>QUÉ MIRAR</p>
              <p style={{ color: "#a8cccd", margin: 0 }}>{activeExperiment.whatToWatch}</p>
            </div>
          )}
        </div>
      )}

      {/* Controles — columna vertical a la DERECHA, lejos del panel explicativo */}
      <div
        style={{
          position: "absolute",
          top: "50%",
          right: "20px",
          transform: "translateY(-40%)",
          zIndex: 1000,
          display: "flex",
          flexDirection: "column",
          gap: "8px",
        }}
      >
        <button style={btnStyle("#00ff00")} onClick={handleInject}>INJECT</button>
        <button style={btnStyle(running ? "#ffaa00" : "#00ffff")} onClick={handlePlayPause}>
          {running ? "❚❚ PAUSE" : "▶ PLAY"}
        </button>
        <button style={btnStyle("#44aaff")} onClick={handleStep}>⏭ STEP</button>

        <div style={{ height: "1px", background: "#333", margin: "4px 0" }} />

        <button style={btnStyle("#ff44ff")} onClick={handleHeadOn}>⇄ HEAD-ON</button>
        <button style={btnStyle("#ff44ff")} onClick={handleMultiHeadOn}>⇉ FRENTE</button>
        <button style={btnStyle("#ffaa44")} onClick={handleCloud}>✦ NUBE</button>

        <div style={{ height: "1px", background: "#333", margin: "4px 0" }} />

        <button style={btnStyle("#ff4444")} onClick={handleReset}>⟲ RESET</button>
      </div>

      <Canvas
        camera={{ position: [0, 0, 145], fov: 50 }}
        gl={{ antialias: true, alpha: false }}
      >
        <color attach="background" args={["#000511"]} />
        <ambientLight intensity={0.03} />
        <SimulationViewer ref={simulationViewerRef} onModeChange={setModeId} />
        <OrbitControls
          enableZoom={true}
          enablePan={false}
          maxDistance={220}
          minDistance={60}
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