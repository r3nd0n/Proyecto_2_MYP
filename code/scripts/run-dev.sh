#!/usr/bin/env bash
set -euo pipefail

# run-dev: ayuda a preparar folders y correr docker composew
# Uso: dentro de `code/` directorio: ./scripts/run-dev.sh

BASE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$BASE_DIR"

echo "Preparing folders in $BASE_DIR"
mkdir -p data music

if [ ! -f .env ]; then
  if [ -f .env.example ]; then
    cp .env.example .env
    echo "Created .env from .env.example"
  else
    echo "Warning: .env.example not found; create .env manually if needed"
  fi
fi

# Revisar accesibilidad
if docker info > /dev/null 2>&1; then
  echo "Docker daemon reachable. Starting docker compose..."
  docker compose up --build
else
  echo "Docker daemon not reachable from this user. Useful checks and options:" >&2
  echo "  1) Run with sudo (temporary): sudo docker compose up --build" >&2
  echo "  2) Add your user to the 'docker' group and re-login (recommended): sudo usermod -aG docker \$USER" >&2
  echo "  3) Inspect socket permissions: ls -l /var/run/docker.sock" >&2
  echo "  4) If you want to run headless, run the binary directly inside a container or use a non-GUI mode." >&2
  exit 1
fi
