fn main() {
  cc::Build::new()
    .file("c/layout.c")
    .compile("layout");
  println!("cargo::rerun-if-changed=c/layout.c");
  println!("cargo::rustc-link-arg=-llayout");
}