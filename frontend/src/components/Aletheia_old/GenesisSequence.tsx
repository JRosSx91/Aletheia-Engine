/* eslint-disable @typescript-eslint/no-explicit-any */
import { useRef, useEffect, useMemo } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";
import { useTexture } from "@react-three/drei";
import { createTimeline } from "animejs";
import { useControls } from "leva";
import { QuantumField } from "./QuantumField";
import { GenesisDrop } from "./GenesisDrop";
import { MetamorphosisMaterial } from "./materials/MetamorphosisMaterial";
import { GodRays, EffectComposer, Bloom } from "@react-three/postprocessing";
import { BlendFunction } from "postprocessing";

const TIMELINE_DURATION = 12000;

const FINAL_FIREBALL_STATE = {
  color: "#ff4400",
  emissiveIntensity: 6.4,
  lightIntensity: 8.0,
  alphaTest: 0.86, // El valor que encontraste satisfactorio
  rotationSpeed: 1.3,
};

export function GenesisSequence() {
  const dropRef = useRef<THREE.Group>(null!);
  const pointLightRef = useRef<THREE.PointLight>(null!);
  const fireballRef = useRef<THREE.Mesh>(null!);

  const alphaMap = useTexture("/textures/fire-alpha.jpg");

  const metamorphosisMaterial = useMemo(
    () =>
      new MetamorphosisMaterial({
        transparent: true,
        blending: THREE.AdditiveBlending,
        depthWrite: false,
      }),
    []
  );

  const fireballMaterial = useMemo(
    () =>
      new THREE.MeshStandardMaterial({
        color: new THREE.Color("#ff4400"),
        emissive: new THREE.Color("#ff4400"),
        alphaMap: alphaMap,
        alphaTest: 0.5,
        side: THREE.DoubleSide,
        transparent: false,
        toneMapped: false,
      }),
    [alphaMap]
  );

  // Propiedades de la animación. Las mantenemos todas para no romper la estructura.
  const animProps = useRef({
    displacement: 1.0,
    rotation: 0.0,
    quantumOpacity: 1.0,
    fireballOpacity: 0.0,
    fireballAlphaTest: 0.0,
    r: 1.0,
    g: 0.2,
    b: 0.0,
    emissiveIntensity: 0.0,
    scale: 1.0,
    genesisAmplitude: 0.0,
    fieldTurbulence: 1.0,
    collapseProgress: 0.0,
    lightIntensity: 0.0,
    bloomIntensity: 1.25,
    godRaysWeight: 0.0,
  }).current;

  const timelineRef = useRef(createTimeline({ autoplay: false }));

  // --- PANEL DE CONTROL PARA EL FIREBALL ---
  const fireballControls = useControls("Fireball Tuning", {
    "Activar Modo Tuning": { value: false },
    color: {
      value: FINAL_FIREBALL_STATE.color, // <-- Usar valor de la constante
      label: "Color",
      render: (get) => get("Fireball Tuning.Activar Modo Tuning"),
    },
    emissiveIntensity: {
      value: FINAL_FIREBALL_STATE.emissiveIntensity, // <-- Usar valor de la constante
      min: 0.0,
      max: 10.0,
      label: "Intensidad Emisiva",
      render: (get) => get("Fireball Tuning.Activar Modo Tuning"),
    },
    lightIntensity: {
      value: FINAL_FIREBALL_STATE.lightIntensity, // <-- Usar valor de la constante
      min: 0.0,
      max: 50.0,
      label: "Intensidad de Luz",
      render: (get) => get("Fireball Tuning.Activar Modo Tuning"),
    },
    alphaTest: {
      value: FINAL_FIREBALL_STATE.alphaTest, // <-- Usar valor de la constante
      min: 0.0,
      max: 1.0,
      label: "Umbral de Alpha",
      render: (get) => get("Fireball Tuning.Activar Modo Tuning"),
    },
    rotationSpeed: {
      value: FINAL_FIREBALL_STATE.rotationSpeed, // <-- Usar valor de la constante
      min: 0.0,
      max: 2.0,
      label: "Velocidad Rotación",
      render: (get) => get("Fireball Tuning.Activar Modo Tuning"),
    },
  });

  const { progress } = useControls("Secuencia de Génesis", {
    progress: { value: 0.0, min: 0.0, max: 1.0 },
  });

  // Tu línea de tiempo original, intacta.
  useEffect(() => {
    const tl = timelineRef.current;
    tl.reset();

    const redColor = new THREE.Color(FINAL_FIREBALL_STATE.color);
    const yellowColor = new THREE.Color("#ffee00");
    const blueColor = new THREE.Color("#00aaff");

    tl.add(animProps, {
      genesisAmplitude: [0, 4.0, 0],
      duration: 2000,
      easing: "easeOutSine",
    })
      .add(
        animProps,
        {
          displacement: 0.0,
          rotation: 10,
          quantumOpacity: 0.0,
          fireballOpacity: 1.0,
          fireballAlphaTest: FINAL_FIREBALL_STATE.alphaTest,
          emissiveIntensity: FINAL_FIREBALL_STATE.emissiveIntensity,
          lightIntensity: FINAL_FIREBALL_STATE.lightIntensity,
          r: redColor.r,
          g: redColor.g,
          b: redColor.b,
          duration: 3000,
          easing: "easeInOutQuad",
        },
        "-=1000"
      )
      .add(animProps, {
        rotation: 40, // Aumenta la rotación
        scale: 0.5, // Se encoge
        r: yellowColor.r,
        g: yellowColor.g,
        b: yellowColor.b,
        emissiveIntensity: 3.0,
        lightIntensity: 15.0, // La luz se intensifica
        duration: 2000,
        easing: "easeInQuad",
      })
      .add(animProps, {
        rotation: 100, // Rotación mucho más rápida
        scale: 0.2, // Se encoge aún más
        r: blueColor.r,
        g: blueColor.g,
        b: blueColor.b,
        emissiveIntensity: 6.0,
        lightIntensity: 30.0, // Luz muy intensa
        duration: 2000,
        easing: "easeOutQuad",
      })
      .add(animProps, {
        scale: 0.25,
        emissiveIntensity: 20.0,
        lightIntensity: 50.0,
        bloomIntensity: 5.0, // Intensificamos el Bloom
        godRaysWeight: 0.8, // Hacemos aparecer los GodRays
        duration: 500,
        easing: "easeInExpo",
      })
      .add(
        animProps,
        {
          collapseProgress: 1.0,
          fieldTurbulence: [1.0, 5.0, 1.0],
          duration: 4000,
          easing: "easeOutSine",
        },
        "-=500"
      );
  }, [animProps]);

  const isTuningMode = fireballControls["Activar Modo Tuning"];

  useEffect(() => {
    // Si no estamos en modo tuning, el slider de progreso controla la animación.
    if (!isTuningMode) {
      timelineRef.current.seek(TIMELINE_DURATION * progress);
    }
  }, [progress, isTuningMode]);

  useFrame((state, delta) => {
    if (!dropRef.current || !fireballRef.current) return;

    // MODO TUNING: Control directo con Leva
    if (fireballControls["Activar Modo Tuning"]) {
      const drop = dropRef.current;
      const light = pointLightRef.current;

      (metamorphosisMaterial as any).visible = false;
      (fireballMaterial as any).visible = true;
      fireballMaterial.opacity = 1.0;

      drop.rotation.y += delta * fireballControls.rotationSpeed;

      fireballMaterial.color.set(fireballControls.color);
      fireballMaterial.emissive.copy(fireballMaterial.color);
      fireballMaterial.emissiveIntensity = fireballControls.emissiveIntensity;
      fireballMaterial.alphaTest = fireballControls.alphaTest;

      if (light) {
        light.color.set(fireballControls.color);
        light.intensity = fireballControls.lightIntensity;
      }
    } else {
      // MODO ANIMACIÓN: Tu lógica original, intacta.
      const drop = dropRef.current;
      drop.position.y = animProps.genesisAmplitude + 2.5;
      drop.scale.setScalar(animProps.scale);
      drop.rotation.y = animProps.rotation;

      metamorphosisMaterial.uniforms.uTime.value = state.clock.getElapsedTime();
      metamorphosisMaterial.uniforms.uDisplacementAmount.value =
        animProps.displacement;
      metamorphosisMaterial.uniforms.uOpacity.value = animProps.quantumOpacity;
      (metamorphosisMaterial as any).visible = animProps.quantumOpacity > 0;

      fireballMaterial.opacity = animProps.fireballOpacity;
      fireballMaterial.alphaTest = animProps.fireballAlphaTest;
      fireballMaterial.color.setRGB(animProps.r, animProps.g, animProps.b);
      fireballMaterial.emissive.copy(fireballMaterial.color);
      fireballMaterial.emissiveIntensity = animProps.emissiveIntensity;
      (fireballMaterial as any).visible = animProps.fireballOpacity > 0;
      fireballRef.current.visible = animProps.fireballOpacity > 0;
    }
  });

  return (
    <group>
      <QuantumField
        genesisAmplitude={animProps.genesisAmplitude}
        collapseProgress={animProps.collapseProgress}
        fieldTurbulence={animProps.fieldTurbulence}
      />
      <group ref={dropRef} position={[0, 2.5, 0]}>
        <GenesisDrop
          fireballRef={fireballRef}
          metamorphosisMaterial={metamorphosisMaterial}
          fireballMaterial={fireballMaterial}
        >
          <pointLight ref={pointLightRef} distance={20} />
        </GenesisDrop>
      </group>
      <EffectComposer>
        <Bloom
          intensity={animProps.bloomIntensity} // <-- Prop animada
          luminanceThreshold={0.75}
          luminanceSmoothing={0.8}
          mipmapBlur
        />
        <GodRays
          sun={fireballRef}
          blendFunction={BlendFunction.SCREEN}
          samples={60}
          density={0.97}
          decay={0.97}
          weight={animProps.godRaysWeight}
          exposure={0.35}
          clampMax={1}
          blur={true}
        />
      </EffectComposer>
    </group>
  );
}
