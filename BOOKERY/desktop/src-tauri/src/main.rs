// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;

// use shared::structs::{
//     author::Author, book::BookWithAuthor, costumer::Costumer, rental::RentalWithCostumerAndBook,
// };

#[derive(Deserialize, Clone)]
enum Module {
    Author,
    Book,
    Costumer,
    Rental,
}

#[derive(Deserialize)]
enum ModuleAction {
    Create,
    Delete,
    Update,
    GetRaw,
    Get,
    Search,
    SearchRaw,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_actual_endpoint,
            create_table_head
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn dispatch_module_endpoint(module: Module) -> &'static str {
    match module {
        Module::Author => "author",
        Module::Book => "book",
        Module::Costumer => "costumer",
        Module::Rental => "rental",
    }
}

fn dispatch_action_endpoint(action: ModuleAction, token: String) -> String {
    match action {
        ModuleAction::Create => String::from("create"),
        ModuleAction::Delete => String::from("delete"),
        ModuleAction::Update => String::from("update"),
        ModuleAction::GetRaw => String::from("get-raw"),
        ModuleAction::Get => String::from("get"),
        ModuleAction::Search => format!("search?token={token}"),
        ModuleAction::SearchRaw => format!("search-raw?token={token}"),
    }
}

#[tauri::command]
fn handle_actual_endpoint(module: Module, action: ModuleAction) -> String {
    let module: &str = dispatch_module_endpoint(module);
    let action: String = dispatch_action_endpoint(action, "".to_string());

    format!("http://localhost:3000/{module}/{action}")
}

#[tauri::command]
fn create_table_head(module: Module) -> &'static str {
    match module {
        Module::Author => {
            "<tr>
                <th>Name</th>
                <th>Born</th>
            </tr>"
        }
        Module::Book => {
            "<tr>
                <th>Name</th>
                <th>Author</th>
                <th>Editor</th>
                <th>Release</th>
            </tr>"
        }
        Module::Costumer => {
            "<tr>
                <th>Name</th>
                <th>Document</th>
                <th>Born</th>
            </tr>"
        }
        Module::Rental => {
            "<tr>
                <th>Costumer</th>
                <th>Book</th>
                <th>Borrowed at</th>
                <th>Due Date</th>
                <th>Returned at</th>
            </tr>"
        }
    }
}
