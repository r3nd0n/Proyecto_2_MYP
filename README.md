# Proyecto_2_MYP
Pequeña app Rust/GTK para indexar y buscar música (Proyecto 2 — Modelado y Programación 2026-2).

Este repositorio puede ejecutarse en Docker. Las instrucciones y archivos han sido adaptados para que un usuario pueda clonar el repo y ejecutar todo sin depender de rutas de usuario específicas.

## Preparar el repositorio antes de ejecutar
En la raíz del repo crea las carpetas que el contenedor montará:

```bash
mkdir -p code/data
mkdir -p code/music
```

Coloca algunos archivos mp3 en `code/music` si quieres probar el minero.

## Ejecutar con Docker (recomendado: docker compose)
1. Desde la carpeta `code` construye y levanta con Compose:

```bash
cd code
cp .env.example .env  
docker compose up --build
```

Esto:
- construye la imagen usando `code/Dockerfile`.
- monta `./data` como persistencia de la base de datos y `./music` como carpeta de escaneo.
- pasa `DISPLAY` del host al contenedor para la GUI (si usas X11).

Opcional: helper script

Dentro de `code/` hay un pequeño helper que prepara las carpetas y lanza `docker compose`:

```bash
./scripts/run-dev.sh
```

El script intentará copiar `.env.example` a `.env` si falta, creará `data/` y `music/`, y comprobará si el demonio Docker es accesible desde el usuario.

## Ejecutar con `docker run` (alternativa)
Desde `code/` (ejemplo portable):

```bash
docker build -t proyecto2-gui .
docker run --rm -e DISPLAY="$DISPLAY" \
  -v /tmp/.X11-unix:/tmp/.X11-unix:rw \
  -v "$(pwd)/music":/music:ro \
  -v "$(pwd)/data":/app/data \
  proyecto2-gui
```

## Notas sobre la GUI (X11 / Wayland)
- Para mostrar la UI desde el contenedor en Linux X11: puedes necesitar permitir conexiones al servidor X temporalmente en el host:

```bash
xhost +local:root
# ejecutar docker (ejemplo arriba)
xhost -local:root
```

## Permisos de Docker
- Si reciben `permission denied` al acceder a `/var/run/docker.sock`, pueden añadir su usuario al grupo `docker` (acción local del profesor, no parte del repo):

```bash
sudo usermod -aG docker $USER
# después, cerrar sesión y volver a entrar o usar `newgrp docker`
```

No incluimos cambios de usuarios ni comandos `sudo` en scripts del repo.

## Archivo de ejemplo `.env`
- `code/.env.example` contiene valores por defecto:

- `DB_PATH=/app/data/music.db`
- `MUSIC_SCAN_DIR=/music`