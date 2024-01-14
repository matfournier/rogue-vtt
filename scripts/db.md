# Instructions

```
 brew install podman
 podman machine init
 podman machine start
 podman pull docker.io/library/postgres:14
```

Migrations managed by sqlx-cli 

```
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

need psql installed

```
brew install postgresql@16
```

Need to figure out where to get this on linux
