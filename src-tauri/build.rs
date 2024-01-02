

fn main() {
  println!("cargo:rerun-if-changed=diesel-migrations");
  tauri_build::build()
}
