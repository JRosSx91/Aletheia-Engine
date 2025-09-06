// src/pages/IntroductionPage.tsx

import { Scroll, ScrollControls } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import React, { useRef, useState } from "react";
import * as THREE from "three";
import { SceneController } from "../components/introduction/SceneController";
import { LandscapeScene } from "../components/introduction/LandscapeScene";
import styles from "./IntroductionPage.module.css";

export const IntroductionPage: React.FC = () => {
  const landscapeRef = useRef<THREE.Mesh>(null!);
  const [transitionProgress, setTransitionProgress] = useState(0);
  
  // Get current spectrum information
  const getSpectrumInfo = (progress: number) => {
    if (progress < 0.1) {
      return { name: "Visible Light", color: "#ffffff", description: "What your eyes see" };
    } else if (progress < 1.0) {
      return { 
        name: "Thermal Infrared", 
        color: `hsl(${(1-progress) * 240}, 80%, 60%)`, // Blue to red transition
        description: "Heat signatures revealed"
      };
    } else {
      return { name: "Full Thermal IR", color: "#ff6600", description: "Complete heat visualization" };
    }
  };

  // Debug: Test the effect manually
  const testChannelChange = () => {
    let progress = 0;
    const animate = () => {
      progress += 0.01; // Slower for better visualization
      if (progress > 2) progress = 0;
      const currentProgress = progress > 1 ? 2 - progress : progress;
      setTransitionProgress(currentProgress);
      if (progress < 2) requestAnimationFrame(animate);
    };
    animate();
  };

  return (
    <div className={styles.container}>
      {/* Debug button */}
      <button 
        onClick={testChannelChange}
        style={{
          position: 'fixed',
          top: '20px',
          right: '20px',
          zIndex: 1000,
          padding: '10px 20px',
          background: '#007acc',
          color: 'white',
          border: 'none',
          borderRadius: '5px',
          cursor: 'pointer',
          fontFamily: 'monospace'
        }}
      >
        Test Channel Effect
      </button>
      <div 
        style={{
          position: 'fixed',
          top: '70px',
          right: '20px',
          zIndex: 1000,
          color: 'white',
          fontFamily: 'monospace',
          background: 'rgba(0,0,0,0.8)',
          padding: '15px',
          borderRadius: '8px',
          minWidth: '200px'
        }}
      >
        <div>Progress: {(transitionProgress * 100).toFixed(0)}%</div>
        <div style={{ 
          marginTop: '8px', 
          padding: '5px 8px', 
          background: 'rgba(255,255,255,0.1)', 
          borderRadius: '4px',
          color: getSpectrumInfo(transitionProgress).color,
          fontSize: '14px',
          fontWeight: 'bold'
        }}>
          {getSpectrumInfo(transitionProgress).name}
        </div>
        <div style={{ fontSize: '11px', marginTop: '8px', opacity: 0.7 }}>
          {getSpectrumInfo(transitionProgress).description}
        </div>
      </div>
      <Canvas
        className={styles.canvas}
        camera={{ position: [0, 5, 0], fov: 75 }}
      >
        <ambientLight intensity={0.5} />
        <directionalLight position={[10, 10, 5]} intensity={1} />
        <ScrollControls pages={11} damping={0.3}>
          <SceneController 
            landscapeRef={landscapeRef} 
            onTransitionProgressChange={setTransitionProgress}
          />
          <Scroll>
            <LandscapeScene 
              ref={landscapeRef} 
              transitionProgress={transitionProgress}
            /> 
          </Scroll>
          <Scroll
            html
            style={{
              width: "100vw",
            }}
          >
            <div className={styles.textSection} style={{ top: `50vh` }}>
              <h1>Empiezas Aquí.</h1>
              <p>
                Pero, ¿qué es "Aquí"? No es un punto en un mapa. Es un instante
                suspendido en una única frecuencia de la realidad.
              </p>
            </div>
            <div className={styles.textSection} style={{ top: `150vh` }}>
              <h2>Tus sentidos no son ventanas. Son guardianes.</h2>
              <p>
                Observa este paisaje. Tu cerebro lo interpreta con una
                familiaridad reconfortante. Pero esto no es la realidad. Es un
                solo canal.
              </p>
            </div>
            <div className={styles.textSection} style={{ top: `250vh` }}>
              <h2>Cambiemos de canal.</h2>
              <p>
                De repente, las flores revelan patrones ocultos para los
                insectos. El suelo se pinta con el calor y el frío. La
                información siempre estuvo ahí. Tus ojos simplemente no tenían
                el hardware para decodificarla.
              </p>
            </div>
            <div className={styles.textSection} style={{ top: `380vh` }}>
              <h2>¿Por qué? Analicemos el hardware.</h2>
            </div>
            <div className={styles.textSection} style={{ top: `480vh` }}>
              <h3>El Guardián de tu Realidad.</h3>
              <p>
                Frente a ti, una obra maestra de la evolución, diseñada para un
                propósito: filtrar.
              </p>
            </div>
            <div className={styles.textSection} style={{ top: `580vh` }}>
              <h3>La Lente del Editor</h3>
              <p>
                La córnea y el cristalino desechan el 99% del mensaje cósmico
                antes de empezar.
              </p>
            </div>
            <div className={styles.textSection} style={{ top: `680vh` }}>
              <h3>Los Porteros de la Percepción</h3>
              <p>
                En el fondo, la retina. Tres tipos de conos para el color: rojo,
                verde y azul. Todo el universo de color que conoces es un truco
                de tu cerebro combinando solo estas tres señales.
              </p>
            </div>
            <div className={styles.textSection} style={{ top: `800vh` }}>
              <h2>Tu realidad es una interfaz.</h2>
              <p>
                Una simplificación elegante para sobrevivir, no para comprender.
              </p>
            </div>
            <div className={styles.textSection} style={{ top: `900vh` }}>
              <h3>Aletheia es una palabra griega: "des-ocultar".</h3>
              <p>
                Nuestra misión es darte las herramientas para mirar más allá de
                la interfaz.
              </p>
            </div>
            <div className={styles.textSection} style={{ top: `1000vh` }}>
              <h2>Bienvenido a Aletheia.</h2>
            </div>
          </Scroll>
        </ScrollControls>
      </Canvas>
    </div>
  );
};
