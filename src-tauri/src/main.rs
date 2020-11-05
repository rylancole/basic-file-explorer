use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
// use std::ffi::OsString;

// The commands definitions
// Deserialized from JS
#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
enum Cmd {
  GetEntries {
    path: String,
    callback: String,
    error: String,
  },
}

#[derive(Serialize)]
struct Entry {
  label: String,
  full_path: String,
  is_folder: bool,
}

#[derive(Serialize)]
struct GetEntriesResponse {
  entries: Vec<Entry>,
}

// An error type we define
// We could also use the `anyhow` lib here
#[derive(Debug, Clone)]
struct CommandError<'a> {
  message: &'a str,
}

impl<'a> CommandError<'a> {
  fn new(message: &'a str) -> Self {
    Self { message }
  }
}

impl<'a> std::fmt::Display for CommandError<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.message)
  }
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl<'a> std::error::Error for CommandError<'a> {}

fn get_entries(dir: &Path) -> io::Result<GetEntriesResponse> {
  let mut entries: Vec<Entry> = Vec::new();

  for entry in fs::read_dir(dir)? {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir() {
      let data = Entry {
        label: entry.file_name().into_string().unwrap_or("".to_string()),
        full_path: path.to_string_lossy().to_string(),
        is_folder: true,
      };
      entries.push(data);
    } else {
      let data = Entry {
        label: entry.file_name().into_string().unwrap_or("".to_string()),
        full_path: path.to_string_lossy().to_string(),
        is_folder: false,
      };
      entries.push(data);
    }
  }
  let response = GetEntriesResponse { entries: entries };
  Ok(response)
}

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            GetEntries {
              path,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || {
                let path_obj = Path::new(&path);
                let response = get_entries(path_obj);
                if response.is_ok() {
                  Ok(response.ok())
                } else {
                  Err(CommandError::new("Could not run get_entries()").into())
                }
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
