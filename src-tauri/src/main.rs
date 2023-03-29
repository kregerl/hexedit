// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cmp::min;
use std::io::{Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::{fs::File, io::Read};

use serde::Serialize;
use tauri::api::dialog;
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

const PARTITION_SIZE: usize = 64;
#[tauri::command]
fn read_file_partitioned(path: &Path, offset: u64) -> Vec<u8> {
    let mut file = File::open(path).expect("Error opening file");
    let size = file.metadata().expect("Unable to get file metadata").len();
    let real_buffer_size = min(size as usize, PARTITION_SIZE);

    let mut buffer = vec![0u8; real_buffer_size];
    file.seek(SeekFrom::Start(offset)).expect("Error seeking");
    file.read_exact(&mut buffer)
        .expect("Error reading into buffer");

    buffer
}

fn create_menu() -> Menu {
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let file = Submenu::new("File", Menu::new().add_item(open));
    Menu::new().add_submenu(file)
}

#[derive(Serialize, Clone)]
struct PartitionedBytes {
    path: PathBuf,
    bytes: Vec<u8>,
}

fn main() {
    tauri::Builder::default()
        .menu(create_menu())
        .on_menu_event(|event| {
            let app_handle = event.window().app_handle();
            match event.menu_item_id() {
                "open" => {
                    dialog::FileDialogBuilder::new()
                        .set_title("Open file")
                        .pick_file(move |file_path| {
                            if let Some(path) = file_path {
                                let buffer = read_file_partitioned(&path, 0);
                                app_handle
                                    .emit_all(
                                        "file-opened",
                                        buffer
                                    )
                                    .expect("Error emitting event");
                            }
                        });
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![read_file_partitioned])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
