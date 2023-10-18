# Fiora

penamaan fiora diambil dari salah satu hero di dunia league of legend

## About

contoh aplikasi microservice perpus dengan rest api pada bahasa program rust menggunakan actix sebagai framework dan mengimplementasi [Hexagonal architecture](<https://en.wikipedia.org/wiki/Hexagonal_architecture_(software)>) / [Domain driven design architecture](https://en.wikipedia.org/wiki/Domain-driven_design) dengan [Dependency injection](https://en.wikipedia.org/wiki/Dependency_injection) dan [Clean code](https://gist.github.com/wojteklu/73c6914cc446146b8b533c0988cf8d29)

<center><img src="hexagonal.png" /></center>
karna ini hanya contoh jadi belum sekompleks pada gambar yg sudah menggunakan even driven message broker, email adapter dll

tapi kurang lebih model dan design aplikasi ini mirip dengan contoh gambar di atas

## Entity

disini sebagai contoh saya menggunakan entity book dan user, setiap user hanya bisa meminjam 1 buku disaat yg bersamaan book yg dipinjam sangat terbatas tergantung dari berapa banyak stok dari buku tersebut yg bisa dipinjam

## Env

untuk env silakan isi file `.env` menggunakan sample dari `env.example`

## How to
untuk menjalankan nya pastikan env sudah terisi dengan benar
lalu cukup ketikan perintah
```bash
cargo run
```
pada terminal

lalu bisa dapat di akses di [localhost](http://localhost:8000)