const { markdownToHtml } = require('./index')

const markdown = `
# Bonjour de Rust ! 🦀

Ceci est un test de **pulldown-cmark** porté sur Node.js.

- Liste 1
- Liste 2

| Table | Header |
|-------|--------|
| Cell  | Content|
`

const html = markdownToHtml(markdown)

console.log('--- Markdown original ---')
console.log(markdown)
console.log('\n--- HTML généré par Rust ---')
console.log(html)
