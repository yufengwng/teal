" vim: set et sw=2 sts=2 ts=2:

if exists("b:did_ftplugin")
  finish
endif
let b:did_ftplugin = 1

let s:save_cpo = &cpo
set cpo&vim

setlocal comments=:#!,://!,:///,://
setlocal commentstring=//\ %s
setlocal formatoptions-=t formatoptions+=croqnl
silent! setlocal formatoptions+=jp  " only in 7.3+ (?)

setlocal include=^\\s*use
setlocal suffixesadd=.tl

setlocal indentexpr=GetTealIndent()
setlocal indentkeys-=0# indentkeys+==end,=elif,=else
if !exists("*GetTealIndent")
  function GetTealPrevIndentWithPatt(curr_num, pattern)
    let l:curr_ind = indent(a:curr_num)
    let l:ind = l:curr_ind
    let l:num = a:curr_num - 1
    while l:num > 0
      let l:prev = getline(l:num)
      let l:prev_ind = indent(l:num)
      if (l:prev =~ a:pattern) && (l:prev_ind < l:curr_ind)
        let l:ind = l:prev_ind
        break
      endif
      let l:num = l:num - 1
    endwhile
    return l:ind
  endfunction

  function IsNotTealCommentLine(line)
    return a:line !~ '.*//.*' &&
          \a:line !~ '.*///.*' &&
          \a:line !~ '.*//!.*' &&
          \a:line !~ '.*#!.*'
  endfunction

  function GetTealIndent()
    let l:num = prevnonblank(v:lnum - 1)
    let l:ind = indent(l:num)

    let l:prev = getline(l:num)
    if l:prev =~ '\(:\|(\|[\|{\)\s*$' && IsNotTealCommentLine(l:prev)
      if l:prev !~ '\s*match\s*'
        let l:ind = l:ind + shiftwidth()
      endif
    elseif l:prev =~ '\(=>\).*$' && IsNotTealCommentLine(l:prev)
      let l:ind = l:ind + shiftwidth()
    endif

    let l:curr = getline(v:lnum)
    if l:curr =~ '^\s*end'
      let l:ind = GetTealPrevIndentWithPatt(v:lnum, '^\s*\(if\|for\|loop\|match\)')
    elseif l:curr =~ '^\s*\(elif\|else\)'
      let l:ind = GetTealPrevIndentWithPatt(v:lnum, '^\s*if')
    elseif l:curr =~ '^\s*)'
      let l:ind = GetTealPrevIndentWithPatt(v:lnum, '(\s*$')
    elseif l:curr =~ '^\s*]'
      let l:ind = GetTealPrevIndentWithPatt(v:lnum, '[\s*$')
    elseif l:curr =~ '^\s*}'
      let l:ind = GetTealPrevIndentWithPatt(v:lnum, '{\s*$')
    endif

    return l:ind
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
    \ "\\<teal\\%(Str\\|CommentLine\\|CommentDoc\\|CommentInnerDoc\\|CommentShebang" .
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
