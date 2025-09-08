// frontend/src/components/SimulationViewer/index.tsx

import { useState, useEffect } from "react";
import { Canvas } from "@react-three/fiber";
import { OrbitControls, Box } from "@react-three/drei";

// Definimos el tipo de datos que esperamos recibir del backend
type GridState = number[][];

const SimulationGrid = ({ gridData }: { gridData: GridState }) => {
  const gridSize = gridData.length;
  const offset = gridSize / 2; // Para centrar la rejilla en el origen

  return (
    <group>
      {gridData.map((row, y) =>
        row.map((cell, x) => {
          // No renderizamos nada para el estado '0' (vacío)
          if (cell === 0) {
            return null;
          }

          // Renderizamos un cubo para los estados '1' y '-1'
          return (
            <Box
              key={`${y}-${x}`}
              position={[x - offset, y - offset, 0]}
              args={[0.8, 0.8, 0.8]} // Tamaño del cubo, con un pequeño margen
            >
              <meshStandardMaterial
                color={cell === 1 ? "#00ff88" : "#ff4444"}
              />
            </Box>
          );
        })
      )}
    </group>
  );
};

export const SimulationViewer = () => {
  // 1. Estado para almacenar la rejilla de la simulación
  const [grid, setGrid] = useState<GridState>([]);

  useEffect(() => {
    // 2. Establecer la conexión con el servidor WebSocket de Rust
    const ws = new WebSocket("ws://127.0.0.1:9001");

    ws.onopen = () => {
      console.log("✅ Conectado al motor de simulación de Rust.");
    };

    ws.onmessage = (event) => {
      try {
        // 3. Cuando llega un mensaje, lo parseamos de JSON al formato GridState
        const newGridState: GridState = JSON.parse(event.data);
        setGrid(newGridState); // Actualizamos el estado de React
      } catch (error) {
        console.error("Error al parsear los datos del backend:", error);
      }
    };

    ws.onerror = (error) => {
      console.error("Error en la conexión WebSocket:", error);
    };

    // 4. Limpieza: Nos aseguramos de cerrar la conexión cuando el componente se desmonte
    return () => {
      console.log("🔌 Desconectando del motor de simulación.");
      ws.close();
    };
  }, []); // El array vacío asegura que este efecto se ejecute solo una vez (al montar el componente)

  return (
    <div style={{ width: "100%", height: "80vh", background: "#111" }}>
      <Canvas camera={{ position: [0, 0, 25], fov: 50 }}>
        {/* Iluminación básica y controles de cámara */}
        <ambientLight intensity={0.5} />
        <pointLight position={[10, 10, 10]} />
        <OrbitControls />

        {/* El componente que renderiza la rejilla basada en el estado */}
        {grid.length > 0 && <SimulationGrid gridData={grid} />}
      </Canvas>
    </div>
  );
};
