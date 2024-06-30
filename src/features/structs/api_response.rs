use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T, E> {
    pub code: String,
    pub message: String,
    pub args: Option<E>,
    pub data: Option<T>
}
