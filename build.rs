use protobuf_codegen::Codegen;

fn main() {
    // Build our realtime feed structure
    Codegen::new()
        .out_dir("src/yahoo/gen")
        .inputs(&["src/yahoo/realtime.proto"])
        .includes(&["src"])
        .run()
        .expect("Codegen failed.");
}
