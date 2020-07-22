Statics
=====================

Development of statics is carried out in the `./frontend` folder. Each folder exists for a separate logical element of the application.

Each element includes:
- dir `styles` - for create css|less styles
- dir `js` - for creating js code. code building to ***boundle file***
- dir `templates` - for create html (using **pug**)

All statics are rendered to the `./pub` folder while maintaining the element structure

Getting starting
---------------------

1) Install `Node.js` stable version
2) Run command:

```sh
$ npm install
```

After downloading all the dependencies, you can build the project

Development
----------------------

Development is carried out `only in the ./frontend folder`

For build development version all statics:

```sh
$ npm run dev
```

For build development version js code:

```sh
$ npm run dev-code
```

For build development version styles and html:

```sh
$ npm run dev-styles
```

For build production version for all statics:

```sh
$ npm run prod
```