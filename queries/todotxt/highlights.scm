;; Forked from https://github.com/nvim-treesitter/nvim-treesitter/blob/master/queries/todotxt/highlights.scm
;; Licensed under the Apache License 2.0
(done_task) @comment

(task
  (priority) @keyword
)

(task
  (date) @comment
)

(task
  (kv) @comment
)

(task
  (project) @string
)

(task
  (context) @type
)
