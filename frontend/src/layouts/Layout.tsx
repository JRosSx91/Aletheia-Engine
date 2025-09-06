import { Outlet } from "react-router-dom";
import styles from "./Layout.module.css";

export const Layout = () => {
  return (
    <main className={styles.layoutContainer}>
      {/* El componente Outlet de react-router-dom renderizará 
          aquí el componente de la ruta activa (nuestras páginas). */}
      <Outlet />
    </main>
  );
};
