import { useState, useEffect } from "react";
import { fetchGraphData } from "../api/graph.api";
import type { GraphData } from "../types/graph.types";

export const useGraphData = () => {
  const [data, setData] = useState<GraphData | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const getData = async () => {
      try {
        setLoading(true);
        const graphData = await fetchGraphData();
        setData(graphData);
      } catch (err) {
        setError("Graph data could not be fetched");
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    getData();
  }, []);

  return { data, loading, error };
};
