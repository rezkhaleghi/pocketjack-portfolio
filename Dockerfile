# Build stage
FROM rust:1.86.0 as builder

# Install trunk and wasm-bindgen-cli with pinned versions
RUN cargo install trunk --version 0.22.0 wasm-bindgen-cli --version 0.2.87

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app

# Copy Cargo.toml and Cargo.lock for dependency caching
COPY ./frontend/Cargo.toml ./frontend/
COPY ./frontend/Cargo.lock ./frontend/
COPY ./shared/Cargo.toml ./shared/
COPY ./Cargo.toml ./

# Create dummy source files
RUN mkdir -p ./frontend/src ./shared/src \
    && echo "fn main() {}" > ./frontend/src/main.rs \
    && echo "pub fn dummy() {}" > ./shared/src/lib.rs

# Cache dependencies
WORKDIR /usr/src/app/frontend
RUN cargo fetch

# Copy actual source code
WORKDIR /usr/src/app
COPY ./frontend ./frontend
COPY ./shared ./shared

# Build the Yew app
WORKDIR /usr/src/app/frontend
RUN trunk build --release

# Runtime stage
FROM nginx:1.27-alpine

# Copy the built static files
COPY --from=builder /usr/src/app/frontend/dist /usr/share/nginx/html

# Copy custom nginx configuration (optional)
COPY ./frontend/nginx.conf /etc/nginx/conf.d/default.conf

# Expose port
EXPOSE 8080

# Healthcheck
HEALTHCHECK --interval=30s --timeout=3s \
    CMD curl -f http://localhost:8080/ || exit 1