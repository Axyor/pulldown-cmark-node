# pulldown-cmark-node

Native Node.js bindings for the Rust [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) library.

## Introduction

pulldown-cmark-node provides high-performance Markdown parsing and HTML rendering for Node.js applications. By leveraging Rust's native performance and memory safety, it offers a fast and robust alternative to pure JavaScript implementations.

## Features

- High performance: Native execution for efficient parsing.
- Memory safety: Built on Rust's ownership model.
- Extensive GFM support: Tables, task lists, and strikethrough.
- Heading extraction: Support for Table of Contents (TOC) generation.
- Metadata support: YAML and Plus-style frontmatter extraction.
- Plain text conversion: Formatting-free text extraction.
- Math support: LaTeX math block parsing.

## Installation

```bash
npm install pulldown-cmark-node
```

## Usage

## Security Note (XSS)

**Important:** This library converts Markdown to HTML directly. It does **not** sanitize the output. If you are rendering Markdown from untrusted user input, you **must** sanitize the resulting HTML to prevent Cross-Site Scripting (XSS) attacks.

We recommend using a library like [DOMPurify](https://github.com/cure53/DOMPurify) (on the client or server) or [ammonia](https://github.com/rust-ammonia/ammonia) (if processing in Rust) to clean the generated HTML before rendering it in a browser.

### Basic Conversion

```javascript
const { markdownToHtml } = require('pulldown-cmark-node');

const markdown = '# Hello World\nThis is **fast**.';
const html = markdownToHtml(markdown);
```

### Advanced Configuration

Enable GFM extensions, math, and metadata processing:

```javascript
const html = markdownToHtml(markdown, {
  gfm: true,
  math: true,
  metadataBlocks: true,
  headingAttributes: true
});
```

### Heading Extraction

Extract document structure for navigation or TOCs:

```javascript
const { getHeadings } = require('pulldown-cmark-node');

const headings = getHeadings('# Introduction\n## Installation');
// Returns: [{ level: 1, text: 'Introduction' }, { level: 2, text: 'Installation' }]
```

### Metadata and Plain Text

```javascript
const { extractMetadata, markdownToPlainText } = require('pulldown-cmark-node');

const metadata = extractMetadata('---\ntitle: Post\n---\nContent');
const plainText = markdownToPlainText('# Title\n**Bold** content');
```

## API Reference

### `markdownToHtml(input: string, options?: CompileOptions): string`
Main conversion function.

**CompileOptions**
- `tables`: Enable GFM tables.
- `footnotes`: Enable footnote parsing.
- `strikethrough`: Enable GFM strikethrough.
- `tasklists`: Enable GFM task lists.
- `smartPunctuation`: Enable smart quotes and dashes.
- `headingAttributes`: Allow custom IDs via `{#id}`.
- `metadataBlocks`: Support YAML/Plus frontmatter.
- `math`: Support LaTeX math blocks.
- `gfm`: Enable all GitHub Flavored Markdown extensions.
- `maxLength`: Maximum length of the input Markdown string (in bytes) to prevent OOM/DoS attacks.

### `getHeadings(input: string): Array<Heading>`
Returns document headings with levels and optional IDs.

### `extractMetadata(input: string): string | null`
Returns the raw content of the document's metadata block.

### `markdownToPlainText(input: string): string`
Strips Markdown syntax to return plain text content.

## Acknowledgements

This library is a Node.js wrapper for [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark), an excellent and high-performance Markdown parsing library written in Rust. Special thanks to its maintainers and contributors for their foundational work.

## License

[MIT](LICENSE)
