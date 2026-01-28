;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/scfg/highlights.scm
;; Licensed under the Apache License 2.0
[
  "{"
  "}"
] @punctuation.bracket

(comment) @comment @spell

(directive_name) @type

(directive_params) @variable.parameter
