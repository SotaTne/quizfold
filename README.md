# QuizFold

QuizFold is the open-source core for QuizFold Markdown, parser, CLI, MCP schemas, and public packages.

```txt
quizfold        = OSS core
quizfold-cloud  = hosted service / SaaS implementation
```

The dependency direction must remain one-way:

```txt
quizfold-cloud -> quizfold
quizfold       -> no dependency on quizfold-cloud
```
