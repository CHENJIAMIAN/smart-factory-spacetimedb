#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

echo "[1/3] build dashboard deps"
cd "$ROOT_DIR/dashboard"
pnpm install --frozen-lockfile >/dev/null

echo "[2/3] start edge gateway (background)"
cd "$ROOT_DIR/edge-gateway"
set -a
[ -f "$ROOT_DIR/.env" ] && source "$ROOT_DIR/.env"
set +a

export LIVE_JSON_PATH="${LIVE_JSON_PATH:-$ROOT_DIR/dashboard/public/live.json}"
export SPACETIME_CALL_ENABLED="${SPACETIME_CALL_ENABLED:-false}"

cargo run > "$ROOT_DIR/.edge.log" 2>&1 &
EDGE_PID=$!
echo "edge pid: $EDGE_PID (log: $ROOT_DIR/.edge.log)"

echo "[3/3] start dashboard"
cd "$ROOT_DIR/dashboard"
pnpm dev
