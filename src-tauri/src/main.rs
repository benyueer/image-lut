// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs,
    sync::{Arc, Mutex},
};

use lut::LutBuilder;
use serde::{Deserialize, Serialize};
use tauri::{api::file, State};
pub mod lut;
// use tokio::{sync::mpsc, time::error::Error};

#[derive(Debug)]
struct AppState {
    dir: Arc<Mutex<Option<String>>>,
    images: Mutex<Vec<String>>,
    lut_builder: Arc<Mutex<LutBuilder>>,
}

impl Default for AppState {
    fn default() -> Self {
        let mut lut_builder = LutBuilder::default();
        lut_builder.init_lut();
        Self { dir: Default::default(), images: Default::default(), lut_builder: Arc::new(Mutex::new(lut_builder)) }
    }
}

impl AppState {
    fn add_image(&mut self, file_name: String) {
        self.images.lock().unwrap().push(file_name);
    }

    fn reset_image(&mut self) {
        self.images.lock().unwrap().clear();
    }
}

#[derive(Serialize, Debug)]
struct ImageItem {
    name: String,
    data: String,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        // .setup(|app| {
        //     let app_handler = app.clone();
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![set_dir, read_images, get_lut_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn set_dir(new_dir: String, state: State<'_, AppState>) {
    let mut dir = state.dir.lock().unwrap();
    *dir = Some(new_dir);
}

#[tauri::command]
async fn read_images(state: State<'_, AppState>) -> Result<Vec<ImageItem>, String> {
    let mut images = vec![];
    let dir = state.dir.lock().unwrap().clone().unwrap();
    if let Ok(entries) = fs::read_dir(dir.clone()) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().into_string().unwrap();
                if file_name.ends_with(".jpg")
                    || file_name.ends_with(".JPG")
                    || file_name.ends_with(".png")
                {
                    if let Some(base64_str) = lut::image_to_base64(dir.clone() + "/" + &file_name) {
                        images.push(ImageItem {
                            name: file_name,
                            data: base64_str,
                        });
                    }
                }
            }
        }
    }

    Ok(images)
}

#[tauri::command]
async fn get_lut_image(state: State<'_, AppState>, image_path: &str, lut_name: &str) -> Result<String, String> {
    println!("lut start  {} {}", image_path, lut_name);
    let mut lut_builder = state.lut_builder.lock().unwrap();
    let dir = state.dir.lock().unwrap().clone().unwrap();

    let base64_str = lut_builder.use_lut(lut_name, &(dir.clone() + "/" + image_path));
    println!("lut end");

    Ok(base64_str.unwrap())
}
