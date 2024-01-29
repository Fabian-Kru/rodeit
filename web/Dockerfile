FROM node:20-slim AS runtime
RUN corepack enable
WORKDIR /app

COPY . .

ENV VITE_RODEIT_API_URL=https://rodeit.mabezi.de/api/
ENV VITE_CAPTAIN_COASTER_API_KEY=ee2d7ceb-eabe-4583-8b58-be70ff1ab79e

RUN pnpm install
RUN pnpm run build

ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080
CMD node ./dist/server/entry.mjs
