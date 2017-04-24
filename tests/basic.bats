#!/usr/bin/env bats

# checking basic func
@test "checking help option" {
  run getsb --help
  [ "${lines[0]}" = "Getsb 0.1.0" ]
  [ "${lines[1]}" = "Nazarii Sheremet. <nazarii.sheremet@gmail.com>" ]
}

@test "checking GET request" {
  run getsb GET https://www.rust-lang.org/
  [[ "${lines[0]}" =~ "Status: 200" ]]
}

@test "checking POST request with body" {
  run getsb POST http://www.mocky.io/v2/58f48af0100000b60f68cad8 -b "key=value"
  [[ "${lines[0]}" =~ "Status: 400" ]]
}

@test "checking PUT request with header" {
  run getsb PUT http://www.mocky.io/v2/58f48af0100000b60f68cad8 -h "Content-Type: application/x-www-form-urlencoded"
  [[ "${lines[0]}" =~ "Status: 200" ]]
}

@test "checking GET request with header" {
  run getsb PUT http://www.mocky.io/v2/58f48af0100000b60f68cad8 -h "Content-Type: multipart/form-data"
  [[ "${lines[0]}" =~ "Status: 413" ]]
}

@test "checking POST request with file" {
  run getsb -r examples/request.json
  [[ "${lines[0]}" =~ "Status: 200" ]]
}

@test "checking Saving Request to file" {
  run getsb -r examples/request.json -f test.txt
  [[ "${lines[0]}" =~ "Response was saved to file: test.txt" ]]
}
