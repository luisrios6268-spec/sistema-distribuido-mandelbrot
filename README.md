# Mandelbrot Distribuido con Rust y Docker

Este proyecto implementa un sistema de computación distribuida para generar el **Conjunto de Mandelbrot** utilizando una arquitectura **Coordinator–Workers**.

El sistema distribuye bloques del plano complejo entre múltiples workers que calculan su porción del fractal y envían los resultados al coordinator, quien reconstruye la imagen final.

---

# Tecnologías utilizadas

* Rust
* Actix Web
* Docker
* Docker Compose
* Comunicación HTTP
* Arquitectura distribuida
* VPN con WireGuard

---

# Estructura del proyecto

```
mandelbrot-distribuido
│
├── docker
│   ├── docker-compose.yml
│   ├── coordinator.Dockerfile
│   └── worker.Dockerfile
│
├── rust
│   ├── coordinator
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   │
│   └── worker
│       ├── Cargo.toml
│       └── src/main.rs
│
├── vpn
│   └── configuraciones_sanitizadas
│
├── docs
│   └── reportes
│
└── README.md
```

---

# Requisitos

Antes de ejecutar el proyecto necesitas tener instalado:

* Rust
* Cargo
* Docker
* Docker Compose
* Conectividad de red entre coordinator y workers (VPN o red local)

Verificar instalación de Docker:

```
docker --version
```

Verificar Docker Compose:

```
docker compose version
```

---

# Construcción y ejecución del sistema

El sistema distribuido se ejecuta en múltiples máquinas conectadas mediante una VPN.

La arquitectura es la siguiente:

- 1 máquina ejecuta el **Coordinator**
- 4 máquinas **Peer**
- Cada Peer ejecuta **4 contenedores Worker** utilizando Docker Compose

---

# Ejecución del Coordinator

En la máquina designada como HUB (Coordinator):

Ir a la carpeta del proyecto:


cd mandelbrot-distribuido


Construir la imagen del coordinator:


docker build -f docker/coordinator.Dockerfile -t mandelbrot-coordinator .


Ejecutar el coordinator:


docker run -d
--name coordinator
-p 3000:3000
mandelbrot-coordinator


Verificar ejecución:


docker ps


---

# Ejecución de Workers (Peers)

En cada máquina PEER:

Ir a la carpeta del proyecto:


cd mandelbrot-distribuido/docker


Construir las imágenes:


docker compose build


Iniciar los workers:


docker compose up -d


Esto iniciará **4 contenedores worker en cada máquina peer**.

Ver contenedores en ejecución:


docker ps

---

# Flujo de ejecución

1. Los **workers** solicitan tareas al coordinator.
2. El **coordinator** asigna bloques de filas del plano complejo.
3. Los workers calculan su parte del fractal.
4. Los workers envían los resultados al coordinator.
5. El coordinator reconstruye la imagen final.

Se genera el archivo:

```
mandelbrot.png
```

---

# Obtener la imagen generada

Copiar la imagen desde el contenedor del coordinator:

```
docker cp coordinator:/app/mandelbrot.png .
```

Abrir la imagen:

```
xdg-open mandelbrot.png
```

---

# Escalabilidad

El sistema es **escalable horizontalmente**.

Actualmente la arquitectura del proyecto contempla:

- 1 nodo **Coordinator**
- 4 máquinas **Peer**
- 4 contenedores **Worker por Peer**

Esto da un total de **16 workers ejecutándose en paralelo**.

## Escalar dentro de un Peer

Se pueden agregar más workers en una máquina Peer modificando el archivo:


docker/docker-compose.yml


y agregando más servicios worker.

## Escalar el sistema distribuido

También es posible agregar **más máquinas Peer** conectadas a la VPN.

Cada nueva máquina puede ejecutar su propio `docker-compose` con múltiples workers, aumentando la capacidad de procesamiento
---

# Configuración y uso de la VPN (WireGuard)

Para permitir la comunicación entre los nodos del sistema distribuido se utiliza una red privada virtual basada en **WireGuard**.

Cada integrante del equipo debe:

1. Instalar WireGuard en su máquina Linux.

2. Copiar su archivo de configuración correspondiente:

```
/etc/wireguard/wg0.conf
```

3. Levantar la interfaz de red:

```
sudo wg-quick up wg0
```

4. Verificar el estado de la conexión:

```
wg show
```

5. Comprobar conectividad con otros nodos de la VPN:

```
ping <IP_DEL_OTRO_NODO>
```

Las plantillas de configuración utilizadas en el proyecto se encuentran en:

```
vpn/configuraciones_sanitizadas
```

**Nota:**
En estas plantillas las llaves privadas han sido eliminadas por motivos de seguridad.

---

# Notas importantes y supuestos

* Cada integrante ejecuta los contenedores Docker en una máquina Linux independiente.
* Los nodos del sistema se comunican a través de la red VPN WireGuard.
* El coordinator es accesible mediante su dirección IP dentro de la VPN.
* Cada host puede ejecutar múltiples contenedores worker.
* Las configuraciones de WireGuard publicadas en este repositorio han sido **sanitizadas**, eliminando las llaves privadas.
* Para pruebas locales es posible ejecutar todos los contenedores en una sola máquina utilizando Docker Compose.

---

# Resultado

El sistema genera una imagen del **Conjunto de Mandelbrot** calculada de forma distribuida utilizando múltiples workers.

---

# Autor

Proyecto académico de Sistemas Distribuidos.
