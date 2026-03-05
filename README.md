# christ-cli

A beautiful Bible TUI for Christian developers. Read Scripture in your terminal.

Built with Rust. Single binary. Works offline with bundled KJV.

![christ-cli demo](assets/demo.gif)

## Install

```sh
npm install -g christ-cli
```

Or with curl:
```sh
curl -fsSL https://raw.githubusercontent.com/whoisyurii/christ-cli/main/install.sh | sh
```

## Usage

Launch the interactive TUI browser:
```sh
christ
```

Read a specific verse:
```sh
christ read John 3:16
```

Read a chapter:
```sh
christ read Genesis 1
```

Read a verse range:
```sh
christ read Psalm 23:1-6
```

Search the Bible:
```sh
christ search "love one another"
```

Random verse:
```sh
christ random
```

Verse of the day:
```sh
christ today
```

Replay the startup animation:
```sh
christ intro
```

## Interactive TUI

When you run `christ` with no arguments, it launches a full-screen terminal browser:

- **Left/Right arrows** - switch between panels (Books, Chapters, Scripture)
- **Up/Down arrows** - navigate within a panel
- **Enter** - select a book or chapter
- **/** - live search the Bible
- **t** - cycle themes (Slate, Midnight, Parchment, Gospel)
- **qq** - quit (press q twice)

Your reading position and theme are saved automatically.

## Features

- Full-screen TUI with 3-panel browser (Books | Chapters | Scripture)
- Animated startup banner
- Live search with instant results as you type
- 4 themes: Slate (dark), Midnight (shadcn/Vercel dark), Parchment (warm light), Gospel (bright white)
- Bundled KJV Bible (works 100% offline, no internet required)
- Online API fallback for 50+ other translations via Bolls.life
- Forgiving reference parser (jn 3:16, 1cor 13, Ps 23:1-6 all work)
- Pipe-friendly (plain text when piped, rich TUI when interactive)
- Session persistence (remembers where you left off)

## Tech

- Rust single binary (~5MB)
- ratatui + crossterm for the TUI
- Bundled KJV (4.7MB embedded, public domain)
- Bolls.life API for other translations (no auth key needed)
- Cross-platform: macOS, Linux, Windows

## License

MIT
