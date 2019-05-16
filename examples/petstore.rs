use actix_web::{
    web::{self, Form, Json, Path, Query},
    App, HttpRequest, HttpServer, Result,
};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// A pet for sale in the pet store
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pet {
    id: Option<i64>,
    category: Option<Category>,
    name: String,
    photo_urls: Vec<String>,
    tags: Option<Vec<Tag>>,
    status: Option<Status>,
}

/// Pet status in the store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Available,
    Pending,
    Sold,
}

/// A category for a pet
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Category {
    id: Option<i64>,
    name: Option<String>,
}

/// A tag for a pet
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tag {
    id: Option<i64>,
    name: Option<String>,
}

/// An uploaded response
///
/// Describes the result of uploading an image resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    code: Option<i32>,
    _type: Option<String>,
    message: Option<String>,
}

/// Pet Order
///
/// An order for a pets from the pet store
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    id: Option<i64>,
    pet_id: Option<i64>,
    quantity: Option<i32>,
    ship_date: Option<String>,
    status: Option<OrderStatus>,
    #[serde(default)]
    complete: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum OrderStatus {
    Placed,
    Approved,
    Delivered,
}

/// a User
///
/// A User who is purchasing from the pet store
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: Option<i64>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    phone: Option<String>,
    /// User Status
    user_status: Option<i32>,
}

/// List of user object
pub type UserArray = Vec<User>;

/// Add a new pet to the store
fn add_pet(_pet: Json<Pet>) {
    unimplemented!();
}

/// Update an existing pet
fn update_pet(_pet: Json<Pet>) {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
struct FindPetsByStatusQuery {
    status: Vec<Status>,
}

/// Finds Pets by status
///
/// Multiple status values can be provided with comma separated strings
fn find_pets_by_status(_query: Query<FindPetsByStatusQuery>) -> Result<Json<Vec<Pet>>> {
    unimplemented!();
}

/// Finds Pets by tags
///
/// Multiple tags can be provided with comma separated strings. Use tag1, tag2, tag3 for testing.
fn find_pets_by_tags(_query: Query<FindPetsByStatusQuery>) -> Result<Json<Vec<Pet>>> {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PetIdPath {
    pet_id: i64,
}

/// Find pet by ID
///
/// Returns a single pet
fn get_pet_by_id(_path: Path<PetIdPath>) -> Result<Json<Pet>> {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
struct UpdatePetByIdForm {
    /// Updated name of the pet
    name: Option<String>,
    /// Updated status of the pet
    status: Option<Status>,
}

/// Updates a pet in the store with form data
fn update_pet_with_form((_path, _form): (Path<PetIdPath>, Form<UpdatePetByIdForm>)) {
    unimplemented!();
}

/// Deletes a pet
fn delete_pet(_path: Path<PetIdPath>) {
    // TODO: extract header
    unimplemented!();
}

/// Uploads an image
fn upload_file(_path: Path<PetIdPath>) -> Result<Json<ApiResponse>> {
    // TODO: Describe multiform data.
    unimplemented!();
}

/// Returns pet inventories by status
///
/// Returns a map of status codes to quantities
fn get_inventory(_req: HttpRequest) -> Result<Json<HashMap<String, i32>>> {
    unimplemented!();
}

/// Place an order for a pet
fn place_order(_order: Json<Order>) -> Result<Json<Order>> {
    unimplemented!();
}

/// Find purchase order by ID
///
/// For valid response try integer IDs with value <= 5 or > 10. Other values
/// will generated exceptions
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OrderIdPath {
    // TODO: Add validation 1 <= x <= 5
    order_id: i64,
}

/// Find purchase order by ID
fn get_order_by_id(_path: Path<OrderIdPath>) -> Result<Json<Order>> {
    unimplemented!();
}

/// Delete purchase order by ID
fn delete_order(_path: Path<OrderIdPath>) {
    unimplemented!();
}

/// Create user
///
/// This can only be done by the logged in user.
fn create_user(_user: Json<User>) {
    unimplemented!();
}

/// Creates list of users with given input array
fn create_users_with_array_input(_user: Json<UserArray>) {
    unimplemented!();
}

/// Creates list of users with given input array
fn create_users_with_list_input(_user: Json<UserArray>) {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
struct LoginUserQuery {
    /// The user name for login
    username: String,
    /// The password for login in clear text
    password: String,
}

/// Logs user into the system
fn login_user(_query: Query<LoginUserQuery>) -> Result<String> {
    // TODO: can we specify headers in Response
    unimplemented!();
}

/// Logs out current logged in user session
fn logout_user(_query: Query<LoginUserQuery>) {
    unimplemented!();
}

#[derive(Debug, Deserialize)]
struct UsernamePath {
    /// The name that needs to be fetched. Use user1 for testing.
    username: String,
}

/// Get user by user name
fn get_user_by_name(_path: Path<UsernamePath>) -> Json<User> {
    unimplemented!();
}

/// Update user
///
/// This can only be done by the logged in user.
fn update_user(_path: Path<UsernamePath>) -> Json<User> {
    unimplemented!();
}

/// Delete user
///
/// This can only be done by the logged in user.
fn delete_user(_path: Path<UsernamePath>) {
    unimplemented!();
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=debug");
    env_logger::init();

    let app_factory = |openapi_spec: String| {
        App::new()
            .service(
                web::resource("/pet")
                    .route(web::post().to(add_pet))
                    .route(web::put().to(update_pet)),
            )
            .service(
                web::resource("/pet/findByStatus")
                    .route(web::get().to(find_pets_by_status)),
            )
            .service(
                web::resource("/pet/findByTags").route(web::get().to(find_pets_by_tags)),
            )
            .service(
                web::resource("/pet/{petId}")
                    .route(web::get().to(get_pet_by_id))
                    .route(web::post().to(update_pet_with_form))
                    .route(web::delete().to(delete_pet)),
            )
            .service(
                web::resource("/pet/{petId}/uploadImage")
                    .route(web::post().to(upload_file)),
            )
            .service(
                web::resource("/store/inventory").route(web::get().to(get_inventory)),
            )
            .service(web::resource("/store/order").route(web::post().to(place_order)))
            .service(
                web::resource("/store/order/{orderId}")
                    .route(web::get().to(get_order_by_id))
                    .route(web::delete().to(delete_order)),
            )
            .service(web::resource("/user").route(web::post().to(create_user)))
            .service(
                web::resource("/user/createWithArray")
                    .route(web::post().to(create_users_with_array_input)),
            )
            .service(
                web::resource("/user/createWithList")
                    .route(web::post().to(create_users_with_list_input)),
            )
            .service(web::resource("/user/login").route(web::get().to(login_user)))
            .service(web::resource("/user/logout").route(web::get().to(logout_user)))
            .service(
                web::resource("/user/{username}")
                    .route(web::get().to(get_user_by_name))
                    .route(web::put().to(update_user))
                    .route(web::delete().to(delete_user)),
            )
            .service(
                web::resource("/openapi")
                    .route(web::get().to(move || openapi_spec.clone())),
            )
    };

    let openapi_spec = {
        #[cfg(feature = "openapi-spec")]
        {
            let app = app_factory(String::new());
            let spec: openapi::v3_0::Spec =
                app.openapi_spec("Extended Petstore API v1", "1.0.0");
            serde_json::to_string_pretty(&spec)?
        }
        #[cfg(not(feature = "openapi-spec"))]
        {
            "no open API spec defined".to_string()
        }
    };

    HttpServer::new(move || app_factory(openapi_spec.clone()))
        .bind("127.0.0.1:8080")?
        .run()
}
