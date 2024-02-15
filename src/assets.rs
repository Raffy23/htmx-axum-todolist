pub static HTMX_JS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/node_modules",
    "/htmx.org",
    "/dist/htmx.min.js"
));

pub static PICO_CSS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/node_modules",
    "/@picocss/pico",
    "/css/pico.min.css"
));

pub static APP_CSS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates",
    "/_layout.style.css",
));
