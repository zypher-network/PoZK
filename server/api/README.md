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

## get: prover/image/list?page_size=1&page_count=10

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
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": [
              "/bin/sh",
              "-c",
              "/app/zkevm-node run"
            ],
            "Domainname": "",
            "Entrypoint": null,
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
            ],
            "ExposedPorts": {
              "8123/tcp": {}
            },
            "Hostname": "",
            "Image": "",
            "Labels": null,
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "",
            "WorkingDir": ""
          },
          "Created": "2024-04-22T02:08:39.494316732Z",
          "DockerVersion": "",
          "Id": "sha256:4264dcfa9dc981b1faa3f2804dd47952bfdbd66f08002c33c75771d40605f301",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [],
          "RepoTags": [
            "zkevm-node:latest"
          ],
          "Size": 71797195,
          "VirtualSize": 71797195
        },
        "image": {
          "Created": "2024-04-22T02:08:39Z",
          "Id": "sha256:4264dcfa9dc981b1faa3f2804dd47952bfdbd66f08002c33c75771d40605f301",
          "Labels": null,
          "ParentId": "",
          "RepoDigests": [],
          "RepoTags": [
            "zkevm-node:latest"
          ],
          "VirtualSize": 71797195
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": null,
            "Domainname": "",
            "Entrypoint": null,
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
            ],
            "ExposedPorts": null,
            "Hostname": "",
            "Image": "",
            "Labels": {
              "org.opencontainers.image.ref.name": "ubuntu",
              "org.opencontainers.image.version": "22.04"
            },
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "",
            "WorkingDir": "/app"
          },
          "Created": "2024-03-24T14:20:28.297491635Z",
          "DockerVersion": "",
          "Id": "sha256:24e767e151b7e9a4d892cecddbbd3b138107ffbd31144ae639f987296b36be52",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "hermeznetwork/zkevm-prover@sha256:45efb1aa7ec7d95c3cebec0aca774a429e7ca7f46ef4252966c3a46976606daa"
          ],
          "RepoTags": [
            "hermeznetwork/zkevm-prover:v6.0.0"
          ],
          "Size": 1163712437,
          "VirtualSize": 1163712437
        },
        "image": {
          "Created": "2024-03-24T14:20:28Z",
          "Id": "sha256:24e767e151b7e9a4d892cecddbbd3b138107ffbd31144ae639f987296b36be52",
          "Labels": {
            "org.opencontainers.image.ref.name": "ubuntu",
            "org.opencontainers.image.version": "22.04"
          },
          "ParentId": "",
          "RepoDigests": [
            "hermeznetwork/zkevm-prover@sha256:45efb1aa7ec7d95c3cebec0aca774a429e7ca7f46ef4252966c3a46976606daa"
          ],
          "RepoTags": [
            "hermeznetwork/zkevm-prover:v6.0.0"
          ],
          "VirtualSize": 1163712437
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": null,
            "Domainname": "",
            "Entrypoint": [
              "python",
              "python/proof.py"
            ],
            "Env": [
              "PATH=/usr/local/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
              "LANG=C.UTF-8",
              "GPG_KEY=A035C8C19219BA821ECEA86B64E628F8D684696D",
              "PYTHON_VERSION=3.10.13",
              "PYTHON_PIP_VERSION=23.0.1",
              "PYTHON_SETUPTOOLS_VERSION=65.5.1",
              "PYTHON_GET_PIP_URL=https://github.com/pypa/get-pip/raw/dbf0c85f76fb6e1ab42aa672ffca6f0a675d9ee4/public/get-pip.py",
              "PYTHON_GET_PIP_SHA256=dfe9fd5c28dc98b5ac17979a953ea550cec37ae1b47a5116007395bfacff2ab9"
            ],
            "ExposedPorts": null,
            "Hostname": "",
            "Image": "",
            "Labels": null,
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "",
            "WorkingDir": "/claim"
          },
          "Created": "2024-03-12T02:08:38.341039793Z",
          "DockerVersion": "",
          "Id": "sha256:3737a2452d6d0475a880a882e0f82c3c50714eee5f7f833c6048f2dc9cf482b4",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [],
          "RepoTags": [
            "dev-reward-script:latest"
          ],
          "Size": 1354573508,
          "VirtualSize": 1354573508
        },
        "image": {
          "Created": "2024-03-12T02:08:38Z",
          "Id": "sha256:3737a2452d6d0475a880a882e0f82c3c50714eee5f7f833c6048f2dc9cf482b4",
          "Labels": null,
          "ParentId": "",
          "RepoDigests": [],
          "RepoTags": [
            "dev-reward-script:latest"
          ],
          "VirtualSize": 1354573508
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": [
              "celestia"
            ],
            "Domainname": "",
            "Entrypoint": [
              "/bin/bash",
              "/opt/entrypoint.sh"
            ],
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
              "CELESTIA_HOME=/home/celestia",
              "NODE_TYPE=bridge",
              "P2P_NETWORK=mocha"
            ],
            "ExposedPorts": {
              "2121/tcp": {}
            },
            "Hostname": "",
            "Image": "",
            "Labels": {
              "commitUrl": "https://github.com/celestiaorg/celestia-node/commit/32bc7a973b2f0b730c8ec1c2f7d6be5e5484e85b",
              "dockerPull": "docker pull ghcr.io/celestiaorg/celestia-node:32bc7a97",
              "maintainer": "celestiaorg",
              "org.opencontainers.image.created": "2024-03-05T17:36:35.764Z",
              "org.opencontainers.image.description": "celestiaorg repository celestiaorg/celestia-node",
              "org.opencontainers.image.licenses": "Apache-2.0",
              "org.opencontainers.image.revision": "32bc7a973b2f0b730c8ec1c2f7d6be5e5484e85b",
              "org.opencontainers.image.source": "https://github.com/celestiaorg/celestia-node",
              "org.opencontainers.image.title": "celestia-node",
              "org.opencontainers.image.url": "https://github.com/celestiaorg/celestia-node",
              "org.opencontainers.image.version": "v0.13.1"
            },
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "celestia",
            "WorkingDir": ""
          },
          "Created": "2024-03-05T17:40:02.718488222Z",
          "DockerVersion": "",
          "Id": "sha256:b9c0bdb9f6b489a7da3b35e26d7652c826b2b1fc1c7edb6957d745e0635deebe",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "ghcr.io/celestiaorg/celestia-node@sha256:08bd9d01a5bb423d783b1ad2c5563400e67511bb2211f0d40068275e5606319c"
          ],
          "RepoTags": [
            "ghcr.io/celestiaorg/celestia-node:v0.13.1"
          ],
          "Size": 223537160,
          "VirtualSize": 223537160
        },
        "image": {
          "Created": "2024-03-05T17:40:02Z",
          "Id": "sha256:b9c0bdb9f6b489a7da3b35e26d7652c826b2b1fc1c7edb6957d745e0635deebe",
          "Labels": {
            "commitUrl": "https://github.com/celestiaorg/celestia-node/commit/32bc7a973b2f0b730c8ec1c2f7d6be5e5484e85b",
            "dockerPull": "docker pull ghcr.io/celestiaorg/celestia-node:32bc7a97",
            "maintainer": "celestiaorg",
            "org.opencontainers.image.created": "2024-03-05T17:36:35.764Z",
            "org.opencontainers.image.description": "celestiaorg repository celestiaorg/celestia-node",
            "org.opencontainers.image.licenses": "Apache-2.0",
            "org.opencontainers.image.revision": "32bc7a973b2f0b730c8ec1c2f7d6be5e5484e85b",
            "org.opencontainers.image.source": "https://github.com/celestiaorg/celestia-node",
            "org.opencontainers.image.title": "celestia-node",
            "org.opencontainers.image.url": "https://github.com/celestiaorg/celestia-node",
            "org.opencontainers.image.version": "v0.13.1"
          },
          "ParentId": "",
          "RepoDigests": [
            "ghcr.io/celestiaorg/celestia-node@sha256:08bd9d01a5bb423d783b1ad2c5563400e67511bb2211f0d40068275e5606319c"
          ],
          "RepoTags": [
            "ghcr.io/celestiaorg/celestia-node:v0.13.1"
          ],
          "VirtualSize": 223537160
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": null,
            "Domainname": "",
            "Entrypoint": [
              "/bin/sh",
              "-c"
            ],
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
              "LANG=C.UTF-8"
            ],
            "ExposedPorts": null,
            "Hostname": "",
            "Image": "",
            "Labels": {
              "org.label-schema.build-date": "",
              "org.label-schema.description": "Foundry",
              "org.label-schema.name": "Foundry",
              "org.label-schema.schema-version": "1.0",
              "org.label-schema.url": "https://getfoundry.sh",
              "org.label-schema.vcs-ref": "",
              "org.label-schema.vcs-url": "https://github.com/foundry-rs/foundry.git",
              "org.label-schema.vendor": "Foundry-rs",
              "org.label-schema.version": "",
              "org.opencontainers.image.created": "2024-03-01T01:05:21.325Z",
              "org.opencontainers.image.description": "Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.",
              "org.opencontainers.image.licenses": "Apache-2.0",
              "org.opencontainers.image.revision": "4a91072e326126cd852b9c43f577e98c8e13f84f",
              "org.opencontainers.image.source": "https://github.com/foundry-rs/foundry",
              "org.opencontainers.image.title": "foundry",
              "org.opencontainers.image.url": "https://github.com/foundry-rs/foundry",
              "org.opencontainers.image.version": "master"
            },
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "",
            "WorkingDir": ""
          },
          "Created": "2024-03-01T01:24:13.997583469Z",
          "DockerVersion": "",
          "Id": "sha256:0039cab0830eb2e053358be24b573ee65af981b95897c0e217647f2ad4a385e7",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "ghcr.io/foundry-rs/foundry@sha256:8b843eb65cc7b155303b316f65d27173c862b37719dc095ef3a2ef27ce8d3c00",
            "ghcr.m.daocloud.io/foundry-rs/foundry@sha256:8b843eb65cc7b155303b316f65d27173c862b37719dc095ef3a2ef27ce8d3c00",
            "ghcr.m.daocloud.io/ghcr.io/foundry-rs/foundry@sha256:8b843eb65cc7b155303b316f65d27173c862b37719dc095ef3a2ef27ce8d3c00",
            "ghcr.nju.edu.cn/foundry-rs/foundry@sha256:8b843eb65cc7b155303b316f65d27173c862b37719dc095ef3a2ef27ce8d3c00"
          ],
          "RepoTags": [
            "ghcr.io/foundry-rs/foundry:latest",
            "ghcr.m.daocloud.io/foundry-rs/foundry:latest",
            "ghcr.m.daocloud.io/ghcr.io/foundry-rs/foundry:latest",
            "ghcr.nju.edu.cn/foundry-rs/foundry:latest"
          ],
          "Size": 150695478,
          "VirtualSize": 150695478
        },
        "image": {
          "Created": "2024-03-01T01:24:13Z",
          "Id": "sha256:0039cab0830eb2e053358be24b573ee65af981b95897c0e217647f2ad4a385e7",
          "Labels": {
            "org.label-schema.build-date": "",
            "org.label-schema.description": "Foundry",
            "org.label-schema.name": "Foundry",
            "org.label-schema.schema-version": "1.0",
            "org.label-schema.url": "https://getfoundry.sh",
            "org.label-schema.vcs-ref": "",
            "org.label-schema.vcs-url": "https://github.com/foundry-rs/foundry.git",
            "org.label-schema.vendor": "Foundry-rs",
            "org.label-schema.version": "",
            "org.opencontainers.image.created": "2024-03-01T01:05:21.325Z",
            "org.opencontainers.image.description": "Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.",
            "org.opencontainers.image.licenses": "Apache-2.0",
            "org.opencontainers.image.revision": "4a91072e326126cd852b9c43f577e98c8e13f84f",
            "org.opencontainers.image.source": "https://github.com/foundry-rs/foundry",
            "org.opencontainers.image.title": "foundry",
            "org.opencontainers.image.url": "https://github.com/foundry-rs/foundry",
            "org.opencontainers.image.version": "master"
          },
          "ParentId": "",
          "RepoDigests": [
            "ghcr.io/foundry-rs/foundry@sha256:8b843eb65cc7b155303b316f65d27173c862b37719dc095ef3a2ef27ce8d3c00",
            "ghcr.m.daocloud.io/foundry-rs/foundry@sha256:8b843eb65cc7b155303b316f65d27173c862b37719dc095ef3a2ef27ce8d3c00",
            "ghcr.m.daocloud.io/ghcr.io/foundry-rs/foundry@sha256:8b843eb65cc7b155303b316f65d27173c862b37719dc095ef3a2ef27ce8d3c00",
            "ghcr.nju.edu.cn/foundry-rs/foundry@sha256:8b843eb65cc7b155303b316f65d27173c862b37719dc095ef3a2ef27ce8d3c00"
          ],
          "RepoTags": [
            "ghcr.io/foundry-rs/foundry:latest",
            "ghcr.m.daocloud.io/foundry-rs/foundry:latest",
            "ghcr.m.daocloud.io/ghcr.io/foundry-rs/foundry:latest",
            "ghcr.nju.edu.cn/foundry-rs/foundry:latest"
          ],
          "VirtualSize": 150695478
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": [
              "postgres"
            ],
            "Domainname": "",
            "Entrypoint": [
              "docker-entrypoint.sh"
            ],
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/lib/postgresql/16/bin",
              "GOSU_VERSION=1.17",
              "LANG=en_US.utf8",
              "PG_MAJOR=16",
              "PG_VERSION=16.2-1.pgdg120+2",
              "PGDATA=/var/lib/postgresql/data"
            ],
            "ExposedPorts": {
              "5432/tcp": {}
            },
            "Hostname": "",
            "Image": "",
            "Labels": null,
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "",
            "WorkingDir": ""
          },
          "Created": "2024-02-21T00:46:13Z",
          "DockerVersion": "",
          "Id": "sha256:b9390dd1ea18e34fa4bf7b43c99faac1455f712a9095ffc2c4071994bb7df148",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "postgres@sha256:5e027a0e4b3eebb181af3630815c087f17f867abb9bc5860b4e2a203019a04c2"
          ],
          "RepoTags": [
            "postgres:latest"
          ],
          "Size": 431419939,
          "VirtualSize": 431419939
        },
        "image": {
          "Created": "2024-02-21T00:46:13Z",
          "Id": "sha256:b9390dd1ea18e34fa4bf7b43c99faac1455f712a9095ffc2c4071994bb7df148",
          "Labels": null,
          "ParentId": "",
          "RepoDigests": [
            "postgres@sha256:5e027a0e4b3eebb181af3630815c087f17f867abb9bc5860b4e2a203019a04c2"
          ],
          "RepoTags": [
            "postgres:latest"
          ],
          "VirtualSize": 431419939
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": [
              "postgres"
            ],
            "Domainname": "",
            "Entrypoint": [
              "docker-entrypoint.sh"
            ],
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/usr/lib/postgresql/15/bin",
              "GOSU_VERSION=1.17",
              "LANG=en_US.utf8",
              "PG_MAJOR=15",
              "PG_VERSION=15.6-1.pgdg120+2",
              "PGDATA=/var/lib/postgresql/data"
            ],
            "ExposedPorts": {
              "5432/tcp": {}
            },
            "Hostname": "",
            "Image": "",
            "Labels": null,
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "",
            "WorkingDir": ""
          },
          "Created": "2024-02-21T00:46:13Z",
          "DockerVersion": "",
          "Id": "sha256:8056eb3aa10520a3f2fc359f770e38ca902462263e1d4d1f9415911f28d9c640",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "postgres@sha256:a72bcd42bfd486ab88fdbf25e5c00001a3af209f7a3e66ab25e2ba8c0cf76278"
          ],
          "RepoTags": [
            "postgres:15"
          ],
          "Size": 425381483,
          "VirtualSize": 425381483
        },
        "image": {
          "Created": "2024-02-21T00:46:13Z",
          "Id": "sha256:8056eb3aa10520a3f2fc359f770e38ca902462263e1d4d1f9415911f28d9c640",
          "Labels": null,
          "ParentId": "",
          "RepoDigests": [
            "postgres@sha256:a72bcd42bfd486ab88fdbf25e5c00001a3af209f7a3e66ab25e2ba8c0cf76278"
          ],
          "RepoTags": [
            "postgres:15"
          ],
          "VirtualSize": 425381483
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": [
              "--rpc.allow-unprotected-txs",
              "--http",
              "--http.addr",
              "0.0.0.0",
              "--http.corsdomain",
              "*",
              "--http.vhosts",
              "*",
              "--ws",
              "--ws.origins",
              "*",
              "--ws.addr",
              "0.0.0.0",
              "--dev",
              "--dev.period",
              "1",
              "--datadir",
              "/geth_data"
            ],
            "Domainname": "",
            "Entrypoint": [
              "/usr/local/bin/entrypoint.sh"
            ],
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
            ],
            "ExposedPorts": {
              "30303/tcp": {},
              "30303/udp": {},
              "8545/tcp": {},
              "8546/tcp": {},
              "9545/tcp": {}
            },
            "Hostname": "",
            "Image": "",
            "Labels": {
              "buildnum": "28655",
              "commit": "e501b3b05db8e169f67dc78b7b59bc352b3c638d",
              "version": "1.12.0-stable"
            },
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "",
            "WorkingDir": ""
          },
          "Created": "2024-02-20T12:28:57.565709929Z",
          "DockerVersion": "",
          "Id": "sha256:7f18d47fcd56e28d29e243ac807b115210fc695212bf6f9d62dd88578a685c56",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "hermeznetwork/geth-zkevm-contracts@sha256:05bbea2b02a69442f0db474eeb0eeb874fa8f960ce3909ca5ec5ee55f1f40ee9"
          ],
          "RepoTags": [
            "hermeznetwork/geth-zkevm-contracts:v2.1.3-fork.8-geth1.12.0"
          ],
          "Size": 70779125,
          "VirtualSize": 70779125
        },
        "image": {
          "Created": "2024-02-20T12:28:57Z",
          "Id": "sha256:7f18d47fcd56e28d29e243ac807b115210fc695212bf6f9d62dd88578a685c56",
          "Labels": {
            "buildnum": "28655",
            "commit": "e501b3b05db8e169f67dc78b7b59bc352b3c638d",
            "version": "1.12.0-stable"
          },
          "ParentId": "",
          "RepoDigests": [
            "hermeznetwork/geth-zkevm-contracts@sha256:05bbea2b02a69442f0db474eeb0eeb874fa8f960ce3909ca5ec5ee55f1f40ee9"
          ],
          "RepoTags": [
            "hermeznetwork/geth-zkevm-contracts:v2.1.3-fork.8-geth1.12.0"
          ],
          "VirtualSize": 70779125
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": [
              "celestia-da"
            ],
            "Domainname": "",
            "Entrypoint": [
              "/bin/bash",
              "/opt/entrypoint.sh"
            ],
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
              "CELESTIA_HOME=/home/celestia",
              "NODE_TYPE=bridge",
              "P2P_NETWORK=mocha"
            ],
            "ExposedPorts": {
              "2121/tcp": {}
            },
            "Hostname": "",
            "Image": "",
            "Labels": {
              "commitUrl": "https://github.com/rollkit/celestia-da/commit/5892064d5539177b97d5ac406eb0def6e0fc4bf5",
              "dockerPull": "docker pull ghcr.io/rollkit/celestia-da:5892064d",
              "maintainer": "rollkit",
              "org.opencontainers.image.created": "2024-02-06T19:19:37.760Z",
              "org.opencontainers.image.description": "rollkit repository rollkit/celestia-da",
              "org.opencontainers.image.licenses": "Apache-2.0",
              "org.opencontainers.image.revision": "5892064d5539177b97d5ac406eb0def6e0fc4bf5",
              "org.opencontainers.image.source": "https://github.com/rollkit/celestia-da",
              "org.opencontainers.image.title": "celestia-da",
              "org.opencontainers.image.url": "https://github.com/rollkit/celestia-da",
              "org.opencontainers.image.version": "v0.12.10"
            },
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "celestia",
            "WorkingDir": ""
          },
          "Created": "2024-02-06T19:21:49.636216673Z",
          "DockerVersion": "",
          "Id": "sha256:e3714512e64587806631e6e478b0bddf49c010520b0904830a282b50cdfd3bf0",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "ghcr.io/rollkit/celestia-da@sha256:e8b5a04ae9ee9d721dc0610d91773eb8f57ddd433edf205cb206c9366e31b98c"
          ],
          "RepoTags": [
            "ghcr.io/rollkit/celestia-da:v0.12.10"
          ],
          "Size": 128362090,
          "VirtualSize": 128362090
        },
        "image": {
          "Created": "2024-02-06T19:21:49Z",
          "Id": "sha256:e3714512e64587806631e6e478b0bddf49c010520b0904830a282b50cdfd3bf0",
          "Labels": {
            "commitUrl": "https://github.com/rollkit/celestia-da/commit/5892064d5539177b97d5ac406eb0def6e0fc4bf5",
            "dockerPull": "docker pull ghcr.io/rollkit/celestia-da:5892064d",
            "maintainer": "rollkit",
            "org.opencontainers.image.created": "2024-02-06T19:19:37.760Z",
            "org.opencontainers.image.description": "rollkit repository rollkit/celestia-da",
            "org.opencontainers.image.licenses": "Apache-2.0",
            "org.opencontainers.image.revision": "5892064d5539177b97d5ac406eb0def6e0fc4bf5",
            "org.opencontainers.image.source": "https://github.com/rollkit/celestia-da",
            "org.opencontainers.image.title": "celestia-da",
            "org.opencontainers.image.url": "https://github.com/rollkit/celestia-da",
            "org.opencontainers.image.version": "v0.12.10"
          },
          "ParentId": "",
          "RepoDigests": [
            "ghcr.io/rollkit/celestia-da@sha256:e8b5a04ae9ee9d721dc0610d91773eb8f57ddd433edf205cb206c9366e31b98c"
          ],
          "RepoTags": [
            "ghcr.io/rollkit/celestia-da:v0.12.10"
          ],
          "VirtualSize": 128362090
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": null,
            "Domainname": "",
            "Entrypoint": [
              "/bin/bash",
              "/opt/entrypoint.sh"
            ],
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
              "CELESTIA_HOME=/home/celestia",
              "NODE_TYPE=bridge",
              "P2P_NETWORK=mocha"
            ],
            "ExposedPorts": {
              "2121/tcp": {},
              "26650/tcp": {},
              "26657/tcp": {},
              "26658/tcp": {},
              "26659/tcp": {},
              "9090/tcp": {}
            },
            "Hostname": "",
            "Image": "",
            "Labels": {
              "commitUrl": "https://github.com/rollkit/local-celestia-devnet/commit/8ffd0196fc88cba7c9c3a54e248d0555755307e7",
              "dockerPull": "docker pull ghcr.io/rollkit/local-celestia-devnet:8ffd0196",
              "maintainer": "rollkit",
              "org.opencontainers.image.created": "2024-01-12T17:02:15.086Z",
              "org.opencontainers.image.description": "rollkit repository rollkit/local-celestia-devnet",
              "org.opencontainers.image.licenses": "Apache-2.0",
              "org.opencontainers.image.revision": "8ffd0196fc88cba7c9c3a54e248d0555755307e7",
              "org.opencontainers.image.source": "https://github.com/rollkit/local-celestia-devnet",
              "org.opencontainers.image.title": "local-celestia-devnet",
              "org.opencontainers.image.url": "https://github.com/rollkit/local-celestia-devnet",
              "org.opencontainers.image.version": "v0.12.6"
            },
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "celestia",
            "WorkingDir": ""
          },
          "Created": "2024-01-12T17:02:20.581498788Z",
          "DockerVersion": "",
          "Id": "sha256:22805d354ce988a862e19826652bfff502768eb0186c00351c80305db5e19b08",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "ghcr.io/rollkit/local-celestia-devnet@sha256:a98c4d623444121f99eab3b81e2d92e35a65facceef1ba6c0ed9959e2f25311a"
          ],
          "RepoTags": [
            "ghcr.io/rollkit/local-celestia-devnet:v0.12.6"
          ],
          "Size": 226125572,
          "VirtualSize": 226125572
        },
        "image": {
          "Created": "2024-01-12T17:02:20Z",
          "Id": "sha256:22805d354ce988a862e19826652bfff502768eb0186c00351c80305db5e19b08",
          "Labels": {
            "commitUrl": "https://github.com/rollkit/local-celestia-devnet/commit/8ffd0196fc88cba7c9c3a54e248d0555755307e7",
            "dockerPull": "docker pull ghcr.io/rollkit/local-celestia-devnet:8ffd0196",
            "maintainer": "rollkit",
            "org.opencontainers.image.created": "2024-01-12T17:02:15.086Z",
            "org.opencontainers.image.description": "rollkit repository rollkit/local-celestia-devnet",
            "org.opencontainers.image.licenses": "Apache-2.0",
            "org.opencontainers.image.revision": "8ffd0196fc88cba7c9c3a54e248d0555755307e7",
            "org.opencontainers.image.source": "https://github.com/rollkit/local-celestia-devnet",
            "org.opencontainers.image.title": "local-celestia-devnet",
            "org.opencontainers.image.url": "https://github.com/rollkit/local-celestia-devnet",
            "org.opencontainers.image.version": "v0.12.6"
          },
          "ParentId": "",
          "RepoDigests": [
            "ghcr.io/rollkit/local-celestia-devnet@sha256:a98c4d623444121f99eab3b81e2d92e35a65facceef1ba6c0ed9959e2f25311a"
          ],
          "RepoTags": [
            "ghcr.io/rollkit/local-celestia-devnet:v0.12.6"
          ],
          "VirtualSize": 226125572
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": [
              "redis-server"
            ],
            "Domainname": "",
            "Entrypoint": [
              "docker-entrypoint.sh"
            ],
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
              "GOSU_VERSION=1.17",
              "REDIS_VERSION=7.2.4",
              "REDIS_DOWNLOAD_URL=http://download.redis.io/releases/redis-7.2.4.tar.gz",
              "REDIS_DOWNLOAD_SHA=8d104c26a154b29fd67d6568b4f375212212ad41e0c2caa3d66480e78dbd3b59"
            ],
            "ExposedPorts": {
              "6379/tcp": {}
            },
            "Hostname": "",
            "Image": "",
            "Labels": null,
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "",
            "WorkingDir": "/data"
          },
          "Created": "2024-01-09T16:09:57Z",
          "DockerVersion": "",
          "Id": "sha256:170a1e90f8436daa6778aeea3926e716928826c215ca23a8dfd8055f663f9428",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "redis@sha256:3134997edb04277814aa51a4175a588d45eb4299272f8eff2307bbf8b39e4d43"
          ],
          "RepoTags": [
            "redis:latest"
          ],
          "Size": 137704575,
          "VirtualSize": 137704575
        },
        "image": {
          "Created": "2024-01-09T16:09:57Z",
          "Id": "sha256:170a1e90f8436daa6778aeea3926e716928826c215ca23a8dfd8055f663f9428",
          "Labels": null,
          "ParentId": "",
          "RepoDigests": [
            "redis@sha256:3134997edb04277814aa51a4175a588d45eb4299272f8eff2307bbf8b39e4d43"
          ],
          "RepoTags": [
            "redis:latest"
          ],
          "VirtualSize": 137704575
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": null,
            "Domainname": "",
            "Entrypoint": null,
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
            ],
            "ExposedPorts": null,
            "Hostname": "",
            "Image": "",
            "Labels": {
              "org.opencontainers.image.ref.name": "ubuntu",
              "org.opencontainers.image.version": "22.04"
            },
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "",
            "WorkingDir": "/app"
          },
          "Created": "2023-11-10T22:11:55.635610649Z",
          "DockerVersion": "",
          "Id": "sha256:7676707edcfbbaf61d787b7327f4aedc2059402f0ac951883c320bb70cd4463d",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "hermeznetwork/zkevm-prover@sha256:de701fe585b462d49846af44c503e09b134dbcac6322b87975657e353375c8e6"
          ],
          "RepoTags": [
            "hermeznetwork/zkevm-prover:v3.0.2"
          ],
          "Size": 838126514,
          "VirtualSize": 838126514
        },
        "image": {
          "Created": "2023-11-10T22:11:55Z",
          "Id": "sha256:7676707edcfbbaf61d787b7327f4aedc2059402f0ac951883c320bb70cd4463d",
          "Labels": {
            "org.opencontainers.image.ref.name": "ubuntu",
            "org.opencontainers.image.version": "22.04"
          },
          "ParentId": "",
          "RepoDigests": [
            "hermeznetwork/zkevm-prover@sha256:de701fe585b462d49846af44c503e09b134dbcac6322b87975657e353375c8e6"
          ],
          "RepoTags": [
            "hermeznetwork/zkevm-prover:v3.0.2"
          ],
          "VirtualSize": 838126514
        }
      },
      {
        "details": {
          "Architecture": "amd64",
          "Author": "",
          "Comment": "buildkit.dockerfile.v0",
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": [
              "/bin/ryuk"
            ],
            "Domainname": "",
            "Entrypoint": null,
            "Env": [
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
            ],
            "ExposedPorts": null,
            "Hostname": "",
            "Image": "",
            "Labels": {
              "org.testcontainers.ryuk": "true"
            },
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "",
            "WorkingDir": ""
          },
          "Created": "2023-05-17T15:11:02.784684507Z",
          "DockerVersion": "",
          "Id": "sha256:ec913eeff75a6b5ed284cf17e186213a466ec10a8f471798318ffa0813b5d828",
          "Os": "linux",
          "Parent": "",
          "RepoDigests": [
            "testcontainers/ryuk@sha256:533abc56c07b52a26c955d1e7ae428d810582ab01c156384ae79960eb5fa0775"
          ],
          "RepoTags": [
            "testcontainers/ryuk:0.5.1"
          ],
          "Size": 12695732,
          "VirtualSize": 12695732
        },
        "image": {
          "Created": "2023-05-17T15:11:02Z",
          "Id": "sha256:ec913eeff75a6b5ed284cf17e186213a466ec10a8f471798318ffa0813b5d828",
          "Labels": {
            "org.testcontainers.ryuk": "true"
          },
          "ParentId": "",
          "RepoDigests": [
            "testcontainers/ryuk@sha256:533abc56c07b52a26c955d1e7ae428d810582ab01c156384ae79960eb5fa0775"
          ],
          "RepoTags": [
            "testcontainers/ryuk:0.5.1"
          ],
          "VirtualSize": 12695732
        }
      }
    ],
    "total": 13
  },
  "msg": null,
  "uid": "b5321ef2-204f-4bbb-9a1a-1d6f2d559124"
}

