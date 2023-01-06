####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=appuser
ENV UID=10001

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-