<p float="left">
  <img src="src/assets/logo.svg" height="100px" alt="Logo" />
  <img src="src/assets/logo_text.svg" height="100px" alt="SMTPclient" />
</p>

[![License](https://img.shields.io/github/license/MoQuEs/smtp_client)](https://github.com/MoQuEs/smtp_client/tree/main)
[![Last commit](https://img.shields.io/github/last-commit/MoQuEs/smtp_client/develop)](https://github.com/MoQuEs/smtp_client/tree/develop)

[![Release](https://img.shields.io/github/v/release/MoQuEs/smtp_client?display_name=tag&sort=semver)](https://github.com/MoQuEs/smtp_client/releases)
[![Release date](https://img.shields.io/github/release-date/MoQuEs/smtp_client)](https://github.com/MoQuEs/smtp_client/releases)

- [Changelog](CHANGELOG.md)
- [Roadmap](ROADMAP.md)

---

## Introduction

SMTPclient is a simple gui smtp client, mainly used for testing smtp servers and send simple mails

<p float="left">
  <img src=".github/assets/img_2.png" width="48%" alt="Logo" />
  <img src=".github/assets/img_3.png" width="48%" alt="SMTPclient" />
</p>

## Development

### Requirements

#### Main

- rust >= 1.77.2
- node >= 20.18.0

#### Additional tools to install

```bash
cargo install typeshare-cli
cargo install cargo-bump
```

#### Run locally

```bash
npm run tauri dev
```

#### Build locally

```bash
npm run tauri build
```

#### Scripts

- `npm run check` - Run check on frontend
- `npm run format` - Reformat whole project
- `npm run generate` - Generate typescript definitions from rust
- `npm run logo <logo_image_name_in_logo_folder>` - Run generator on logo to generate icons
- `npm run version <sver_version_to_prepare>` - Will bump version in:
    - `package.json` - frontend
    - `cargo.toml` - backend
    - `src-tauri/tauri.conf.json` - build script
    - `CHANGELOG.md` - will change `Unreleased` to set version
