FROM node:20-alpine AS base

# --- Dependencies ---
### Rebuild deps only when needed ###
FROM base AS deps
RUN apk add --no-cache libc6-compat git
RUN apk add --update python3 make g++\
   && rm -rf /var/cache/apk/*
RUN echo Building nextjs image with corepack

WORKDIR /app

COPY frontend/package.json frontend/yarn.lock ./
RUN yarn config set network-timeout 300000
RUN yarn install --frozen-lockfile

# --- Builder ---
FROM base AS builder

WORKDIR /app

ARG API_BASE_URL=BAKED_API_BASE_URL

COPY --from=deps /app/node_modules ./node_modules
COPY frontend .
RUN yarn build

# --- Production runner ---
FROM base AS runner
# Set NODE_ENV to production
ENV NODE_ENV=production

# Disable Next.js telemetry
# Learn more here: https://nextjs.org/telemetry
ENV NEXT_TELEMETRY_DISABLED=1

# Set correct permissions for nextjs user
# Don't run as root
RUN addgroup nodejs
RUN adduser -SDH nextjs
RUN mkdir .next
RUN chown nextjs:nodejs .next
RUN apk add --no-cache bash

# Automatically leverage output traces to reduce image size
# https://nextjs.org/docs/advanced-features/output-file-tracing
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static
COPY --from=builder --chown=nextjs:nodejs /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/script ./script
RUN chmod +x ./script/replace.sh
RUN chmod +x ./script/start.sh

USER nextjs

# Expose ports (for orchestrators and dynamic reverse proxies)
EXPOSE 4000
ENV PORT=4000
ENV HOSTNAME="0.0.0.0"

# Run the nextjs app
# CMD ["./script/replace.sh"]
# CMD ["node", "server.js"]
CMD ["./script/start.sh"]
