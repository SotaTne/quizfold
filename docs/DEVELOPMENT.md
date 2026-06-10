# QuizFold OSS Repository Structure

## 1. Overview

`quizfold` is the open-source core of QuizFold.

It provides the reusable foundations used by `quizfold-cloud`, CLI users, MCP integrations, and external developers.

The hosted SaaS implementation, billing, authentication, storage, ads, user management, and Cloudflare infrastructure are intentionally kept outside of this repository.

```txt
quizfold        = OSS core
quizfold-cloud  = hosted service / SaaS implementation
```

The dependency direction must always be one-way:

```txt
quizfold-cloud -> quizfold
quizfold       -> no dependency on quizfold-cloud
```

This keeps the OSS core clean, reusable, testable, and independent from the hosted business logic.

---

## 2. Repository Goals

The `quizfold` repository should provide:

- QuizFold Markdown parser
- QuizFold Markdown validator
- Diagnostics with source ranges
- Rust CLI
- WASM parser package for Web and Cloudflare Workers
- MCP tool definitions and schemas
- npm package wrappers for public usage
- Documentation for QuizFold Markdown, CLI, MCP, and parser behavior

The repository should not include:

- Billing logic
- Ads logic
- Cloudflare D1/R2/Queues bindings
- User authentication implementation
- Hosted MCP token verification
- Plus/Pro plan enforcement
- Private material storage implementation
- Web application UI
- Notion Embed hosting implementation

---

## 3. Top-level Structure

```txt
quizfold/
  Cargo.toml
  package.json
  pnpm-workspace.yaml
  README.md
  LICENSE

  crates/
    parser/
    parser-wasm/
    cli/

  packages/
    parser/
    mcp/
    cli/

  docs/
    architecture.md
    quizfold-markdown.md
    diagnostics.md
    parser.md
    mcp.md
    cli.md
    npm-packages.md

  examples/
    basic.qf.md
    cloze.qf.md
    math.qf.md
    image-data.qf.md
    mcp-request.json

  .github/
    workflows/
      test.yml
      release.yml
```

---

## 4. `crates/` Structure

The `crates/` directory contains Rust crates.

```txt
crates/
  parser/        # Pure Rust QuizFold Markdown parser
  parser-wasm/   # WASM wrapper around parser
  cli/           # Rust CLI binary
```

---

## 5. `crates/parser`

`crates/parser` is the core Rust parser.

It is the most important crate in the OSS repository.

### Responsibilities

`crates/parser` is responsible for:

- Parsing QuizFold Markdown
- Extracting Q/A blocks
- Extracting Fold lines
- Extracting `${...}` inline answers
- Detecting math blocks
- Detecting inline math where supported
- Detecting image references
- Returning AST nodes
- Returning source ranges
- Returning structural diagnostics
- Providing normalized output helpers where appropriate

### Non-responsibilities

`crates/parser` must not know about:

- Users
- Billing
- Free/Plus/Pro plans
- Cloudflare
- R2 assets
- D1 database rows
- MCP tokens
- Whether `qf-attachment:image1` actually exists in a request

That kind of validation belongs to `quizfold-cloud` or to a higher-level request validator.

### Suggested files

```txt
crates/parser/
  Cargo.toml
  src/
    lib.rs
    ast.rs
    parser.rs
    lexer.rs
    diagnostics.rs
    validate.rs
    normalize.rs
    references.rs
    source.rs
    format.rs
  tests/
    qa_block.rs
    fold_line.rs
    inline_answer.rs
    math_block.rs
    image_ref.rs
    diagnostics.rs
```

### Core API sketch

```rust
pub fn parse_quizfold(input: &str) -> ParseResult;

pub fn validate_quizfold(input: &str) -> Vec<Diagnostic>;

pub fn extract_quiz_items(input: &str) -> ExtractResult;
```

### Parse result sketch

```rust
pub struct ParseResult {
    pub document: QuizFoldDocument,
    pub diagnostics: Vec<Diagnostic>,
    pub references: References,
    pub stats: ParseStats,
}
```

### Diagnostic severity

QuizFold diagnostics use three levels:

```rust
pub enum Severity {
    Fatal,
    Error,
    Warning,
}
```

Meaning:

```txt
fatal:
  Safe recovery is not possible. Web, MCP, and CLI must reject.

error:
  The content is not valid QuizFold content.
  MCP and CLI must reject.
  Web editor may keep it as an editable draft.

warning:
  The content was accepted, but something should be shown to the user or AI.
```

