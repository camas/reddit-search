require('./main.css')
import('./pkg').then(module => {
  module.run_app()
})
