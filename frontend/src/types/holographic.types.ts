import type { ReactNode } from "react";

export interface HolographicMaterialProps {
  fresnelAmount?: number;
  fresnelOpacity?: number;
  scanlineSize?: number;
  hologramBrightness?: number;
  signalSpeed?: number;
  hologramColor?: string;
  enableBlinking?: boolean;
  blinkFresnelOnly?: boolean;
  enableAdditive?: boolean;
  hologramOpacity?: number;
  side?: "FrontSide" | "BackSide" | "DoubleSide";
  children?: ReactNode;
}
