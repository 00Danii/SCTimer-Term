# ⏱️ SCTimer-Term

¡Bienvenido a **SCTimer-Term**!  
Un cronómetro de cubo de Rubik para terminal, rápido, visual y personalizable.  
Ideal para speedcubers que aman la eficiencia y el minimalismo, pero no quieren renunciar a una interfaz atractiva.

---

## 🚀 Características

- **Scrambles aleatorios** con visualización en ASCII-art.
- **Inspección automática** de 20 segundos.
- **Timer grande** en ASCII-art, fácil de leer.
- **Historial de tiempos** con scroll y barra visual.
- **Promedios** Ao5, Ao12, Ao25 y Ao50, también en ASCII-art.
- **Control total desde el teclado**:  
  - `Espacio`: Iniciar/terminar resolución  
  - `↑/↓`: Navegar por el historial  
  - `q`: Salir
- **Minimalista, rápido y multiplataforma** (Windows, Linux, Mac).

---

## 🖥️ Captura de pantalla

```
┌─────────────── Scramble ───────────────┐
|  U2 R' F2 ... (en ASCII-art)           |
└────────────────────────────────────────┘
┌─────────────── Timer ──────────────────┐
|      12.345                            |
|      (ASCII-art grande)                |
└────────────────────────────────────────┘
┌──── Promedios ────┬──── Historial ─────┐
| Ao5:  13.123      |  1: 12.345s - ...  |
| Ao12: 14.567      |  2: 13.222s - ...  |
| Ao25: 15.000      |  ...               |
| Ao50: 14.999      |  ...               |
└───────────────────┴────────────────────┘
```

---

## ⚡ Instalación

1. **Clona el repositorio:**
   ```sh
   git clone https://github.com/00Danii/SCTimer-Term.git
   cd sctimer-term
   ```

2. **Asegúrate de tener Rust instalado:**  
   [Instalar Rust](https://www.rust-lang.org/tools/install)

3. **Instala las dependencias y ejecuta:**
   ```sh
   cargo run --release
   ```

---

## 🎹 Controles

- `Espacio`: Iniciar inspección, empezar y terminar resolución.
- `↑ / ↓`: Navegar por el historial de tiempos.
- `q`: Salir del programa.

---

## 🧩 Personalización

- Puedes cambiar las fuentes ASCII-art en la carpeta `assets/`.
- Modifica los colores y el layout en `src/ui.rs` para adaptarlo a tu estilo.

---

## 🛠️ Tecnologías

- [Rust](https://www.rust-lang.org/)
- [ratatui](https://github.com/ratatui/ratatui) (TUI framework)
- [figlet-rs](https://github.com/aldanor/figlet-rs) (ASCII-art)
- [crossterm](https://github.com/crossterm-rs/crossterm) (eventos de teclado)
- [rand](https://crates.io/crates/rand) (scrambles aleatorios)

---

## 🤝 Contribuciones

¡Pull requests y sugerencias son bienvenidas!  
Si tienes ideas para nuevas funciones, abre un issue o crea un fork.

---

## 📄 Licencia

MIT

---

¡Que tus solves sean rápidos y tus averages