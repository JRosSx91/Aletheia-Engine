import {
  useEffect,
  useState,
  useMemo,
  forwardRef,
  useImperativeHandle,
  useRef,
} from "react";
import * as THREE from "three";
import { QuantumCell } from "../SimulationViewer/QuantumCell";
import { QuantumGrid } from "../QuantumGrid";

interface Cell {
  x: number;
  y: number;
  z: number;
  state: number;
}

interface SimulationUpdate {
  tick: number;
  cells: Cell[];
}

export const SimulationViewer = forwardRef((_props, ref) => {
  // Estado para almacenar solo las celdas activas que vienen del backend
  const [activeCells, setActiveCells] = useState(
    new Map<
      string,
      { color: THREE.Color; position: [number, number, number] }
    >()
  );

  const wsRef = useRef<WebSocket | null>(null);

  // Parámetros del grid para coincidir con el backend
  const BACKEND_RANGE = 25; // Backend usa -25 a +25
  const CELL_SPACING = 2; // Espaciado entre celdas para visualización

  // Función para convertir coordenadas del backend a posiciones de visualización
  const backendToWorldPos = (
    x: number,
    y: number,
    z: number
  ): [number, number, number] => {
    return [x * CELL_SPACING, y * CELL_SPACING, z * CELL_SPACING];
  };

  useEffect(() => {
    const ws = new WebSocket("ws://127.0.0.1:9001");
    wsRef.current = ws;

    ws.onopen = () =>
      console.log("✅ Conectado al motor de simulación de Rust.");

    // Reemplaza el bloque ws.onmessage con la siguiente versión corregida
    ws.onmessage = (event) => {
      try {
        const data: SimulationUpdate = JSON.parse(event.data);
        console.log("📥 Datos recibidos del backend:", data);

        if (data && data.cells && Array.isArray(data.cells)) {
          setActiveCells((prevCells) => {
            const newCells = new Map(prevCells);

            // Procesar cada celda recibida del backend
            data.cells.forEach((cell: Cell) => {
              const posKey = `${cell.x},${cell.y},${cell.z}`;
              const worldPos = backendToWorldPos(cell.x, cell.y, cell.z);

              if (cell.state === 1) {
                // Materia/energía - rojo
                newCells.set(posKey, {
                  color: new THREE.Color(1.0, 0.2, 0.2),
                  position: worldPos,
                });
                console.log(
                  `🔴 Celda activa en (${cell.x},${cell.y},${cell.z}) -> mundo (${worldPos[0]},${worldPos[1]},${worldPos[2]})`
                );
              } else if (cell.state === -1) {
                // Antimateria - azul
                newCells.set(posKey, {
                  color: new THREE.Color(0.2, 0.2, 1.0),
                  position: worldPos,
                });
                console.log(
                  `🔵 Celda activa en (${cell.x},${cell.y},${cell.z}) -> mundo (${worldPos[0]},${worldPos[1]},${worldPos[2]})`
                );
              } else if (cell.state === 0) {
                // Vacío - remover de celdas activas
                newCells.delete(posKey);
                console.log(
                  `⚫ Celda desactivada en (${cell.x},${cell.y},${cell.z})`
                );
              }
            });

            console.log(`📊 Total de celdas activas: ${newCells.size}`);
            return newCells;
          });
        } else {
          console.warn("⚠️ Formato de datos inesperado:", data);
        }
      } catch (error) {
        console.error("❌ Error al parsear los datos del backend:", error);
      }
    };

    ws.onerror = (error) =>
      console.error("❌ Error en la conexión WebSocket:", error);

    ws.onclose = () => console.log("🔌 Conexión WebSocket cerrada");

    return () => {
      ws.close();
    };
  }, []);

  useImperativeHandle(ref, () => ({
    ws: wsRef.current,
  }));

  // Convertir el Map a un array para renderizar
  const activeCellsArray = useMemo(() => {
    return Array.from(activeCells.entries()).map(([key, cellData]) => ({
      key,
      position: cellData.position,
      color: cellData.color,
      active: true,
    }));
  }, [activeCells]);

  return (
    <group>
      {/* Grid de referencia - usar el mismo tamaño que el backend */}
      <QuantumGrid
        gridSize={BACKEND_RANGE * 2 * CELL_SPACING}
        divisions={BACKEND_RANGE * 2}
      />

      {/* Renderizar solo las celdas activas */}
      {activeCellsArray.map((cellData) => (
        <QuantumCell
          key={cellData.key}
          position={cellData.position}
          color={cellData.color}
          active={cellData.active}
        />
      ))}

      {/* Debug: Mostrar el número de celdas activas */}
      {activeCellsArray.length > 0 && (
        <mesh position={[0, BACKEND_RANGE * CELL_SPACING + 10, 0]}>
          <boxGeometry args={[0.1, 0.1, 0.1]} />
          <meshBasicMaterial color="white" />
        </mesh>
      )}
    </group>
  );
});
