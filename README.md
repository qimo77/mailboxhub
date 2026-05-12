# mailboxhub

mailboxhub is a macOS desktop client for batch-managing Outlook mailboxes. It imports multiple Outlook accounts, syncs Inbox messages through IMAP, extracts likely verification codes, and keeps mailbox data in a local SQLite database.

The app is built with Tauri 2, Rust, React, TypeScript, Tailwind CSS, and SQLite.

## Features

- Batch import Outlook accounts in `email----password----client_id----refresh_token` format.
- Paste account lists or drag a `.txt` file into the import page.
- Local SQLite storage for accounts, settings, email summaries, email bodies, unread state, and extracted codes.
- Outlook IMAP sync with OAuth refresh-token based XOAUTH2 authentication.
- Automatic background polling with a 5-second default refresh interval.
- Manual refresh for the selected mailbox.
- Copy mailbox addresses from the account list.
- Mark the current mailbox as read, or mark all imported mailboxes as read at once.
- Email preview with MIME-aware parsing, quoted-printable decoding, HTML cleanup, and fallback text extraction.
- Context-aware 4-8 digit verification-code extraction to avoid footer, address, tracking, and CSS numbers.
- macOS desktop notifications for new mail.
- Light default theme and settings for refresh interval, notifications, and auto-connect.
- macOS `.app` and `.dmg` packaging.

## Download

The macOS Apple Silicon DMG is published on the GitHub Releases page for version `0.1.0`:

- `mailboxhub_0.1.0_aarch64.dmg`

## Account import format

Each line should contain one account:

```text
email@outlook.com----password----client_id----refresh_token
```

The password is accepted for compatibility with exported account lists. IMAP login uses the OAuth client ID and refresh token.

## Development requirements

- macOS
- Node.js and npm
- Rust and Cargo
- Tauri 2 system requirements for macOS development

Install JavaScript dependencies:

```bash
npm install
```

Run the desktop app in development mode:

```bash
npm run tauri:dev
```

Run only the Vite frontend:

```bash
npm run dev
```

## Tests and validation

Run frontend tests:

```bash
npm test
```

Run the frontend build:

```bash
npm run build
```

Run Rust tests and checks:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml
```

Build the production macOS bundles:

```bash
npm run tauri:build
```

Build output:

```text
src-tauri/target/release/bundle/macos/mailboxhub.app
src-tauri/target/release/bundle/dmg/mailboxhub_0.1.0_aarch64.dmg
```

## Data and privacy

mailboxhub stores account records, settings, and synced email data locally in SQLite under the app data directory. The app connects directly to Outlook IMAP and Microsoft OAuth endpoints from your machine. Do not commit local databases, refresh tokens, `.env` files, or account import files to source control.

## Tech stack

- Tauri 2
- Rust
- React 19
- TypeScript
- Tailwind CSS
- SQLite / sqlx
- Zustand
- Vitest

## License

No license file has been added yet. Add a license before distributing or accepting external contributions.
