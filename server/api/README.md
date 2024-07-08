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