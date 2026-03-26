# pulldown-cmark-node

A high-performance Markdown to HTML renderer for Node.js, powered by the Rust crate [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark).

## Installation

```bash
npm install pulldown-cmark-node
```

## Usage

```javascript
const { markdownToHtml } = require('pulldown-cmark-node')

const markdown = '# Hello from Rust'
const html = markdownToHtml(markdown)

console.log(html) // <h1>Hello from Rust</h1>
```

## Features (Enabled)
- Tables
- Task lists
- Strikethrough
- Footnotes

## Development

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Install dependencies: `npm install`
3. Build the native module: `npm run build`
4. Run tests: `npm test`

## License
MIT
