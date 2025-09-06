# scripts/analyze_champion.py

import json
import pandas as pd

try:
    with open('champion_genome.json', 'r') as f:
        champion_data = json.load(f)
    
    with open('our_universe.json', 'r') as f:
        our_data = json.load(f)
        
except FileNotFoundError as e:
    print(f"Error: No se pudo encontrar el archivo {e.filename}.")
    print("Asegúrate de tener 'champion_genome.json' y 'our_universe.json' en la raíz del proyecto.")
    exit()

# Convertimos los datos a Series de Pandas para facilitar la comparación
champion_series = pd.Series(champion_data, name='Champion_Universe')
our_series = pd.Series(our_data, name='Our_Universe')

# Creamos un DataFrame para la comparación
df_comparison = pd.concat([champion_series, our_series], axis=1)

# Calculamos la diferencia porcentual
df_comparison['Difference_%'] = ((champion_series - our_series) / our_series) * 100

# Formato para la visualización
pd.options.display.float_format = '{:,.6e}'.format

print("--- ANÁLISIS COMPARATIVO DEL GENOMA CÓSMICO ---")
print(df_comparison)

print("\n--- CONCLUSIONES PRELIMINARES ---")
print("Observa la increíble similitud en 'alpha_s' y la jerarquía de masas de los quarks.")
print("Sin embargo, hay diferencias drásticas en otras constantes, indicando una 'solución' alternativa y viable.")