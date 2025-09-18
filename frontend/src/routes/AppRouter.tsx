import { createBrowserRouter } from "react-router-dom";
import { Layout } from "../layouts/Layout";
import { GraphVisualizerPage } from "../pages/GraphVisualizerPage";
import { AletheiaAvatarPage } from "../pages/AletheiaAvatarPage"; // <-- Importamos la nueva página

export const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      {
        index: true,
        element: <AletheiaAvatarPage />, // <-- Usamos la nueva página aquí
      },
      {
        path: "graph",
        element: <GraphVisualizerPage />,
      },
    ],
  },
]);
