use std::path;
use std::process::Command;
use toml::map;
// use toml::value;

pub fn run_satysfi(destination: &path::PathBuf, config: map::Map<String, toml::Value>) -> Vec<u8> {
  let mut args: Vec<String> = vec![destination.join("main.saty").to_str().unwrap().to_string()];
  if let Some(is_bytecomp) = config.get("is-bytecomp").map(|v| v.as_bool()).flatten() {
    if is_bytecomp {
      args.push("--bytecomp".to_string())
    }
  };
  if let Some(is_type_check_only) = config
    .get("is-type-check-only")
    .map(|v| v.as_bool())
    .flatten()
  {
    if is_type_check_only {
      args.push("--type-check-only".to_string())
    }
  };
  if let Some(is_full_path) = config.get("is-full-path").map(|v| v.as_bool()).flatten() {
    if is_full_path {
      args.push("--full-path".to_string())
    }
  };
  if let Some(is_show_fonts) = config.get("is-show-fonts").map(|v| v.as_bool()).flatten() {
    if is_show_fonts {
      args.push("--show-fonts".to_string())
    }
  };
  if let Some(output_file_name) = config.get("output-file-name").map(|v| v.as_str()).flatten() {
    let path = format!("{}/{}", destination.to_str().unwrap(), output_file_name);
    args.push(format!("--output {}", path))
  };
  if let Some(config_path) = config.get("config-path").map(|v| v.as_str()).flatten() {
    let paths = config_path.split(',').map(|s| s.trim()).collect::<Vec<_>>();
    let mut paths_str = String::new();
    for path in paths.iter() {
      paths_str.push_str(&format!("{}/{},", destination.to_str().unwrap(), path))
    }
    args.push(format!("--config {}", paths_str))
  };
  if let Some(is_no_default_config) = config
    .get("is-no-default-config")
    .map(|v| v.as_bool())
    .flatten()
  {
    if is_no_default_config {
      args.push("--no-default-config".to_string())
    }
  };
  if let Some(page_number_limit) = config
    .get("page-number-limit")
    .map(|v| v.as_integer())
    .flatten()
  {
    args.push(format!("--page-number-limit {}", page_number_limit))
  };
  if let Some(text_mode_configs) = config
    .get("text-mode-configs")
    .map(|v| v.as_str())
    .flatten()
  {
    args.push(format!("--text-mode {}", text_mode_configs))
  };
  if let Some(is_debug_show_bbox) = config
    .get("is-debug-show-bbox")
    .map(|v| v.as_bool())
    .flatten()
  {
    if is_debug_show_bbox {
      args.push("--debug-show-bbox".to_string())
    }
  };
  if let Some(is_debug_show_space) = config
    .get("is-debug-show-space")
    .map(|v| v.as_bool())
    .flatten()
  {
    if is_debug_show_space {
      args.push("--debug-show-space".to_string())
    }
  };
  if let Some(is_debug_show_block_bbox) = config
    .get("is-debug-show-block-bbox")
    .map(|v| v.as_bool())
    .flatten()
  {
    if is_debug_show_block_bbox {
      args.push("--debug-show-block-bbox".to_string())
    }
  };
  if let Some(is_debug_show_block_space) = config
    .get("is-debug-show-block-space")
    .map(|v| v.as_bool())
    .flatten()
  {
    if is_debug_show_block_space {
      args.push("--debug-show-block-space".to_string())
    }
  };
  if let Some(is_debug_show_overfull) = config
    .get("is-debug-show-overfull")
    .map(|v| v.as_bool())
    .flatten()
  {
    if is_debug_show_overfull {
      args.push("--debug-show-overfull".to_string())
    }
  };
  let run_satysfi_command = Command::new("satysfi").args(&args).output().unwrap();
  let mut arg_str = String::new();
  for s in args.iter() {
    arg_str.push_str(&format!(" {}", s))
  }
  println!("satysfi {}", arg_str);
  run_satysfi_command.stdout
}
