# pulldown-cmark-node

[![NPM Version](https://img.shields.io/npm/v/pulldown-cmark-node.svg)](https://www.npmjs.com/package/pulldown-cmark-node)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Blazingly fast Markdown to HTML renderer for Node.js, powered by the industry-standard Rust library [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark).

## Key Features

- **🚀 Blazing Performance**: Up to 10-20x faster than pure JavaScript parsers.
- **🛡️ Safe & Secure**: Built with Rust, ensuring memory safety.
- **📦 Zero Dependency**: Compiled to a native binary (NAPI), no heavy Node.js dependencies.
- **✨ Extended Features**: 
    - Full GFM (GitHub Flavored Markdown) support.
    - TOC generation via Heading Extraction.
    - Metadata (Frontmatter) extraction.
    - Math (LaTeX) support.
    - Plain text conversion.

## Installation

```bash
npm install pulldown-cmark-node
```

## Usage

### 1. Simple HTML Conversion

```javascript
const { markdownToHtml } = require('pulldown-cmark-node');

const md = '# Hello World\nThis is **fast**.';
const html = markdownToHtml(md);
console.log(html); 
// <h1>Hello World</h1><p>This is <strong>fast</strong>.</p>
```

### 2. With Options (GFM, Math, Metadata)

```javascript
const html = markdownToHtml(md, {
  gfm: true,
  math: true,
  metadataBlocks: true,
  headingAttributes: true
});
```

### 3. Heading Extraction (Table of Contents)

Useful for generating navigation sidebars or TOCs.

```javascript
const { getHeadings } = require('pulldown-cmark-node');

const headings = getHeadings('# Introduction\n## Installation');
console.log(headings);
// [
//   { level: 1, text: 'Introduction', id: undefined },
//   { level: 2, text: 'Installation', id: undefined }
// ]
```

### 4. Metadata Extraction (Frontmatter)

```javascript
const { extractMetadata } = require('pulldown-cmark-node');

const md = '---\ntitle: My Post\n---\nContent';
const yaml = extractMetadata(md);
console.log(yaml); // "title: My Post"
```

### 5. Plain Text Snippet

```javascript
const { markdownToPlainText } = require('pulldown-cmark-node');

const text = markdownToPlainText('# Title\n**Bold** content');
console.log(text); // "Title\nBold content"
```

## API Reference

### `markdownToHtml(input: string, options?: CompileOptions): string`
Converts Markdown to HTML string.

**CompileOptions:**
- `tables`: boolean (GFM Tables)
- `footnotes`: boolean
- `strikethrough`: boolean (GFM Strikethrough)
- `tasklists`: boolean (GFM Tasklists)
- `smartPunctuation`: boolean
- `headingAttributes`: boolean (Allow `{#id}` syntax)
- `metadataBlocks`: boolean (Allow YAML frontmatter)
- `math`: boolean (Allow LaTeX `$ ... $`)
- `gfm`: boolean (Enable all GFM extensions)

### `getHeadings(input: string): Array<Heading>`
Returns a list of headings found in the document.
`Heading` object: `{ level: number, text: string, id?: string }`.

### `extractMetadata(input: string): string | null`
Extracts the raw content of the first metadata block found.

### `markdownToPlainText(input: string): string`
Strips all Markdown formatting and returns the text content.

## License

[MIT](LICENSE)
