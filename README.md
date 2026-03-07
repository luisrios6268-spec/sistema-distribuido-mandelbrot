# Mandelbrot Distribuido con Rust y Docker

Este proyecto implementa un sistema de **computación distribuida** para generar el **Conjunto de Mandelbrot** utilizando una arquitectura **Coordinator–Workers**.

El sistema distribuye bloques del plano complejo entre múltiples workers que calculan su porción del fractal y envían los resultados al coordinator, quien reconstruye la imagen final.

---

# Arquitectura

El sistema está compuesto por:

- **Coordinator (HUB)**  
  Asigna bloques de trabajo y reconstruye la imagen final.

- **Workers (PEERS)**  
  Solicitan tareas, calculan su región del fractal y envían los resultados.

        HUB
   [ Coordinator ]
         ↑
         ↓
        VPN
         ↓
   [ Worker ][ Worker ][ Worker ]
          PEER
Tecnologías utilizadas

Rust

Actix Web

Docker

Comunicación HTTP

Arquitectura distribuida

Estructura del proyecto
mandelbrot-distribuido
│
├── rust
│   ├── coordinator
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/main.rs
│   │
│   └── worker
│       ├── Cargo.toml
│       ├── Dockerfile
│       └── src/main.rs
Requisitos

Antes de ejecutar el proyecto necesitas:

Rust

Cargo

Docker

Conectividad de red entre coordinator y workers (VPN o red local)

Verificar Docker:

docker --version
Compilar el Coordinator

Ir a la carpeta del coordinator:

cd mandelbrot-distribuido/rust/coordinator

Compilar:

cargo build --release

Construir imagen Docker:

docker build -t mandelbrot-coordinator .
Ejecutar el Coordinator
docker run -d \
--name coordinator \
-p 3000:3000 \
mandelbrot-coordinator

Ver logs:

docker logs -f coordinator
Compilar el Worker

Ir a la carpeta del worker:

cd mandelbrot-distribuido/rust/worker

Compilar:

cargo build --release

Construir imagen Docker:

docker build -t mandelbrot-worker .
Configurar dirección del Coordinator

En el archivo:

worker/src/main.rs

Asegurarse de que el worker apunte a la IP del coordinator:

http://IP_DEL_COORDINATOR:3000/task
http://IP_DEL_COORDINATOR:3000/result

Ejemplo:

http://10.10.10.1:3000
Ejecutar Workers

En la máquina PEER ejecutar:

docker run -d mandelbrot-worker
docker run -d mandelbrot-worker
docker run -d mandelbrot-worker
docker run -d mandelbrot-worker

Esto iniciará múltiples workers en paralelo.

Flujo de ejecución

Workers solicitan tareas al coordinator

Coordinator asigna bloques de filas

Workers calculan su parte del fractal

Workers envían resultados

Coordinator reconstruye la imagen

Se genera el archivo:

mandelbrot.png
Obtener la imagen generada

Copiar la imagen desde el contenedor:

docker cp coordinator:/app/mandelbrot.png .

Abrir imagen:

xdg-open mandelbrot.png
Escalabilidad

El sistema es escalable horizontalmente.

Se pueden agregar más workers simplemente ejecutando más contenedores:

docker run -d mandelbrot-worker

Incluso en múltiples máquinas PEER conectadas a la red.

Resultado

El sistema genera una imagen del Conjunto de Mandelbrot calculada de forma distribuida utilizando múltiples workers.

Autor

Proyecto académico de sistemas distribuidos.
