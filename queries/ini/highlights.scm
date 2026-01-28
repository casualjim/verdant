;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/ini/highlights.scm
;; Licensed under the Apache License 2.0
(section_name
  (text) @markup.heading
)

(comment) @comment @spell

[
  "["
  "]"
] @punctuation.bracket

"=" @operator

(setting
  (setting_name) @property
)

(setting_value) @string
