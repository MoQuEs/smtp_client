# SMTP Client

## Development

### Requirements

- Rust >= 1.66.0
- NodeJS >= 18.16.0

```bash
cargo install typeshare-cli
cargo install cargo-bump
```

Run dev:

```bash
npm run tauri dev
```

Generate `typescript` interfaces:

```bash
typeshare --lang=typescript --output-file=./src/generated/tauri.ts .
```

## TODO

- Add reply_to to frontend
- Add cc to frontend
- Add bcc to frontend
- Add option to change custom headers types
- Add saving/loading configurations
- Add saving/loading messages
- Add hasher for saved passwords
- Add main password to unlock saved configurations
-

## TODO (need to check if can do it with library that I use):

- Add sending amp mail
-
