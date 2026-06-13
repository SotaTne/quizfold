# QuizFold OSS リポジトリ構成

## 1. 概要

`quizfold` は QuizFold のオープンソース・コアです。

`quizfold-cloud`、CLI ユーザー、MCP 連携、外部開発者が利用する再利用可能な基盤を提供します。

ホステッド SaaS 実装、課金、認証、ストレージ、広告、ユーザー管理、Cloudflare インフラは、このリポジトリには意図的に含めません。

```txt
quizfold        = OSS core
quizfold-cloud  = hosted service / SaaS implementation
```

依存方向は常に一方向でなければなりません。

```txt
quizfold-cloud -> quizfold
quizfold       -> no dependency on quizfold-cloud
```

これにより、OSS コアをクリーンで、再利用可能で、テストしやすく、ホステッドサービスのビジネスロジックから独立した状態に保ちます。

---

## 2. リポジトリの目標

`quizfold` リポジトリは、以下を提供するべきです。

- QuizFold Markdown パーサー
- QuizFold Markdown バリデーター
- ソース範囲付き diagnostics
- Rust CLI
- Web と Cloudflare Workers 向け WASM パーサーパッケージ
- MCP tool definitions と schemas
- 公開利用向け npm package wrappers
- QuizFold Markdown、CLI、MCP、parser behavior のドキュメント

このリポジトリには、以下を含めるべきではありません。

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

## 3. トップレベル構成

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

## 4. `crates/` 構成

`crates/` ディレクトリには Rust crate を配置します。

```txt
crates/
  parser/        # Pure Rust QuizFold Markdown parser
  parser-wasm/   # WASM wrapper around parser
  cli/           # Rust CLI binary
```

---

## 5. `crates/parser`

`crates/parser` は中核となる Rust parser です。

OSS リポジトリの中で最も重要な crate です。

### 責務

`crates/parser` は以下を担当します。

- QuizFold Markdown の解析
- Q/A blocks の抽出
- Fold lines の抽出
- `${...}` inline answers の抽出
- math blocks の検出
- サポートされる場合の inline math の検出
- image references の検出
- AST nodes の返却
- source ranges の返却
- structural diagnostics の返却
- 適切な場合の normalized output helpers の提供

### 非責務

`crates/parser` は以下を知ってはいけません。

- Users
- Billing
- Free/Plus/Pro plans
- Cloudflare
- R2 assets
- D1 database rows
- MCP tokens
- `qf-attachment:image1` が実際にリクエスト内に存在するかどうか

この種の validation は `quizfold-cloud` または上位の request validator に属します。

### 推奨ファイル

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

QuizFold diagnostics は 3 つのレベルを使います。

```rust
pub enum Severity {
    Fatal,
    Error,
    Warning,
}
```

意味:

```txt
fatal:
  安全な recovery ができない。Web、MCP、CLI は reject しなければならない。

error:
  content が有効な QuizFold content ではない。
  MCP と CLI は reject しなければならない。
  Web editor は editable draft として保持してもよい。

warning:
  content は受理されたが、user または AI に何かを表示するべき状態。
```

### Diagnostic handling

QuizFold は parser-level の `Editing` / `Commit` modes を必要としません。

呼び出し側は diagnostic severity を見て挙動を決めます。

```txt
fatal:
  致命的失敗。どの caller も処理を続行してはいけない。
  Web、MCP、CLI、API はすべて reject する。

error:
  validation failure。
  Web は invalid part を無効化しつつ editable content として保持してよい。
  MCP、CLI、API は add/commit operations を reject する。

warning:
  warning 付きで accepted。
  Web、MCP、CLI、API は warning を表示しつつ続行してよい。
```

---

## 6. `crates/parser-wasm`

`crates/parser-wasm` は `crates/parser` の薄い WASM wrapper です。

### 責務

- parser functions を JavaScript と TypeScript に公開する
- Rust の結果を JS-friendly な JSON values に変換する
- browser apps から利用できるようにする
- Cloudflare Workers から利用できるようにする
- `packages/parser` から利用される

### 非責務

parser logic 自体を含めるべきではありません。

実際の parser logic は `crates/parser` に留めなければなりません。

### 推奨ファイル

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

parser values は `serde-wasm-bindgen` で JavaScript objects に直接変換する。
生成される declaration file は `ParseResult` と `Diagnostic[]` を公開するため、consumer 側で JSON strings の parse や public result types の重複定義を行わない。

---

## 7. `crates/cli`

`crates/cli` は native Rust CLI です。

### 責務

- `quizfold` binary を提供する
- ローカルの QuizFold Markdown files を validate する
- parse results を preview する
- QuizFold Cloud API 経由で Markdown を import/append する
- login/token configuration を管理する
- Bearer tokens を使って hosted APIs を呼び出す
- local validation では `crates/parser` を直接使う

### 推奨コマンド

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

### 推奨ファイル

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

### 依存関係

```txt
crates/cli -> crates/parser
```

