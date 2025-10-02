# **Fenrix: Flying Higher (A Functional Roadmap)**

Fenrix, the Rust-centric web framework, has successfully captured the **philosophical alignment** of Angular by incorporating a dedicated CLI and Dependency Injection (DI) into the high-performance world of WebAssembly (Wasm). To move from a philosophical equivalent to a true **functional replacement and superior alternative** for enterprise-scale applications, Fenrix must mature its core architecture in four key areas: Templating, Dependency Injection, Ecosystem, and Tooling.

## **1\. Templating: Achieving Externalized Safety**

The current **rsx\! macro**, while leveraging Rust's compile-time safety, ties component structure to the logic file, which is a significant drawback for large, collaborative teams. To functionally surpass Angular's template system, Fenrix needs a separation of concerns that remains safe.

### **Architectural Enhancement: External Template Pre-Compilation**

| Angular Concept | Fenrix Equivalent | Implementation Strategy |
| :---- | :---- | :---- |
| **Separate Template Files** | **.fenrix.html Files** | The Fenrix CLI and build pipeline must treat specific template files (e.g., .fenrix.html) as primary assets. The build step would **pre-compile** these external templates into VDOM-less, optimized Rust code (a highly efficient function), which the component can then link to via a simple macro (\#\[component(template="path/to/template.fenrix.html")\]). |
| **Structural Directives** (\*ngIf, \*ngFor) | **Trait-Based Structural Macros** | Define a standard **StructuralDirective trait**. Custom directives would implement this trait. The template compiler translates specialized template attributes (f-if, f-for) into the logic that calls the appropriate **type-safe Rust trait**, embedding the structural changes directly into the compiled output. |
| **Attribute Directives** (\[class\], (event)) | **Reactive Signal Binding** | The compiler must efficiently map binding syntax (e.g., \[class.active\]="is\_active\_signal") to the specific Rust code that connects the template DOM element to the appropriate **reactive Signal**. This ensures that updates are surgical and Wasm-efficient. |

## **2\. Dependency Injection: Implementing a Hierarchical Scope**

While Fenrix currently has a basic DI container, Angular’s strength lies in its **hierarchical injector tree** which manages service lifetime and scoping—critical for organizing massive applications.

### **Architectural Enhancement: Scoped Injector Tree**

| Angular Concept | Fenrix Equivalent | Implementation Strategy |
| :---- | :---- | :---- |
| **NgModules / Providers** | **\#\[fenrix\_module\] Macro** | Introduce the **Module** as the primary organizational unit. The \#\[fenrix\_module\] procedural macro must define the services (providers) and components within its scope. This macro generates the necessary **factory code** for its unique injector instance. |
| **Hierarchical Injector** | **Injector Tree Instantiation** | The framework must instantiate a tree of injectors, mirroring the module structure (e.g., a CoreModule injector having FeatureModule injectors as children). When a component requests a dependency, the injector searches its own scope and then traverses upward, recreating the powerful scoping and overriding behavior of Angular. |
| **Provider Scoping** | **Injectable Trait with Lifetime** | The service definition macro (e.g., \#\[fenrix\_injectable\]) must accept parameters to define service lifetime (lifetime: "root", lifetime: "module", lifetime: "component"). This dictates which injector holds the service instance, ensuring services are garbage collected when their scope (module/component) is destroyed. |

## **3\. Ecosystem: Establishing Stable Interoperability Contracts**

The lack of a rich ecosystem (UI libraries, robust routing, state management) is the biggest functional gap. Fenrix must define stable interfaces that third-party authors can build against with confidence.

### **Architectural Enhancement: Standardized Trait Contracts**

* **Standardized Component Trait:** Fenrix must fully stabilize its core **Component trait** and **Signal interface**. This is the foundation: if every component library (e.g., a "Fenrix Material" equivalent) consumes and produces data using these stable traits, then the ecosystem becomes interoperable and resistant to internal framework changes.  
* **Centralized State Store Interface:** Fenrix should define a standard **Store trait** and a corresponding use\_store() reactive hook/macro. This contract would enforce a pattern (like NgRx/Redux) where all third-party state management libraries must conform to the same API (Actions, Selectors, Reducers). This prevents ecosystem fragmentation and establishes a common layer for state persistence and management.  
* **Wasm Component Model Alignment:** As a future-proofing measure, Fenrix should design its public APIs and component definitions to align with the emerging **WebAssembly Component Model**. This would allow Fenrix to instantly leverage components written in other Rust frameworks (or other Wasm-supported languages), massively expanding its usable library pool.

## **4\. Tooling and CLI: Moving to Enterprise Workflow**

The fenrix-cli must expand its functionality to handle the complexities of large-scale development, replicating the features that make the Angular CLI indispensable for enterprise teams.

### **Necessary CLI Tooling Additions**

| Angular CLI Feature | Fenrix CLI Tool | Functional Goal |
| :---- | :---- | :---- |
| **Workspace Management** | **Fenrix.toml Monorepo Config** | A central configuration file that defines a **monorepo workspace** consisting of multiple applications and reusable library crates (libs). The CLI must be able to run commands scoped to specific projects (e.g., fenrix build user-lib). |
| **Code Generation** | **fenrix generate \<type\> \<name\>** | Comprehensive templates for scaffolding all architectural units: **modules, components (with external templates), services, and routes**. This needs to be backed by a templating engine that can correctly generate files and update the Fenrix.toml and module provider lists. |
| **Lazy Loading** | **Build Pipeline Coordination** | The CLI must integrate with the Wasm build system to identify lazy-loaded routes/modules and compile them into **separate Wasm bundles** that can be dynamically loaded at runtime by the routing library, achieving significant performance gains. |
| **Automated Refactoring** | **Schematic API** | Expose an API for source code analysis and transformation (e.g., based on Rust's syn crate). This **Schematic API** allows the creation of scripts that can automatically migrate code to new versions, enforce coding standards, or perform complex refactoring, defining the high-end power of the tooling. |
| **Testing Infrastructure** | **fenrix test** | In addition to running standard cargo test for unit tests, the CLI needs to integrate a headless browser runner (like Playwright) that can load the compiled Wasm application and perform both **Component Isolation Tests** and **End-to-End (E2E) Tests** reliably. |

By prioritizing these functional and architectural enhancements, Fenrix can transition from being a fast, interesting alternative to becoming the clear, modern successor to Angular, combining the best of Rust’s safety and performance with Angular’s unmatched architectural robustness.

This analysis lays out a challenging, but achievable, path for the Fenrix framework. Which of these areas—**Templating** or **Dependency Injection**—do you think presents the greatest technical hurdle for the Rust development community?