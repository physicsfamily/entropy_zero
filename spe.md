# PROJECT ARCHITECTURE SPECIFICATION: "STRATEGY IRON LOGIC"

## 1. Executive Summary & Strategic Directive
**Core Strategy:** Subdomain Isolation Strategy.
**Objective:** To decouple high-performance simulation logic from marketing/SEO requirements completely.
**Implication:** We are building TWO distinct software systems under one brand. There is NO shared runtime environment between them.

---

## 2. System A: The Marketing Portal (www.yourdomain.com)
**Primary Goal:** Maximum SEO score, LCP (Largest Contentful Paint) < 1s, Zero CLS.
**Architecture:** Static Site Generation (SSG) / Islands Architecture.

### Technology Stack
* **Framework:** Astro (v4+).
* **UI Library:** React or Vue (Only for interactive "islands" like mobile menus).
* **Styling:** Tailwind CSS.
* **Rendering:** Server-Side Rendering (SSG focus) -> Pure HTML.
* **JavaScript Policy:** Zero-JS by default. JS is only hydrated where explicitly required (`client:visible`).

---

## 3. System B: The Simulation Core (app.yourdomain.com)
**Primary Goal:** 60FPS+ Simulation, Type Safety, Long-term Robustness.
**Architecture:** Client-side WASM application using Entity-Component-System (ECS) pattern.
**Note:** This is an SPA (Single Page Application) but with NO DOM manipulation. All rendering occurs within a WebGPU Canvas.

### Technology Stack
* **Language:** Rust (Edition 2021).
* **Build Tool:** Trunk (for WASM compilation and bundling).
* **Core Engine:** **Bevy Engine** (Latest stable).
* **Graphics Backend:** wgpu (WebGPU preferred, fallback to WebGL2).
* **GUI Framework:** `bevy_egui` (Immediate Mode GUI for in-simulation tools).

### 🏛️ Engineering Standards for System B (Strict Enforcement)

**Rule 1: The ECS Paradigm (Entity-Component-System)**
* **Strict Prohibition:** Do NOT use Object-Oriented Programming (OOP) inheritance patterns.
* **Components:** Must be `struct` or `enum` containing **PURE DATA**. No logic, no methods inside components. Derive `Component`, `Reflect`, `Default`.
* **Systems:** Must be stateless Rust functions. Logic resides ONLY in Systems.
* **Resources:** Use Resources for global singleton data (e.g., Gravity, TimeScale).

**Rule 2: Project Structure (Modular Plugins)**
Organize the codebase by domain-specific plugins to maintain scalability:
```text
src/
├── main.rs            // Entry point: App builder & Plugin registration ONLY.
├── components/        // Pure data structs.
├── systems/           // Logic functions.
├── resources/         // Global configs.
└── plugins/           // Domain organizers (e.g., PhysicsPlugin, UIPlugin).
```

**Rule 3: User Interface**
* **Do not use HTML/CSS** for the simulation UI.
* **Use `bevy_egui`** to draw floating panels, sliders, and buttons directly inside the canvas.
* **Reason:** This ensures the UI is tightly coupled with the simulation state and written entirely in Rust.

**Rule 4: Error Handling**
* **Target:** Zero Runtime Panics.
* **Strategy:** Use `Option` and `Result` extensively. `unwrap()` is forbidden in production code; use `expect()` with meaningful messages or handle errors gracefully.

---

## 4. Development Workflow Protocol for AI Agent
When generating code for this project, you (The AI Agent) must adhere to the following protocol:

1. **Context Check:** Determine if the request is for System A (Astro/HTML) or System B (Rust/Bevy). Do not mix contexts.
2. **Bevy First:** When working on System B, always prioritize Bevy's ECS API (`Query`, `Commands`, `Res`, `ResMut`) over standard Rust logic.
3. **Atomic Implementation:**
    * **Step 1:** Define the Component (Data).
    * **Step 2:** Define the System (Logic).
    * **Step 3:** Register them in a Plugin.
4. **No Hallucination:** If a Bevy API has changed in recent versions, explicitly state the version assumption or ask for verification.

---

## 5. Immediate Action Plan
Please initialize the scaffolding for System B following the structure above.

1. Create the `Cargo.toml` with dependencies: `bevy`, `bevy_egui`, `log`, `wasm-bindgen`.
2. Create the `src/main.rs` with a basic Window setup (Canvas ID: `#canvas`).
3. Create a dummy plugin `plugins/hello_world.rs` that spawns a 3D cube to verify the WebGPU pipeline.
