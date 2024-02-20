FROM rust:1.76

WORKDIR /usr/src/studyscraper
COPY . .

RUN cargo install --path .

EXPOSE 8080
CMD ["studyscraper"]
