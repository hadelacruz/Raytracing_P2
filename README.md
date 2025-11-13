# Minecraft Raytracer

Diorama de Minecraft  utilizando cubos texturizados y conceptos de Raytracer.

## Video 

## https://youtu.be/PtnjEEyfiZ4 ##

## Estructura del Proyecto

```
src/
├── main.rs              # Loop principal y controles
├── framebuffer.rs       # Framebuffer personalizado
├── raytracer.rs         # Motor de raytracing
├── scene.rs             # Generación del mundo
├── camera.rs            # Sistema de cámara FPS
├── math.rs              # Vectores y matemáticas
└── materials/           # Sistema de materiales
    ├── mod.rs           # Definición base
    ├── stone.rs         # Piedra
    ├── water.rs         # Agua con reflejos
    ├── wood.rs          # Madera
    ├── grass.rs         # Hierba
    ├── obsidian.rs      # Obsidiana (portales)
    ├── sun.rs           # Sol emisivo
    ├── fire_particle.rs # Partículas de fuego
    └── leaves.rs        # Hojas
```

## Materiales

El proyecto incluye 8 materiales con propiedades físicas:
- **Stone**: Base del mundo
- **Water**: Agua con reflejos y transparencia
- **Wood**: Madera con textura procedural
- **Grass**: Hierba con variaciones
- **Obsidian**: Bloques oscuros para portales
- **Sun**: Luz emisiva
- **Fire Particle**: Partículas animadas
- **Leaves**: Follaje con textura

## Ejecutar

```bash
cargo run --release
```

## Controles

- **WASD**: Movimiento
- **Flechas**: Arriba/abajo
- **Mouse**: Mirar (clic izquierdo)
- **ESC**: Salir


#### Modo Release (optimización máxima):
```bash
cargo build --release
cargo run --release
```

## 🎨 Materiales Implementados

### Materiales Base (8 materiales únicos):

1. **Piedra Blanca**
   - Albedo: Blanco puro
   - Reflectividad: Baja (0.1)
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

5. **Hojas**
   - Albedo: Verde variado
   - **Textura pixelada** con múltiples tonos
   - Animación de viento y movimiento

6. **Obsidian**
   - Albedo: Negro púrpura
   - **Textura pixelada ultra-fina** 
   - Píxeles pequeños y mayormente oscuros
   - **Torre hueca de obsidiana**

7. **Partículas de Fuego**
   - **Material emisivo** intenso
   - Animación de titilante dinámico
   - Colores naranja-amarillo vibrantes

8. **Sol**
   - **Material emisivo** muy brillante
   - Animación pulsante
   - Fuente principal de luz

## � Mundo Generado

### Estructuras Principales

1. **Plano Base**
   - Tierra con césped
   - Área de agua irregular
   - Diseño estilo Minecraft

2. **Torre de Obsidiana**
   - Base 3x3, 8 bloques de altura
   - **Estructura hueca** (solo bordes exteriores)

3. **Árbol**
   - Tronco de madera 
   - Copa de hojas irregular
   - Diseño orgánico tipo Minecraft

4. **Fogata**
   - Estructura de madera en cruz
   - **Partículas de fuego dinámicas** (esferas emisivas)
   - Efecto tipo antorcha con rayos de fuego

## 🎯 Características Técnicas

### Rendering
- ✅ **Ciclo Día-Noche** con sol dinámico
- ✅ **Noches extremadamente oscuras** (luz ambiente 0.003)
- ✅ **Texturas Animadas** procedurales
- ✅ **Refracción** en agua
- ✅ **Reflexión** en superficies
- ✅ **Materiales Emisivos** con boost nocturno
- ✅ **Cámara FPS** con controles suaves



