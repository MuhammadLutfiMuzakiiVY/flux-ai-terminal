# <img src="assets/images/logo.png" width="48" height="48" style="vertical-align:middle"> Flux AI Terminal (Documentación en Español)
### *La Estación de Trabajo Móvil Nativa de Rust Definitiva*

![Flux AI Terminal Banner](assets/images/banner.png)

---

## 🌍 Documentación Global
- [English (Main)](README.md)
- [中文 (Chinese)](README.zh.md)
- [日本語 (Japanese)](README.jp.md)
- [한국어 (Korean)](README.kr.md)
- [العربية (Arabic)](README.ar.md)
- [Español (Spanish)](README.es.md)

---

## 🎯 Visión del Proyecto
**Flux AI Terminal** no es solo un emulador; es un entorno de ejecución nativo construido desde cero para empoderar a los desarrolladores en movimiento. Al aprovechar el rendimiento de **Rust** y la inteligencia de la **IA local**, Flux proporciona una estación de trabajo Linux de latencia cero, segura y altamente extensible que cabe en su bolsillo.

### 🌟 ¿Por qué Flux?
1. **Potencia de Escritorio en el Móvil:** Ejecute compiladores, herramientas de construcción y servidores web de forma nativa.
2. **Inteligencia Fuera de Línea:** IA que funciona sin conexión a Internet, preservando su privacidad.
3. **Seguridad Reforzada:** Su código fuente está protegido por cifrado biométrico y un cortafuegos de comandos en tiempo real.

---

## 🏗️ Arquitectura Profunda

### 🦀 El Motor Core de Rust
El corazón de Flux es un núcleo asíncrono y no bloqueante implementado en Rust. Utiliza el tiempo de ejecución `tokio` para gestionar miles de tareas concurrentes con una sobrecarga mínima de CPU.

#### Componentes:
- **Emulación de PTY:** Un pseudoterminal completo compatible con Xterm para renderizar aplicaciones TUI complejas como NeoVim y htop.
- **VFS (Sistema de Archivos Virtual):** Un sandbox al estilo OverlayFS que proporciona un rootfs completo de Ubuntu sin modificar su sistema host.
- **Gestor de Paquetes:** Una implementación nativa de `dpkg` y `apt` para la gestión atómica de paquetes.

### 🧠 Motor de IA RAG
Flux incluye un motor de **Generación Aumentada por Recuperación (RAG)** localizado. Indexa su documentación local y páginas de manual en una base de datos vectorial, lo que permite que la IA proporcione sugerencias precisas según el contexto sin conexión.

---

## 🛡️ Libro Blanco de Seguridad

### 1. Enclave Criptográfico
Todos los tokens sensibles y credenciales de git se almacenan en una bóveda cifrada **AES-256-GCM**. La clave solo se deriva después de un apretón de manos biométrico exitoso con el enclave de hardware del dispositivo (Secure Element).

### 2. Cortafuegos de Comandos Regex
Cada cadena de entrada es auditada por un cortafuegos heurístico. Si se detecta un patrón destructivo como `rm -rf /` o acceso no autorizado a la red, el comando es interceptado y registrado.

---

## 📅 Hoja de Ruta 2026 - 2027

### Fase 1: Estabilidad (Actual)
- [x] Soporte multiarquitectura (ARM64, x86_64).
- [x] Apt/Dpkg nativo.
- [x] Bóveda biométrica.

### Fase 2: GUI y GPU (Q3 2026)
- [ ] **Servidor de Pantalla Wayland:** Ejecute aplicaciones GUI de forma nativa.
- [ ] **Aceleración de GPU:** Soporte de Vulkan para inferencia de IA.

---

## 📄 Licencia
Licencia MIT. Copyright (c) 2026 Equipo Flux AI.
