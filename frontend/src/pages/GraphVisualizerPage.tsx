import { useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import { useGraphData } from "../hooks/useGraphData";
import { animate, createScope, stagger } from "animejs";
import { AletheiaScene } from "../components/Aletheia_v1/AletheiaScene";

export function GraphVisualizerPage() {
  const { t } = useTranslation();
  const { data, loading, error } = useGraphData();
  const listRef = useRef<HTMLUListElement>(null);

  useEffect(() => {
    if (data && data.nodes.length > 0 && listRef.current) {
      const scope = createScope({ root: listRef.current }).add(() => {
        animate(".node-item", {
          opacity: [0, 1],
          translateY: [20, 0],
          delay: stagger(75),
          duration: 800,
          easing: "easeOutExpo",
        });
      });

      return () => scope.revert();
    }
  }, [data]);

  if (loading) {
    return <div>{t("loadingMessage")}</div>;
  }

  if (error) {
    return <div>Error: {error}</div>;
  }

  return (
    <div style={{ position: "relative", width: "100vw", height: "100vh" }}>
      <AletheiaScene />

      <div
        style={{
          position: "absolute",
          top: 0,
          left: 0,
          width: "100%",
          height: "100%",
          pointerEvents: "none",
          color: "white",
          padding: "2rem",
        }}
      >
        <h1 style={{ pointerEvents: "auto" }}>{t("appTitle")}</h1>
        <div
          style={{
            position: "absolute",
            bottom: "2rem",
            pointerEvents: "auto",
          }}
        >
          <button>{t("askAletheia")}</button>
        </div>
      </div>
    </div>
  );
}
