# Thiên Hạc: one Rust binary and its asset bundle. Any container host runs it.
#
#   docker build -t thien-hac .
#   docker run -p 3000:3000 thien-hac        # http://localhost:3000
#
# The app reads HOST and PORT (set below; hosts such as Render set PORT).

FROM rust:1-bookworm AS build
# The Topcoat CLI bundles the CSS, JS, fonts and images next to the binary.
RUN cargo install topcoat-cli --version 0.12.1 --locked
WORKDIR /src
COPY . .
RUN topcoat asset bundle --release -p tuvi-web \
    && mkdir /out \
    && cp target/release/tuvi-web /out/ \
    && cp -r target/release/assets /out/assets

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 app
WORKDIR /app
# The binary loads its assets from the `assets` folder next to it.
COPY --from=build /out/ /app/
USER app
ENV HOST=0.0.0.0 PORT=3000
EXPOSE 3000
CMD ["/app/tuvi-web"]
