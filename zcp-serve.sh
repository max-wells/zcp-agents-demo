#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
unset ZCP_API_KEY
set -a
source .env
set +a
exec zcp serve
