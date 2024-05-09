# Testing Spin Test
A repository to write a Spin app that has spin-test implemented.

# Usage

Clone this repo:

```bash
git clone https://github.com/tpmccallum/testing-spin-test.git
```

Change into app dir:

```bash
cd testing-spin-test
```

Build app:

```bash
spin build
```

Test the actual app out (in a fresh terminal):

```bash
spin up
```

Add data to the application:

```bash
curl localhost:3000/user_id -H 'Content-Type: application/json' -d '{"id":123, "name": "Tim"}'
```

Fetch data from the application:

```bash
curl localhost:3000/user_id?id=123
```

Returns `{"id":123, "name": "Tim"}`

Run the tests:

```bash
spin test
```

At present, returns the following:

```bash
running 4 tests
No value found for the key "/user_id"
thread '<unnamed>' panicked at src/lib.rs:66:5:
assertion `left == right` failed
  left: 404
 right: 200
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test cache-hit                   ... FAILED
No value found for the key "/"
No value found for the key "/not_here"
test request-without-key         ... ok
test request-with-invalid-key    ... ok
No value found for the key "/user_id"
test request-with-invalid-key-id ... ok

failures:

---- cache-hit ----
test 'spin-test-cache-hit' failed 


failures:
    cache-hit

test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.08s
```


