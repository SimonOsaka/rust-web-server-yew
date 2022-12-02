use rust_web_server_yew::http::request::mock_init;

pub mod auth;
pub mod list;
pub mod menu;
pub mod site;

static PROJECT_DIR: include_dir::Dir = include_dir::include_dir!("$CARGO_MANIFEST_DIR/mock");

/// mock api
pub(crate) fn mock() {
    let mock_contents = PROJECT_DIR
        .files()
        .map(|file| file.contents_utf8().unwrap().to_string())
        .collect::<Vec<String>>();

    mock_init(mock_contents);
}
