# getsb

[![Build Status](https://travis-ci.org/nsheremet/getsb-cli.svg?branch=master)](https://travis-ci.org/nsheremet/getsb-cli)
[![Build status](https://ci.appveyor.com/api/projects/status/45mgunbeurixiiwj?svg=true)](https://ci.appveyor.com/project/nsheremet/knocli)
[![Crates.io](https://img.shields.io/crates/v/getsb.svg)](https://crates.io/crates/getsb)

Getsb is a command line tool for sending HTTP request.

## Installation

### Manual
You can download prebuilt binaries in the
[releases section](https://github.com/nsheremet/getsb-cli/releases), or create
from source.
```shell
$ git clone https://github.com/nsheremet/getsb-cli.git
$ cd getsb-cli
$ cargo build --release
```
##### Linux
```
# sudo mv target/release/getsb /usr/local/bin
```
##### OSX
```
# sudo mv target/release/getsb /usr/local/bin/getsb
```
##### Windows
- Create a folder for getsb
- search for `env`
- open "edit your enviroment variables"
- edit `PATH`
- append folder path to the end of the string ie: `<path_stuff_here>;C:/getsb/;`

## How to use Getsb

#### Basic usage

This is the basic way to use `getsb`.

##### GET request example
```shell
$ getsb GET https://www.rust-lang.org/ # =>
# Status: 200
#
# Connection: close
# Last-Modified: Thu, 13 Apr 2017 20:18:15 GMT
# Age: 45525
# Server: AmazonS3
# Date: Thu, 13 Apr 2017 20:20:15 GMT
# Content-Type: text/html
# Content-Length: 1456
# X-Cache: Hit from cloudfront
#
# Body here
```

##### POST request example
```shell
$ getsb POST https://example.com/api/data -b "key=value" -h "Content-Type: application/x-www-form-urlencoded" # =>
# Response
```

##### File as request

You can use json files for sending request.
This is example request file `request.json`
```json
{
  "url": "https://example.com/api/data",
  "method": "PUT",
  "headers": [
    "Content-Type: application/json"
  ],
  "body": {
    "key": "value"
  }
}
```
To send a request using this file:
```shell
$ getsb -r request.json # =>
# Response
```

## Options

![Imgur](http://i.imgur.com/rAzJdFv.png?1)

## Canonical Source
The canonical source of this repo is hosted on
[GitHub](https://github.com/nsheremet/getsb-cli). If you have a GitHub account,
please make your issues, and pull requests there.

## Copyright and License
(C) Copyright 2017 by Nazarii Sheremet

Getsb is distributed under the terms of both the MPL2.0 license.

See [LICENSE](./LICENSE) for more information.
