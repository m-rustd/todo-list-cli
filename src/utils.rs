use std::{path, fs, io};
use dirs::home_dir;

pub const DB_FILE: &str = ".todo_db";

// 获取db文件路径
pub fn get_db_file_path() -> path::PathBuf {
  home_dir().map(|it| it.join(DB_FILE)).unwrap_or_default()
}

// 检查db文件是否存在
pub fn db_exists() -> bool {
  let dir = get_db_file_path();
  fs::metadata(&dir).is_ok()
}

// 创建db
pub fn create_db_file() -> io::Result<()> {
  let dir = get_db_file_path();
  fs::File::create(dir)?;
  Ok(())
}

// 检查db文件是否存在，不存在就创建
pub fn check_db_file() -> io::Result<()> {
  if !db_exists() {
    create_db_file()?;
  }
  Ok(())
}