CLI は `parser-wasm` に依存するべきではありません。

---

## 8. `packages/` 構成

`packages/` ディレクトリには npm packages を配置します。

```txt
packages/
  parser/   # @quizfold/parser
  mcp/      # @quizfold/mcp
  cli/      # @quizfold/cli
```

---

## 9. `packages/parser`

`packages/parser` は以下として publish されます。

```txt
@quizfold/parser
```

これは公開 JavaScript/TypeScript parser package です。

内部では `crates/parser-wasm` の WASM build を使います。

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

- Consumers が WASM を内部で使っていることを知る必要はないようにする。
- Rust wrapper と公開 TypeScript types は全 runtime で共有する。
- 同じ crate から `wasm-pack` の `web`、`bundler`、`nodejs` targets を生成する。
- Conditional exports で `browser`、`workerd`、`node` を選択し、`import` と `default` は bundler build を使う。
- Node target は内部では CommonJS のままでよいが、公開 entry point は ESM にする。
- WASM initialization が async になり得るため、APIs は最初から async にするべき。

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

`packages/mcp` は以下として publish されます。

```txt
@quizfold/mcp
```

MCP 関連の schemas、tool definitions、共有 request/response types を提供します。

### 責務

- QuizFold 用 MCP tools を定義する
- request schemas を定義する
- response schemas を定義する
- clients、hosted service、tests の間で types を共有する
- tools の期待される挙動を記述する

### 非責務

`@quizfold/mcp` は以下をしてはいけません。

- hosted service tokens を verify する
- D1/R2 に access する
- billing plans を enforce する
- MCP write usage を count する
- notes を store する
- images を upload する

これらは `quizfold-cloud` に属します。

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

### 重要な区別

`qf-attachment:image1` references は `@quizfold/parser` によって parsed されますが、`datas.image1` が存在するかどうかは parser の関心ではありません。

その check は `quizfold-cloud` の request/context validation に属します。

---

## 11. `packages/cli`

`packages/cli` は以下として publish されます。

```txt
@quizfold/cli
```

これは Rust CLI binary の npm distribution wrapper です。

CLI 自体は `crates/cli` の Rust で書かれています。

### Platform packages

npm CLI は platform-specific packages を使うべきです。

```txt
@quizfold/cli
@quizfold/cli-aarch64-apple-darwin
@quizfold/cli-x86_64-apple-darwin
@quizfold/cli-aarch64-unknown-linux-gnu
@quizfold/cli-x86_64-unknown-linux-gnu
@quizfold/cli-aarch64-pc-windows-msvc
@quizfold/cli-x86_64-pc-windows-msvc
```

32-bit と legacy PC support は意図的に scope 外です。

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

または:

```bash
npx @quizfold/cli --help
```

---

## 12. QuizFold Markdown v0.1

初期の QuizFold Markdown syntax は小さく strict であるべきです。

### Q/A block

```md
? 江戸幕府はいつ、誰によって開かれた？
---
${1603年} に ${徳川家康} によって開かれた。
```

意味:

```txt
question = 江戸幕府はいつ、誰によって開かれた？
answer   = ${1603年} に ${徳川家康} によって開かれた。
```

### Fold line

```md
! 江戸幕府は ${1603年} に ${徳川家康} によって開かれた。
```

意味:

```txt
行そのものが prompt になる。
`${...}` spans は hidden answers になる。
```

Fold line は少なくとも 1 つの `${...}` を含まなければなりません。

含まない場合は error です。

### Inline answer

```md
${徳川家康}
```

Inline answers は Q/A answers と Fold lines の中で使われます。

### Memo block

```md
@memo
この補足情報は編集画面やMCPでは利用できるが、クイズモードには表示しない。

$$
E = mc^2
$$
@end
```

Memo blockは、行全体が`@memo`の行で開始し、行全体が`@end`の行で終了します。

Memo内には、paragraph、math、code、Mermaid、imageを含む通常のQuizFold blockを記述できます。Memoのネストは禁止します。閉じられていないmemo、対応する`@memo`がない`@end`、memo内の`@memo`はerrorです。

Memo blockはdocument levelとQ/Aのquestion・answer content内に配置できます。Editor、CLI、MCP、AI contextのためにASTへ保持しますが、quiz modeでは必ず除外します。

### Math block

````md
```math
E = \frac{1}{2}mv^2
```
````

Math blocks は LaTeX-like input を使います。

Rendering は web layer の KaTeX で扱ってもよいです。

rendered math が必要な flow で math block を render できない場合は、error として扱うべきです。

### Image reference

```md
![細胞の模式図](qf-stored:img_123)
```

Image alt text は必須です。

空の alt text は error です。

### Temporary data reference

```md
![細胞の模式図](qf-attachment:image1)
```

`qf-attachment:image1` は MCP/API/CLI request で渡される attachment を参照します。

`qf-stored:img_123` は、手動upload済み画像や他のQuizFold contentから再利用する画像など、すでに保存された画像を参照します。

