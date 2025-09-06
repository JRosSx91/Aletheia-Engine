import axios from "axios";
import type { GraphData } from "../types/graph.types";

const API_URL = "http://localhost:3000/graph";

export const fetchGraphData = async (): Promise<GraphData> => {
  const response = await axios.get<GraphData>(API_URL);
  return response.data;
};
