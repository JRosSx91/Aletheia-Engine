import { useRef, useState, useEffect, type ReactNode } from "react";
import { useNavigate } from "react-router-dom";
import { Canvas } from "@react-three/fiber";
import { OrbitControls } from "@react-three/drei";
import { FluidCore } from "../components/core/AletheiaAvatar/FluidCore";

const mono = '"Courier New", monospace';
const CYAN = "#00ffff";

const css = `
  @keyframes alFadeUp { from { opacity:0; transform:translateY(24px);} to { opacity:1; transform:none;} }
  @keyframes alGlow {
    0%,100% { text-shadow:0 0 24px rgba(0,255,255,.45);}
    50%     { text-shadow:0 0 46px rgba(0,255,255,.85);}
  }
  @keyframes alBlink { 0%,100%{opacity:.4;} 50%{opacity:1;} }

  .al-fade { opacity:0; animation:alFadeUp .9s ease forwards; }
  .al-blink { animation:alBlink 1.6s infinite; }

  /* Reveal por scroll */
  .al-reveal { opacity:0; transition:opacity .8s ease, transform .8s ease; will-change:opacity,transform; }
  .al-reveal.up    { transform:translateY(48px); }
  .al-reveal.left  { transform:translateX(-64px); }
  .al-reveal.right { transform:translateX(64px); }
  .al-reveal.in    { opacity:1; transform:none; }

  .al-cta { transition:all .25s ease; }
  .al-cta:hover { background:rgba(0,255,255,.14); box-shadow:0 0 34px rgba(0,255,255,.55); transform:translateY(-2px); }

  .al-card { transition:all .3s ease; }
  .al-card.active:hover { border-color:rgba(0,255,255,.85); box-shadow:0 0 34px rgba(0,255,255,.22); transform:translateY(-4px); }
`;

/* Revela su contenido cuando entra en pantalla (una sola vez). */
const Reveal = ({
  children,
  direction = "up",
}: {
  children: ReactNode;
  direction?: "up" | "left" | "right";
}) => {
  const ref = useRef<HTMLDivElement>(null);
  const [visible, setVisible] = useState(false);

  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    const obs = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setVisible(true);
          obs.unobserve(el);
        }
      },
      { threshold: 0.2 }
    );
    obs.observe(el);
    return () => obs.disconnect();
  }, []);

  return (
    <div ref={ref} className={`al-reveal ${direction} ${visible ? "in" : ""}`}>
      {children}
    </div>
  );
};

const Slide = ({
  kicker,
  title,
  children,
  direction = "up",
}: {
  kicker?: string;
  title: string;
  children: ReactNode;
  direction?: "up" | "left" | "right";
}) => (
  <Reveal direction={direction}>
    <div style={{ maxWidth: "720px", margin: "0 auto", padding: "12px 0" }}>
      {kicker && (
        <div style={{ fontSize: "12px", letterSpacing: "3px", color: CYAN, marginBottom: "14px" }}>
          {kicker}
        </div>
      )}
      <h2 style={{ fontSize: "clamp(24px, 5vw, 40px)", margin: "0 0 18px", color: "#eafdff", letterSpacing: "1px" }}>
        {title}
      </h2>
      <div style={{ fontSize: "clamp(14px, 2vw, 17px)", lineHeight: 1.8, color: "#a8cccd" }}>
        {children}
      </div>
    </div>
  </Reveal>
);

const Chip = ({ value, label, color }: { value: string; label: string; color: string }) => (
  <div style={{ textAlign: "center" }}>
    <div
      style={{
        width: "64px",
        height: "64px",
        border: `1px solid ${color}`,
        borderRadius: "8px",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        fontSize: "22px",
        color,
        boxShadow: `0 0 18px ${color}55`,
        background: "rgba(0,0,0,.5)",
      }}
    >
      {value}
    </div>
    <div style={{ marginTop: "8px", fontSize: "10px", letterSpacing: "1px", color: "#7fa3a4" }}>
      {label}
    </div>
  </div>
);

/* Una fila de la escalera del fractal */
const Level = ({ name, role }: { name: string; role: string }) => (
  <div
    style={{
      display: "flex",
      alignItems: "baseline",
      gap: "14px",
      padding: "10px 0",
      borderBottom: "1px solid #16282a",
    }}
  >
    <span style={{ color: CYAN, fontSize: "14px", letterSpacing: "1px", minWidth: "140px" }}>{name}</span>
    <span style={{ color: "#8aa3a4", fontSize: "13px" }}>{role}</span>
  </div>
);

