# =============================================================================
# AeroXe Broadband Frontend - Multi-stage Docker Build
# =============================================================================

# ── Build Stage ─────────────────────────────────────────────────────────────
FROM node:20-alpine AS builder

WORKDIR /app

COPY package.json package-lock.json ./
RUN npm ci

COPY . .
# Typecheck is part of the build script; VITE_API_URL must be injected at
# build time (e.g. --build-arg) for server-side API URL rewriting.
ARG VITE_API_URL
ENV VITE_API_URL=$VITE_API_URL
RUN npm run build

# ── Runtime Stage ───────────────────────────────────────────────────────────
FROM nginx:1.27-alpine

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/build /usr/share/nginx/html

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget -q -O /dev/null http://localhost/ || exit 1
