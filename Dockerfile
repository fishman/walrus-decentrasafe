FROM rust:alpine3.20 as build

RUN apk add --no-cache musl-dev sqlite-dev sqlite

ENV RUSTFLAGS='-C target-feature=-crt-static'

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN cargo install cargo-watch


FROM rust:alpine3.20

COPY --from=build /usr/local/cargo/bin/diesel /usr/bin/cargo-watch
COPY --from=build /usr/local/cargo/bin/diesel /usr/bin/diesel


RUN apk add --no-cache musl-dev sqlite-dev sqlite

ENV RUSTFLAGS='-C target-feature=-crt-static'
ARG UNAME=walrus
ARG USER_ID=1000
ARG GROUP_ID=1000
RUN addgroup ${UNAME} -g ${GROUP_ID} &&  \
	adduser ${UNAME} -u ${USER_ID} -G ${UNAME} -D

WORKDIR /app
COPY ./ /app
RUN cargo build
RUN chown -R walrus /app
USER ${UNAME}
# ENV RUST_LOG=diesel::sql=trace,debug

CMD ["cargo", "watch", "-i", "*.sqlite*", "-x", "run"]
