// src/router/AppRouter.tsx

import { createBrowserRouter } from "react-router-dom";
import { Layout } from "../layouts/Layout";
import { GraphVisualizerPage } from "../pages/GraphVisualizerPage";
import { IntroductionPage } from "../pages/IntroductionPage";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      // Ruta para la nueva página de introducción.
      // La hemos puesto como la ruta principal por ahora para facilitar el desarrollo.
      {
        index: true, // `index: true` la designa como la página por defecto para "/".
        element: <IntroductionPage />,
      },
      // Ruta para la página original del visualizador de grafos.
      {
        path: "graph", // Se accederá a través de la URL "/graph"
        element: <GraphVisualizerPage />,
      },
    ],
  },
]);
