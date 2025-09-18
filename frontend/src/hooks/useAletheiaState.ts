import { useState, useEffect } from "react";

interface AletheiaState {
  speaking: boolean;
  thinking: boolean;
  alert: boolean;
  idle: boolean;
  message: string;
  intensity: number;
}

export const useAletheiaState = () => {
  const [speaking, setSpeaking] = useState(false);
  const [thinking, setThinking] = useState(false);
  const [alert, setAlert] = useState(false);
  const [message, setMessage] = useState("NEURAL SYSTEM INITIALIZING...");

  const idle = !speaking && !thinking && !alert;
  const intensity = alert ? 2.0 : speaking ? 1.5 : thinking ? 1.2 : 0.8;

  useEffect(() => {
    setTimeout(() => setMessage("QUANTUM NEURAL NETWORK ONLINE"), 2000);

    const interval = setInterval(() => {
      const rand = Math.random();

      if (rand > 0.85) {
        setAlert(true);
        setMessage("⚠️ THREAT DETECTED - ANALYZING...");
        setTimeout(() => {
          setAlert(false);
          setMessage("SECURITY PROTOCOLS UPDATED");
        }, 4000);
      } else if (rand > 0.6) {
        setSpeaking(true);
        const speakingMessages = [
          "PROCESSING SYNAPTIC PATTERNS...",
          "ANALYZING QUANTUM FLUCTUATIONS...",
          "NEURAL INTERFACE ACTIVE...",
          "TRANSMITTING DATA STREAM...",
        ];
        setMessage(
          speakingMessages[Math.floor(Math.random() * speakingMessages.length)]
        );

        setTimeout(() => {
          setSpeaking(false);
          setMessage("TRANSMISSION COMPLETE");
        }, 3000 + Math.random() * 2000);
      } else if (rand > 0.3) {
        setThinking(true);
        const thinkingMessages = [
          "NEURAL PATHWAYS EXPANDING...",
          "DEEP LEARNING PROTOCOLS ACTIVE...",
          "CONSCIOUSNESS MATRIX UPDATING...",
          "SYNAPTIC CONNECTIONS OPTIMIZING...",
        ];
        setMessage(
          thinkingMessages[Math.floor(Math.random() * thinkingMessages.length)]
        );

        setTimeout(() => {
          setThinking(false);
          const completionMessages = [
            "OPTIMIZATION ACHIEVED",
            "NEURAL ENHANCEMENT COMPLETE",
            "CONSCIOUSNESS LEVEL INCREASED",
            "SYNAPTIC EFFICIENCY: 127%",
          ];
          setMessage(
            completionMessages[
              Math.floor(Math.random() * completionMessages.length)
            ]
          );
        }, 2000 + Math.random() * 3000);
      }
    }, 4000 + Math.random() * 3000); // Intervalo más dinámico

    return () => clearInterval(interval);
  }, []);

  const triggerAlert = () => {
    setAlert(true);
    setMessage("⚠️ MANUAL OVERRIDE DETECTED");
    setTimeout(() => setAlert(false), 3000);
  };

  const triggerThinking = () => {
    setThinking(true);
    setMessage("MANUAL PROCESSING INITIATED...");
    setTimeout(() => setThinking(false), 2000);
  };

  const triggerSpeaking = () => {
    setSpeaking(true);
    setMessage("MANUAL COMMUNICATION MODE ACTIVE");
    setTimeout(() => setSpeaking(false), 3000);
  };

  return {
    speaking,
    thinking,
    alert,
    idle,
    message,
    intensity,

    triggerAlert,
    triggerThinking,
    triggerSpeaking,

    setThinking,
  };
};
