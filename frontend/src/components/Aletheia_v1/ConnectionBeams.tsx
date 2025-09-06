import { useMemo } from "react";
import * as THREE from "three";
import { useFrame } from "@react-three/fiber";
import { useControls, folder } from "leva";
import { LightningStorm } from "../../lib/three/LightningStorm.js";
import type { RayParameters } from "../../lib/three/LightningStrike.js";

export function ConnectionBeams({ radius = 1.5 }) {
  const controls = useControls("Configuración de Rayos", {
    Tormenta: folder({
      maxLightnings: {
        value: 6,
        min: 1,
        max: 20,
        step: 1,
        label: "Número de Rayos",
      },
      lightningMinPeriod: {
        value: 0.1,
        min: 0.01,
        max: 5,
        label: "Periodo Mín.",
      },
      lightningMaxPeriod: {
        value: 0.5,
        min: 0.01,
        max: 10,
        label: "Periodo Máx.",
      },
      lightningMinDuration: {
        value: 0.5,
        min: 0.1,
        max: 5,
        label: "Duración Mín.",
      },
      lightningMaxDuration: {
        value: 1.2,
        min: 0.1,
        max: 10,
        label: "Duración Máx.",
      },
    }),
    Estetica: folder({
      straightness: { value: 0.7, min: 0, max: 1, label: "Rectitud" },
      roughness: { value: 0.85, min: 0, max: 1, label: "Aspereza" },
      radius0: { value: 0.1, min: 0.01, max: 1, label: "Radio Tronco" },
      radius1: { value: 0.1, min: 0.01, max: 1, label: "Radio Punta" },
      radius0Factor: { value: 0.5, min: 0, max: 1, label: "Factor Radio Rama" },
      timeScale: { value: 0.25, min: 0, max: 2, label: "Velocidad" },
    }),
    Estructura: folder({
      ramification: {
        value: 5,
        min: 1,
        max: 15,
        step: 1,
        label: "Ramificación",
      },
      maxSubrayRecursion: {
        value: 4,
        min: 1,
        max: 7,
        step: 1,
        label: "Nivel Máx. Recursión",
      },
      recursionProbability: {
        value: 0.8,
        min: 0,
        max: 1,
        label: "Prob. Recursión",
      },
    }),
  });

  const storm = useMemo(() => {
    const lightningParams: RayParameters = {
      ...controls,
      minRadius: 0.1,
      maxIterations: 7,
      isEternal: false,
    };

    return new LightningStorm({
      maxLightnings: controls.maxLightnings,
      lightningMinPeriod: controls.lightningMinPeriod,
      lightningMaxPeriod: controls.lightningMaxPeriod,
      lightningMinDuration: controls.lightningMinDuration,
      lightningMaxDuration: controls.lightningMaxDuration,

      lightningParameters: lightningParams,

      lightningMaterial: new THREE.MeshBasicMaterial({
        color: 0xffffff,
        toneMapped: false,
      }),

      onRayPosition: (source: THREE.Vector3, dest: THREE.Vector3) => {
        source.set(0, 0, 0);
        dest
          .set(Math.random() - 0.5, Math.random() - 0.5, Math.random() - 0.5)
          .normalize()
          .multiplyScalar(radius);
      },
    });
  }, [radius, controls]);

  useFrame((state) => {
    storm.update(state.clock.elapsedTime);
  });

  return <primitive object={storm} />;
}
