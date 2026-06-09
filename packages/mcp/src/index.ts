export type QuizFoldImageData = {
  type: "image";
  mimeType: "image/png" | "image/jpeg" | "image/webp";
  base64: string;
  filename?: string;
  alt?: string;
};

export type QuizFoldData = QuizFoldImageData;

export type AppendQuizMarkdownInput = {
  folderPath?: string;
  noteTitle?: string;
  markdown: string;
  datas?: Record<string, QuizFoldData>;
};

export type PreviewQuizMarkdownInput = {
  markdown: string;
  datas?: Record<string, QuizFoldData>;
};
