# AOC 2024

## Solutions

[You can find the solutions here](./src/solver)

## Usage

The solutions are wrapped with a thin runner working like this:

### Download prompt and input:

Make sure to have a `config.toml` with your session token ready.

```bash
cargo run <day> download
```

You'll need to re-run this step to
download the second half of the prompt after finishing the first part

## Run against sample

Run againtst the data in `storage/samples/<day>.txt`

```bash
cargo run <day> sample
```

## Run against input

Run againtst the data in `storage/input/<day>.txt`

```bash
cargo run <day> input
```
