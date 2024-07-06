FROM rust:1.67

WORKDIR /btrade

COPY  ..

RUN install cargo 
RUN cargo run 

CMD ["cargo","src/main.rs"] 
