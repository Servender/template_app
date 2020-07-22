const yargs = require('yargs').argv
const fs = require('fs')
const path = require('path')
const webpack = require('webpack')
const glob = require('glob')
const Dotenv = require('dotenv-webpack')
const bodyParser = require('body-parser')
const TerserPlugin = require('terser-webpack-plugin')

const apps_path = path.resolve('./frontend')
const output_path = path.resolve('./pub')

const getAllApps = () => glob.sync('@(*|!chunks)', { cwd: apps_path })

const mode = process.env.NODE_ENV = yargs.mode || 'development'
const apps = yargs.apps == 'all' || !yargs.apps ? getAllApps() : yargs.apps.split(',')
const hot = yargs['$0'].includes('webpack-dev-server')

apps.forEach(app => {
    if (!fs.existsSync(`${apps_path}/${app}`)) {
        throw new Error(`No such app as ${app}`)
    }
})

if (hot && apps.length != 1) {
    throw new Error("App name should be provided with hot reload feature using --app argument like: --apps=testApp")
}

const entry = Object.assign(
    {},
    ...apps.map(app => fs.existsSync(`${apps_path}/${app}/js/index.js`) && { [app]: `${apps_path}/${app}/js/index.js` })
)

const bundle_file = 'bundle.js'

const output = {
    path: output_path,
    filename: `[name]/js/${bundle_file}`,
	publicPath: '/',
    chunkFilename: 'chunks/[id].[chunkhash].js'
}

const plugins = [
    new Dotenv()
]

const optimization = mode !== 'production' ? {} : {
    minimizer: [
        new TerserPlugin({
            cache: true,
            parallel: true
        })
    ],
}

module.exports = {
    mode,
    entry,
    output,
    plugins,
    optimization,
    devServer: {
        contentBase: './dist',
        hot: true,
        overlay: true,
        disableHostCheck: true,
        historyApiFallback: true,
        before(app) {
            const mock = fs.existsSync(`${apps_path}/${apps[0]}/mock.js`) && require(`${apps_path}/${apps[0]}/mock.js`)

            mock && Object.entries(mock).map(
                ([method, paths]) => Object.entries(paths).map(([path, mock]) =>
                    app[method](path, bodyParser.json(), (req, res) => setTimeout(() => res.json(mock(req)), 10 /* artificial delay */))
                )
            )
        }
    }
}