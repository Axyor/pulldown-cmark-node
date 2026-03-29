import test from 'ava'

import {
  markdownToHtml,
  getHeadings,
  extractMetadata,
  markdownToPlainText,
} from '../index'

test('markdownToHtml - basic conversion', (t) => {
  const md = '# Hello'
  const html = markdownToHtml(md)
  t.is(html.trim(), '<h1>Hello</h1>')
})

test('markdownToHtml - with options (tables)', (t) => {
  const md = '| a | b |\n|---|---|\n| 1 | 2 |'
  const htmlEnabled = markdownToHtml(md, { tables: true })
  t.regex(htmlEnabled, /<table>/)
  
  const htmlDisabled = markdownToHtml(md, { tables: false })
  t.notRegex(htmlDisabled, /<table>/)
})

test('markdownToHtml - with options (math)', (t) => {
  const md = '$E=mc^2$'
  const html = markdownToHtml(md, { math: true })
  t.regex(html, /class="math math-inline"/)
})

test('markdownToHtml - throws if exceeds maxLength', (t) => {
  const md = '# Hello World\nThis is **fast**.'
  const error = t.throws(() => {
    markdownToHtml(md, { maxLength: 10 })
  })
  t.regex(error!.message, /exceeds maximum allowed length/)
})

test('getHeadings - extracts headings with levels and ids', (t) => {
  const md = '# Intro {#id1}\n## Sub'
  const headings = getHeadings(md)
  t.is(headings.length, 2)
  t.like(headings[0], { level: 1, text: 'Intro', id: 'id1' })
  t.like(headings[1], { level: 2, text: 'Sub' })
  t.false('id' in headings[1])
})

test('getHeadings - ignores frontmatter', (t) => {
  const md = '---\ntitle: doc\n---\n# Real Title'
  const headings = getHeadings(md)
  t.is(headings.length, 1)
  t.is(headings[0].text, 'Real Title')
})

test('extractMetadata - retrieves YAML frontmatter', (t) => {
  const md = '---\ntitle: test\n---\ncontent'
  const metadata = extractMetadata(md)
  t.is(metadata?.trim(), 'title: test')
})

test('extractMetadata - returns null when no metadata', (t) => {
  const md = '# No Metadata'
  const metadata = extractMetadata(md)
  t.is(metadata, null)
})

test('markdownToPlainText - strips formatting', (t) => {
  const md = '# Title\n\nThis is **bold** and *italic*.'
  const text = markdownToPlainText(md).trim()
  t.is(text, 'Title\nThis is bold and italic.')
})

test('markdownToPlainText - excludes metadata', (t) => {
  const md = '---\nsecret: yes\n---\nPublic text'
  const text = markdownToPlainText(md)
  t.is(text.trim(), 'Public text')
})
