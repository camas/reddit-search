module.exports = {
  plugins: [
    require('precss'),
    require('tailwindcss'),
    require('autoprefixer'),
    require('cssnano')({
      preset: 'default'
    })
  ]
}
