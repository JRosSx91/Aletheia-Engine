import "./index.css";
import { SimulationViewer } from "./components/simulation/SimulationViewer";
import { LandscapeTestScene } from "./components/deprecated/introduction/LandscapeTestScene";

function App() {
  const urlParams = new URLSearchParams(window.location.search);
  const testMode = urlParams.get("test");

  if (testMode === "landscape") {
    return <LandscapeTestScene />;
  }

  return (
    <div>
      <SimulationViewer />
    </div>
  );
}

export default App;
