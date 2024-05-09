# Testing Spin Test
A repository to write a Spin app that has [spin test](https://github.com/fermyon/spin-test) implemented.

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

Run the tests:

```bash
spin test
```

At present, returns the following:

```bash
running 4 tests
Found value for the key "123"
test cache-hit                   ... ok
No value found for the key "id=0"
test request-with-invalid-key-id ... ok
No value found for the key ""
No value found for the key "id=123"
test request-without-key         ... ok
test request-with-invalid-key    ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.13s
```


