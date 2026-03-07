use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

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

    loop {
        let response = client
           .get("http://10.10.10.1:3000/task")
            .send()
            .unwrap();

        if response.status() == 204 {
            println!("No more tasks. Worker finaliza.");
            break;
        }

        let task: Task = response.json().unwrap();

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

        client
            .post("http://10.10.10.1:3000/result")
            .json(&result_data)
            .send()
            .unwrap();
    }
}
