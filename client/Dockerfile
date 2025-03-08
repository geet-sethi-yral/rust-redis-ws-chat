# Build stage
FROM node:18-alpine AS builder

# Install pnpm globally
RUN npm install -g pnpm

WORKDIR /app

# Copy package.json and package-lock.json for dependency installation
COPY package.json pnpm-lock.yaml ./

# Install dependencies
RUN pnpm install

# Copy the rest of the application code
COPY . .

# Set environment variables if needed
ENV NEXT_PUBLIC_WS_URL=ws://localhost:4000

# Build the Next.js application
RUN pnpm build

# Runtime stage
FROM node:18-alpine AS runner

WORKDIR /app

ENV NODE_ENV=production

# Copy necessary files from build stage
COPY --from=builder /app/next.config.ts ./
COPY --from=builder /app/public ./public
COPY --from=builder /app/package.json ./package.json
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static

# Expose the port that Next.js runs on
EXPOSE 3000

# Environment variables can be overridden at runtime
ENV NEXT_PUBLIC_WS_URL=ws://localhost:4000/ws

# Start the Next.js application
CMD ["node", "server.js"]