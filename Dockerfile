FROM rust:1.83 AS build

# create a new empty shell project
RUN USER=root cargo new --bin addressbook
WORKDIR /addressbook

RUN USER=root cargo new --bin frontend

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./entity ./entity
COPY ./migration ./migration

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/addressbook*
RUN cargo build --release

# our final base
FROM debian:bookworm-slim
ARG APP=/opt/app
ARG USERNAME=appuser
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

RUN groupadd -g ${USER_GID} ${USERNAME} \
    && useradd -u ${USER_UID} -g ${USER_GID} ${USERNAME} -m -c "Docker image user" \
    && apt-get update \
    && apt-get install -y ca-certificates tzdata openssl \
    && rm -rf /var/lib/apt/lists/* \
    && mkdir -p ${APP}

ENV TZ=Europe/Moscow

# copy the build artifact from the build stage
COPY --from=build ./addressbook/target/release/addressbook ${APP}/addressbook
COPY ./static ${APP}/static

RUN chown -R ${USERNAME}:${USERNAME} ${APP}

USER ${USERNAME}
WORKDIR ${APP}

EXPOSE 3000

# set the startup command to run your binary
CMD ["./addressbook"]