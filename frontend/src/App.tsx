import "./index.css";
import { SimulationViewer } from "./components/SimulationViewer";
import { LandscapeTestScene } from "./components/introduction/LandscapeTestScene";

function App() {
  // Add ?test=landscape to URL to see the channel changing effect
  const urlParams = new URLSearchParams(window.location.search);
  const testMode = urlParams.get("test");

  if (testMode === "landscape") {
    return <LandscapeTestScene />;
  }

  return (
    <div>
      {/* ... otros componentes ... */}
      <h1>Visor del Motor Aletheia</h1>
      <SimulationViewer />
    </div>
  );
}

export default App;
