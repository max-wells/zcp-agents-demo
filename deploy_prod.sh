#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

# shellcheck disable=SC1091
[[ -f .env ]] && source .env

: "${ZEROPS_PROJECT_ID:?Set ZEROPS_PROJECT_ID in .env}"
: "${ZEROPS_SERVICE_ID_APP:?Set ZEROPS_SERVICE_ID_APP in .env}"
: "${DATABASE_URL:?Set DATABASE_URL in .env}"

cargo sqlx prepare

if ! git diff --quiet -- .sqlx || ! git diff --cached --quiet -- .sqlx; then
    git add .sqlx
    git commit -m "Refresh sqlx offline query cache"
    git push
fi

zcli push app --project-id "$ZEROPS_PROJECT_ID" --service-id "$ZEROPS_SERVICE_ID_APP"
