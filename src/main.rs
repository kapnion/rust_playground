use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::path::PathBuf;
use std::io::Write;
use headless_chrome::{Browser, LaunchOptions};
use anyhow::Result;
use std::fs::File;
use std::io::Read;

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

async fn upload_xml(mut payload: Multipart) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => return HttpResponse::BadRequest().body("Upload error"),
        };

        let filename = field.content_disposition()
            .get_filename()
            .unwrap_or("default.xml");
        
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

async fn convert_xml(mut payload: Multipart) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => return HttpResponse::BadRequest().body("Upload error"),
        };

        let filename = field.content_disposition()
            .get_filename()
            .unwrap_or("default.xml");
        
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

async fn check_document(mut payload: Multipart) -> impl Responder {
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => return HttpResponse::BadRequest().body("Upload error"),
        };

        let filename = field.content_disposition()
            .get_filename()
            .unwrap_or("default.xml");
        
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

        let mut content = String::new();
        if let Err(_) = File::open(&filepath).and_then(|mut f| f.read_to_string(&mut content)) {
            return HttpResponse::InternalServerError().body("Read error");
        }

        let elements = vec![
            "CrossIndustryInvoice", "ExchangedDocumentContext", "BusinessProcessSpecifiedDocumentContextParameter",
            "ID", "GuidelineSpecifiedDocumentContextParameter", "ExchangedDocument", "TypeCode", "IssueDateTime",
            "DateTimeString", "format", "SupplyChainTradeTransaction", "ApplicableHeaderTradeAgreement",
            "BuyerReference", "SellerTradeParty", "Name", "SpecifiedLegalOrganization", "PostalTradeAddress",
            "CountryID", "SpecifiedTaxRegistration", "BuyerTradeParty", "BuyerOrderReferencedDocument",
            "ApplicableHeaderTradeDelivery", "ApplicableHeaderTradeSettlement", "InvoiceCurrencyCode",
            "SpecifiedTradeSettlementHeaderMonetarySummation", "TaxBasisTotalAmount", "TaxTotalAmount",
            "GrandTotalAmount", "DuePayableAmount"
        ];

        let mut html_content = String::from("<html><body><h1>Document Elements</h1><ul>");
        for element in elements {
            if content.contains(element) {
                html_content.push_str(&format!("<li>{}</li>", element));
            }
        }
        html_content.push_str("</ul></body></html>");

        let html_path = std::env::temp_dir().join("document_elements.html");
        if let Err(_) = File::create(&html_path).and_then(|mut f| f.write_all(html_content.as_bytes())) {
            return HttpResponse::InternalServerError().body("Write error");
        }

        match convert_html_to_pdf(&html_path).await {
            Ok(pdf_bytes) => {
                std::fs::remove_file(&filepath).ok(); // Clean up temp file
                std::fs::remove_file(&html_path).ok(); // Clean up temp file
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
            .route("/upload_xml", web::post().to(upload_xml))
            .route("/convert_xml", web::post().to(convert_xml))
            .route("/check_document", web::post().to(check_document))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}