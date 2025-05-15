use protobuf_codegen::{Codegen, Customize};

fn main() {
    // Build our realtime feed structure
    Codegen::new()
        .out_dir("src/yahoo/gen")
        .inputs(&["src/yahoo/realtime.proto"])
        .includes(&["src"])
        /*.customize(Customize {
            ..Default::default()
        })*/
        .run()
        .expect("Codegen failed.");
}
