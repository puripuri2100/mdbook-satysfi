use anyhow::Result;
use std::process::Command;
use toml::map;

fn get_command_list(config: &map::Map<String, toml::Value>) -> Option<(String, Vec<String>)> {
  let mut command_name = "".to_string();
  let mut option_lst = vec![];
  #[cfg(target_os = "windows")]
  let os_name = "windows";
  #[cfg(target_os = "macos")]
  let os_name = "macos";
  #[cfg(target_os = "linux")]
  let os_name = "linux";
  #[cfg(all(
    not(target_os = "windows"),
    not(target_os = "macos"),
    not(target_os = "linux")
  ))]
  let os_name = "others";

  if let Some(commands_table) = config.get("commands").map(|v| v.as_table()).flatten() {
    commands_table
      .get(os_name)
      .map(|v| {
        // ["wsl", "satysfi"] or ["satysfi"] or "satysfi"
        match v.as_str() {
          Some(command_name) => Some((command_name.to_string(), vec![])),
          None => match v.as_array() {
            None => None,
            Some(array) => {
              let lst_opt = array.iter().map(|v| v.as_str()).collect::<Option<Vec<_>>>();
              match lst_opt {
                None => {
                  eprintln!("output.satysfi.pdf.command.{{os_name}} require type 'string array' or 'string'");
                  None
                }
                Some(lst) => {
                  let mut iter = lst.iter();
                  if let Some(new_command_name) = iter.next() {
                    command_name = new_command_name.to_string()
                  };
                  for command in iter {
                    option_lst.push(command.to_string())
                  }
                  Some((command_name, option_lst))
                }
              }
            }
          },
        }
      })
      .flatten()
  } else if let Some(command_lst) = config.get("commands").map(|v| v.as_array()).flatten() {
    let lst_opt = command_lst
      .iter()
      .map(|v| v.as_str())
      .collect::<Option<Vec<_>>>();
    match lst_opt {
      None => {
        eprintln!("output.satysfi.pdf.command require type 'string array' or 'string'");
        None
      }
      Some(lst) => {
        let mut iter = lst.iter();
        if let Some(new_command_name) = iter.next() {
          command_name = new_command_name.to_string()
        };
        for command in iter {
          option_lst.push(command.to_string())
        }
        Some((command_name, option_lst))
      }
    }
  } else {
    None
  }
}

pub fn run_satysfi(
  output_file_name: &str,
  config: map::Map<String, toml::Value>,
) -> Result<Vec<u8>> {
  let main_file_path = output_file_name.to_string();
  let (command_name, mut args) = match get_command_list(&config) {
    None => ("satysfi".to_string(), vec![]),
    Some((command_name, args)) => (command_name, args),
  };
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
    args.push("--output".to_string());
    args.push(output_file_name.to_string());
  };
  if let Some(config_path) = config.get("config-path").map(|v| v.as_str()).flatten() {
    args.push("--config".to_string());
    args.push(config_path.to_string());
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
    args.push("--page-number-limit".to_string());
    args.push(page_number_limit.to_string());
  };
  if let Some(text_mode_configs) = config
    .get("text-mode-configs")
    .map(|v| v.as_str())
    .flatten()
  {
    args.push("--text-mode".to_string());
    args.push(text_mode_configs.to_string());
  };
  if let Some(text_mode_configs) = config
    .get("text-mode-configs")
    .map(|v| {
      v.as_array()
        .map(|lst| lst.iter().map(|v| v.as_str()).collect::<Option<Vec<_>>>())
    })
    .flatten()
    .flatten()
  {
    args.push("--text-mode".to_string());
    args.push(text_mode_configs.join(","));
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
  args.push(main_file_path);
  println!("{} {}", command_name, args.join(" "));
  let run_satysfi_command = Command::new(command_name).args(&args).output()?;
  Ok(run_satysfi_command.stdout)
}
