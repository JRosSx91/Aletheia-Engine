import { createBrowserRouter } from "react-router-dom";
import { Layout } from "../layouts/Layout";
import { GraphVisualizerPage } from "../pages/GraphVisualizerPage";
import { AletheiaAvatarPage } from "../pages/AletheiaAvatarPage";
import { LandingPage } from "../pages/LandingPage";

export const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { index: true, element: <LandingPage /> },
      {
        path: "simulation",
        element: <AletheiaAvatarPage />,
      },
      {
        path: "graph",
        element: <GraphVisualizerPage />,
      },
    ],
  },
]);
