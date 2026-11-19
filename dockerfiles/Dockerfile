# syntax=docker/dockerfile:1.29

# Build stage: the dev base image has the Rust toolchain and runs as the app
# user (uid 1001).
FROM ghcr.io/azavalon/base:0.2.0-rust-debian-dev@sha256:c5d45c742729e32404ff552a619099b5154f7f72a99e6d199644ea05b929fff3 AS builder

WORKDIR /opt/app/build
COPY --chown=app:app .cargo ./.cargo
COPY --chown=app:app Cargo.toml Cargo.lock ./
COPY --chown=app:app src ./src

# The registry and target dirs are cache mounts: the binary is copied out of
# target before the mount goes away.
RUN --mount=type=cache,uid=1001,gid=1001,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,uid=1001,gid=1001,id=rust-app-template-target,target=/opt/app/build/target \
    --mount=type=secret,id=git-credentials,env=GIT_CREDENTIALS,uid=1001 \
    if [ -n "${GIT_CREDENTIALS:-}" ]; then \
      git config --global url."https://x-access-token:${GIT_CREDENTIALS}@github.com".insteadOf https://github.com; \
    fi && \
    cargo build --release --locked --bin app && \
    cp target/release/app /opt/app/app


# Runtime stage: Debian slim, CA certificates and the app user, nothing else.
FROM ghcr.io/azavalon/base:0.2.0-rust-debian@sha256:62ac5ef68eac3ca2f6b3fa1c7a49a687ea7b241efb4cb371f219ca40b438cbfd AS production

# Owned by root: the app user can run the binary, not change it
COPY --link --from=builder /opt/app/app /usr/local/bin/app
ENV APP_PORT=8000 \
    RUST_BACKTRACE=1

EXPOSE 8000
CMD ["app"]
