
static BASEPATH: &str = "logs";

pub fn write_to(&self, filename: &str, content: &str) {
  let mut dir_path = PathBuf::from(BASEPATH);
  dir_path.push(filename);
  // Create the directory if it does not exist
  if !dir_path.exists() {
      fs::create_dir_all(&dir_path).unwrap();
  }
  dir_path.push(filename);
  if !dir_path.exists() {
      fs::File::create(&dir_path).unwrap();
  }
  let mut file = OpenOptions::new()
      .write(true)
      .append(true)
      .open(dir_path)
      .unwrap();

  if let Err(e) = writeln!(file, "{}", content) {
      eprintln!("Couldn't write to file: {}", e);
  }
}

pub fn clear_file(&self, filename: &str) {
  let mut dir_path = PathBuf::from(self.path);
  dir_path.push(filename);
  if !dir_path.exists() {
      // Delete the file if it exists
      fs::remove_file(&dir_path).unwrap();
  }
}