use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::path::PathBuf;
use std::io::Write;
use headless_chrome::{Browser, LaunchOptions};
use anyhow::Result;

async fn convert_html_to_pdf(html_path: &PathBuf) -> Result<Vec<u8>> {
    let browser = Browser::new(LaunchOptions::default())?;
    let tab = browser.new_tab()?;
    
    tab.navigate_to(&format!("file://{}", html_path.display()))?;
    tab.wait_until_navigated()?;
    
    let pdf_bytes = tab.print_to_pdf(None)?;
    Ok(pdf_bytes)
}

async fn upload_html(mut payload: Multipart) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => return HttpResponse::BadRequest().body("Upload error"),
        };

        let filename = field.content_disposition()
            .get_filename()
            .unwrap_or("default.html");
        
        let filepath = std::env::temp_dir().join(filename);
        
        let mut file = match std::fs::File::create(&filepath) {
            Ok(file) => file,
            Err(_) => return HttpResponse::InternalServerError().body("Could not create file"),
        };

        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(data) => data.to_vec(),
                Err(_) => return HttpResponse::BadRequest().body("Upload error"),
            };
            
            if let Err(_) = file.write_all(&data) {
                return HttpResponse::InternalServerError().body("Write error");
            }
        }

        match convert_html_to_pdf(&filepath).await {
            Ok(pdf_bytes) => {
                std::fs::remove_file(&filepath).ok(); // Clean up temp file
                return HttpResponse::Ok()
                    .content_type("application/pdf")
                    .body(pdf_bytes);
            },
            Err(_) => return HttpResponse::InternalServerError().body("PDF conversion failed")
        }
    }

    HttpResponse::BadRequest().body("No file uploaded")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload_html))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}