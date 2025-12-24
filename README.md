# sread

Semantic code reader. Extract specific symbols from source files using tree-sitter.

Built for AI agents - read one function instead of reading 500 lines of context.

## Usage

```
sread <file>:<symbol>
sread <file>:<Class>.<method>
sread <file> --list
```

## Examples

```sh
sread src/auth.py:validate_token
sread lib/api.ts:UserService
sread models.py:User.save
sread handlers.ts --list
```

## Output

Prints the extracted code to stdout. Nothing else.

## Exit codes

- 0: success
- 1: symbol not found
- 2: file error or parse error

## Supported languages

- Python (.py)
- TypeScript (.ts, .tsx, .mts, .cts)
- JavaScript (.js, .jsx, .mjs, .cjs)

## Build

```
cargo build --release
```
