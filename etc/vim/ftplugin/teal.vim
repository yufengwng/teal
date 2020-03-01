" vim: set et sw=2 sts=2 ts=2:

if exists("b:did_ftplugin")
  finish
endif
let b:did_ftplugin = 1

let s:save_cpo = &cpo
set cpo&vim

setlocal comments=:#\|,:#/,:#
setlocal commentstring=#%s
setlocal formatoptions-=t formatoptions+=croqnl
silent! setlocal formatoptions+=jp  " only in 7.3+ (?)

setlocal suffixesadd=.tl

setlocal smartindent nocindent
setlocal tabstop=2 shiftwidth=2 softtabstop=2 expandtab
setlocal textwidth=100

let b:undo_ftplugin = "
  \ setlocal comments< commentstring< formatoptions< suffixesadd<
  \|setlocal smartindent< nocindent<
  \|setlocal tabstop< shiftwidth< softtabstop< expandtab<
  \|setlocal textwidth<
  \"

let &cpo = s:save_cpo
unlet s:save_cpo
