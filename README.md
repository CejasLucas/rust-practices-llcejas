# 🦀 Proyecto Personal en Rust — Prácticas y Algoritmos

![Rust](https://img.shields.io/badge/Rust-1.x-orange?logo=rust)
![Status](https://img.shields.io/badge/status-en%20desarrollo-yellow)

Este es mi repositorio personal de **prácticas en Rust**.  
Está diseñado para estudiantes, autodidactas y entusiastas que quieran fortalecer sus conocimientos mediante implementaciones reales de **algoritmos**, **métodos numéricos** y **concurrencia**, todo dentro de una arquitectura **modular** y **escalable**.

---

## 📂 Contenido Principal

- **🌀 Algoritmos de Ordenamiento** — Implementaciones desde cero: burbuja, quicksort, mergesort, heapsort, selección, inserción, etc.
- **📊 Métodos Numéricos** — Sistemas lineales, búsqueda de raíces, interpolación y regresión.
- **⚙️ Módulos de Utilidades** — Funciones para formatear datos, manejar entrada y optimizar código.
- **🖥 Menús Interactivos** — Para navegar entre categorías de algoritmos y métodos.
- **⚡ Concurrencia y Paralelismo** — Ejemplos con `std::thread`, `Mutex`, `Arc`, y más.

---

## 🧰 Características Técnicas

- **Rust moderno y seguro**: uso de propiedad, borrowing, lifetimes y tipos genéricos.
- **Arquitectura modular**: separación por temas para fácil mantenimiento y escalabilidad.
- **Manejo de errores idiomático** con `Result` y `Option`.
- Amplio uso de **patrones de control** (`match`, `if let`, `loop`, `for`, `while`).
- **Código documentado** y listo para ampliaciones.

---

## 📁 Estructura del Proyecto

```bash
src/
├── main.rs
├── utils/                          # Funciones auxiliares
│   ├── format_arrays.rs
│   ├── format_input.rs
│   ├── format_space.rs
│   └── mod.rs
├── sorting_algorithms/             # Algoritmos de ordenamiento
│   ├── menu.rs
│   ├── sort_bubble.rs
│   ├── sort_heap.rs
│   ├── sort_insertion.rs
│   ├── sort_merge.rs
│   ├── sort_quick.rs
│   ├── sort_selection.rs
│   ├── strategy.rs
│   └── mod.rs
├── numerical_methods/              # Métodos numéricos
│   ├── interpolation_and_regression/
│   ├── linear_systems/
│   ├── root_finding/
│   ├── menu.rs
│   └── mod.rs
└── concurrence/                    # Ejemplos de concurrencia
    ├── example_semaphore_crossing.rs
    ├── example_shared_counter.rs
    ├── example_task_scheduler.rs
    ├── menu.rs
    └── mod.rs
```

## 🚀 Cómo compilar y ejecutar
1. Compilar el proyecto:
```bash
    cargo build
```

2. Ejecutar el programa principal:
```bash
    cargo run
```

3. Ejecutar pruebas:
```bash
    cargo test
```


## 🔧 ¿Por qué modularizar en Rust?
La estructura modular en Rust permite separar lógicamente cada tema o algoritmo. Facilitar el testing y mantenimiento. Reutilizar funciones y estructuras. Mantener un código limpio y escalable. 
Cada módulo tiene su propio archivo mod.rs o está declarado desde otro archivo raíz para permitir visibilidad y acceso desde main.rs.

## 📌 Objetivos del proyecto
Consolidar el aprendizaje de Rust mediante la práctica constante. Reforzar conceptos de algoritmos y estructuras de datos. Aprender a estructurar un proyecto real en Rust desde cero. Promover buenas prácticas y el uso idiomático del lenguaje.

## 🧠 Ideas futuras
- Implementación de estructuras como pilas, colas y árboles
- Pruebas unitarias con `#[test]`
- Documentación con `cargo doc`
- Exploración de crates externos (como `ndarray`, `serde`, etc.)
- Aprender y practicar **concurrencia** y **paralelismo** con:
  - `std::thread` y `std::sync`
  - Canales (`mpsc`)
  - Condiciones de carrera y sincronización
  - Uso de `Arc`, `Mutex`, `RwLock` para manejo seguro de memoria compartida
  - Tareas asincrónicas con `async/await` y `tokio`


## 👨‍💻 Autor: Lucas Leonel Cejas
Técnico Universitario en Programación Informática.
Apasionado por el aprendizaje continuo, la mejora progresiva y el código bien estructurado.

### 📬 ¿Sugerencias o mejoras?
¡Sos bienvenido a abrir un issue o enviar un pull request para colaborar!