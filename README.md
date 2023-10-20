# Fiora

penamaan fiora diambil dari salah satu hero di dunia league of legend

## About

contoh aplikasi microservice perpus dengan rest api pada bahasa program rust menggunakan actix sebagai framework dan mengimplementasi [Hexagonal architecture](<https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)>) / [Domain driven design architecture](https://en.wikipedia.org/wiki/Domain-driven_design) dengan [Dependency injection](https://en.wikipedia.org/wiki/Dependency_injection) dan [Clean code](https://gist.github.com/wojteklu/73c6914cc446146b8b533c0988cf8d29)

<center><img src="hexagonal.png" /></center>

dalam "data source" sendiri disini saya menggunakan penamaan "infra/infrastructure" yg berarti dalam context ini adalah hal atau pihak ke 3 yg saya gunakan seperti contoh pada gambar diatas yg berada di dalam folder `src/data/infra` dan untuk repository nya sendiri berada di dalam folder `src/data/repo`

untuk "service" sendiri disini saya menggunakan penamaan "interactor" yg berada di dalam folder `src/interactor`

dan terakhir untuk "interface adapter" sendiri disini saya menggunakan penamaan "protocol" yg berada di dalam folder `src/protocol` karna dasar nya disini adalah suatu hal yg bisa berinteraksi dengan luar seperti menerima dan memberikan response entah menggunakan grpc, http, graphql, web-socket, atau bahkan pub-sub

## How to
untuk menjalankan nya pastikan env sudah terisi dengan benar
lalu cukup ketikan perintah
```bash
cargo run
```
pada terminal

lalu bisa dapat di akses di [localhost](http://localhost:8000)