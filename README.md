# This [Dioxus](https://dioxuslabs.com/) project demonstrates:
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

<div align="center">
  <h3>  
    <a href="https://multilanguage-dioxus.netlify.app/"> Website </a>
  </h3>
</div>

### Important. This project uses the web platform
# Quick start
1. Reinstall the CLI to the git version.
For Windows users need to install the [Netwide Assembler (NASM)](https://www.nasm.us/pub/nasm/releasebuilds/2.16.03/win64/). On startup it will open the shell and inside execute this command.
```bash
cargo install --git https://github.com/dioxuslabs/dioxus dioxus-cli --locked --force
```
2. Clone this repository
```bash
https://github.com/DioxusGrow/dioxus_translator.git
```
and 👇

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

1. Reinstall the CLI:
```bash
cargo install --git https://github.com/dioxuslabs/dioxus dioxus-cli --locked --force
```

2. Check that the library version corresponds to 0.6
```bash
dx --version
// dioxus 0.6.0-alpha.2 (3c699aa)
```

3. Create a new project from the command line:
```bash
// You can change the platform, name, and router as needed.
dx new -> web -> Project Name: project-name -> Tailwind -> true
```

4. Add dependencies to your Cargo.toml file:
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

5. The Script now has some bug
```rust
❌ Script { src: "https://cdn.tailwindcss.com" } //Has some bug
```
You need to set a script reference to use Tailwind CDN inside Dioxus.toml
✔️ Tested
```toml
# include `assets` in web platform
[web.resource]

# CSS style file

style = []

# Javascript code file
script = ["https://cdn.tailwindcss.com"]
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

# How to make a release

1. Make sure you add the languages folder to the monitoring in the Dioxus.toml file:
```toml
# which files or dirs will be watcher monitoring
watch_path = ["src", "assets", "lang"]
```
2. Use the `dx check` command to check that there are no errors in the logic for using the signals.
```bash
dx check
//output No issues found.
```
3. Make a release using the command:
```bash
dx build --release
```
4. The `dist` folder is by default the main project folder where the finished site is located.

# Netlify deployment

Deploying an application in netlify requires a special settings file to be uploaded to the assets folder so that it is automatically loaded when the project is built. In this repository it is already in the assets folder.

netlify.toml
```toml
[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```
