pub fn print_version() {
  const ARCH: &str = "amd64";
  const VERSION: &str = "0.1.0";
  const COMMIT_ID: &str = "301a7154";

  println!("Arch: {}", ARCH);
  println!("Version: {}", VERSION);
  println!("Commit Id: {}", COMMIT_ID);
}
