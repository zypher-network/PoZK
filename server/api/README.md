# api server

## login
nonce: `block number`  
expiry_time: `default 1 min`

### req:
```jsonc
{
    "domain": "0.0.0.0:8090",
    "address": "0xaa6321F2A813c720F0fa19f13789932d05c11e25",
    "uri": "http://0.0.0.0:8090/api/login",
    "version": "1",
    "chain_id": 31337,
    "nonce": "00000000",
    "issued_at": "2024-07-08T11:42:18.807Z",
    "v": 27,
    "r": "0x953391bcbad53d9c770728471840dfd57ce7c1622616a11e9e5385afd998f883",
    "s": "0x69e42b5f14e193d591b94d614fa41995b22f8ed00ca2309deea3753481f86ad0",
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

## controller/new

### req
```jsonc
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

## controller/add

### req
```jsonc
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

## post:controller/set

### req
```jsonc
{
    "address": "0x0afa050c5d068d3d569daa5e50c440e231549141"
}
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

## query get:controller/set

### req
```jsonc
{}
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

## controller/list

### req
```jsonc
{}
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