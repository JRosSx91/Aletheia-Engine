import { Outlet } from "react-router-dom";
import styles from "./Layout.module.css";

export const Layout = () => {
  return (
    <main className={styles.layoutContainer}>
      <Outlet />
    </main>
  );
};
