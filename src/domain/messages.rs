// https://dev.to/alexeagleson/how-to-set-up-a-fullstack-rust-project-with-axum-react-vite-and-shared-types-429e?

use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct InitMap {
    kind: i32,
    xy: (i32, i32),
    id: String,
}

impl InitMap {
    pub fn default(id: String) -> InitMap {
        InitMap {
            kind: 9,
            xy: (100, 150),
            id: id,
        }
    }
}
