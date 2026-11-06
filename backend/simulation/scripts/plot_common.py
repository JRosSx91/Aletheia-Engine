"""Shared plotting helpers. Captions state what the numbers are, not cosmology."""

from __future__ import annotations

from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt
import plotly.express as px
import plotly.io as pio

CAPTION_FITNESS = "Puntuación interna (heurística 0–1). No es probabilidad de habitabilidad."
CAPTION_INVERSE_ALPHA = (
    "Columna histórica `alpha` = 1/α (definición SI). No es la constante de estructura fina."
)
SOURCE_NOTE = "Fuente: CSV Harmonic-137 · semilla documentada HAR-02 cuando se indica · métrica interna"


def ensure_dir(path: Path) -> Path:
    path.mkdir(parents=True, exist_ok=True)
    return path


def save_png(fig: plt.Figure, path: Path) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    fig.savefig(path, dpi=120, bbox_inches="tight")
    plt.close(fig)
    return path


def save_html(fig, path: Path) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    fig.write_html(path, include_plotlyjs="cdn", full_html=True)
    return path


def empty_message(kind: str) -> str:
    return (
        f"El CSV de `{kind}` no contiene filas. No hay figura que generar. "
        "Esto no implica un resultado físico nulo: el filtro de fitness no escribió registros."
    )


def apply_dark_mpl() -> None:
    plt.style.use("dark_background")
