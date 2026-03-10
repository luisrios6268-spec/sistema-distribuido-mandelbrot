use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use image::{ImageBuffer, Rgb};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MAX_ITER: u32 = 1000;
const BLOCK_SIZE: u32 = 50;

#[derive(Serialize)]
struct Task {
    x_start: f64,
    x_end: f64,
    y_start: f64,
    y_end: f64,
    width: u32,
    height: u32,
    max_iter: u32,
    start_row: u32,
    end_row: u32,
}

#[derive(Deserialize)]
struct ResultData {
    start_row: u32,
    end_row: u32,
    data: Vec<u32>,
}

struct AppState {
    next_row: Mutex<u32>,
    image_data: Mutex<Vec<u32>>,
    completed_rows: Mutex<u32>,
}

fn save_image(data: &Vec<u32>) {
    let mut img = ImageBuffer::new(WIDTH, HEIGHT);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let value = data[(y * WIDTH + x) as usize];

            let color = if value >= MAX_ITER {
                [0, 0, 0]
            } else {
                let c = (255 * value / MAX_ITER) as u8;
                [c, c, 255]
            };

            img.put_pixel(x, y, Rgb(color));
        }
    }

    img.save("mandelbrot.png").unwrap();
    println!("Imagen guardada como mandelbrot.png");
}

#[get("/task")]
async fn get_task(data: web::Data<AppState>) -> impl Responder {
    let mut next_row = data.next_row.lock().unwrap();

    if *next_row >= HEIGHT {
        return HttpResponse::NoContent().finish();
    }

    let start = *next_row;
    let end = (*next_row + BLOCK_SIZE).min(HEIGHT);

    *next_row = end;

    let task = Task {
        x_start: -2.0,
        x_end: 1.0,
        y_start: -1.5,
        y_end: 1.5,
        width: WIDTH,
        height: HEIGHT,
        max_iter: MAX_ITER,
        start_row: start,
        end_row: end,
    };

    println!("Asignando filas {} a {}", start, end);

    HttpResponse::Ok().json(task)
}

#[post("/result")]
async fn receive_result(
    data: web::Data<AppState>,
    result: web::Json<ResultData>,
) -> impl Responder {

    let mut image = data.image_data.lock().unwrap();
    let mut completed = data.completed_rows.lock().unwrap();

    let start_index = result.start_row as usize * WIDTH as usize;

    for (i, value) in result.data.iter().enumerate() {
        image[start_index + i] = *value;
    }

    println!(
        "Resultado recibido filas {} a {}",
        result.start_row, result.end_row
    );

    *completed += result.end_row - result.start_row;

    println!("Progreso: {} / {}", *completed, HEIGHT);

    if *completed >= HEIGHT {
        println!("Todas las filas recibidas. Generando imagen...");
        save_image(&image);
    }

    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Coordinator running on port 3000");

    let image_buffer = vec![0; (WIDTH * HEIGHT) as usize];

    let state = web::Data::new(AppState {
        next_row: Mutex::new(0),
        image_data: Mutex::new(image_buffer),
        completed_rows: Mutex::new(0),
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
