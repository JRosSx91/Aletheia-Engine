// Ficha explicativa de cada experimento/premisa. La clave (id) tiene que
// coincidir con el `mode_id()` que reporta el motor de Rust por WebSocket.
// Añadir un experimento nuevo = añadir una entrada aquí + implementar
// mode_id()/mode_name() en el motor correspondiente. Nada más se toca.

export interface ExperimentInfo {
  id: string;
  badge: string;       // etiqueta corta arriba (p.ej. "PREMISA 01")
  title: string;
  hypothesis: string;  // qué se pregunta
  mechanism: string[];  // cómo funciona la regla, en pasos
  whatToWatch: string; // qué mirar en pantalla/consola para "ver" el resultado
  status: "estable" | "en curso" | "resultado inesperado";
}

export const EXPERIMENTS: Record<string, ExperimentInfo> = {
  diffusion_binary: {
    id: "diffusion_binary",
    badge: "PREMISA 01 · SISTEMA CERRADO",
    title: "Difusión Binaria",
    hypothesis:
      "Un universo finito que no se expande. El agujero negro de otro universo vierte bits 1 en un sustrato de 0s. ¿Cómo se transmite esa información por un espacio que no crece?",
    mechanism: [
      "Cada celda vale 0 (vacío) o 1 (energía).",
      "En cada tick, toda celda con 1 elige un vecino vacío al azar y salta a él.",
      "La celda de origen vuelve a 0. Sin memoria de dirección: es un paseo aleatorio (random walk).",
    ],
    whatToWatch:
      "La nube de cubos rojos se expande y diluye con el tiempo, sin volver a concentrarse. No hay conservación de momento — cada salto es independiente del anterior.",
    status: "estable",
  },

  lattice_gas: {
    id: "lattice_gas",
    badge: "EXPERIMENTO · EMERGENCIA",
    title: "Gas de Rejilla — Conservación de Momento",
    hypothesis:
      "¿Puede una regla local, sin conocimiento global, producir estructuras que se propagan e interactúan sin disolverse — el primer paso hacia una 'partícula'?",
    mechanism: [
      "Cada celda tiene 6 canales de dirección (±x, ±y, ±z). Un canal ocupado = una partícula viajando en esa dirección.",
      "Fase STREAMING: cada partícula avanza una celda en su dirección, sin excepción.",
      "Fase COLLISION: si dos partículas se encuentran de frente en la misma celda (p.ej. +x y −x), se desvían juntas a un eje perpendicular. Todo lo demás pasa de largo.",
      "El toro es cerrado: quien sale por un borde entra por el opuesto. No hay paredes que absorban momento.",
    ],
    whatToWatch:
      "En consola, masa y momento se mantienen EXACTAMENTE constantes tick tras tick, pase lo que pase en pantalla — esa es la prueba de que la regla conserva lo que promete. Con configuraciones simétricas (como el HEAD-ON o el FRENTE), el sistema puede volver exactamente a su estado inicial tras muchos ticks: es recurrencia de Poincaré. Ocurre porque la regla es determinista y reversible sobre un número finito de estados — el sistema está obligado, tarde o temprano, a repetirse.",
    status: "resultado inesperado",
  },
};