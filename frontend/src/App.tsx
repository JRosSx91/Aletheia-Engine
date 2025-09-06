import "./index.css";
import { IntroductionPage } from "./pages/IntroductionPage";
import { LandscapeTestScene } from "./components/introduction/LandscapeTestScene";

function App() {
  // Add ?test=landscape to URL to see the channel changing effect
  const urlParams = new URLSearchParams(window.location.search);
  const testMode = urlParams.get('test');
  
  if (testMode === 'landscape') {
    return <LandscapeTestScene />;
  }
  
  return <IntroductionPage />;
}

export default App;
