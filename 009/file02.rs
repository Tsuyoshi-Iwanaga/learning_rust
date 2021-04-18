use std::fs;

fn main() -> std::io::Result<()> {
  fs::create_dir("./misc/subdir")?;
  Ok(())
}