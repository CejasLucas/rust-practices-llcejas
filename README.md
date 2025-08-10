# 🦀 Proyecto Personal en Rust — Prácticas y Algoritmos

![Rust](https://img.shields.io/badge/Rust-1.x-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-green)
![Status](https://img.shields.io/badge/status-en%20desarrollo-yellow)

Bienvenido a mi repositorio personal de prácticas en **Rust**.  
Este proyecto está pensado para estudiantes, autodidactas y entusiastas que deseen fortalecer su comprensión del lenguaje a través de implementaciones reales de **algoritmos** y **métodos numéricos**, con una estructura modular y escalable.

---

## 📂 Contenido del Repositorio

- **Algoritmos de Ordenamiento** — Implementados desde cero, con diferentes estrategias (burbuja, quicksort, mergesort, heapsort, etc.).
- **Métodos Numéricos** — Incluye métodos para sistemas lineales, búsqueda de raíces e interpolación/regresión.
- **Módulos de Utilidades** — Funciones para manejo de entrada, formateo y reutilización de código.
- **Menús interactivos** para navegar entre las distintas categorías de algoritmos y métodos.

---

## 🧰 Características Técnicas

- **Rust moderno y seguro**: aprovechando propiedad, borrowing y tipos genéricos.
- **Organización modular**: cada carpeta corresponde a un tema principal.
- **Manejo de errores idiomático** con `Result` y `Option`.
- Uso de `match`, `if let`, bucles (`loop`, `for`, `while`) y patrones.
- Código documentado y preparado para ampliaciones futuras.

---

## 📁 Estructura del Proyecto

```bash
src/
├── main.rs
├── utils/                          # Funciones auxiliares
│   ├── format_arrays.rs
│   ├── format_input.rs
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
└── numerical_methods/              # Métodos numéricos
    ├── interpolation_and_regression/
    │   ├── interpolation_lagrange.rs
    │   ├── interpolation_newton.rs
    │   ├── strategy.rs
    │   └── mod.rs
    ├── linear_systems/
    │   ├── method_factorization.rs
    │   ├── method_gauss.rs
    │   ├── method_jacobi.rs
    │   ├── strategy.rs
    │   └── mod.rs
    ├── root_finding/
    │   ├── method_bisection.rs
    │   ├── method_newton_raphson.rs
    │   ├── method_secant.rs
    │   ├── strategy.rs
    │   └── mod.rs
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