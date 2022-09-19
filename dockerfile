FROM debian:latest

ENV PORT=80
EXPOSE $PORT

WORKDIR /usr/src/{{project-name}}/

COPY common ./common
COPY index ./index
COPY server ./server
COPY Cargo.toml .
COPY makefile .

RUN apt update
RUN apt install npm make curl build-essential gcc -y
RUN npm install -g tailwindcss
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target install wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN make release

CMD ["sh", "-c", "make run_inner"]
