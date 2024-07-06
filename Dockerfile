FROM rust:1.67

WORKDIR /btrade

COPY  . .

RUN install cargo 
RUN cargo run 
RUN serde-derive

CMD ["cargo","src/main.rs"] 

