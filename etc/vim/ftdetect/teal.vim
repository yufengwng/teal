" vim: set et sw=2 sts=2 ts=2:

function! s:TrimTrailingWhitespace()
  let l:num = line(".")
  let l:col = col(".")
  %s/\s\+$//e
  call cursor(l:num, l:col)
endfunction

autocmd BufRead,BufNewFile *.tl set filetype=teal
autocmd BufWritePre *.tl :call <SID>TrimTrailingWhitespace()
