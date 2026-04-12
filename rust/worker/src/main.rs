use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration};

#[derive(Debug, Deserialize)]
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

#[derive(Serialize)]
struct ResultData {
    start_row: u32,
    end_row: u32,
    data: Vec<u32>,
}

fn mandelbrot(c_re: f64, c_im: f64, max_iter: u32) -> u32 {
    let mut z_re = 0.0;
    let mut z_im = 0.0;

    for i in 0..max_iter {
        let new_re = z_re * z_re - z_im * z_im + c_re;
        let new_im = 2.0 * z_re * z_im + c_im;

        z_re = new_re;
        z_im = new_im;

        if z_re * z_re + z_im * z_im > 4.0 {
            return i;
        }
    }

    max_iter
}

fn main() {
    println!("Worker iniciado...");
    let client = Client::new();

    // 🔥 USAR VARIABLE DE ENTORNO
    let base_url = std::env::var("COORDINATOR_URL")
        .unwrap_or("http://10.236.223.107:3000".to_string());

    let task_url = format!("{}/task", base_url);
    let result_url = format!("{}/result", base_url);

    loop {
        let response = match client
            .get(&task_url)
            .send()
        {
            Ok(resp) => resp,
            Err(_) => {
                println!("No se pudo conectar al coordinator. Reintentando...");
                thread::sleep(Duration::from_secs(2));
                continue;
            }
        };

        if response.status() == 204 {
            println!("No hay tareas disponibles. Esperando...");
            thread::sleep(Duration::from_secs(2));
            continue;
        }

        let task: Task = match response.json() {
            Ok(t) => t,
            Err(_) => {
                println!("Error al leer la tarea. Reintentando...");
                thread::sleep(Duration::from_secs(2));
                continue;
            }
        };

        println!(
            "Calculando filas {} a {}",
            task.start_row, task.end_row
        );

        let mut result = Vec::new();

        for row in task.start_row..task.end_row {
            for col in 0..task.width {
                let x = task.x_start
                    + (col as f64 / task.width as f64)
                        * (task.x_end - task.x_start);

                let y = task.y_start
                    + (row as f64 / task.height as f64)
                        * (task.y_end - task.y_start);

                let value = mandelbrot(x, y, task.max_iter);
                result.push(value);
            }
        }

        let result_data = ResultData {
            start_row: task.start_row,
            end_row: task.end_row,
            data: result,
        };

        match client
            .post(&result_url)
            .json(&result_data)
            .send()
        {
            Ok(_) => println!("Resultado enviado al coordinator."),
            Err(_) => println!("Error enviando resultado."),
        }
    }
}
