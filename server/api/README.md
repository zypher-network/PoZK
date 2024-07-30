# api server

## login
nonce: `block number`  
expiry_time: `default 1 min`

### req:
```jsonc
{
    "domain": "localhost:4000", 
    "address": "0x28B9FEAE1f3d76565AAdec86E7401E815377D9Cc",
    "uri": "http://0.0.0.0:8090/api/login",
    "version": "1",
    "chain_id": 31337,
    "nonce": "00000000",
    "issued_at": "2024-07-23T11:42:18.807Z",
    "v": 27,
    "r": "0xf69e02fdeb811acab1d39938de4bec54931e2f0b4357f3793f6717e1f39d4665",
    "s": "0x126cc57b71ffad79ceabffbc3d88a39afb0a62e0132e0cff3450a9db2e06ade9",
    "statement": "Welcome to Zytron!",
    "resources": []
}
```
### success resp
```jsonc
{
  "code": 0,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k"
  },
  "msg": null,
  "uid": "cc888004-2853-4e48-93c6-cbd0c680b332"
}
```
### fail resp
```jsonc
HTTP/1.1 401 Unauthorized
content-type: text/plain; charset=utf-8
content-length: 19
date: Mon, 08 Jul 2024 07:43:28 GMT

authorization error
```

## post:controller/new

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

{}
```

### resp
```jsonc
{
  "code": 0,
  "data": {
    "controller": "0x1f84ab26cbda7d70c32171382873d9c2a4a234c2"
  },
  "msg": null,
  "uid": "f5768e72-3ed1-42a1-94f7-ddd773b211c0"
}
```

## post:controller/add

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

{
    "signing_key": "0x7f0ba51f7e74878e488bb6a0200bf4e9d38b8700a04ce7b3fe0034f69228837e"
}
```

### resp
```jsonc
{
  "code": 0,
  "data": null,
  "msg": null,
  "uid": "1f724f42-8c3e-4f5f-be6d-7a319ce332aa"
}
```

## post:controller/set/{address}

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

{}
```

### resp
```jsonc
{
  "code": 0,
  "data": null,
  "msg": null,
  "uid": "522f4bf2-77f8-4392-add0-a5986fbdbb82"
}
```

## get:controller/export/{address}

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

{}
```

### resp
```jsonc
{
  "code": 0,
  "data": {
    "controller": "0x0afa050c5d068d3d569daa5e50c440e231549141",
    "singing_key": "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
  },
  "msg": null,
  "uid": "1aa10f9d-be51-4549-823d-36063888d02b"
}
```

## query get:controller/set

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

```

### resp
```jsonc
{
  "code": 0,
  "data": {
    "controller": "0x0afa050c5d068d3d569daa5e50c440e231549141"
  },
  "msg": null,
  "uid": "1aa10f9d-be51-4549-823d-36063888d02b"
}
```

## get: controller/list?page_size=1&page_count=10

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

```

### resp
```jsonc
{
  "code": 0,
  "data": {
    "data": [
        "0x0afa050c5d068d3d569daa5e50c440e231549141",
        "0x1f84ab26cbda7d70c32171382873d9c2a4a234c2"
    ],
    "total": 2
  },
  "msg": null,
  "uid": "5966b408-8ac5-4f7a-bc80-679df6100430"
}
```


## get: prover/list?page_size=1&page_count=10

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

```

### resp
```jsonc
TODO
```


## get: prover/{image_id}/list?page_size=1&page_count=10

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

```

### resp
```jsonc
TODO
```


## prover/add/

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

{
    "repository": "zyphernetwork/0x2e1c9adc548963273d9e767413403719019bd639",
    "tag": "v1",
    "name": "shuffle"
}
```

### resp
```jsonc
{
  "code": 0,
  "data": null,
  "msg": null,
  "uid": "5966b408-8ac5-4f7a-bc80-679df6100430"
}
```