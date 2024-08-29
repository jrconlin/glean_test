use glean_build::Builder;

fn main() {
    Builder::default()
        .file("tags.yaml") // make sure to include all required files in order
        .file("metrics.yaml")
        .file("pings.yaml")
        .generate()
        .expect("Error generating Glean Rust bindings");
}
