# Java Script GUI for Rust

Use Java Script as a GUI toolkit for Rust

# Installation
```bash
$ cargo new project_name && cd project_name
```

```toml
[dependencies]
js_gui_rs = { git = "https://github.com/pisgah-fogel/js_gui_rs.git" }
```

```bash
$ cp ~/.cargo/registry/src/github.com-*/js_gui_rs/frontend .
```

```Rust
extern crate js_gui_rs;
fn main() {
    let js_gui = js_gui_rs::JsGui::new("127.0.0.1:2794");
    js_gui_rs::print_link();
    std::thread::sleep(std::time::Duration::from_millis(5000));
    js_gui.clear();
    js_gui.draw_text(100, 100, "Hello world!", "30px Arial");
    std::thread::sleep(std::time::Duration::from_millis(1000));
}
```

```bash
$ cargo build
```

# Development dependency
We use websocket for rust to interact with the web page.
No other crate for the moment.

# External dependency
The main purpose of this project is to have a simple gui without dependency.
Therefore the only external dependency is to have a web browser.
