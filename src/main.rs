use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_multipart::Multipart;
use futures_util::StreamExt;
use std::path::PathBuf;
use std::io::Write;
use headless_chrome::{Browser, LaunchOptions};
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use quick_xml::Reader;
use quick_xml::events::Event;

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

        let elements: Vec<(String, &str)> = vec![
            ("CrossIndustryInvoice".to_string(), "BG-0"), ("ExchangedDocumentContext".to_string(), "BG-2"), 
            ("BusinessProcessSpecifiedDocumentContextParameter".to_string(), "BT-23-00"), ("ID".to_string(), "BT-23"), 
            ("GuidelineSpecifiedDocumentContextParameter".to_string(), "BT-24-00"), ("ID".to_string(), "BT-24"), 
            ("ExchangedDocument".to_string(), "BT-1-00"), ("ID".to_string(), "BT-1"), ("TypeCode".to_string(), "BT-3"), 
            ("IssueDateTime".to_string(), "BT-2-00"), ("DateTimeString".to_string(), "BT-2"), ("format".to_string(), "BT-2-0"), 
            ("SupplyChainTradeTransaction".to_string(), "BG-25-00"), ("ApplicableHeaderTradeAgreement".to_string(), "BT-10-00"), 
            ("BuyerReference".to_string(), "BT-10"), ("SellerTradeParty".to_string(), "BG-4"), ("Name".to_string(), "BT-27"), 
            ("SpecifiedLegalOrganization".to_string(), "BT-30-00"), ("ID".to_string(), "BT-30"), ("schemeID".to_string(), "BT-30-1"), 
            ("PostalTradeAddress".to_string(), "BG-5"), ("CountryID".to_string(), "BT-40"), 
            ("SpecifiedTaxRegistration".to_string(), "BT-31-00"), ("ID".to_string(), "BT-31"), ("schemeID".to_string(), "BT-31-0"), 
            ("SpecifiedTaxRegistration".to_string(), "BT-32-00"), ("ID".to_string(), "BT-32"), ("schemeID".to_string(), "BT-32-0"), 
            ("BuyerTradeParty".to_string(), "BG-7"), ("Name".to_string(), "BT-44"), ("SpecifiedLegalOrganization".to_string(), "BT-47-00"), 
            ("ID".to_string(), "BT-47"), ("schemeID".to_string(), "BT-47-1"), ("BuyerOrderReferencedDocument".to_string(), "BT-13-00"), 
            ("IssuerAssignedID".to_string(), "BT-13"), ("ApplicableHeaderTradeDelivery".to_string(), "BG-13-00"), 
            ("ApplicableHeaderTradeSettlement".to_string(), "BG-19"), ("InvoiceCurrencyCode".to_string(), "BT-5"), 
            ("SpecifiedTradeSettlementHeaderMonetarySummation".to_string(), "BG-22"), 
            ("TaxBasisTotalAmount".to_string(), "BT-109"), ("TaxTotalAmount".to_string(), "BT-110"), 
            ("currencyID".to_string(), "BT-110-0"), ("GrandTotalAmount".to_string(), "BT-112"), 
            ("DuePayableAmount".to_string(), "BT-115")
        ];

        let mut reader = Reader::from_str(&content);
        reader.trim_text(true);
        let mut buf = Vec::new();

        let mut current_path = Vec::new();
        let mut html_content = String::from("<html><body><h1>Document Elements</h1><ul>");

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    current_path.push(String::from_utf8_lossy(e.name()).to_string());
                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            let key = String::from_utf8_lossy(attr.key).to_string();
                            let value = attr.unescape_and_decode_value(&reader).unwrap();
                            let path = current_path.join(" > ");
                            html_content.push_str(&format!("<li>{} @{}: {}</li>", path, key, value));
                        }
                    }
                },
                Ok(Event::End(ref _e)) => {
                    current_path.pop();
                },
                Ok(Event::Text(e)) => {
                    let text = e.unescape_and_decode(&reader).unwrap();
                    if !text.trim().is_empty() {
                        let path = current_path.join(" > ");
                        if let Some((_, label)) = elements.iter().find(|(el, _)| current_path.contains(el)) {
                            html_content.push_str(&format!("<li>{} ({}): {}</li>", path, label, text));
                        }
                    }
                },
                Ok(Event::Eof) => break,
                Err(e) => return HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
                _ => (),
            }
            buf.clear();
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