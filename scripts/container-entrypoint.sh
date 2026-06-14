#!/usr/bin/env sh
set -eu

query-in-backend &
backend_pid="$!"

caddy run --config /etc/caddy/Caddyfile --adapter caddyfile &
caddy_pid="$!"

shutdown() {
  kill "$backend_pid" "$caddy_pid" 2>/dev/null || true
  wait "$backend_pid" "$caddy_pid" 2>/dev/null || true
}

trap shutdown INT TERM

while true; do
  if ! kill -0 "$backend_pid" 2>/dev/null; then
    wait "$backend_pid"
    shutdown
    exit 1
  fi

  if ! kill -0 "$caddy_pid" 2>/dev/null; then
    wait "$caddy_pid"
    shutdown
    exit 1
  fi

  sleep 1
done
