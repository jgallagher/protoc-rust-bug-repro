fn main() {
    println!("cargo:rerun-if-changed=src/BugReproducer.proto");
    protoc_rust::Codegen::new()
        .out_dir("src")
        .inputs(&["src/BugReproducer.proto"])
        .run()
        .expect("failed to codegen BugReproducer.proto");
}
