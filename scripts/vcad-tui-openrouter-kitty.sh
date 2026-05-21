#!/usr/bin/env bash
set -euo pipefail

# Start vcad's local app API backed by OpenRouter, then launch the CLI TUI
# against that API with Kitty/termview pixel graphics enabled.
#
# Usage:
#   scripts/vcad-tui-openrouter-kitty.sh [optional/path/to/file.vcad]
#
# Env overrides:
#   OPENROUTER_KEY or OPENROUTER_API_KEY  OpenRouter key (loaded from ~/.bashrc if present)
#   OPENROUTER_MODEL                     Default: openai/gpt-4o-mini
#   VCAD_APP_PORT                        Default: 5173
#   TERMVIEW_PROTOCOL                    Default: kitty

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HOST="127.0.0.1"
if [[ -n "${VCAD_APP_PORT:-}" ]]; then
  PORT="$VCAD_APP_PORT"
else
  PORT="$(python3 - <<'PY'
import socket
for port in range(5173, 5200):
    with socket.socket() as s:
        try:
            s.bind(('127.0.0.1', port))
        except OSError:
            continue
        print(port)
        raise SystemExit
raise SystemExit('no free port in 5173..5199')
PY
)"
fi
# Use plain HTTP for the local dev API. The CLI's reqwest client correctly
# rejects Vite's self-signed HTTPS cert, which makes chat fail even though
# `curl -k` works. CLAUDE_PREVIEW=1 below disables Vite's basicSsl plugin.
APP_URL="http://${HOST}:${PORT}"
CHAT_ENDPOINT="${APP_URL}/api/chat"
LOG="${TMPDIR:-/tmp}/vcad-tui-openrouter-${PORT}.log"

# Pull in the user's OpenRouter key/model if they keep it in bashrc.
if [[ -f "${HOME}/.bashrc" ]]; then
  # shellcheck disable=SC1090
  source "${HOME}/.bashrc" >/dev/null 2>&1 || true
fi

export VCAD_CHAT_PROVIDER="${VCAD_CHAT_PROVIDER:-openrouter}"
export OPENROUTER_MODEL="${OPENROUTER_MODEL:-deepseek/deepseek-v4-pro}"
export TERMVIEW_PROTOCOL="${TERMVIEW_PROTOCOL:-kitty}"
export VCAD_CHAT_ENDPOINT="$CHAT_ENDPOINT"
export CLAUDE_PREVIEW="${CLAUDE_PREVIEW:-1}"

if [[ -z "${OPENROUTER_API_KEY:-}" && -z "${OPENROUTER_KEY:-}" ]]; then
  echo "error: OPENROUTER_KEY or OPENROUTER_API_KEY is not set (checked env and ~/.bashrc)" >&2
  exit 1
fi

cd "$ROOT"

echo "Starting vcad app API on ${APP_URL} (log: ${LOG})"
rm -f "$LOG"

if command -v setsid >/dev/null 2>&1; then
  setsid npm run dev -w @vcad/app -- --host "$HOST" --port "$PORT" --strictPort >"$LOG" 2>&1 &
else
  npm run dev -w @vcad/app -- --host "$HOST" --port "$PORT" --strictPort >"$LOG" 2>&1 &
fi
SERVER_PID=$!

cleanup() {
  # Kill the whole server process group so Vite's child node process does not
  # survive after the TUI exits. If setsid is unavailable, fall back to the
  # parent process.
  kill -TERM "-${SERVER_PID}" >/dev/null 2>&1 || kill "$SERVER_PID" >/dev/null 2>&1 || true
  wait "$SERVER_PID" >/dev/null 2>&1 || true
}
trap cleanup EXIT INT TERM

# Wait for Vite and the dev API to be reachable.
echo "Waiting for local chat endpoint..."
for _ in $(seq 1 90); do
  if ! kill -0 "$SERVER_PID" >/dev/null 2>&1; then
    echo "error: app server exited early. Last log lines:" >&2
    tail -80 "$LOG" >&2 || true
    exit 1
  fi
  if curl -ksS --max-time 2 -X OPTIONS "$CHAT_ENDPOINT" >/dev/null 2>&1; then
    break
  fi
  sleep 1
done

if ! curl -ksS --max-time 2 -X OPTIONS "$CHAT_ENDPOINT" >/dev/null 2>&1; then
  echo "error: timed out waiting for ${CHAT_ENDPOINT}. Last log lines:" >&2
  tail -80 "$LOG" >&2 || true
  exit 1
fi

echo "Launching vcad TUI"
echo "  VCAD_CHAT_ENDPOINT=${VCAD_CHAT_ENDPOINT}"
echo "  VCAD_CHAT_PROVIDER=${VCAD_CHAT_PROVIDER}"
echo "  OPENROUTER_MODEL=${OPENROUTER_MODEL}"
echo "  TERMVIEW_PROTOCOL=${TERMVIEW_PROTOCOL}"

# Use cargo from a checkout so this script works before the vcad binary is installed.
cargo run -p vcad-cli -- tui "$@"
