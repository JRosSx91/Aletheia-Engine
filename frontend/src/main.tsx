import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { extend } from "@react-three/fiber";
import { HolographicMaterial } from "./materials/HolographicMaterial";
import { ParticleGalaxyMaterial } from "./components/Aletheia_v1/ParticlesGalaxy.tsx";
import { FractalEnergyMaterial } from "./components/Aletheia_v1/EnergyContainer.tsx";
import { QuantumFieldMaterial } from "./components/Aletheia_v1/QuantumField.tsx";
import { MetamorphosisMaterial } from "./components/Aletheia_v1/materials/MetamorphosisMaterial.tsx";
import { LandscapeMaterial } from "./components/introduction/materials/LandscapeMaterial";
import "./index.css";
import { RouterProvider } from "react-router-dom";
import { router } from "./routes/AppRouter";
import "./i18n/config.ts";
import { Leva } from "leva";

extend({
  HolographicMaterial,
  ParticleGalaxyMaterial,
  FractalEnergyMaterial,
  QuantumFieldMaterial,
  MetamorphosisMaterial,
  LandscapeMaterial,
});

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <RouterProvider router={router} />
    <Leva collapsed />
  </StrictMode>
);
