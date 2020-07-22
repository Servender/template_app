const gulp = require('gulp')
const plumber = require('gulp-plumber')
const pug = require('gulp-pug')
const less = require('gulp-less')
const autoprefixer = require('gulp-autoprefixer')
const styleLint = require('gulp-stylelint')
const path = require('path')

const apps_path = path.resolve('./frontend')
const output_path = path.resolve('./pub')

gulp.task('templates', () => {
    return gulp.src(`${apps_path}/**/templates/[^_]*.pug`)
        .pipe(plumber())
        .pipe(pug({
            pretty: true, //minimizer
            data: {
                styles: [],
                scripts: []
            }
        }))
        .pipe(gulp.dest(output_path))
})

gulp.task('styles', () => {
    return gulp.src(`${apps_path}/**/styles/**/[^_]*.less`)
        .pipe(plumber())
        .pipe(less())
        .pipe(autoprefixer())
        .pipe(gulp.dest(output_path))
})

gulp.task('stylesLint', () => {
    return gulp.src(`${apps_path}/**/styles/**/*.less`)
        .pipe(styleLint({
            configFile: '.stylelintrc',
            failAfterError: false,
            debug: true,
            syntax: 'less',
            reporters: [
                { formatter: 'string', console: true }
            ]
        }))
})

exports.default = gulp.series('templates', 'stylesLint', 'styles')
