use actix_web::{get, web, App, HttpServer, HttpResponse};
use scraper::{Selector, Html};
use reqwest;
use std::error::Error;
use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct Res {
    isError: bool,
    schemaVersion: u8,
    label: String,
    message: String,
    color: String,
    cacheSeconds: u16,
    logoSvg: String
}


#[derive(Debug)]
struct SaunadgeError(String);

impl fmt::Display for SaunadgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SaunadgeError {}

fn get_sakatsu(id : &str) -> Result<String, Box<dyn Error>> {
    let selector = Selector::parse(".p-localNav_count").unwrap();

    let url = format!("https://sauna-ikitai.com/saunners/{}", id);
    let res = reqwest::blocking::get(&url)?;
    let status = res.status();
    if status != reqwest::StatusCode::OK{
        return Err(Box::new(SaunadgeError("failed to get sakatsu by id".into())));
    };

    let body = res.text()?;

    let document = Html::parse_document(&body);
    let element = document.select(&selector).next().unwrap();
    let counts = element.text().collect::<Vec<_>>();
    let sakatsu = counts.first().unwrap();
    Ok(sakatsu.to_string())
}


#[get("/api/v1/badge/{id}")]
async fn badge(web::Path(id): web::Path<String>) -> HttpResponse {
    let sakatsu = match get_sakatsu(&id){
        Ok(s) => s,
        Err(e) => {
            println!("{}",e );
            return HttpResponse::InternalServerError()
                .json(Res {
                    isError: true,
                    schemaVersion: 1,
                    label: "Sakatsu".to_string(),
                    message: "error".to_string(),
                    color: "0051e0".to_string(),
                    cacheSeconds: 1800,
                    logoSvg: r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 54.5 57.5"><defs><style>.cls-1{fill:#fff;}</style></defs><g id="レイヤー_2" data-name="レイヤー 2"><g id="レイヤー_1-2" data-name="レイヤー 1"><path class="cls-1" d="M6.6,36,6,46.2c7.5-3.5,13.1-6.1,16.1-7.4l.3-7.7c2.6-1.8,4.7-2.4,6.8-1.9C27.1,39.2,16.4,55,14.3,57.5,23.1,55.4,37.1,52,37.1,52A69.42,69.42,0,0,0,47.2,25.3c3.5.4,5.6-.3,6.4-1.7l.9-18.1a8.94,8.94,0,0,1-5.9,1.2L48.9,0,30.2,4.4l-.4,6.5a10.93,10.93,0,0,0-6.3,1.5l.3-6.5L8.1,9.9l-.5,7.3c-1.5.7-4,.7-6.6.4L0,36.5C2.3,37.2,4.4,37.4,6.6,36Z"/></g></g></svg>"#.to_string(),
                });
        }
    };

   HttpResponse::Ok()
    .json(Res {
        isError: false,
        schemaVersion: 1,
        label: "Sakatsu".to_string(),
        message: sakatsu,
        color: "0051e0".to_string(),
        cacheSeconds: 1800,
        logoSvg: r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 54.5 57.5"><defs><style>.cls-1{fill:#fff;}</style></defs><g id="レイヤー_2" data-name="レイヤー 2"><g id="レイヤー_1-2" data-name="レイヤー 1"><path class="cls-1" d="M6.6,36,6,46.2c7.5-3.5,13.1-6.1,16.1-7.4l.3-7.7c2.6-1.8,4.7-2.4,6.8-1.9C27.1,39.2,16.4,55,14.3,57.5,23.1,55.4,37.1,52,37.1,52A69.42,69.42,0,0,0,47.2,25.3c3.5.4,5.6-.3,6.4-1.7l.9-18.1a8.94,8.94,0,0,1-5.9,1.2L48.9,0,30.2,4.4l-.4,6.5a10.93,10.93,0,0,0-6.3,1.5l.3-6.5L8.1,9.9l-.5,7.3c-1.5.7-4,.7-6.6.4L0,36.5C2.3,37.2,4.4,37.4,6.6,36Z"/></g></g></svg>"#.to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(badge))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
