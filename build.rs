use std::path::Path;

fn main() {
    let lexer_cpp = Path::new("src/cpp/lexer.cpp");
    if lexer_cpp.exists() {
        // Compile the C++ bridge code only if lexer.cpp exists.
        cxx_build::bridge("src/cpp/lexer.rs")
            .file("src/cpp/bridge.cpp")
            .file("src/cpp/lexer.cpp")
            .include("src/cpp")
            .compile("lexer");

        // Re-run this build script if any of these files change.
        println!("cargo:rerun-if-changed=src/cpp/lexer.rs");
        println!("cargo:rerun-if-changed=src/cpp/bridge.cpp");
        println!("cargo:rerun-if-changed=src/cpp/bridge.h");
        println!("cargo:rerun-if-changed=src/cpp/lexer.cpp");
    } else {
        // Print a warning and optionally fail the build.
        println!("cargo:warning=src/cpp/lexer.cpp not found. Skipping C++ lexer build.");
    }
}