### Diagnostic handling

QuizFold does not need parser-level `Editing` / `Commit` modes.

Callers decide behavior from diagnostic severity.

```txt
fatal:
  Fatal failure. No caller may continue.
  Web, MCP, CLI, and API must all reject.

error:
  Validation failure.
  Web may keep the content editable while disabling the invalid part.
  MCP, CLI, and API must reject add/commit operations.

warning:
  Accepted with a warning.
  Web, MCP, CLI, and API may continue while showing the warning.
```

---

## 6. `crates/parser-wasm`

`crates/parser-wasm` is a thin WASM wrapper around `crates/parser`.

### Responsibilities

- Expose parser functions to JavaScript and TypeScript
- Convert Rust results into JS-friendly JSON values
- Be usable from browser apps
- Be usable from Cloudflare Workers
- Be consumed by `packages/parser`

### Non-responsibilities

It should not contain parser logic itself.

The actual parser logic must stay in `crates/parser`.

### Suggested files

```txt
crates/parser-wasm/
  Cargo.toml
  src/
    lib.rs
```

### API

```rust
#[wasm_bindgen(js_name = parseQuizFold)]
pub fn parse_quizfold(input: &str) -> Result<JsParseResult, JsValue>;

#[wasm_bindgen(js_name = validateQuizFold)]
pub fn validate_quizfold(input: &str) -> Result<JsDiagnostics, JsValue>;
```

`serde-wasm-bindgen` converts parser values directly into JavaScript objects.
The generated declaration file exposes `ParseResult` and `Diagnostic[]`, so consumers do not parse JSON strings or define duplicate public result types.

---

## 7. `crates/cli`

`crates/cli` is the native Rust CLI.

### Responsibilities

- Provide the `quizfold` binary
- Validate local QuizFold Markdown files
- Preview parse results
- Import/append Markdown through QuizFold Cloud API
- Manage login/token configuration
- Call hosted APIs with Bearer tokens
- Use `crates/parser` directly for local validation

### Suggested commands

```bash
quizfold --help
quizfold validate notes.qf.md
quizfold preview notes.qf.md
quizfold login
quizfold logout
quizfold whoami
quizfold append notes.qf.md --folder "Biology/Evolution" --note "Exam Review"
quizfold mcp info
```

### Suggested files

```txt
crates/cli/
  Cargo.toml
  src/
    main.rs
    commands/
      mod.rs
      validate.rs
      preview.rs
      login.rs
      logout.rs
      whoami.rs
      append.rs
      mcp.rs
    config/
      mod.rs
      paths.rs
      token.rs
    api/
      mod.rs
      client.rs
      types.rs
```

### Dependency

```txt
crates/cli -> crates/parser
```

The CLI should not depend on `parser-wasm`.

---

## 8. `packages/` Structure

The `packages/` directory contains npm packages.

```txt
packages/
  parser/   # @quizfold/parser
  mcp/      # @quizfold/mcp
  cli/      # @quizfold/cli
```

---

## 9. `packages/parser`

`packages/parser` is published as:

```txt
@quizfold/parser
```

It is the public JavaScript/TypeScript parser package.

Internally, it uses the WASM build from `crates/parser-wasm`.

### Public API

```ts
import {
  parseQuizFold,
  validateQuizFold,
  extractQuizItems,
} from "@quizfold/parser";
```

### API sketch

```ts
export async function parseQuizFold(
  input: string,
): Promise<ParseResult>;

export async function validateQuizFold(
  input: string,
): Promise<Diagnostic[]>;

export async function extractQuizItems(
  input: string,
): Promise<ExtractResult>;
```

### Notes

- Consumers should not need to know that WASM is used internally.
- The Rust wrapper and public TypeScript types are shared by every runtime.
- `wasm-pack` emits `web`, `bundler`, and `nodejs` targets from the same crate.
- Conditional exports select `browser`, `workerd`, or `node`; `import` and `default` use the bundler build.
- The Node target may remain CommonJS internally, but its public entry point must be ESM.
- APIs should be async from the beginning because WASM initialization may be async.

### Files

```txt
packages/parser/
  package.json
  scripts/
    build.ts
  src/
    index.ts
    index.browser.ts
    index.worker.ts
    index.node.ts
  dist/
    index.js
    index.browser.js
    index.worker.js
    index.node.js
    index.d.ts
    wasm/
      browser/
      bundler/
      node/
```

