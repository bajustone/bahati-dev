use actix_web::{get, App, HttpServer, Responder, HttpResponse, web, HttpRequest};
use tera::{Tera, Context};
use serde::Serialize;


#[derive(Serialize)]
struct IndexPage<'a> {
    title: &'a str,
}


#[get("/{path:.*}")]
async fn page_handler( req: HttpRequest, data: web::Data<AppState>)-> impl Responder{
    let path = match req.path() {
        "/" => "index.html".to_owned(),
        _ => {
            let p = req.path().trim_matches('/').to_owned();
            match p.ends_with(".html") {
                true => p,
                false => p + ".html"
            }
        }
    };

    let templates = &data.templates;
    let mut template_names = templates.get_template_names();
    let template = match template_names.find(|&t| t == path) {
        Some(t) => t,
        _ => "404.html"
    };

    let index_context: IndexPage = IndexPage{
        title: "BAHATI Justin", 
    };
    
    let body = templates.render(
        template,
        &Context::from_serialize(index_context).unwrap()
    ).unwrap();

    HttpResponse::Ok().body(body)
}

struct AppState{
    templates: Tera
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let templates =  Tera::new("templates/**/*.html").unwrap();

    let port = 8081;
    let server = HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState{templates: templates.clone()}))
        .service(page_handler)
    })
    .bind(("0.0.0.0", port))?;

    println!("Server started: http://localhost:{port}");

    server
    .run()
    .await
}
