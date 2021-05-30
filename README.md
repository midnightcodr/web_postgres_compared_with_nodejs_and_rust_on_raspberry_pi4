### Why
I wanted to do a performance comparision between actix_web (rust) and hapijs (nodejs).


### Setup
  - Raspberry pi 4 with 8GB, raspbe boot from a Samsung 512GB ssd
  - Raspberry pi OS 64bit
  - [https://hub.docker.com/_/postgres](postgres docker image)
  - nodejs 16.2.0, installed with nvm
  - rustc 1.52.1 (9bc8c42bb 2021-05-09)
  - when running the rust version of the web server, released version of the binary is used (`./target/release/rust_posts`)
  - when running the nodejs version of the web server, pm2 was used (`pm2 start main.js -i max`)

### Results
Based on my tests, actix_web is 3+ times as fast as nodejs on RPI4.
See [results.md](results.md) for details.


## Postgres table strcture
See [db.sql](db.sql).