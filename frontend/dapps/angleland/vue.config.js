const publicPath = './'
module.exports = {
  publicPath,
  devServer: {
    publicPath,
    open: true,
    port: 8888
  },
  lintOnSave: true,
  chainWebpack: config => {
    config
      .plugin('html')
      .tap(args => {
        args[0].title= 'angleland'
        return args
      })
  }
}
