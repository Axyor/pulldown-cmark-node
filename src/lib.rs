#![deny(clippy::all)]

//! # pulldown-cmark-node
//!
//! Blazingly fast Markdown to HTML renderer for Node.js using Rust's `pulldown-cmark`.

use napi_derive::napi;
use pulldown_cmark::{html, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

/// Options for the Markdown compiler.
#[napi(object)]
#[derive(Default, Clone)]
pub struct CompileOptions {
  /// Enables tables (GFM).
  pub tables: Option<bool>,
  /// Enables footnotes.
  pub footnotes: Option<bool>,
  /// Enables strikethrough (GFM).
  pub strikethrough: Option<bool>,
  /// Enables task lists (GFM).
  pub tasklists: Option<bool>,
  /// Enables smart punctuation (e.g. smart quotes, dashes).
  pub smart_punctuation: Option<bool>,
  /// Enables heading attributes (e.g. {#id}).
  pub heading_attributes: Option<bool>,
  /// Enables metadata blocks (YAML and Plus-style frontmatter).
  pub metadata_blocks: Option<bool>,
  /// Enables LaTeX math blocks.
  pub math: Option<bool>,
  /// Enables all GFM extensions.
  pub gfm: Option<bool>,
  /// Maximum length of the input Markdown string (in bytes) to prevent OOM/DoS attacks.
  pub max_length: Option<u32>,
  /// Enables HTML sanitization using the ammonia crate to prevent XSS.
  pub sanitize: Option<bool>,
}

fn get_options(opts: Option<CompileOptions>) -> Options {
  let mut options = Options::empty();
  if let Some(opts) = opts {
    if opts.tables.unwrap_or(false) {
      options.insert(Options::ENABLE_TABLES);
    }
    if opts.footnotes.unwrap_or(false) {
      options.insert(Options::ENABLE_FOOTNOTES);
    }
    if opts.strikethrough.unwrap_or(false) {
      options.insert(Options::ENABLE_STRIKETHROUGH);
    }
    if opts.tasklists.unwrap_or(false) {
      options.insert(Options::ENABLE_TASKLISTS);
    }
    if opts.smart_punctuation.unwrap_or(false) {
      options.insert(Options::ENABLE_SMART_PUNCTUATION);
    }
    if opts.heading_attributes.unwrap_or(false) {
      options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    }
    if opts.metadata_blocks.unwrap_or(false) {
      options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
      options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    }
    if opts.math.unwrap_or(false) {
      options.insert(Options::ENABLE_MATH);
    }
    if opts.gfm.unwrap_or(false) {
      options.insert(Options::ENABLE_GFM);
    }
  } else {
    // Default compatibility (ENABLE_GFM standard extensions)
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
  }
  options
}

/// Converts a Markdown string to HTML.
///
/// **Security Note (XSS):** This function does not sanitize the generated HTML.
/// If `markdown_input` comes from an untrusted source, the resulting HTML may contain
/// malicious tags (like `<script>`). You **must** sanitize the output (e.g. using `DOMPurify`
/// or `ammonia`) before rendering it.
///
/// # Arguments
/// * `markdown_input` - The Markdown source string.
/// * `options` - Optional compiler configuration.
#[napi]
pub fn markdown_to_html(
  markdown_input: String,
  options: Option<CompileOptions>,
) -> napi::Result<String> {
  if let Some(opts) = &options {
    if let Some(max_length) = opts.max_length {
      if markdown_input.len() > max_length as usize {
        return Err(napi::Error::new(
          napi::Status::InvalidArg,
          format!(
            "Input markdown exceeds maximum allowed length of {} bytes",
            max_length
          ),
        ));
      }
    }
  }

  let parse_options = get_options(options.clone());
  let parser = Parser::new_ext(&markdown_input, parse_options);

  let mut html_output = String::with_capacity(markdown_input.len() * 3 / 2);
  html::push_html(&mut html_output, parser);

  if let Some(opts) = options {
    if opts.sanitize.unwrap_or(false) {
      html_output = ammonia::clean(&html_output);
    }
  }

  Ok(html_output)
}

pub struct MarkdownTask {
  markdown_input: String,
  options: Option<CompileOptions>,
}

#[napi]
impl napi::Task for MarkdownTask {
  type Output = String;
  type JsValue = String;

  fn compute(&mut self) -> napi::Result<Self::Output> {
    markdown_to_html(self.markdown_input.clone(), self.options.clone())
  }

  fn resolve(&mut self, env: napi::Env, output: Self::Output) -> napi::Result<Self::JsValue> {
    Ok(output)
  }
}

/// Converts a Markdown string to HTML asynchronously (non-blocking).
///
/// Under the hood, this uses libuv's thread pool, making it ideal for processing
/// large Markdown files without blocking the Node.js event loop.
#[napi]
pub fn markdown_to_html_async(
  markdown_input: String,
  options: Option<CompileOptions>,
) -> napi::bindgen_prelude::AsyncTask<MarkdownTask> {
  napi::bindgen_prelude::AsyncTask::new(MarkdownTask {
    markdown_input,
    options,
  })
}

/// Represents a heading in the document.
#[napi(object)]
pub struct Heading {
  /// Level of the heading (1 to 6).
  pub level: u32,
  /// Text content of the heading.
  pub text: String,
  /// Optional ID attribute (e.g. from `{#id}`).
  pub id: Option<String>,
}

/// Parses the Markdown document and returns a list of headings.
///
/// Useful for building a Table of Contents (TOC).
#[napi]
pub fn get_headings(markdown_input: String) -> Vec<Heading> {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
  options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
  options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
  let parser = Parser::new_ext(&markdown_input, options);
  let mut headings = Vec::new();
  let mut current_heading: Option<(u32, Option<String>, String)> = None;

  for event in parser {
    match event {
      Event::Start(Tag::Heading { level, id, .. }) => {
        let level_num = match level {
          HeadingLevel::H1 => 1,
          HeadingLevel::H2 => 2,
          HeadingLevel::H3 => 3,
          HeadingLevel::H4 => 4,
          HeadingLevel::H5 => 5,
          HeadingLevel::H6 => 6,
        };
        current_heading = Some((level_num, id.map(|s| s.to_string()), String::new()));
      }
      Event::Text(text) => {
        if let Some(ref mut h) = current_heading {
          h.2.push_str(&text);
        }
      }
      Event::End(TagEnd::Heading(_)) => {
        if let Some((level, id, text)) = current_heading.take() {
          headings.push(Heading { level, text, id });
        }
      }
      _ => {}
    }
  }
  headings
}

/// Extracts metadata (frontmatter) from the beginning of the Markdown document.
///
/// Returns the raw content of the YAML or Plus-style metadata block.
#[napi]
pub fn extract_metadata(markdown_input: String) -> Option<String> {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
  options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
  let parser = Parser::new_ext(&markdown_input, options);

  let mut in_metadata = false;
  let mut metadata_content = String::new();

  for event in parser {
    match event {
      Event::Start(Tag::MetadataBlock(_)) => {
        in_metadata = true;
      }
      Event::Text(text) if in_metadata => {
        metadata_content.push_str(&text);
      }
      Event::End(TagEnd::MetadataBlock(_)) => {
        return Some(metadata_content);
      }
      _ => {}
    }
  }
  None
}

/// Converts Markdown to plain text by stripping all formatting.
///
/// Excludes metadata blocks and ensures proper spacing between block elements.
#[napi]
pub fn markdown_to_plain_text(markdown_input: String) -> String {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
  options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
  let parser = Parser::new_ext(&markdown_input, options);
  let mut plain_text = String::new();
  let mut in_metadata = false;

  for event in parser {
    match event {
      Event::Start(Tag::MetadataBlock(_)) => {
        in_metadata = true;
      }
      Event::End(TagEnd::MetadataBlock(_)) => {
        in_metadata = false;
      }
      Event::Text(text) | Event::Code(text) => {
        if !in_metadata {
          plain_text.push_str(&text);
        }
      }
      Event::SoftBreak | Event::HardBreak => {
        if !in_metadata {
          plain_text.push('\n');
        }
      }
      Event::End(
        TagEnd::Heading(_) | TagEnd::Paragraph | TagEnd::Item | TagEnd::Table | TagEnd::TableRow,
      ) => {
        if !in_metadata && !plain_text.ends_with('\n') {
          plain_text.push('\n');
        }
      }
      _ => {}
    }
  }
  plain_text
}
