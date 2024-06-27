// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;

use reqwest::get;
use serde::Deserialize;

use shared::structs::{
    author::Author, book::BookWithAuthor, costumer::Costumer, rental::RentalWithCostumerAndBook,
};

static API_URL: &str = env!("API_URL");

#[derive(Deserialize, Clone)]
enum Module {
    Author,
    Book,
    Costumer,
    Rental,
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            create_table_head,
            create_table_body_search_author,
            create_table_body_search_book,
            create_table_body_search_costumer,
            create_table_body_search_rental,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_table_head(module: Module) -> &'static str {
    match module {
        Module::Author => {"<tr><th>Name</th><th>Born</th></tr>"}
        Module::Book => {"<tr><th>Name</th><th>Author</th><th>Editor</th><th>Release</th></tr>"}
        Module::Costumer => {"<tr><th>Name</th><th>Document</th><th>Born</th></tr>"}
        Module::Rental => {"<tr><th>Costumer</th><th>Book</th><th>Borrowed at</th><th>Due Date</th><th>Returned at</th></tr>"}
    }
}

#[tauri::command]
async fn create_table_body_search_author() -> Result<String, &'static str> {
    let mut table_body_vec: Vec<String> = vec![];
    let endpoint: String = format!("{API_URL}/author/search?token=");

    let response: reqwest::Response = match get(endpoint).await {
        Ok(res) => res,
        Err(_) => return Err("Failed to fetch data: API is out."),
    };

    let data: Vec<Author> = match response.json::<Vec<Author>>().await {
        Ok(data) => data,
        Err(_) => return Err("Failed to parse data: object response different from expected."),
    };

    if data.is_empty() {
        return Ok("<tr><td>-</td><td>-</td></tr>".to_string());
    }

    for author in data {
        table_body_vec.push(format!(
            "<tr><td>{}</td><td>{}</td></tr>",
            author.name.as_str(),
            author.born
        ));
    }

    Ok(table_body_vec.join(""))
}

#[tauri::command]
async fn create_table_body_search_book() -> Result<String, &'static str> {
    let mut table_body_vec: Vec<String> = vec![];
    let endpoint: String = format!("{API_URL}/book/search?token=");

    let response: reqwest::Response = match get(endpoint).await {
        Ok(res) => res,
        Err(_) => return Err("Failed to fetch data: API is out."),
    };

    let data: Vec<BookWithAuthor> = match response.json::<Vec<BookWithAuthor>>().await {
        Ok(data) => data,
        Err(_) => return Err("Failed to parse data: object response different from expected."),
    };

    if data.is_empty() {
        return Ok("<tr><td>-</td><td>-</td><td>-</td><td>-</td></tr>".to_string());
    }

    for book in data {
        table_body_vec.push(format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            book.name.as_str(),
            book.author_name.as_str(),
            book.editor.as_str(),
            book.release
        ));
    }

    Ok(table_body_vec.join(""))
}

#[tauri::command]
async fn create_table_body_search_costumer() -> Result<String, &'static str> {
    let mut table_body_vec: Vec<String> = vec![];
    let endpoint: String = format!("{API_URL}/costumer/search?token=");

    let response: reqwest::Response = match get(endpoint).await {
        Ok(res) => res,
        Err(_) => return Err("Failed to fetch data: API is out."),
    };

    let data: Vec<Costumer> = match response.json::<Vec<Costumer>>().await {
        Ok(data) => data,
        Err(_) => return Err("Failed to parse data: object response different from expected."),
    };

    if data.is_empty() {
        return Ok("<tr><td>-</td><td>-</td><td>-</td></tr>".to_string());
    }

    for costumer in data {
        table_body_vec.push(format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
            costumer.name.as_str(),
            costumer.document.as_str(),
            costumer.born,
        ));
    }

    Ok(table_body_vec.join(""))
}

#[tauri::command]
async fn create_table_body_search_rental() -> Result<String, &'static str> {
    let mut table_body_vec: Vec<String> = vec![];
    let endpoint: String = format!("{API_URL}/rental/search?token=");

    let response: reqwest::Response = match get(endpoint).await {
        Ok(res) => res,
        Err(_) => return Err("Failed to fetch data: API is out."),
    };

    let data: Vec<RentalWithCostumerAndBook> =
        match response.json::<Vec<RentalWithCostumerAndBook>>().await {
            Ok(data) => data,
            Err(_) => return Err("Failed to parse data: object response different from expected."),
        };

    if data.is_empty() {
        return Ok("<tr><td>-</td><td>-</td><td>-</td><td>-</td><td>-</td></tr>".to_string());
    }

    for rental in data {
        let mut rental_returned_at: String = String::new();

        if rental.returned_at.is_none() {
            rental_returned_at.push('-');
        } else if rental.returned_at.is_some() {
            rental_returned_at = format!("{}", rental.returned_at.unwrap());
        }

        table_body_vec.push(format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            rental.costumer_name.as_str(),
            rental.book_name.as_str(),
            rental.borrowed_at,
            rental.due_date,
            rental_returned_at
        ));
    }

    Ok(table_body_vec.join(""))
}
