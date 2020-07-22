module.exports = function(api) {
    api.cache(true);

    return {
        plugins: [
            "@babel/plugin-syntax-dynamic-import",
            "@babel/plugin-proposal-optional-chaining"
        ],
        presets: [
            [
                "@babel/preset-env",
                {
                    modules: 'commonjs'
                }
            ],
        ]
    }
}