---

## 10. `packages/mcp`

`packages/mcp` is published as:

```txt
@quizfold/mcp
```

It provides MCP-related schemas, tool definitions, and shared request/response types.

### Responsibilities

- Define MCP tools for QuizFold
- Define request schemas
- Define response schemas
- Share types between clients, hosted service, and tests
- Describe the expected behavior of tools

### Non-responsibilities

`@quizfold/mcp` must not:

- Verify hosted service tokens
- Access D1/R2
- Enforce billing plans
- Count MCP write usage
- Store notes
- Upload images

Those belong to `quizfold-cloud`.

### Initial MCP tools

```txt
preview_quiz_markdown
append_quiz_markdown
```

Future tools:

```txt
upload_data_asset
list_notes
list_folders
create_note
```

### `append_quiz_markdown` input sketch

```ts
export type AppendQuizMarkdownInput = {
  folderPath?: string;
  noteTitle?: string;
  markdown: string;
  datas?: Record<string, QuizFoldData>;
};
```

### `datas` sketch

```ts
export type QuizFoldData =
  | {
      type: "image";
      mimeType: "image/png" | "image/jpeg" | "image/webp";
      base64: string;
      filename?: string;
      alt?: string;
    };
```

### Important distinction

`qf-attachment:image1` references are parsed by `@quizfold/parser`, but whether `datas.image1` exists is not a parser concern.

That check belongs to request/context validation in `quizfold-cloud`.

---

## 11. `packages/cli`

`packages/cli` is published as:

```txt
@quizfold/cli
```

It is an npm distribution wrapper for the Rust CLI binary.

The CLI itself is written in Rust in `crates/cli`.

### Platform packages

The npm CLI should use platform-specific packages:

```txt
@quizfold/cli
@quizfold/cli-aarch64-apple-darwin
@quizfold/cli-x86_64-apple-darwin
@quizfold/cli-aarch64-unknown-linux-gnu
@quizfold/cli-x86_64-unknown-linux-gnu
@quizfold/cli-aarch64-pc-windows-msvc
@quizfold/cli-x86_64-pc-windows-msvc
```

32-bit and legacy PC support is intentionally out of scope.

### Supported Rust targets

```txt
aarch64-apple-darwin
x86_64-apple-darwin

aarch64-unknown-linux-gnu
x86_64-unknown-linux-gnu

aarch64-pc-windows-msvc
x86_64-pc-windows-msvc
```

### User install command

```bash
npm install -g @quizfold/cli
```

or:

```bash
npx @quizfold/cli --help
```

---

## 12. QuizFold Markdown v0.1

The initial QuizFold Markdown syntax should be small and strict.

### Q/A block

```md
? 江戸幕府はいつ、誰によって開かれた？
---
${1603年} に ${徳川家康} によって開かれた。
```

Meaning:

```txt
question = 江戸幕府はいつ、誰によって開かれた？
answer   = ${1603年} に ${徳川家康} によって開かれた。
```

### Fold line

```md
! 江戸幕府は ${1603年} に ${徳川家康} によって開かれた。
```

Meaning:

```txt
The line itself becomes the prompt.
The `${...}` spans are hidden answers.
```

A Fold line must contain at least one `${...}`.

If it does not, it is an error.

### Inline answer

```md
${徳川家康}
```

Inline answers are used inside Q/A answers and Fold lines.

### Math block

````md
```math
E = \frac{1}{2}mv^2
```
````

Math blocks use LaTeX-like input.

Rendering may be handled by KaTeX in the web layer.

If a flow requires rendered math and the math block cannot be rendered, it should be treated as an error.

### Image reference

```md
![細胞の模式図](qf-stored:img_123)
```

Image alt text is required.

Empty alt text is an error.

### Temporary data reference

```md
![細胞の模式図](qf-attachment:image1)
```

`qf-attachment:image1` refers to an attachment supplied in an MCP/API/CLI request.

`qf-stored:img_123` refers to an already stored image, such as one uploaded manually or reused from another QuizFold content item.

External images may use normal Markdown HTTP(S) URLs:

```md
![細胞の模式図](https://example.com/cell.png)
```

The parser only recognizes the external reference. It must not fetch the URL.

Parser responsibility:

