use std::fs;
use std::path::PathBuf;

pub fn copy_files_except_ext(
  from: &PathBuf,
  to: &PathBuf,
  avoid_dir: Option<&PathBuf>,
  ignore_ext_lst: &[&str],
) {
  if from != to {
    for entry_result in fs::read_dir(from).unwrap() {
      let entry = entry_result.unwrap();
      let metadata = &entry.metadata().unwrap();
      if metadata.is_file() {
        let path = entry.path();
        let ext_str_opt = path.extension().map(|ext| ext.to_str().unwrap());
        let b = match ext_str_opt {
          Some(ext_str) => !ignore_ext_lst
            .iter()
            .any(|ignore_ext| ignore_ext == &ext_str),
          None => true,
        };
        if b {
          fs::copy(entry.path(), to.join(entry.path().file_name().unwrap())).unwrap();
        }
      } else if metadata.is_dir() {
        let b = match avoid_dir {
          None => true,
          Some(avoid) => &entry.path() != avoid,
        };
        if (&entry.path() != to) && b {
          if let Ok(()) = fs::create_dir(&to.join(entry.file_name())) {};
          copy_files_except_ext(
            &from.join(entry.file_name()),
            &to.join(entry.file_name()),
            avoid_dir,
            ignore_ext_lst,
          );
        }
      }
    }
  }
}
