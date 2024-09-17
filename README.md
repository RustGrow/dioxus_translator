# This project demonstrates:
- Multilingual Logic Implementation
    - Retrieve a value from browser storage.
    - If unavailable, use the browser's default language.
    - Adjust the HTML tag and direction accordingly.
- Using the Native Fluent-Templates Library
    - Integrate the fluent-templates library into your project.
    - Manage localization files efficiently.
- Advanced Language Control with Eval
    - Pass variables from Dioxus to JavaScript.
    - Handle complex site language management within JavaScript.
    - Accept and process JavaScript array structures in Dioxus code (commented out).
- Language Routing
    - Set the default language to English (en).
    - Maintain the same language path when reloading the page.

## Important
Reinstall the CLI to the git version
```bash
cargo install --git https://github.com/dioxuslabs/dioxus dioxus-cli --locked --force
```

# Development

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload true
```

- Open the browser to http://localhost:8080

# Hot reloading with Tailwind CSS
Hot reloading Tailwind CSS will work with [Tailwind CDN](https://tailwindcss.com/docs/installation/play-cdn) and Manganis using these settings.

1. Create a new project from the command line:

```bash
dx new -> web -> Project Name: project-name -> Tailwind -> true
```
You can change the platform, name, and router as needed.

No changes needed to the Dioxus.toml file.

2. Reinstall the CLI:
```bash
cargo install --git https://github.com/dioxuslabs/dioxus dioxus-cli --locked --force
```

```bash
dx --version
dioxus 0.6.0-alpha.2 (3c699aa)
```

3. Add dependencies to your Cargo.toml file:
```rust
[dependencies]
dioxus = { git = "https://github.com/DioxusLabs/dioxus", features = ["web", "router"] }
dioxus-logger = "0.5.1"
```

4. Start the Tailwind CSS compiler and the Dioxus dev server in different terminals:
```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
dx serve --hot-reload true
```

5. Add the following support to main.rs inside rsx:
```rust
rsx!{
    script { src: asset!("https://cdn.tailwindcss.com") }
    head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }
}
```

Example component:
```rust
#[component]
fn App() -> Element {
    rsx! {
        // For Play CDN to try Tailwind
        script { src: asset!("https://cdn.tailwindcss.com") }
        // For Manganis
        head::Link { rel: "stylesheet", href: asset!("./assets/tailwind.css") }

        img { src: "header.svg", id: "header" }
        div { id: "links",
            div { class: "p-4 bg-yellow-300", "I" }
            p { "I" }
            div { class: "red p-2", "love" }
            div { class: "yellow", "Dioxus" }
            p { class: "red bg-slate-300", "team" }
        }
    }
}
```

# If you need a local stylesheet for custom styles inside input.css.
1. Insert your custom styles inside input.css:
```css
@layer components {
  p {
    @apply p-10 bg-yellow-600;
  }
  .red {
    @apply bg-red-600;
  }
  .yellow {
    @apply bg-yellow-600;
  }
  .blue {
    @apply bg-blue-600;
  }
}
```
2. Insert custom classes into the page:
```rust
rsx!{
    p { "I" }
    div { class: "red", "want to" }
    div { class: "yellow", "burger" }
    div { class: "blue", "burger" }
}
```
3. Rebuild the app:

button r on terminal 

or 

```bash
dx serve --hot-reload true
```