外部画像は通常のMarkdown HTTP(S) URLを利用できます。

```md
![細胞の模式図](https://example.com/cell.png)
```

parserは外部参照として認識するだけで、URLを取得してはいけません。

Parser responsibility:

```txt
`qf-attachment:image1` と `qf-stored:img_123` を references として検出する。
`http://` と `https://` の画像URLをexternal referencesとして検出する。
外部URLを取得しない。
```

Cloud/request validation responsibility:

```txt
datas.image1 が存在し、有効で、user's plan で許可されているかを確認する。
外部URLを取得する場合は、SSRF、redirect、MIME type、sizeをvalidateする。
```

---

## 13. Diagnostics Policy

QuizFold は 3 つの diagnostic levels を使います。

```txt
fatal
error
warning
```

### Fatal

Fatal は safe recovery ができないことを意味します。

Examples:

- 安全に処理するには input が大きすぎる
- internal parser recovery が失敗した
- source location tracking が失敗した
- resource limits を超えた

Fatal diagnostics は Web と MCP/CLI operations の両方を reject します。

### Error

Error は content が有効な QuizFold content ではないことを意味します。

Examples:

- Q/A block に `---` がない
- answer が空
- `${...}` が閉じていない
- Fold line に `${...}` がない
- Image alt text が空
- 必要な箇所で math block を parse/render できない
- request validation で unsupported image type がある

MCP/CLI は error で reject しなければなりません。

Web editor は content を draft として保持し、error を inline で表示してもよいです。

### Warning

Warning は content が正常に accepted されたが、user または AI に何かを表示するべき状態を意味します。

Examples:

- Note title が自動生成された
- Image が自動圧縮された
- `qf-attachment:image1` がuploadされ、`qf-stored:img_123` に解決された
- Answer が unusually long

Warnings は MCP/CLI commit をブロックしません。

---

## 14. Mode-specific Behavior

```txt
              fatal    error    warning
Web           reject   allow*   allow
MCP/CLI       reject   reject   allow
Preview       show     show     show
```

`allow*` は、Web が draft を保持し、invalid portions を無効化または mark してよいという意味です。error-level content を valid QuizFold content として登録してはいけません。

### Web

Web だけが error-level content を editable な状態で保持できます。

Web が error diagnostics を受け取った場合、inline で表示し、該当箇所を有効な QuizFold content として扱わずに無効化するべきです。

### MCP/CLI

MCP と CLI は commit paths です。

operation に fatal または error diagnostic が 1 つでも含まれる場合、必ず reject しなければなりません。

successful commit で許可されるのは warnings のみです。

---

## 15. Parser vs Context Validation

この区別は非常に重要です。

### Parser / structural validation

parser は Markdown だけから見えるものを validate できます。

Examples:

- `?` block structure
- `---` placement
- Fold line syntax
- `${...}` syntax
- math block boundaries
- image alt presence
- source ranges

### Context validation

Context validation には外部の request data または user/service state が必要です。

これは parsing とは別ですが、MCP、CLI、API、Web commit flows は operation を受理する前に必ず実行しなければなりません。

Examples:

- `datas.image1` は存在するか
- `datas.image1` は valid base64 か
- image MIME type は許可されているか
- image size は許可されているか
- この user は `qf-stored:img_123` を利用できるか
- この user は images を upload できるか
- この user は MCP/CLI write limits を超過しているか

Context validation は `crates/parser` ではなく、`quizfold-cloud`、hosted MCP/API implementations、または CLI request handling に属します。

たとえば parser は Markdown が `qf-attachment:image1` を参照していることを報告できますが、`datas.image1` が実際に upload されたかどうかを判断してはいけません。その request-level fact は MCP/CLI/API validation が確認します。同様に、`qf-stored:img_123` を利用できるかどうかも parser の外側で確認します。

---

## 16. Public npm packages

OSS リポジトリは最初にちょうど 3 つの npm packages を publish するべきです。

```txt
@quizfold/parser
@quizfold/mcp
@quizfold/cli
```

強い必要が出るまでは、追加 packages を増やすべきではありません。

これにより public API を小さく理解しやすい状態に保ちます。

---

## 17. `quizfold-cloud` との関係

`quizfold-cloud` は closed source で、`quizfold` を consume します。

以下を使ってもよいです。

```txt
@quizfold/parser
@quizfold/mcp
```

通常、以下を使う必要はありません。

```txt
@quizfold/cli
```

`quizfold-cloud` は以下を所有します。

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

parser は MCP と CLI を大きく promote する前に stable にするべきです。

---

## 19. Design Principle

QuizFold OSS は以下であるべきです。

```txt
Small core.
Strict syntax.
Recoverable diagnostics.
One parser everywhere.
Human-readable Markdown.
AI-writable Markdown.
Cloud-independent OSS.
```

hosted service は convenience、persistence、storage、integration を追加するべきですが、QuizFold core language と tools は open なままにするべきです。
