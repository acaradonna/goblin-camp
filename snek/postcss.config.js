module.exports = {
  plugins: [
    require('postcss-preset-env')({
      stage: 0,
      features: {
        'custom-properties': false, // Keep CSS custom properties as-is
        'nesting-rules': true,
        'custom-media-queries': true,
        'custom-selectors': true
      },
      autoprefixer: {
        grid: true,
        flexbox: 'no-2009'
      }
    })
  ]
};