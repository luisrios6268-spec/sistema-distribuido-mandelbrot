# 🖥️ Sistema Distribuido de Cálculo del Conjunto de Mandelbrot

**Proyecto Final — Sistemas Distribuidos**  
Universidad de Guadalajara (CUCEI) | Primavera 2026

---

## 📖 Descripción

Este proyecto implementa un sistema distribuido capaz de generar el fractal de Mandelbrot utilizando múltiples nodos conectados mediante una red virtual privada.

El sistema emplea:

- 🦀 **Rust** → cálculo paralelo de alto rendimiento  
- 🐳 **Docker** → despliegue reproducible  
- 🌐 **ZeroTier** → red privada distribuida entre nodos  
- 📋 **Scrum** → gestión ágil del desarrollo  

El procesamiento se distribuye bajo el modelo **Coordinator-Worker**, permitiendo escalabilidad horizontal.

---

## ✨ Características

- División dinámica de tareas  
- Comunicación segura entre nodos  
- Ejecución distribuida real  
- Escalabilidad por contenedores  
- Generación automática de imagen final  
- Pruebas de rendimiento de red  

---

## 🏗️ Arquitectura del Sistema

Modelo **Maestro-Trabajador** sobre VPN ZeroTier.

### 📸 Diagrama del Sistema

![Arquitectura Distribuida](docs/evidencias/diagram.png)

---

## 🛠️ Tecnologías Utilizadas

| Tecnología | Uso |
|---|---|
| Rust | Cálculo Mandelbrot |
| Docker | Contenerización |
| Docker Compose | Orquestación |
| ZeroTier | Red privada |
| iperf3 | Pruebas de red |
| GitHub | Control de versiones |

---

## 📋 Requisitos Previos

- Linux / WSL2
- Docker
- Docker Compose
- Git
- ZeroTier One
- Rust (opcional)

---

## 🚀 Instalación y Configuración

### 1️⃣ Clonar repositorio

git clone https://github.com/luisrios6268-spec/sistema-distribuido-mandelbrot.git
cd sistema-distribuido-mandelbrot
2️⃣ Configurar ZeroTier

Instalar:

curl -s https://install.zerotier.com | sudo bash

Unirse a red:

sudo zerotier-cli join 88c5b1f339bd4e00

Verificar:

zerotier-cli listnetworks

Ping entre nodos:

ping 10.236.223.107

![📸 Evidencia — Conectividad ZeroTier](docs/evidencias/zerotier_ping.png)
3️⃣ Despliegue con Docker

Nodo Coordinador:

docker-compose up -d --scale worker=4

Nodo Worker:

Editar:

COORDINATOR_URL=http://10.236.223.107:8080

Luego:

docker-compose up -d worker
🧪 Compilación Manual (Sin Docker)

Coordinator:

cd coordinator
cargo build --release
cargo run

Worker:

cd worker
cargo run
🎯 Uso del Sistema

Iniciar cálculo:

curl -X POST http://10.236.223.107:8080/start \
-H "Content-Type: application/json" \
-d '{"width":1920,"height":1080,"max_iter":1000}'

Ver logs:

docker-compose logs -f worker

Resultado:

output/mandelbrot.png

![📸 Evidencia — Imagen Generada](docs/evidencias/image_generated.png)
✅ Verificación del Sistema
Verificar ZeroTier
zerotier-cli listnetworks
Verificar contenedores
docker ps

![📸 Evidencia:](docs/evidencias/docker_ps.png)


Verificar comunicación Worker
docker-compose logs worker

![📸 Evidencia:](docs/evidencias/workers_logs.png)

Verificar imagen generada
ls output/
📊 Pruebas de Rendimiento (iperf3)

Servidor:

iperf3 -s

![📸 Evidencia:](docs/evidencias/iperf3-s.png)

Cliente:

iperf3 -c 10.236.223.107 -t 10

![📸 Evidencia:](docs/evidencias/iperf3_test.png)

Reverse Test:

iperf3 -c 10.236.223.107 -t 10 -R

![📸 Evidencia:](docs/evidencias/iperf3_test2.png)

📁 Estructura del Proyecto
.
├── coordinator/
├── worker/
├── docker-compose.yml
├── docs/
│   ├── planeacion-scrum.pdf
│   └── evidencias/
└── README.md
👥 Equipo y Roles Scrum
Integrante	Rol	Área
Cristopher Said Ramírez Ruiz	Product Owner	VPN & Red
Luis Rogelio Ríos Arellano	Product Owner	Docker
Sofía Gómez Alton	Scrum Master	Worker Rust
Jorge Iván Ramírez Llamas	Scrum Master	Coordinador Rust
📅 Metodología Scrum
Sprint	Objetivo	Estado
Sprint 1	Red ZeroTier + Rust	✅
Sprint 2	Docker distribuido	✅
Sprint 3	Integración y pruebas	✅ COMPLETADO
✅ Definition of Done
✔ Nodos conectados mediante ZeroTier
✔ Workers distribuidos funcionando
✔ Imagen Mandelbrot generada
✔ Pruebas iperf3 realizadas
✔ Documentación completa
📄 Licencia

Proyecto académico — Universidad de Guadalajara
Licencia MIT.
