# Proyecto_2_MYP
Aquí se incluye el proyecto 2 del curso de Modelado y Programación 2026-2 de la carrera de Ciencias de la Computación.

## Ejecutar con Docker y GUI

Esta configuración crea una imagen que compila y ejecuta la app GTK4.

### 1) Construir imagen

Desde la raíz del proyecto:

docker build -t proyecto2-gui .

### 2) Ejecutar en Linux (X11)

Permite acceso del contenedor al servidor X:

xhost +local:docker

Ejecutar el contenedor:

docker run --rm \
	-e DISPLAY=$DISPLAY \
	-v /tmp/.X11-unix:/tmp/.X11-unix \
	-v $HOME/Musica:/music:ro \
	-v $(pwd)/docker-data:/data \
	proyecto2-gui

Al terminar, se pueden revocar permisos:

xhost -local:docker

### 3) Ejecutar en macOS

1. Instalar y abrir XQuartz.
2. En XQuartz, habilitar conexiones de red (Security).
3. Ejecutar en terminal:

xhost + 127.0.0.1

docker run --rm \
	-e DISPLAY=host.docker.internal:0 \
	-v $HOME/Music:/music:ro \
	-v $(pwd)/docker-data:/data \
	proyecto2-gui

### 4) Ejecutar en Windows

1. Instalar y abrir VcXsrv (o Xming).
2. Iniciar el servidor X permitiendo conexiones.
3. Ejecutar en PowerShell:

docker run --rm ^
	-e DISPLAY=host.docker.internal:0.0 ^
	-v ${env:USERPROFILE}\Music:/music:ro ^
	-v ${PWD}\docker-data:/data ^
	proyecto2-gui

## Variables de entorno de la app

- MUSIC_SCAN_DIR: carpeta que se recorre para encontrar mp3. Default: /music.
- DB_PATH: ruta del archivo sqlite. Default: data/music.db (en Docker se configura como /data/music.db).
