" vim: set et sw=2 sts=2 ts=2:

if exists("b:did_ftplugin")
  finish
endif
let b:did_ftplugin = 1

let s:save_cpo = &cpo
set cpo&vim

setlocal comments=:#/\|,:#//,:#
setlocal commentstring=#\ %s
setlocal formatoptions-=t formatoptions+=croqnl
silent! setlocal formatoptions+=jp  " only in 7.3+ (?)

setlocal include=^\\s*use
setlocal suffixesadd=.tl

setlocal indentexpr=GetTealIndent()
setlocal indentkeys-=0# indentkeys+==end,=elif,=else
if !exists("*GetTealIndent")
  function GetTealIndent()
    let lnum = prevnonblank(v:lnum - 1)
    let ind = indent(lnum)

    let prev_text = getline(lnum)
    if prev_text =~ '^\s*\(if\|elif\|else\|for\|loop\)'
      let ind = ind + shiftwidth()
    endif

    let cur_text = getline(v:lnum)
    if cur_text =~ '^\s*\(end\|elif\|else\)'
      let ind = ind - shiftwidth()
    endif

    return ind
  endfunction
endif

if exists("loaded_matchit")
  let b:match_ignorecase = 0
  let b:match_words =
    \ '{\|\<\%(if\|match\|for\|loop\|=\@<!begin\)\>=\@!' .
    \ ':' .
    \ '\<\%(elif\|else\)\>' .
    \ ':' .
    \ '}\|\%(^\|[^.]\)\@<=\<end\:\@!\>'
  let b:match_skip =
    \ "synIDattr(synID(line('.'),col('.'),0),'name') =~ '" .
    \ "\\<teal\\%(Str\\|CommentLine\\|CommentDoc\\|CommentInnerDoc" .
    \ "\\)\\>'"
endif

setlocal tabstop=2 shiftwidth=2 softtabstop=2 expandtab
setlocal textwidth=90

let b:undo_ftplugin = "
  \ setlocal comments< commentstring< formatoptions<
  \|setlocal include< suffixesadd<
  \|setlocal indentexpr< indentkeys<
  \|setlocal tabstop< shiftwidth< softtabstop< expandtab<
  \|setlocal textwidth<
  \|unlet! b:match_ignorecase b:match_words b:match_skip
  \"

let &cpo = s:save_cpo
unlet s:save_cpo
