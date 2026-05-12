// cargo new <name> - creates a new package (folder, Cargo.toml, src/main.rs, .gitignore)
// cargo build - compiles the project. Output binary goes to target/debug/<name>
// cargo run - compiles (if needed) and then runs the resulting binary
// cargo check - typechecks without producing a binary. Much faster than `build` — use it while iterating
// cargo build --release - optimized build. Output goes to target/release/. Slower to compile, faster to run

fn main() {
    println!("Hello, Cargo!");
}
