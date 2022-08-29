# Libraries

- Used for graphQL [juniper](https://github.com/graphql-rust/juniper)
- Used for serve GraphQL [actix](https://actix.rs/)

- Used for connect mongoDB [mongo-rust-driver](https://github.com/mongodb/mongo-rust-driver)
- Required for `mongo-rust-driver` [futures](https://docs.rs/futures/latest/futures/)
- Used for deserialize mongoDB result [serde](https://serde.rs/)

- Used for the env [config](https://github.com/mehcode/config-rs)

- Used for generate UTC datetime [chrono](https://docs.rs/chrono/latest/chrono/)
- Covert `Chrono` DateTime to MongoDB DateTime, and accept by the `juniper` [bson](https://docs.rs/bson/latest/bson/)

- Used for generate Id [nanoid](https://docs.rs/nanoid/latest/nanoid/)

---

## CMDs

##### Run in development

```bash
cargo watch -c -x run
RUN_ENV=Development cargo watch -c -x run
RUN_ENV=Development cargo run
```

```
RUN_ENV=Development cargo run
RUN_ENV=Production cargo run
RUN_ENV=DockerPro cargo run
RUN_ENV=DockerPro /Users/tony/Desktop/Git/rust-graphQl-markdown-server/target/release/rust-graphql-markdown-server
```

###### Run in production

- Create `production.toml` under `\configs`

```
./start.sh
```

## Docker

- [Ref](https://dev.to/rogertorres/first-steps-with-docker-rust-30oi)
- [Good Ref](https://kerkour.com/deploy-rust-on-heroku-with-docker)

```
docker build -t rust-graphql-markdown-server .
docker images
docker run rust-graphql-markdown-server
docker run -dp 8081:3030 --rm --name markdown-server rust-graphql-markdown-server
docker run -dp 127.0.0.1:8082:8082 --rm --name markdown-server rust-graphql-markdown-server
docker stop markdown-server
```

---

### GraphQL

```
query AllMarkdown {
  allMarkdowns {
    id
    title
    context
  }
}

query markdownById {
  markdownById(id: "2LvLZxGEF8") {
    title
    context
  }
}

mutation CreateMarkdown{
  createMarkdown(
    newMarkdown: { title: "Fresh Kid Ice", context: "## what" }
  ) {
    title
    context
  }
}

mutation UpdateMarkdown{
  updateMarkdown(
    input: { id: "2LvLZxGEF8", title: "Update", context: "## what updated1" }
  ) {
    id
  }
}

mutation deleteMarkdown{
  deleteMarkdown(
    id: "2LvLZxGEF8"
  ) {
    id
  }
}


```
