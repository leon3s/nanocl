lazy_static! {
  static ref RUST_VERSION: String = std::env::var("RUST_VERSION")
    .unwrap_or_else(|_| String::from("only set on release build."));
  static ref VERSION: String = std::env::var("VERSION")
    .unwrap_or_else(|_| String::from("only set on release build."));
  static ref COMMIT_ID: String = std::env::var("COMMIT_ID")
    .unwrap_or_else(|_| String::from("only set on release build."));
  static ref ARCH: String = std::env::var("ARCH")
    .unwrap_or_else(|_| String::from("only set on release build."));
}

pub fn print_version() {
  println!("Arch: {}", *ARCH);
  println!("Version: {}", *VERSION);
  println!("Rust Version: {}", *RUST_VERSION);
  println!("Commit Id: {}", *COMMIT_ID);
}
