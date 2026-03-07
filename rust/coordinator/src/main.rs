use actix_web::{get, post, App, HttpServer, Responder, HttpResponse, web};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

struct AppState {
    next_row: Mutex<u32>,
}

#[derive(Serialize)]
struct Task {
    start_row: u32,
    end_row: u32,
    width: u32,
    height: u32,
    max_iter: u32,
}

#[get("/task")]
async fn get_task(data: web::Data<AppState>) -> impl Responder {
    let mut next_row = data.next_row.lock().unwrap();

    if *next_row >= 600 {
        return HttpResponse::NoContent().finish();
    }

    let start = *next_row;
    let end = (*next_row + 50).min(600);

    *next_row = end;

    let task = Task {
        start_row: start,
        end_row: end,
        width: 800,
        height: 600,
        max_iter: 1000,
    };

    HttpResponse::Ok().json(task)
}

#[derive(Deserialize)]
struct ResultData {
    start_row: u32,
    data: Vec<u8>,
}

#[post("/result")]
async fn receive_result(info: web::Json<ResultData>) -> impl Responder {
    println!("Resultado recibido desde fila {}", info.start_row);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Coordinator running...");

    let state = web::Data::new(AppState {
        next_row: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_task)
            .service(receive_result)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
