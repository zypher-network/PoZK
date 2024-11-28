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
    "statement": "Welcome to PoZK!",
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
{
  "code": 0,
  "data": {
    "data": [
      {
        "created": "2024-07-26T03:33:36+00:00",
        "image_id": "40d7a381b0c23decd9efd2f87410568d30debab6f2ed1b9a8dba7816d093fbcc",
        "name": "shuffle",
        "prover": "0x48a7fb14fd5711cf057bc7392973680231e8aebb-v1"
      },
      {
        "created": "2024-08-14T11:36:52+00:00",
        "image_id": "963163fbb53d4c8c01138305f6ccc7de87015ff23dbd376cfd5a4962c1f37947",
        "name": "shuffle",
        "prover": "0x764ae46f345be77ef2a1f707842a9e7cffb1f2fb-v1"
      }
    ],
    "total": 2
  },
  "msg": null,
  "uid": "0ec50824-d4ed-4fe2-9c95-44c4e3ec6808"
}
```


## get: prover/{image_id}/list?page_size=1&page_count=10

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

```

### resp
```jsonc
{
  "code": 0,
  "data": {
    "container_list": [
      {
        "created": "2024-07-31T08:27:27.217254197+00:00",
        "id": "582e98690aa872bfd6a6e08860ff50a63f3efc45574aed37af5247fa29992a7a",
        "image": "sha256:40d7a381b0c23decd9efd2f87410568d30debab6f2ed1b9a8dba7816d093fbcc",
        "running": false,
        "status": "created"
      }
    ],
    "created": "2024-07-26T03:33:36.846402506+00:00",
    "id": "sha256:40d7a381b0c23decd9efd2f87410568d30debab6f2ed1b9a8dba7816d093fbcc",
    "repository": "docker.registry.cyou/zyphernetwork/0x2e1c9adc548963273d9e767413403719019bd639",
    "tag": "v1"
  },
  "msg": null,
  "uid": "8e83bc56-2fb0-4d6b-85a1-5e0fff1a4f57"
}
```


## post: prover/pull/

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

{
    "repository": "docker.registry.cyou/zyphernetwork/0x2e1c9adc548963273d9e767413403719019bd639",
    "tag": "v1",
    "name": "shuffle",
    "prover": "0x2e1c9adc548963273d9e767413403719019bd639",
    "overtime": 0
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

## post: prover/{container_id}/start

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

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

## post: prover/new/

### req
```jsonc
X-API-Key: eyJhbGciOiJIUzI1NiJ9.eyJhZGRyZXNzIjoiMHhhYTYzMjFmMmE4MTNjNzIwZjBmYTE5ZjEzNzg5OTMyZDA1YzExZTI1IiwiY3JlYXRlX3RpbWUiOjE3MjA0MjQ1NDUsImV4cGlyeV90aW1lIjoxNzIwNDI0NjA1fQ.1wgqrBQU-fwwfb4n2rKvCeJEvZwsq43m-w-E4TD679k

{
    "option": {
        "env": [
            "INPUT=/data/0x2e1c9adc548963273d9e767413403719019bd639.input",
            "OUTPUT=/data/0x2e1c9adc548963273d9e767413403719019bd639.publics",
            "PROOF=/data/0x2e1c9adc548963273d9e767413403719019bd639.proof"
        ],
        "volumes": [
            {
                "src_volumes": "/data",
                "host_volumes": "/home/cloud/zypher/pozk-shuffle/prover/examples"
            }
        ]
    },
    "prover": "0x2e1c9adc548963273d9e767413403719019bd639",
    "tag": "v1"
}
```

### resp
```jsonc
{
  "code": 0,
  "data": {
    "container_id": "582e98690aa872bfd6a6e08860ff50a63f3efc45574aed37af5247fa29992a7a"
  },
  "msg": null,
  "uid": "c89efdbc-f4da-409f-9904-fa82a0197d73"
}
```
