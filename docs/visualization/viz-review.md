# Diagram Generation Tool Options for MPST Protocols in Rust

Visualizing multiparty session protocols (message sequences and data flows) can greatly aid understanding. There are both static image tools and interactive web libraries to generate sequence diagrams, flowcharts, etc. Below we survey relevant options, compare them, and discuss integration into Rust projects (Cargo build, mdBook, rustdoc, etc.), with recommendations for smooth workflows.

## Static Diagram Tools

**Rust-native libraries.** Some crates can generate diagrams directly from Rust or custom DSLs. For example, layout-rs is a Rust library for parsing and rendering GraphViz (DOT) files[docs.rs](https://docs.rs/layout-rs/latest/layout/#:~:text=This%20crate%20provides%20a%20library,for%20constructing%20and%20rendering%20graphs). Using it (or dot_writer or petgraph), you can construct a graph in code and output SVG/PNG via `layout::gv`. The Flou crate provides a simple _flowchart_ DSL and CLI: write a textual flowchart (`grid { ... }`) and produce SVG[github.com](https://github.com/Asha20/flou#:~:text=Flou%20is%20a%20domain,description%20into%20an%20SVG%20file)[github.com](https://github.com/Asha20/flou#:~:text=define%20%7B%20block%28class%3A%20,stay%28%22Yes%22%29%3B%20%7D%29%3B). Another emerging crate is _fishextract_ (v0.0.0) – a pure Rust _Mermaid_ parser/renderer that can convert Mermaid diagram definitions into SVG without calling Node.js (useful for sequence or flow charts). Pros of Rust-native: no external dependencies (C/Java/Node) at build time; cons: fewer features or less maturity (e.g. Flou is still beta[github.com](https://github.com/Asha20/flou#:~:text=Reasons%20NOT%20to%20use%20Flou%3F)).

**CLI tools (standalone).** Widely used tools include **PlantUML**, **Mermaid CLI**, **Graphviz**, etc. For example, PlantUML (Java-based) can read a `*.puml` file or stdin and output PNG or SVG (`plantuml -tpng`/`-tsvg`). A Rust crate plantuml-server-client-rs provides a CLI interface that can either call a local PlantUML JAR or send to the PlantUML server. For example:

`cargo install plantuml-server-client-rs { echo '@startuml'; echo 'A->B: msg'; echo '@enduml'; } | plantuml-server-client > out.svg`

generates an SVG[docs.rs](https://docs.rs/plantuml-server-client-rs#:~:text=cargo%20install%20plantuml,client%20%3E%20out.svg). **Mermaid CLI** (`@mermaid-js/mermaid-cli`) runs on Node; it converts `.mmd` or Markdown with `​```mermaid` blocks into SVG/PNG (e.g. `mmdc -i diagram.mmd -o diagram.svg`[github.com](https://github.com/mermaid-js/mermaid-cli#:~:text=Convert%20Mermaid%20mmd%20Diagram%20File,To%20SVG)). **Graphviz** (`dot`) is a classic: output your protocol as a directed graph (e.g. flowchart) and run `dot -Tpng` or `dot -Tsvg` to get an image. Even if no Rust library, you can write DOT text and shell out. Other options: **goSeq** (Go CLI) is a text-based sequence diagram generator (outputs images); **mscgen** is an older text-to-sequence-diagram tool; **AsciiDoctor Diagram** (Ruby) supports UML.

**Output formats.** All of the above tools can produce SVG (vector) or PNG (bitmap) images. SVG is preferable for high-quality docs or web; PNG is simpler for static docs or if you need compatibility. (Mermaid CLI and PlantUML can both do either.) Rust crates may output SVG directly (layout-rs has a CLI for `.dot`→`.svg`[docs.rs](https://docs.rs/layout-rs/latest/layout/#:~:text=The%20project%20also%20comes%20with,svg)).

**Comparison Table (static tools):**

|Tool / Lib|Type|Diagrams|Output|Integration|Pros/Cons|
|---|---|---|---|---|---|
|**layout-rs**[docs.rs](https://docs.rs/layout-rs/latest/layout/#:~:text=This%20crate%20provides%20a%20library,for%20constructing%20and%20rendering%20graphs)|Rust crate|General Graphs (DOT)|SVG/PNG|Use in build.rs or code to render `.dot`→SVG|+ Rust native; – limited to GraphViz features|
|**Flou**[github.com](https://github.com/Asha20/flou#:~:text=Flou%20is%20a%20domain,description%20into%20an%20SVG%20file)|Rust CLI|Flowcharts (grid-based DSL)|SVG|Install CLI or call via Cargo/xtask|+ Simple flow DSL; – still beta|
|**fishextract** (0.0.0)|Rust crate|Mermaid (sequence/flow)|SVG|Call from Rust code (TBA)|+ No Node; – very new (unstable)|
|**PlantUML** [CLI]|Java CLI|UML seq, flow, etc.|PNG/SVG|Call via build.rs or cargo tool or plugin|+ Very feature-rich; – Java dep.|
|**Mermaid CLI** [Node]|Node CLI|Sequence, Flowchart, etc.|PNG/SVG|Call via build.rs or cargo tool|+ Flexible; – Node/npm dependency|
|**Graphviz (dot)** [C]|System CLI|Flowcharts, graphs|PNG/SVG|Call `dot` from script or crate (layout-rs)|+ Ubiquitous; – manual layout control|
|**goseq** [Go CLI]|Go CLI|Sequence diagrams (simple DSL)|PNG/SVG(?)|Call externally from build script|+ Easy DSL; – Go install needed|
|**mscgen** [C CLI]|C CLI|Sequence charts (old tool)|PNG/SVG|Call from script|+ Mature; – dated syntax|

_Table:_ Comparison of static diagram generation tools. “Type” indicates language/runtime. (SVG output is supported by all above; some only output PNG by default.)

## Integration Strategies

### Build-time and CLI Integration

You can call any CLI tool (PlantUML, Mermaid, Graphviz, etc.) from Rust’s build process or scripts. Common approaches:

- **build.rs**: Write a `build.rs` that invokes a command (via `std::process::Command`) to generate diagrams before compilation. For example, run `mmdc -i protocol.mmd -o protocol.svg` so that `protocol.svg` is created in `OUT_DIR` or source directory. You might include the result in the compiled docs or copy to `target/`. This keeps diagram sources (e.g. `.mmd` or `.puml`) under version control and automatically regenerates on build.
    
- **Cargo subcommand / xtask**: Create a custom Cargo command (e.g. `cargo gen-diagrams`) or an external `xtask` binary in your repo that reads DSL definitions and outputs images. This can encapsulate complex generation logic (e.g. parsing your Rust DSL and feeding a diagram tool). You could even publish a `cargo-my-dsl` plugin for it.
    
- **CI / Makefile**: Some projects simply script generation (in `Makefile`, `CI`, or a `post-checkout` hook). For example, a CI job might run `mmdc` on all `.mmd` files and commit generated SVGs to docs.
    
### Doc-generation Integration

- **Rustdoc (`cargo doc`)**: To include diagrams in `cargo doc`, use doc-comment macros. For example, the simple_mermaid crate provides a `#[doc = mermaid!("file.mmd")]` macro – this embeds the Mermaid text into the generated HTML[docs.rs](https://docs.rs/simple-mermaid/latest/simple_mermaid/#:~:text=1,Done). The rsdoc crate provides `#[doc = rsdoc::plantuml!(...)]` to inline PlantUML as an SVG in docs[github.com](https://github.com/cocalon/rsdoc#:~:text=). These produce SVG embedded in `rustdoc` output, requiring no separate image files. The `rsdoc::image!("file.png")` macro can also include static PNG/SVG.
    
- **mdBook**: Use preprocessors. The mdbook-mermaid plugin will detect `mermaid` code blocks, render them (via Mermaid) into SVG, and include them in the HTML[github.com](https://github.com/badboy/mdbook-mermaid#:~:text=mdbook). Similarly, mdbook-plantuml replaces PlantUML code blocks with inline SVG[docs.rs](https://docs.rs/crate/mdbook-plantuml/0.3.0#:~:text=%5Bpreprocessor.plantuml%5D%20plantuml). To use, add e.g. `[preprocessor.mermaid] command = "mdbook-mermaid"` in `book.toml`. The preprocessors also inject the needed JS (`mermaid.min.js`) so diagrams render client-side.
    
- **Markdown/HTML**: For generic docs or web UI, you can embed static images (`![Alt](diagram.svg)`) or client-rendered Mermaid. Many static site generators (Docusaurus, MkDocs) support Mermaid code blocks out of the box or via plugins. In custom HTML (Tailwind/UI), you can include `<script src="mermaid.min.js">` and use `<div class="mermaid">sequenceDiagram ...</div>`. Alternatively, use React components or wrappers (e.g. `react-mermaid2`) to integrate into a Tailwind CSS–styled app.
    
### Ease of Integration

- CLI tools (GraphViz/PlantUML/Mermaid) are straightforward but require managing external deps (Java, Node, dot). They can be invoked from scripts or build.rs with minimal Rust code.
    
- Rust libraries (layout-rs, Flou, fishextract) avoid external binaries, but may need more glue code in Rust to feed data and write output.
    
- Doc macros and mdBook plugins handle automation: once set up, diagrams update on each build with no manual steps. This reduces friction in docs workflows.
    
## Interactive Visualization Tools

For interactive, browser-based diagrams (zoom, hover, click actions), JavaScript libraries are used:

- **Mermaid.js** – client-side rendering of many diagram types (sequence, flowchart, state, etc.). Can run in any browser. With `mermaid.min.js`, any `<div class="mermaid">...</div>` is converted to SVG/Canvas on page load. Useful for interactive docs or UIs – the user can hover over lifelines or animate steps. It supports Tailwind-styled sites via CSS. (Mermaid can also animate or highlight elements in response to user events.)
    
- **js-sequence-diagrams** – a lightweight JS library to render sequence diagrams from text. It produces clickable SVG elements (though less actively maintained than Mermaid).
    
- **viz.js** – a WebAssembly port of GraphViz, allowing client-side rendering of `.dot` graphs in a browser canvas or SVG.
    
- **goJS / JointJS / Rappid** – commercial/OSS JavaScript diagramming frameworks offering rich interactive diagrams (flowcharts, sequence diagrams). These are heavier but support editing.
    
- **Diagramming frameworks** – e.g. draw.io (diagrams.net) can be embedded or used with an API, though typically used as a standalone app.
    
In a TailwindCSS + shadcn/ui context, Mermaid is often easiest: include `<script>` and then use a `<div class="mermaid max-w-full">sequenceDiagram ...</div>`. The diagrams will inherit your site’s CSS (Tailwind) for fonts, etc. Tools like Excalidraw or Mermaid Live Editor are web-based but not embeddable charts.

**Pros/Cons (interactive):** Interactive diagrams allow user exploration (e.g. collapse lifelines), but they require the viewer’s browser to run the JS. In static documentation (e.g. `mdbook build` output), the JS runs client-side, so diagrams are “live”. In `cargo doc`, embedding a `<div class="mermaid">` won’t auto-render unless the JS is injected – however, `mdbook-mermaid` actually injects the script automatically[github.com](https://github.com/badboy/mdbook-mermaid#:~:text=mdbook). For a self-contained PDF or offline doc, static images (PNG/SVG) might be simpler.

## Embedding in Documentation

- **mdBook:** To embed diagrams, use preprocessor plugins. For Mermaid, install mdbook-mermaid and run `mdbook-mermaid install path/to/book`, which adds the config[github.com](https://github.com/badboy/mdbook-mermaid#:~:text=mdbook). For PlantUML, use mdbook-plantuml and configure `[preprocessor.plantuml] plantuml-cmd`. Then any `plantuml or` mermaid block in your Markdown becomes an inline SVG. mdBook output thus contains the images (and needed scripts) without extra manual steps.
    
- **cargo doc:** Use doc macros. For example, with simple_mermaid you can write:
    
- `/// Protocol sequence: #[doc = mermaid!("protocol_seq.mmd")] fn _dummy() {}`
    
    In the generated HTML, the Mermaid code is embedded so that docs.rs or `cargo doc` (with Mermaid support) renders it[docs.rs](https://docs.rs/simple-mermaid/latest/simple_mermaid/#:~:text=1,Done). For PlantUML or other diagrams, `rsdoc::plantuml!` wraps the UML code in a doc comment and replaces it with an SVG[github.com](https://github.com/cocalon/rsdoc#:~:text=).
    
- **README or Markdown:** On platforms like GitHub, simple Mermaid syntax in Markdown (fenced code with `mermaid`) is now supported and renders as interactive SVG on the site. Otherwise, one can pre-generate an SVG and link it `![Diagram](path/to/diagram.svg)` in the Markdown. For custom sites, adding Mermaid’s JS to the page lets you write diagrams directly in the Markdown.
    
**Recommendations:** Use interactive embedding (Mermaid JS) in HTML-based docs (mdBook or a custom doc site) for a smooth authoring experience – you edit text and see updated diagrams on refresh. For `cargo doc`, macros like `simple_mermaid`/`rsdoc` are practical, but note that rustdoc may not execute JS, so these macros generate static SVG at doc-build time. For API documentation, static images (embedded SVG via macros) are most reliable.

## Example Usage Flows

- **Static build script:** Suppose you have a protocol file `protocol.mmd` (Mermaid format). In `build.rs` you could write:
    
- `use std::process::Command; fn main() {     let status = Command::new("mmdc")         .args(&["-i", "protocol.mmd", "-o", "target/protocol.svg"])         .status().expect("Mermaid CLI failed");     assert!(status.success()); }`
    
    This ensures `protocol.svg` is always updated. You can then include it in docs or copy it to `target/doc` in a doc pipeline.
    
- **MdBook with PlantUML:** In `book.toml`, add:
    
`[preprocessor.plantuml] plantuml-cmd = "plantuml" [output.html] additional-js = ["mermaid.min.js","mermaid-init.js"]`

and ensure `mdbook-plantuml` is installed. Now writing:

- ` ```plantuml @startuml A -> B: hello @enduml ``` `
    
    in a chapter will produce an inline SVG diagram[docs.rs](https://docs.rs/crate/mdbook-plantuml/0.3.0#:~:text=%5Bpreprocessor.plantuml%5D%20plantuml).
    
- **Cargo doc with `rsdoc`:** In your Rust code:
    
- `#[doc = rsdoc::plantuml!(   /// @startuml   /// A -> B: greet   /// @enduml )] pub fn greet() {}`
    
    When running `cargo doc`, this will generate an SVG for the sequence and embed it in the HTML doc for `greet()`[github.com](https://github.com/cocalon/rsdoc#:~:text=).
    
- **Interactive UI (Tailwind):** In a React/Tailwind app, install `mermaid` (e.g. via `npm i mermaid`). In your JSX:
    
- `import mermaid from 'mermaid'; mermaid.initialize({ startOnLoad: true }); export function Sequence() {   return <div className="mermaid">     sequenceDiagram     Alice->>Bob: Ping     Bob-->>Alice: Pong   </div>; }`
    
    The `div` will render as an interactive SVG. Style with Tailwind by assigning classes (e.g. `max-w-screen-md mx-auto` for layout).
    
## Minimizing Developer Friction

- **Keep sources under version control.** Store your protocol DSL or diagram text (`.mmd`, `.puml`, or custom DSL files) in the repo alongside code. This way changes to protocols automatically drive diagram updates.
    
- **Automate generation.** Use CI or `cargo doc`/mdBook pipelines to regenerate diagrams so manual steps aren’t forgotten. For example, a `pre-commit` hook or `cargo gen-diagrams` subcommand can check timestamps or diff images.
    
- **Use watchers during development.** Tools like `cargo watch` or `watchexec` can rerun a diagram build script when source files change, giving near-instant visual feedback.
    
- **Favor vector graphics.** SVG images scale well and embed in docs cleanly. Many tools (Mermaid CLI, PlantUML with `-tsvg`) support SVG. Even if you produce PNG, consider also keeping an SVG for high-res needs.
    
- **Leverage macros/preprocessors.** Embedding diagrams via macros or mdBook plugins means that docs are always in sync with sources without separate “export” steps. This reduces the chance diagrams get out of date.
    
- **Dependencies management.** If using Java/Node tools, document how to install them (e.g. `brew install plantuml` or `npm install -g @mermaid-js/mermaid-cli`). Using containerized builds or CI caching can mitigate heavy installs. Rust-native tools (layout-rs, Flou) can simplify this if their features suffice.
    
In summary, **for static documentation**, PlantUML, Mermaid CLI or GraphViz are solid (with mdBook/rsdoc integration to automate). **For interactive docs or web apps**, Mermaid.js in the browser is very convenient. By automating through build scripts, Cargo plugins, or doc preprocessors, you ensure diagrams update with minimal manual effort. Regularly regenerate diagrams in your CI (or as part of `cargo doc`/`mdbook build`) to keep them current with the protocol definitions. The net result is easy, version-controlled, and consistently formatted visualizations embedded into your Rust project’s docs and UI.

**Sources:** Tools and crates referenced above have documentation and examples, e.g. Mermaid CLI usage[github.com](https://github.com/mermaid-js/mermaid-cli#:~:text=Convert%20Mermaid%20mmd%20Diagram%20File,To%20SVG), mdBook Mermaid/PlantUML preprocessors[github.com](https://github.com/badboy/mdbook-mermaid#:~:text=mdbook)[docs.rs](https://docs.rs/crate/mdbook-plantuml/0.3.0#:~:text=%5Bpreprocessor.plantuml%5D%20plantuml), Rustdoc macros[docs.rs](https://docs.rs/simple-mermaid/latest/simple_mermaid/#:~:text=1,Done)[github.com](https://github.com/cocalon/rsdoc#:~:text=), and Rust crates like layout-rs[docs.rs](https://docs.rs/layout-rs/latest/layout/#:~:text=This%20crate%20provides%20a%20library,for%20constructing%20and%20rendering%20graphs). These confirm the feasibility of generating and embedding diagrams as described.

Цитати

[

![Favicon](https://www.google.com/s2/favicons?domain=https://docs.rs&sz=32)

layout - Rust

<https://docs.rs/layout-rs/latest/layout/>

](<https://docs.rs/layout-rs/latest/layout/#:~:text=This%20crate%20provides%20a%20library,for%20constructing%20and%20rendering%20graphs)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://github.com&sz=32)

GitHub - Asha20/flou: A flowchart description language.

<https://github.com/Asha20/flou>

](<https://github.com/Asha20/flou#:~:text=Flou%20is%20a%20domain,description%20into%20an%20SVG%20file)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://github.com&sz=32)

GitHub - Asha20/flou: A flowchart description language.

<https://github.com/Asha20/flou>

](<https://github.com/Asha20/flou#:~:text=define%20%7B%20block%28class%3A%20,stay%28%22Yes%22%29%3B%20%7D%29%3B)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://github.com&sz=32)

GitHub - Asha20/flou: A flowchart description language.

<https://github.com/Asha20/flou>

](<https://github.com/Asha20/flou#:~:text=Reasons%20NOT%20to%20use%20Flou%3F)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://docs.rs&sz=32)

plantuml_server_client_rs - Rust

<https://docs.rs/plantuml-server-client-rs>

](<https://docs.rs/plantuml-server-client-rs#:~:text=cargo%20install%20plantuml,client%20%3E%20out.svg)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://github.com&sz=32)

GitHub - mermaid-js/mermaid-cli: Command line tool for the Mermaid library

<https://github.com/mermaid-js/mermaid-cli>

](<https://github.com/mermaid-js/mermaid-cli#:~:text=Convert%20Mermaid%20mmd%20Diagram%20File,To%20SVG)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://docs.rs&sz=32)

layout - Rust

<https://docs.rs/layout-rs/latest/layout/>

](<https://docs.rs/layout-rs/latest/layout/#:~:text=The%20project%20also%20comes%20with,svg)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://docs.rs&sz=32)

simple_mermaid - Rust

<https://docs.rs/simple-mermaid/latest/simple_mermaid/>

](<https://docs.rs/simple-mermaid/latest/simple_mermaid/#:~:text=1,Done)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://github.com&sz=32)

GitHub - cocalon/rsdoc: PlandUML and Drawio diagrams in doc comments as PNG or SVG images.

<https://github.com/cocalon/rsdoc>

](<https://github.com/cocalon/rsdoc#:~:text=)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://github.com&sz=32)

GitHub - badboy/mdbook-mermaid: A preprocessor for mdbook to add mermaid support

<https://github.com/badboy/mdbook-mermaid>

](<https://github.com/badboy/mdbook-mermaid#:~:text=mdbook)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://docs.rs&sz=32)

mdbook-plantuml 0.3.0 - Docs.rs

<https://docs.rs/crate/mdbook-plantuml/0.3.0>

](<https://docs.rs/crate/mdbook-plantuml/0.3.0#:~:text=%5Bpreprocessor.plantuml%5D%20plantuml>)

Всички източници

[

![Favicon](https://www.google.com/s2/favicons?domain=https://docs.rs&sz=32)

docs

](<https://docs.rs/layout-rs/latest/layout/#:~:text=This%20crate%20provides%20a%20library,for%20constructing%20and%20rendering%20graphs)[>

![Favicon](https://www.google.com/s2/favicons?domain=https://github.com&sz=32)

github

](<https://github.com/Asha20/flou#:~:text=Flou%20is%20a%20domain,description%20into%20an%20SVG%20file>)