```txt
Detect `qf-attachment:image1` and `qf-stored:img_123` as references.
Detect `http://` and `https://` image URLs as external references.
Do not fetch external URLs.
```

Cloud/request validation responsibility:

```txt
Check whether datas.image1 exists, is valid, and is allowed by the user's plan.
When fetching an external URL, validate SSRF risk, redirects, MIME type, and size.
```

---

## 13. Diagnostics Policy

QuizFold uses three diagnostic levels.

```txt
fatal
error
warning
```

### Fatal

Fatal means safe recovery is not possible.

Examples:

- Input is too large to process safely
- Internal parser recovery failed
- Source location tracking failed
- Resource limits were exceeded

Fatal diagnostics reject both Web and MCP/CLI operations.

### Error

Error means the content is not valid QuizFold content.

Examples:

- Q/A block missing `---`
- Empty answer
- Unclosed `${...}`
- Fold line without `${...}`
- Image alt text is empty
- Math block cannot be parsed or rendered where rendering is required
- Unsupported image type in request validation

MCP/CLI must reject on error.

Web editor may keep the content as a draft and show the error inline.

### Warning

Warning means the content was successfully accepted, but something should be shown to the user or AI.

Examples:

- Note title was automatically generated
- Image was automatically compressed
- `qf-attachment:image1` was uploaded and resolved to `qf-stored:img_123`
- Answer is unusually long

Warnings do not block MCP/CLI commit.

---

## 14. Mode-specific Behavior

```txt
              fatal    error    warning
Web           reject   allow*   allow
MCP/CLI       reject   reject   allow
Preview       show     show     show
```

`allow*` means Web may preserve the draft and disable or mark invalid portions. It must not treat error-level content as successfully registered QuizFold content.

### Web

Web is the only caller allowed to keep error-level content editable.

When Web receives error diagnostics, it should show them inline and disable the invalid part instead of accepting that part as valid QuizFold content.

### MCP/CLI

MCP and CLI are commit paths.

If an operation contains any fatal or error diagnostic, it must be rejected.

Only warnings are allowed on successful commit.

---

## 15. Parser vs Context Validation

This distinction is critical.

### Parser / structural validation

The parser can validate things that are visible from Markdown alone.

Examples:

- `?` block structure
- `---` placement
- Fold line syntax
- `${...}` syntax
- math block boundaries
- image alt presence
- source ranges

### Context validation

Context validation requires external request data or user/service state.

It is separate from parsing, but MCP, CLI, API, and Web commit flows must run it before accepting an operation.

Examples:

- Does `datas.image1` exist?
- Is `datas.image1` valid base64?
- Is the image MIME type allowed?
- Is the image size allowed?
- Is this user allowed to use `qf-stored:img_123`?
- Is this user allowed to upload images?
- Has this user exceeded MCP/CLI write limits?

Context validation belongs to `quizfold-cloud`, hosted MCP/API implementations, or CLI request handling, not to `crates/parser`.

For example, the parser can report that Markdown references `qf-attachment:image1`, but it must not decide whether `datas.image1` was actually uploaded. MCP/CLI/API validation must check that request-level fact. Likewise, permission to use `qf-stored:img_123` must be checked outside the parser.

---

## 16. Public npm packages

The OSS repository should initially publish exactly three npm packages:

```txt
@quizfold/parser
@quizfold/mcp
@quizfold/cli
```

Additional packages should not be added until there is a strong need.

This keeps the public API small and understandable.

---

## 17. Relation to `quizfold-cloud`

`quizfold-cloud` is closed source and consumes `quizfold`.

It may use:

```txt
@quizfold/parser
@quizfold/mcp
```

It generally does not need to use:

```txt
@quizfold/cli
```

`quizfold-cloud` owns:

- Web app
- Hosted Remote MCP
- Cloudflare Workers API
- D1 schema and migrations
- R2 storage
- Queues jobs
- Billing
- Ads
- Authentication
- Token management
- Free/Plus/Pro limits
- Notion Embed hosting
- Private material storage

---

## 18. Release Strategy

Initial release order:

1. `crates/parser`
2. `crates/parser-wasm`
3. `packages/parser`
4. `crates/cli`
5. `packages/cli`
6. `packages/mcp`

The parser should be stable before MCP and CLI are promoted heavily.

---

## 19. Design Principle

QuizFold OSS should be:

```txt
Small core.
Strict syntax.
Recoverable diagnostics.
One parser everywhere.
Human-readable Markdown.
AI-writable Markdown.
Cloud-independent OSS.
```

The hosted service should add convenience, persistence, storage, and integration, but the core QuizFold language and tools should remain open.
