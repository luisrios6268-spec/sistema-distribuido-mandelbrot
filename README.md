# 🌀 Mandelbrot Distribuido con Rust y Docker

Este proyecto implementa un sistema de **computación distribuida** para generar el **Conjunto de Mandelbrot** utilizando una arquitectura **Coordinator–Workers**.

El sistema divide el plano complejo en bloques que son procesados en paralelo por múltiples workers. Cada worker calcula una parte del fractal y envía los resultados al coordinator, quien reconstruye la imagen final.

---

## 🧰 Tecnologías utilizadas

- 🦀 Rust
- ⚡ Actix Web
- 🐳 Docker
- 📦 Docker Compose
- 🌐 Comunicación HTTP
- 🧠 Arquitectura distribuida
- 🔗 Red virtual con ZeroTier

---

## 📁 Estructura del proyecto


mandelbrot-distribuido
│
├── docker
│ ├── docker-compose.yml
│ ├── coordinator.Dockerfile
│ └── worker.Dockerfile
│
├── rust
│ ├── coordinator
│ │ ├── Cargo.toml
│ │ └── src/main.rs
│ │
│ └── worker
│ ├── Cargo.toml
│ └── src/main.rs
│
├── vpn
│ └── configuraciones_sanitizadas
│
├── docs
│ └── reportes
│
└── README.md


---

## ⚙️ Requisitos

Antes de ejecutar el proyecto necesitas:

- Rust
- Cargo
- Docker
- Docker Compose
- Cuenta en ZeroTier

Verificar instalación:

```bash
docker --version
docker compose version
🌐 Configuración de red con ZeroTier

Para permitir la comunicación entre nodos distribuidos se utiliza ZeroTier, que crea una red privada virtual entre máquinas.

1. Instalar ZeroTier

En Linux:

curl -s https://install.zerotier.com | sudo bash
2. Unirse a la red
sudo zerotier-cli join <NETWORK_ID>
3. Autorizar el dispositivo
Ir a: https://my.zerotier.com
Autorizar el nodo en la red creada
4. Verificar conexión
ip a

o

ping <IP_DE_OTRO_NODO>
🏗️ Arquitectura del sistema
1 nodo Coordinator
N máquinas Peer
Cada Peer ejecuta múltiples Workers en contenedores Docker

Ejemplo:

1 Coordinator
4 Peers
4 Workers por Peer
👉 Total: 16 workers en paralelo
🚀 Ejecución del Coordinator

En la máquina principal:

cd mandelbrot-distribuido

docker build -f docker/coordinator.Dockerfile -t mandelbrot-coordinator .

docker run -d \
--name coordinator \
-p 3000:3000 \
mandelbrot-coordinator

Verificar:

docker ps
⚙️ Ejecución de Workers (Peers)

En cada máquina peer:

cd mandelbrot-distribuido/docker

docker compose build
docker compose up -d

Esto iniciará múltiples workers automáticamente.

🔄 Flujo de ejecución
Los workers solicitan tareas al coordinator
El coordinator asigna bloques del plano complejo
Los workers procesan su parte del fractal
Los workers envían resultados
El coordinator reconstruye la imagen final

Salida:

mandelbrot.png
🖼️ Obtener la imagen
docker cp coordinator:/app/mandelbrot.png .
xdg-open mandelbrot.png
📈 Escalabilidad

El sistema es horizontalmente escalable:

Escalar workers en un peer

Modificar:

docker/docker-compose.yml

y agregar más servicios.

Escalar el sistema completo

Agregar más máquinas conectadas a la red de ZeroTier.

⚠️ Notas importantes
Los nodos deben estar conectados a la misma red de ZeroTier
El coordinator debe ser accesible por su IP virtual
Cada host puede ejecutar múltiples workers
Se pueden realizar pruebas locales con Docker Compose en una sola máquina
🎯 Resultado

El sistema genera una imagen del Conjunto de Mandelbrot utilizando procesamiento distribuido en múltiples nodos.

# Autor

Proyecto académico de Sistemas Distribuidos.
