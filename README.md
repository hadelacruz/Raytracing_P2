# Minecraft Raytracer

**Autor:** Humberto Alexander de la Cruz  

## 📖 Descripción del Proyecto

Este proyecto implementa un **raytracer en tiempo real** inspirado en Minecraft, desarrollado completamente en **Rust**. El programa genera un mundo 3D pixelado que simula el estilo visual característico de Minecraft, incluyendo materiales, texturas animadas, ciclos día-noche, efectos de partículas y mucho más.

El raytracer utiliza técnicas avanzadas de renderizado como **reflexión**, **refracción**, **materiales emisivos**, **sombreado dinámico** y **paralelización multi-core** para lograr un rendimiento óptimo y efectos visuales impresionantes.

## 🌟 Video Demostración

**https://youtu.be/O6nXA2zB95E**


## 🚀 Cómo Ejecutar el Proyecto

### Prerrequisitos
- **Rust** (versión 1.70 o superior)
- **Cargo** (incluido con Rust)
- Sistema operativo compatible (Windows, macOS, Linux)

### Compilación y Ejecución

#### Modo Debug (desarrollo):
```bash
cargo build
cargo run
```

#### Modo Release (optimización máxima):
```bash
cargo build --release
cargo run --release
```

**Recomendación:** Usar siempre el modo `--release` para obtener el mejor rendimiento y FPS.

### Controles

| Tecla/Acción | Función |
|--------------|---------|
| **W** | Mover cámara hacia adelante |
| **S** | Mover cámara hacia atrás |
| **A** | Mover cámara hacia la izquierda |
| **D** | Mover cámara hacia la derecha |
| **Flecha ↑** | Subir cámara |
| **Flecha ↓** | Bajar cámara |
| **Mouse + Click izquierdo** | Rotar cámara (mirar alrededor) |
| **ESC** | Salir del programa |

## 🎨 Materiales Implementados

### Materiales Base (5 materiales únicos):

1. **Piedra Blanca**
   - Albedo: Blanco puro
   - Reflectividad: Baja (0.1)
   - Transparencia: 0%
   - Textura: Sólida

2. **Agua**
   - Albedo: Azul cyan
   - Reflectividad: Alta (0.6)
   - Transparencia: 80%
   - Índice refractivo: 1.33
   - **Textura animada** con píxeles verticales

3. **Madera**
   - Albedo: Café oscuro
   - Reflectividad: Muy baja (0.02)
   - **Textura pixelada vertical** con vetas beige
   - Patrón estilo Minecraft auténtico

4. **Césped**
   - Albedo: Verde natural
   - Reflectividad: Mínima (0.01)
   - **Textura animada** con variaciones de verde
   - Efecto de viento sutil

5. **Hojas**
   - Albedo: Verde variado
   - **Textura pixelada** con múltiples tonos
   - Animación de viento y movimiento

### Materiales Especiales:

6. **Obsidian**
   - Albedo: Negro púrpura
   - **Textura pixelada ultra-fina** 
   - Píxeles pequeños y mayormente oscuros

7. **Portal**
   - Albedo: Púrpura
   - **Material emisivo** con animación pulsante
   - Efectos de transparencia y refracción

8. **Partículas de Fuego**
   - **Material emisivo** intenso
   - Animación de titilante dinámico
   - Colores naranja-amarillo vibrantes

9. **Sol**
   - **Material emisivo** muy brillante
   - Animación pulsante
   - Fuente principal de luz

## 🎯 Puntos de Rúbrica Implementados

- ✅ **Rendimiento (FPS)**
- ✅ **Ciclo Día-Noche con Sol**
- ✅ **Texturas Animadas**
- ✅ **Paralelización (Threads)**
- ✅ **Cámara y Rotación**
- ✅ **Materiales Diversos**
- ✅ **Refracción**
- ✅ **Efectos de Portal**
- ✅ **Reflexión**
- ✅ **Materiales Emisivos (Antorchas/Fogata)**

## 🛠️ Tecnologías Utilizadas

- **Lenguaje**: Rust (2021 Edition)
- **Paralelización**: Rayon
- **Renderizado**: Pixels + Winit
- **Matemáticas**: Implementación propia de vectores y rayos
- **Optimización**: Profile release con LTO

## 📁 Estructura del Código

```
src/
├── main.rs          # Punto de entrada y configuración
├── app.rs           # Lógica principal de la aplicación
├── render.rs        # Motor de raytracing paralelo
├── scene.rs         # Generación del mundo y objetos
├── materials.rs     # Sistema de materiales y texturas
├── camera.rs        # Sistema de cámara y controles
├── input.rs         # Manejo de entrada (teclado/mouse)
└── math.rs          # Matemáticas vectoriales y rayos
```

## 🚀 Optimizaciones de Rendimiento

- **Renderizado paralelo** con división automática de trabajo
- **Compilación optimizada** con LTO y codegen-units=1
- **Chunking inteligente** para balance de carga
- **Gamma correction** para mejor calidad visual
- **Clamp de colores** para prevenir overflow
