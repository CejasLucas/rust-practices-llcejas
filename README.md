# 🦀 Proyecto personal en Rust

![Rust](https://img.shields.io/badge/Rust-1.x-orange?logo=rust)

¡Bienvenido! Este es mi repositorio personal donde pongo en práctica mis habilidades como programador.  

Este proyecto está orientado a estudiantes y autodidactas que buscan fortalecer su comprensión de **Rust**, un lenguaje moderno, seguro y eficiente. Aquí encontrarás implementaciones prácticas de algoritmos y métodos numéricos, organizados de manera modular y escalable, siguiendo buenas prácticas de programación.

---

## 📂 Contenido del repositorio

- Algoritmos de ordenamiento implementados desde cero
- Métodos numéricos clásicos (bisección, Newton-Raphson, secante)
- Módulos de utilidades para facilitar el reuso de funciones

Todo está estructurado en módulos y submódulos para facilitar la lectura, reutilización y mantenimiento del código.

---

## 🧰 Temas y herramientas

### Lenguaje y características de Rust:

- Propiedad y **borrowing**
- Tipos genéricos y funciones parametrizadas
- Uso de **`Result`** y manejo de errores
- Organización modular con `mod.rs`
- `match`, `if let`, `loop`, `for`, `while`
- Crates estándar (`std`) y estructura de proyectos con `Cargo`

---

## 📁 Estructura del Proyecto

Cada carpeta dentro de `src/` representa un tema específico del aprendizaje.

```bash
.
├── src/
│   ├── numerical_methods/      # Métodos numéricos clásicos
│   │   ├── method_bisection.rs
│   │   ├── method_newton_raphson.rs
│   │   ├── method_secant.rs
│   │   ├── menu.rs
│   │   └── mod.rs
│   ├── sorting_algorithms/     # Algoritmos de ordenamiento
│   │   ├── sort_bubble.rs
│   │   ├── sort_heap.rs
│   │   ├── sort_insertion.rs
│   │   ├── sort_merge.rs
│   │   ├── sort_quick.rs
│   │   ├── sort_selection.rs
│   │   ├── menu.rs
│   │   └── mod.rs
│   ├── utils/                  # Funciones de utilidad
│   │   ├── assistant.rs
│   │   └── mod.rs
│   └── main.rs                 # Punto de entrada del proyecto
├── Cargo.toml                  # Metadata del proyecto y dependencias
├── Cargo.lock                  # Versiones exactas usadas
└── .gitignore

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