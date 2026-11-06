import { useLayoutEffect, useRef } from "react";
import { Outlet, useLocation } from "react-router-dom";
import styles from "./Layout.module.css";

export const Layout = () => {
  const { pathname } = useLocation();
  const mainRef = useRef<HTMLElement>(null);

  useLayoutEffect(() => {
    window.scrollTo(0, 0);
    document.documentElement.scrollTop = 0;
    document.body.scrollTop = 0;
    document.getElementById("root")?.scrollTo(0, 0);
    mainRef.current?.scrollTo(0, 0);
  }, [pathname]);

  return (
    <main ref={mainRef} className={styles.layoutContainer}>
      <Outlet />
    </main>
  );
};