export const LandingPage = () => {
  const navigate = useNavigate();

  const sectionStyle: React.CSSProperties = {
    minHeight: "70vh",
    display: "flex",
    flexDirection: "column",
    justifyContent: "center",
    padding: "80px 24px",
  };

  return (
    <div
      style={{
        position: "relative",
        minHeight: "100vh",
        height: "auto",
        width: "100%",
        background: "radial-gradient(ellipse at 50% 0%, #001a22 0%, #000511 55%, #000 100%)",
        color: "#cfeff0",
        fontFamily: mono,
        overflowX: "hidden",
        overflowY: "visible",
      }}
    >
      <style>{css}</style>

      {/* Viñeta de fondo */}
      <div
        style={{
          position: "fixed",
          inset: 0,
          zIndex: 1,
          pointerEvents: "none",
          background: "radial-gradient(ellipse at center, transparent 30%, rgba(0,5,17,.82) 100%)",
        }}
      />

      <main style={{ position: "relative", zIndex: 2 }}>
        {/* CABECERA / AVATAR */}
        <header
          style={{
            minHeight: "100vh",
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            alignItems: "center",
            textAlign: "center",
            padding: "0 24px",
          }}
        >
          <div
            className="al-fade"
            style={{
              animationDelay: ".1s",
              color: CYAN,
              fontSize: "13px",
              letterSpacing: "3px",
              border: `1px solid ${CYAN}40`,
              borderRadius: "6px",
              padding: "8px 16px",
              background: "rgba(0,0,0,.55)",
              marginBottom: "32px",
            }}
          >
            ◉ ALETHEIA NEURAL CORE — <span className="al-blink">[ONLINE]</span>
          </div>

          {/* La esfera neuronal, acotada y por encima del título */}
          <div className="al-fade" style={{ animationDelay: ".15s", width: "min(340px, 70vw)", height: "min(340px, 70vw)", marginBottom: "8px" }}>
            <Canvas camera={{ position: [0, 0, 4.2], fov: 50 }} gl={{ alpha: true }} style={{ background: "transparent" }}>
              <ambientLight intensity={0.2} />
              <pointLight position={[5, 5, 5]} intensity={1.2} color="#00ffff" />
              <FluidCore speaking={false} thinking={true} scale={1.4} isStatic={false} />
              <OrbitControls autoRotate autoRotateSpeed={0.4} enableZoom={false} enablePan={false} enableRotate={false} />
            </Canvas>
          </div>

          <h1
            className="al-fade"
            style={{
              animationDelay: ".25s",
              fontSize: "clamp(48px, 13vw, 140px)",
              margin: 0,
              letterSpacing: "8px",
              color: "#eafdff",
              animation: "alFadeUp .9s ease forwards, alGlow 4s ease-in-out infinite 1s",
            }}
          >
            ALETHEIA
          </h1>

          <p className="al-fade" style={{ animationDelay: ".4s", fontSize: "clamp(14px,2.4vw,20px)", color: "#8fcdd0", letterSpacing: "2px", margin: "16px 0 0" }}>
            ἀλήθεια — el desvelamiento de lo real
          </p>

          <p className="al-fade" style={{ animationDelay: ".55s", maxWidth: "620px", fontSize: "clamp(13px,2vw,16px)", lineHeight: 1.7, color: "#9fc4c6", marginTop: "28px" }}>
            Un motor para simular cómo la información construye la realidad —{" "}
            <strong style={{ color: CYAN }}>del agujero negro que engendra universos al bit que cruza un campo</strong>.
          </p>

          <div className="al-fade al-blink" style={{ animationDelay: ".7s", marginTop: "56px", fontSize: "12px", color: `${CYAN}cc`, letterSpacing: "2px" }}>
            ▼ DESPLAZA PARA EXPLORAR
          </div>
        </header>

        {/* 01 — EL ORIGEN */}
        <section style={sectionStyle}>
          <Slide kicker="01 · EL ORIGEN" title="La Biblioteca Cósmica">
            <p>
              Aletheia empezó como <strong style={{ color: CYAN }}>Harmonic-137</strong>. La imagen: una
              biblioteca de cinco dimensiones (el <em>bulk</em>). En sus estantes, libros de 4D; cada
              libro contiene la historia completa de un universo. El lomo de cada libro lleva un número —
              el denominador de la constante de estructura fina, <strong>α = 1/X</strong>. El{" "}
              <strong style={{ color: CYAN }}>libro 137</strong> es el nuestro.
            </p>
            <p style={{ marginTop: "16px" }}>
              Recorríamos los estantes buscando patrones: ¿los números primos engendran universos más
              viables? ¿sirve cambiar las constantes si conservas la misma proporción? ¿hay más "números
              mágicos" además del 137?
            </p>
          </Slide>
        </section>

        {/* 02 — LA REPRODUCCIÓN */}
        <section style={sectionStyle}>
          <Slide kicker="02 · LA REPRODUCCIÓN" title="Universos que se reproducen" direction="left">
            <p>
              ¿Por qué <strong style={{ color: CYAN }}>1/137</strong> y no cualquier otro número?
              Hipótesis: los universos se reproducen. Cada <strong>agujero negro</strong> es el parto de
              un universo hijo, que hereda las leyes del padre — con mutaciones.
            </p>
            <p style={{ marginTop: "16px" }}>
              Los universos cuyas leyes favorecen estrellas masivas colapsan en más agujeros negros, y
              dejan más descendencia. Tras innumerables generaciones, las constantes que sobreviven no son
              arbitrarias: son las que <strong style={{ color: CYAN }}>mejor se reproducen</strong>.
              Selección natural, a escala cósmica.
            </p>
          </Slide>
        </section>

        {/* 03 — EL FRACTAL */}
        <section style={sectionStyle}>
          <Slide kicker="03 · EL FRACTAL" title="La información sube de nivel" direction="right">
            <p>
              La herencia no termina en la física. La información se propaga por niveles, y cada salto es
              un filtro más estrecho que el anterior:
            </p>
            <div style={{ margin: "28px 0 8px" }}>
              <Level name="AGUJEROS NEGROS" role="transmiten las leyes físicas al universo hijo" />
              <Level name="ESTRELLAS" role="forjan elementos pesados · transmisión química" />
              <Level name="PLANETAS" role="mundos en zona habitable · el umbral de lo biológico" />
              <Level name="VIDA" role="ADN, código heredable · transmisión biológica" />
              <Level name="CONSCIENCIA" role="lenguaje y cultura · información sin genes" />
            </div>
            <p style={{ marginTop: "16px" }}>
              Aletheia modela ese fractal: cómo la información crea realidad, capa sobre capa.
            </p>
          </Slide>
        </section>

        {/* 04 — EL SUSTRATO (BINARIO) */}
        <section style={sectionStyle}>
          <Slide kicker="04 · EL SUSTRATO" title="Todo empieza en un bit" direction="left">
            <p>¿De qué está hecha la información, en el fondo? La respuesta arranca <strong style={{ color: CYAN }}>binaria</strong>: un campo de bits.</p>
            <div style={{ display: "flex", gap: "32px", justifyContent: "center", margin: "30px 0" }}>
              <Chip value="1" label="ENERGÍA" color="#ff4444" />
              <Chip value="0" label="VACÍO" color="#778888" />
            </div>
            <p>
              Las <strong style={{ color: CYAN }}>Premisas 1 y 2</strong> trabajan con este sistema
              binario: el agujero negro vierte bits <code>1</code> en un mar de <code>0</code>s, y
              observamos cómo viajan.
            </p>
          </Slide>
        </section>

        {/* 05 — LA CAPA PROFUNDA (TERNARIO) */}
        <section style={sectionStyle}>
          <Slide kicker="05 · LA CAPA PROFUNDA" title="Hacia un campo ternario" direction="right">
            <p>
              El binario es el comienzo. La evolución natural del modelo es{" "}
              <strong style={{ color: CYAN }}>ternaria</strong> — como una cuerda que vibra a ambos lados
              de su línea media:
            </p>
            <div style={{ display: "flex", gap: "28px", justifyContent: "center", margin: "30px 0", flexWrap: "wrap" }}>
              <Chip value="+1" label="ENERGÍA" color="#ff4444" />
              <Chip value="0" label="EQUILIBRIO" color="#778888" />
              <Chip value="−1" label="DEUDA · ANTIMATERIA" color="#4488ff" />
            </div>
            <p>
              Un +1 y un −1 se aniquilan a 0; la suma del universo se conserva. Pero esto todavía{" "}
              <strong style={{ color: CYAN }}>no vive en los grids</strong>: es el siguiente sustrato, aún
              sobre el papel.
            </p>
          </Slide>
        </section>

        {/* 06 — ARQUITECTURA */}
        <section style={sectionStyle}>
          <Slide kicker="06 · BAJO EL CAPÓ" title="Cómo está construido">
            <p>
              Un <strong style={{ color: CYAN }}>autómata celular en Rust</strong> calcula la evolución
              del campo en cada tick. Un canal <strong style={{ color: CYAN }}>WebSocket</strong>{" "}
              transmite, en tiempo real, solo las celdas que cambian. Una escena en{" "}
              <strong style={{ color: CYAN }}>React Three Fiber</strong> las renderiza en 3D, aquí mismo.
            </p>
          </Slide>
        </section>

        {/* PREMISAS */}
        <section style={{ ...sectionStyle, minHeight: "auto", paddingBottom: "120px" }}>
          <Reveal>
            <h2 style={{ fontSize: "13px", letterSpacing: "4px", color: "#6fb3b6", textAlign: "center", marginBottom: "12px" }}>
              — ELIGE UN UNIVERSO —
            </h2>
            <p style={{ textAlign: "center", color: "#7fa3a4", fontSize: "14px", marginBottom: "48px" }}>
              Cada premisa es un experimento binario. Entra y vierte el primer bit.
            </p>

            <div style={{ display: "flex", gap: "24px", flexWrap: "wrap", justifyContent: "center" }}>
              {/* PREMISA 1 */}
              <article
                className="al-card active"
                style={{ flex: "1 1 380px", maxWidth: "440px", background: "rgba(0,10,18,.78)", backdropFilter: "blur(6px)", border: `1px solid ${CYAN}55`, borderRadius: "10px", padding: "28px" }}
              >
                <div style={{ fontSize: "11px", letterSpacing: "2px", color: "#00ff88", marginBottom: "14px" }}>
                  PREMISA 01 · BINARIO · [DISPONIBLE]
                </div>
                <h3 style={{ margin: "0 0 14px", fontSize: "24px", color: "#eafdff" }}>Sistema Cerrado</h3>
                <p style={{ fontSize: "14px", lineHeight: 1.7, color: "#9fc4c6", minHeight: "108px" }}>
                  Un universo finito que no se expande. El agujero negro de otro universo vierte bits{" "}
                  <code>1</code> en un sustrato de <code>0</code>s. ¿Cómo se transmite la información
                  cuando el espacio no crece?
                </p>
                <button
                  className="al-cta"
                  onClick={() => navigate("/simulation")}
                  style={{ marginTop: "20px", width: "100%", padding: "14px", background: "rgba(0,0,0,.6)", border: `1px solid ${CYAN}`, color: CYAN, fontFamily: mono, fontSize: "14px", letterSpacing: "1px", cursor: "pointer", borderRadius: "6px" }}
                >
                  ENTRAR A LA PREMISA 01 ▶
                </button>
              </article>

              {/* PREMISA 2 */}
              <article
                className="al-card"
                style={{ flex: "1 1 380px", maxWidth: "440px", background: "rgba(0,5,10,.6)", border: "1px solid #2a3a3c", borderRadius: "10px", padding: "28px", opacity: 0.6 }}
              >
                <div style={{ fontSize: "11px", letterSpacing: "2px", color: "#c8922e", marginBottom: "14px" }}>
                  PREMISA 02 · BINARIO · [EN CONSTRUCCIÓN]
                </div>
                <h3 style={{ margin: "0 0 14px", fontSize: "24px", color: "#c3d6d7" }}>Sistema Abierto</h3>
                <p style={{ fontSize: "14px", lineHeight: 1.7, color: "#8aa3a4", minHeight: "108px" }}>
                  El mismo campo binario, pero el espacio se expande mientras recibe bits. Ahora la
                  velocidad de transmisión compite con la tasa de expansión. ¿Se comporta igual?
                </p>
                <button
                  disabled
                  style={{ marginTop: "20px", width: "100%", padding: "14px", background: "transparent", border: "1px solid #2a3a3c", color: "#5a6a6c", fontFamily: mono, fontSize: "14px", letterSpacing: "1px", cursor: "not-allowed", borderRadius: "6px" }}
                >
                  PRÓXIMAMENTE
                </button>
              </article>
            </div>

            <footer style={{ marginTop: "80px", textAlign: "center", fontSize: "11px", letterSpacing: "2px", color: "#4f6466" }}>
              DEL AGUJERO NEGRO AL BIT · HARMONIC-137 → ALETHEIA · RUST + R3F
            </footer>
          </Reveal>
        </section>
      </main>
    </div>
  );
};

export default LandingPage;