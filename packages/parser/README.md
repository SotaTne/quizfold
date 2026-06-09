# @quizfold/parser

QuizFold parser for browsers, Cloudflare Workers, bundlers, and Node.js.

```ts
import { parseQuizFold, validateQuizFold } from "@quizfold/parser";

const result = await parseQuizFold(source);
const diagnostics = await validateQuizFold(source);
```

The package selects an environment-specific WebAssembly loader through conditional exports while exposing the same asynchronous TypeScript API.
