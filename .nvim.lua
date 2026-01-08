local overseer = require('overseer')

overseer.register_template({
  name = 'cargo build',
  builder = function()
    return {
      name = 'cargo build',
      cmd = {
        'cargo',
        'build',
      },
      components = {
        'default',
        'unique',
      },
    }
  end,
})

overseer.register_template({
  name = 'test',
  builder = function()
    return {
      name = 'test',
      cmd = {
        'mpv',
        '--mute',
        'test.mkv'
      },
      components = {
        'default',
        'unique',
        { 'restart_on_save', paths = { 'src', 'xmake.lua', }, },
        { 'dependencies', task_names = { 'cargo build', }, },
      },
    }
  end,
})