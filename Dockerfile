# --- base ---
FROM rustlang/rust:nightly AS base

WORKDIR /usr/web/app


# --- migration ---
FROM base AS migration

COPY migrations /usr/web/app/migrations
COPY diesel.toml /usr/web/app/diesel.toml

RUN cargo install diesel_cli --no-default-features --features "postgres"


# --- web ---
FROM base AS web

COPY . .