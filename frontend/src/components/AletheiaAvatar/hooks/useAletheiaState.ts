import { useState, useEffect } from "react";

export const useAletheiaState = () => {
  const [speaking, setSpeaking] = useState(false);
  const [thinking, setThinking] = useState(false);
  const [message, setMessage] = useState("NEURAL SYSTEM INITIALIZING...");

  // Este useEffect simula los cambios de estado de la IA
  useEffect(() => {
    // Mensaje inicial después de 2 segundos
    setTimeout(() => setMessage("QUANTUM NEURAL NETWORK ONLINE"), 2000);

    // Bucle principal de cambio de estado
    const interval = setInterval(() => {
      const rand = Math.random();
      if (rand > 0.7) {
        setSpeaking(true);
        setMessage("PROCESSING SYNAPTIC PATTERNS...");
        setTimeout(() => {
          setSpeaking(false);
          setMessage("ANALYSIS COMPLETE");
        }, 3000);
      } else if (rand > 0.4) {
        setThinking(true);
        setMessage("NEURAL PATHWAYS EXPANDING...");
        setTimeout(() => {
          setThinking(false);
          setMessage("OPTIMIZATION ACHIEVED");
        }, 2000);
      }
    }, 5000);

    // Limpieza al desmontar el componente para evitar fugas de memoria
    return () => clearInterval(interval);
  }, []);

  return { speaking, thinking, message, setThinking };
};
