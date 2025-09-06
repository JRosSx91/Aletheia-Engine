import { useScroll } from "@react-three/drei";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";
import { createTimeline } from "animejs";
import { useEffect, useRef, useState } from "react";
import { LandscapeMaterial } from "./materials/LandscapeMaterial";

// ACCIÓN: Derivamos el tipo del timeline directamente de la función 'createTimeline'.
// Esto garantiza que siempre tendremos el tipo correcto sin depender de exportaciones no públicas.
type Timeline = ReturnType<typeof createTimeline>;

interface SceneControllerProps {
  landscapeRef: React.RefObject<THREE.Mesh>;
  onTransitionProgressChange?: (progress: number) => void;
}

export const SceneController = ({ landscapeRef, onTransitionProgressChange }: SceneControllerProps) => {
  const scroll = useScroll();
  // El 'ref' ahora usa el tipo 'Timeline' que hemos derivado.
  const timeline = useRef<Timeline | null>(null);
  const [transitionProgress, setTransitionProgress] = useState(0);

  useEffect(() => {
    if (!landscapeRef.current) return;

    const material = landscapeRef.current.material as LandscapeMaterial;

    timeline.current = createTimeline({
      autoplay: false,
    });

    // Timeline for landscape opacity animations
    timeline.current.add(
      material.uniforms.uOpacity,
      { value: 1.0, duration: 1000, easing: 'linear' },
      (0.15 * 5 * 1000)
    );

    timeline.current.add(
      material.uniforms.uOpacity,
      { value: 0.0, duration: 1000, easing: 'linear' },
      (0.4 * 5 * 1000)
    );
  }, [landscapeRef]);

  useFrame(() => {
    if (timeline.current) {
      timeline.current.seek(scroll.offset * timeline.current.duration);
    }

    // Calculate transition progress based on scroll position
    // Channel changing effect should happen between scroll positions 0.15 and 0.35
    // This corresponds to the text sections about "changing channels"
    const scrollOffset = scroll.offset;
    let channelProgress = 0;
    
    if (scrollOffset >= 0.15 && scrollOffset <= 0.35) {
      // Map scroll range [0.15, 0.35] to transition range [0, 1]
      channelProgress = (scrollOffset - 0.15) / (0.35 - 0.15);
      channelProgress = Math.max(0, Math.min(1, channelProgress));
    } else if (scrollOffset > 0.35) {
      // After the transition, show the multispectral view
      channelProgress = 1;
    }
    
    if (channelProgress !== transitionProgress) {
      setTransitionProgress(channelProgress);
      onTransitionProgressChange?.(channelProgress);
    }
  });

  return null;
};
