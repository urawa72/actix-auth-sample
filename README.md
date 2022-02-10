# actix-auth-sample

actix-web authentication and authorization sample app.

## backend

```
$ cargo run
Listening on: 127.0.0.1:8000
```

## frontend

```
$ yarn install
$ yarn dev
```

## test with sub domain

Edit the /etc/hosts as below.

```
127.0.0.1       sub.localhost.test
127.0.0.1       api.localhost.test
127.0.0.1       localhost
```

You can access the frontend app with `sub.localhost.test:4000`.
You can access the backend app with `api.localhost.test:8000`.
