import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { extend } from "@react-three/fiber";
import { NeuralFluidMaterial } from ".//materials/NeuralFluidMaterial";
import { QuantumGridMaterial } from "./materials/QuantumGridMaterial.tsx";
import { QuantumCellMaterial } from "./materials/QuantumCellMaterial.tsx";
import { RouterProvider } from "react-router-dom";
import { router } from "./routes/AppRouter";
import "./i18n/config.ts";
import { Leva } from "leva";

extend({
  NeuralFluidMaterial,
  QuantumGridMaterial,
  QuantumCellMaterial,
});

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <RouterProvider router={router} />
    <Leva collapsed />
  </StrictMode>
);
