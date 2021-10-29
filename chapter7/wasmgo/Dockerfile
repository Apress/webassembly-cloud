FROM golang:latest

RUN mkdir -p /app
WORKDIR /app
Add . /app
RUN go mod download
RUN go mod tidy
RUN go get github.com/wapc/wapc-go
RUN go get github.com/izinin/json2msgpack

RUN  LD_LIBRARY_PATH="/app/"
RUN go build -o main .

EXPOSE 8080

CMD ["./main","cuckoo_wapc.wasm","check_word_exists"]