```

## get: prover/container/list?page_size=1&page_count=10

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
        "container": {
          "Command": "/bin/bash /opt/entrypoint.sh celestia-da light start --p2p.network=mocha --da.grpc.namespace=000000506f6c61726973 --da.grpc.listen=0.0.0.0:26650 --core.ip=rpc-mocha.pops.one --gateway",
          "Created": "2024-07-16T08:49:08Z",
          "Id": "65bd167c9f012e77584824cd095d43a41daa5189c3046644330d9568fef986ce",
          "Image": "ghcr.io/rollkit/celestia-da:v0.12.10",
          "ImageID": "sha256:e3714512e64587806631e6e478b0bddf49c010520b0904830a282b50cdfd3bf0",
          "Labels": {
            "commitUrl": "https://github.com/rollkit/celestia-da/commit/5892064d5539177b97d5ac406eb0def6e0fc4bf5",
            "dockerPull": "docker pull ghcr.io/rollkit/celestia-da:5892064d",
            "maintainer": "rollkit",
            "org.opencontainers.image.created": "2024-02-06T19:19:37.760Z",
            "org.opencontainers.image.description": "rollkit repository rollkit/celestia-da",
            "org.opencontainers.image.licenses": "Apache-2.0",
            "org.opencontainers.image.revision": "5892064d5539177b97d5ac406eb0def6e0fc4bf5",
            "org.opencontainers.image.source": "https://github.com/rollkit/celestia-da",
            "org.opencontainers.image.title": "celestia-da",
            "org.opencontainers.image.url": "https://github.com/rollkit/celestia-da",
            "org.opencontainers.image.version": "v0.12.10"
          },
          "Names": [
            "/minner-ghcr.io-rollkit-celestia-da-v0.12.10"
          ],
          "Ports": [],
          "SizeRootFs": null,
          "SizeRw": null,
          "State": "exited",
          "Status": "Exited (0) 5 days ago"
        },
        "details": {
          "AppArmorProfile": "docker-default",
          "Args": [
            "/opt/entrypoint.sh",
            "celestia-da",
            "light",
            "start",
            "--p2p.network=mocha",
            "--da.grpc.namespace=000000506f6c61726973",
            "--da.grpc.listen=0.0.0.0:26650",
            "--core.ip=rpc-mocha.pops.one",
            "--gateway"
          ],
          "Config": {
            "AttachStderr": false,
            "AttachStdin": false,
            "AttachStdout": false,
            "Cmd": [
              "celestia-da",
              "light",
              "start",
              "--p2p.network=mocha",
              "--da.grpc.namespace=000000506f6c61726973",
              "--da.grpc.listen=0.0.0.0:26650",
              "--core.ip=rpc-mocha.pops.one",
              "--gateway"
            ],
            "Domainname": "",
            "Entrypoint": [
              "/bin/bash",
              "/opt/entrypoint.sh"
            ],
            "Env": [
              "NODE_TYPE=light",
              "P2P_NETWORK=mocha",
              "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
              "CELESTIA_HOME=/home/celestia"
            ],
            "ExposedPorts": {
              "2121/tcp": {},
              "26650/tcp": {},
              "26658/tcp": {},
              "26659/tcp": {}
            },
            "Hostname": "65bd167c9f01",
            "Image": "ghcr.io/rollkit/celestia-da:v0.12.10",
            "Labels": {
              "commitUrl": "https://github.com/rollkit/celestia-da/commit/5892064d5539177b97d5ac406eb0def6e0fc4bf5",
              "dockerPull": "docker pull ghcr.io/rollkit/celestia-da:5892064d",
              "maintainer": "rollkit",
              "org.opencontainers.image.created": "2024-02-06T19:19:37.760Z",
              "org.opencontainers.image.description": "rollkit repository rollkit/celestia-da",
              "org.opencontainers.image.licenses": "Apache-2.0",
              "org.opencontainers.image.revision": "5892064d5539177b97d5ac406eb0def6e0fc4bf5",
              "org.opencontainers.image.source": "https://github.com/rollkit/celestia-da",
              "org.opencontainers.image.title": "celestia-da",
              "org.opencontainers.image.url": "https://github.com/rollkit/celestia-da",
              "org.opencontainers.image.version": "v0.12.10"
            },
            "OnBuild": null,
            "OpenStdin": false,
            "StdinOnce": false,
            "Tty": false,
            "User": "celestia",
            "WorkingDir": ""
          },
          "Created": "2024-07-16T08:49:08.073641382Z",
          "Driver": "overlay2",
          "HostConfig": {
            "CgroupParent": "",
            "ContainerIDFile": "",
            "CpuShares": 0,
            "CpusetCpus": "",
            "Memory": 0,
            "MemorySwap": 0,
            "NetworkMode": "default",
            "PidMode": "",
            "PortBindings": {
              "26650/tcp": [
                {
                  "HostIp": "",
                  "HostPort": "26650"
                }
              ],
              "26658/tcp": [
                {
                  "HostIp": "",
                  "HostPort": "26658"
                }
              ],
              "26659/tcp": [
                {
                  "HostIp": "",
                  "HostPort": "26659"
                }
              ]
            },
            "Privileged": false,
            "PublishAllPorts": false,
            "ReadonlyRootfs": false
          },
          "HostnamePath": "/var/snap/docker/common/var-lib-docker/containers/65bd167c9f012e77584824cd095d43a41daa5189c3046644330d9568fef986ce/hostname",
          "HostsPath": "/var/snap/docker/common/var-lib-docker/containers/65bd167c9f012e77584824cd095d43a41daa5189c3046644330d9568fef986ce/hosts",
          "Id": "65bd167c9f012e77584824cd095d43a41daa5189c3046644330d9568fef986ce",
          "Image": "sha256:e3714512e64587806631e6e478b0bddf49c010520b0904830a282b50cdfd3bf0",
          "LogPath": "/var/snap/docker/common/var-lib-docker/containers/65bd167c9f012e77584824cd095d43a41daa5189c3046644330d9568fef986ce/65bd167c9f012e77584824cd095d43a41daa5189c3046644330d9568fef986ce-json.log",
          "MountLabel": "",
          "Mounts": [
            {
              "Destination": "/home/celestia/.celestia-light-mocha-4",
              "Mode": "",
              "RW": true,
              "Source": "/home/cloud/.celestia-light-mocha-4"
            }
          ],
          "Name": "/minner-ghcr.io-rollkit-celestia-da-v0.12.10",
          "NetworkSettings": {
            "Bridge": "",
            "Gateway": "",
            "IPAddress": "",
            "IPPrefixLen": 0,
            "MacAddress": "",
            "Networks": {
              "bridge": {
                "EndpointID": "",
                "Gateway": "",
                "GlobalIPv6Address": "",
                "GlobalIPv6PrefixLen": 0,
                "IPAddress": "",
                "IPPrefixLen": 0,
                "IPv6Gateway": "",
                "MacAddress": "",
                "NetworkID": "fc30e345df0b882e17a752430ababfe7c9b0e9598e6dc7f913d2822b6d4d42b7"
              }
            },
            "Ports": {}
          },
          "Path": "/bin/bash",
          "ProcessLabel": "",
          "ResolvConfPath": "/var/snap/docker/common/var-lib-docker/containers/65bd167c9f012e77584824cd095d43a41daa5189c3046644330d9568fef986ce/resolv.conf",
          "RestartCount": 0,
          "State": {
            "Error": "",
            "ExitCode": 0,
            "FinishedAt": "2024-07-16T08:49:45.839438165Z",
            "OOMKilled": false,
            "Paused": false,
            "Pid": 0,
            "Restarting": false,
            "Running": false,
            "StartedAt": "2024-07-16T08:49:08.497967788Z",
            "Status": "exited"
          }
        }
      }
    ],
    "total": 1
  },
  "msg": null,
  "uid": "4b129d1e-a949-4d81-8a77-e9e5e6180aea"
}
```


## prover/image/pull/{image}/{tag}

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