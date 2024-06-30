use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ApiResponse<T, E> {
    pub code: String,
    pub message: String,
    pub args: Option<E>,
    pub data: Option<T>
}
