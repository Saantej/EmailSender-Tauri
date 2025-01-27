// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lettre::message::{header::ContentType, Message, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use native_dialog::FileDialog;
use std::fs;

#[tauri::command]
fn send_email(to_emails: Vec<String>, html_content: &str, headers: &str) -> Result<(), String> {
    let creds = Credentials::new(
        "".to_string(),
        "".to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.yandex.ru")
        .unwrap()
        .credentials(creds)
        .build();

    for to_email in to_emails {
        let email = Message::builder()
            .from("admin@studiya-saitov.com".parse::<Mailbox>().unwrap())
            .to(to_email.parse::<Mailbox>().unwrap())
            .subject(headers)
            .header(ContentType::TEXT_HTML)
            .body(html_content.to_string())
            .expect("Failed to create email");

        if let Err(e) = mailer.send(&email) {
            return Err(e.to_string());
        }
    }

    Ok(())
}

#[tauri::command]
fn load_html_file() -> Result<String, String> {
    let file_path = FileDialog::new().add_filter("HTML file", &["html"]).show_open_single_file();

    match file_path {
        Ok(Some(path)) => {
            let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
            Ok(content)
        }
        Ok(None) => Err("No file selected".into()),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_email, load_html_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
