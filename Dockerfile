FROM dock0/arch

EXPOSE 8000

WORKDIR /srv/api

RUN pacman --noconfirm -Syu \
 && pacman --noconfirm -S gcc rustup postgresql postgresql-libs \
 && rustup toolchain install nightly \
 && rustup default nightly \
 && cargo install diesel_cli --no-default-features --features postgres \
 && pacman -Qtdq | xargs -r pacman --noconfirm -Rcns \
 && pacman --noconfirm -Scc \
 && rm -rf /var/cache/pacman/pkg/* \
 && rm -rf /home/aur/.cache/pacaur

CMD cat /srv/api/.env && (/root/.cargo/bin/diesel setup || /root/.cargo/bin/diesel migration run) && cargo run --release --bin rocket

COPY . /srv/api
RUN cargo build --release \
 && echo DATABASE_URL=postgres://turnierserver:turnierserver@db/turnierserver | tee /srv/api/.env
