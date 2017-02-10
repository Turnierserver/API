FROM jimmycuadra/rust

EXPOSE 80

WORKDIR /srv/api

RUN apt-get update
RUN apt-get install -y wget
RUN echo "deb http://apt.postgresql.org/pub/repos/apt/ jessie-pgdg main" > /etc/apt/sources.list.d/pgdg.list
RUN wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN apt-get remove -y wget

RUN cargo install diesel_cli --no-default-features --features postgres
RUN echo DATABASE_URL=postgres://turnierserver:turnierserver@localhost/turnierserver > /srv/api/.env

CMD (/root/.cargo/bin/diesel setup || true; /root/.cargo/bin/diesel migrations run) && cargo run --release --bin rocket

COPY . /srv/api
