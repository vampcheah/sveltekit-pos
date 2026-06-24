#!/usr/bin/env bash
# 极简后台进程管理：用进程组(setsid)启动，kill 整组解决 cargo/vite 孤儿子进程。
# 用法: run.sh start <name> "<cmd>" | stop <name...> | status <name...> | wait-http <url>
set -euo pipefail
cd "$(dirname "$0")/.."
RUN=.run; LOG="$RUN/logs"; mkdir -p "$LOG"

start() {
  local name="$1" cmd="$2"; local pidf="$RUN/$name.pid"
  if [[ -f "$pidf" ]] && kill -0 "$(cat "$pidf")" 2>/dev/null; then
    echo "  $name 已在运行 (pid $(cat "$pidf"))"; return 0
  fi
  setsid bash -c "$cmd" >"$LOG/$name.log" 2>&1 &
  echo $! >"$pidf"
  echo "  $name 启动 (pgid $!) → $LOG/$name.log"
}

stop_one() {
  local name="$1"; local pidf="$RUN/$name.pid"
  [[ -f "$pidf" ]] || { echo "  $name 未运行"; return 0; }
  local pid; pid="$(cat "$pidf")"
  kill -- "-$pid" 2>/dev/null || kill "$pid" 2>/dev/null || true   # 杀整个进程组
  rm -f "$pidf"; echo "  $name 已停"
}

status_one() {
  local name="$1"; local pidf="$RUN/$name.pid"
  if [[ -f "$pidf" ]] && kill -0 "$(cat "$pidf")" 2>/dev/null; then
    echo "  $name: UP (pid $(cat "$pidf"))"
  else echo "  $name: down"; fi
}

wait_http() {
  local url="$1" i
  for i in $(seq 1 60); do curl -sf "$url" >/dev/null 2>&1 && { echo "  ready: $url"; return 0; }; sleep 1; done
  echo "  timeout waiting $url" >&2; return 1
}

cmd="${1:-}"; shift || true
case "$cmd" in
  start)     start "$1" "$2" ;;
  stop)      for n in "$@"; do stop_one "$n"; done ;;
  status)    for n in "$@"; do status_one "$n"; done ;;
  wait-http) wait_http "$1" ;;
  *) echo "用法: run.sh start <name> <cmd> | stop <name...> | status <name...> | wait-http <url>" >&2; exit 1 ;;
esac
