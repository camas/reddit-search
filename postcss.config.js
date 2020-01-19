module.exports = {
  plugins: [
    require('precss'),
    require('tailwindcss'),
    require('autoprefixer'),
    require('postcss-inherit'),
    require('cssnano')({
      preset: 'default'
    })
  ]
}
