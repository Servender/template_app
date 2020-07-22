-- Your SQL goes here

-- ROUTES --
CREATE TABLE routes (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    route TEXT NOT NULL DEFAULT '/',
    description TEXT,
    publication SMALLINT NOT NULL DEFAULT 1
);

COMMENT ON COLUMN routes.publication IS 'active routes for this element';
COMMENT ON COLUMN routes.name IS 'name route';
COMMENT ON COLUMN routes.route IS 'global path for connect to item';
COMMENT ON COLUMN routes.description IS 'description route';

INSERT INTO routes(publication, name, route, description) VALUES
    (1, 'admin', '/admin', 'active routes for admin page'),
    (1, 'app', '/app', 'active routes for main application (recomended)'),
    (0, 'frame', '/frame', 'active routes for render frame'),
    (1, 'error', '/_error', 'active routes to error pages (recomended)');

-- PAGES --
CREATE TABLE pages (
    id SERIAL PRIMARY KEY,
    route_name TEXT NOT NULL, -- routes.name
    page_name TEXT NOT NULL,
    description TEXT,
    path TEXT NOT NULL
);

COMMENT ON COLUMN pages.route_name IS 'the name of the routing for which the page is taken';
COMMENT ON COLUMN pages.page_name IS 'page alias';
COMMENT ON COLUMN pages.description IS 'description page';
COMMENT ON COLUMN pages.path IS 'path in `./pub/${route_name}/templates/`';

INSERT INTO pages(route_name, page_name, path) VALUES
    ('app', 'main', 'index.html'),
    ('error', '400', '400.html'),
    ('error', '404', '404.html'),
    ('error', '500', '500.html');

CREATE TABLE statics (
    id SERIAL PRIMARY KEY,
    page_id INTEGER NOT NULL DEFAULT 0,
    name TEXT,
    type_file TEXT NOT NULL, -- scripts, styles, images
    status SMALLINT NOT NULL DEFAULT 1,
    mask TEXT NOT NULL,
    FOREIGN KEY (page_id) REFERENCES pages(id) ON DELETE SET DEFAULT
);

COMMENT ON COLUMN statics.page_id IS 'identify page (page.id)';
COMMENT ON COLUMN statics.name IS 'name static files (alias)';
COMMENT ON COLUMN statics.type_file IS 'type files (scripts, styles, images)';
COMMENT ON COLUMN statics.mask IS 'mask for searching files';

INSERT INTO statics(page_id, status, type_file, name, mask) VALUES
    (
        (SELECT id FROM pages WHERE (pages.route_name='app' AND pages.page_name='main')),
        1,
        'scripts',
        'main',
        '/*.js'
    );
