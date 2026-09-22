# zcp-agents-demo

Tiny inventory/orders API (axum + sqlx + Postgres) used for the Zerops ZCP demo.

## Run locally

Copy `.env.example` to `.env` and fill in `ZCP_API_KEY` (single-project Zerops token, keep it out of git).

```
cargo run
```

Migrations run automatically on startup.

## Reset the database

```
chmod +x reset_db.sh
./reset_db.sh
```

## Endpoints

- `GET /health`
- `GET /orders`
- `POST /orders` — see `requests.http` for example payloads
