add_repositories("energostalin https://github.com/EnergoStalin/xmake-repo")

add_requires('vect')

target('stmp')
  set_kind('static')
  set_languages('c11')
  add_files('*.c')
  add_packages('vect')
  add_links('avcodec', 'avformat', 'avutil')
target_end()