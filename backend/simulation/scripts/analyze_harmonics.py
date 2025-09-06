# scripts/analyze_harmonics.py

import pandas as pd
import plotly.express as px

try:
    df = pd.read_csv('harmonic_results.csv')
except FileNotFoundError:
    print("Error: No se pudo encontrar 'harmonic_results.csv'.")
    print("Asegúrate de haber ejecutado el modo 'harmonic' primero.")
    exit()

print("--- ANÁLISIS ESTADÍSTICO POR ARMÓNICO ---")

# Agrupamos por armónico y calculamos las métricas clave
summary = df.groupby('harmonic_n')['fitness'].agg(['count', 'mean', 'max'])

# Ordenamos por el fitness máximo encontrado para ver a los campeones
summary = summary.sort_values(by='max', ascending=False)

pd.options.display.float_format = '{:.6f}'.format
print(summary)

print("\n--- VISUALIZANDO LA DISTRIBUCIÓN DEL FITNESS ---")

# Un box plot es perfecto para comparar las distribuciones de fitness
fig = px.box(
    df, 
    x='harmonic_n', 
    y='fitness',
    title='Distribución del Fitness por Armónico',
    labels={'harmonic_n': 'Armónico (N)', 'fitness': 'Fitness de los Universos Viables'},
    points="all" # Muestra todos los puntos de datos
)

fig.update_layout(template="plotly_dark")
fig.show()