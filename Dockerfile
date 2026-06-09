# ==========================================
# STAGE 1: Build the Rust WebAssembly Pod
# ==========================================
FROM rust:1.78-slim AS rust-builder

# Install curl and build requirements for wasm-pack
RUN apt-get update && apt-get install -y curl pkg-config libssl-dev build-essential \
    && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src/render-pod
COPY render-pod/ .

# Compile the Rust code into a optimized module optimized for Node SSR
RUN wasm-pack build --target nodejs --out-dir /pkg

# ==========================================
# STAGE 2: Build the Astro Frontend Orchestrator
# ==========================================
FROM node:20-alpine AS astro-builder
WORKDIR /app

# Copy dependency configuration files
COPY astro-site/package*.json ./
RUN npm ci

# Copy the built WebAssembly package into the exact location expected by Astro
COPY --from=rust-builder /pkg /pkg
COPY astro-site/ .

# Build the Astro layout runtime engine
RUN npm run build

# ==========================================
# STAGE 3: Slim Production Runtime Runner
# ==========================================
FROM node:20-alpine AS runner
WORKDIR /app

# Set container process execution variables
ENV NODE_ENV=production
ENV HOST=0.0.0.0
ENV PORT=4321

# Copy only production configuration and distributions
COPY astro-site/package*.json ./
RUN npm ci --omit=dev

# Ingest runtime compiled artifacts from Stage 2
COPY --from=astro-builder /app/dist ./dist

EXPOSE 4321

# Execute the self-contained standalone Node entrypoint
CMD ["node", "./dist/server/entry.mjs"]
