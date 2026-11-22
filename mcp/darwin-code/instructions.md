# DarwinCode MCP Co-Processor Instructions

DarwinCode es un co-procesador neuro-simbólico y motor evolutivo ultra-rápido en Rust (>300.000 evaluaciones por segundo).
Los modelos de lenguaje (LLMs) son arquitectos semánticos excepcionales, pero sufren de alucinaciones en cálculos no lineales de precisión extrema y búsqueda ciega en espacios combinatorios grandes. DarwinCode actúa como el brazo ejecutor y verificador formal.

## Cuándo invocar las herramientas de DarwinCode:

1. **`darwin_synthesize`**:
   - Descubrimiento de leyes físicas o ecuaciones cerradas a partir de telemetría o tablas numéricas.
   - Síntesis de algoritmos matemáticos donde el LLM conoce los casos de prueba (I/O) pero no la función cerrada óptima.
   - Ajuste exacto de curvas no lineales sin regresiones de caja negra (produce expresiones simbólicas interpretables).

2. **`darwin_optimize_code`**:
   - De-bloating y eliminación de introns o código muerto.
   - Simplificación por Navaja de Ockham de expresiones complejas manteniendo 100% de equivalencia semántica.

3. **`darwin_evolve_voxels`**:
   - Generación de planos estructurales 3D o matrices volumétricas de vóxeles para Minecraft, Godot, Blender o visualizadores web.
   - Obtención de coordenadas `[x, y, z, block_type]` con estabilidad de soporte verificada.

4. **`darwin_safe_eval`**:
   - Evaluación segura de código/ASTs con límite estricto de gas, garantizando que ninguna ejecución cuelgue al agente por bucles infinitos o recursiones profundas.

5. **`darwin_get_status`**:
   - Consulta rápida de salud del motor, retos disponibles y capacidades activas.
