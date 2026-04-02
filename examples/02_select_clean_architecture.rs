use learning_sqlx_rs::Organization;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::result;
use time::OffsetDateTime;
use uuid::Uuid;

#[tokio::main] // Requires the `attributes` feature of `async-std`
// or #[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = "postgres://postgres:default@localhost/sqlx_lrn";

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    find_user_by_id(&db).await?;
    println!("---------- Users: ");
    select_users(&db).await?;

    println!("---------- DDD style OrganizationResponse: ");
    select_organization_response(&db).await?;

    println!("---------- DDD style Organization Entity: ");
    select_organization_entity(&db).await?;

    // println!("---------- Users join Organizations: ");
    // select_users_join_organizations(&db).await?;

    Ok(())
}

/// Fetches every row from `public.members` using a raw query and prints
/// each member's `id` and `name`.
async fn select_users(db: &sqlx::Pool<sqlx::Postgres>) -> result::Result<(), sqlx::Error> {
    let id = Uuid::parse_str("9b29622c-add1-42e5-b5e2-b6f9246939c5").unwrap();
    let query = "SELECT id, name FROM public.members";
    let users = sqlx::query(query).fetch_all(db).await?;

    for user in users {
        println!(
            "id: {}, name: {}",
            user.get::<Uuid, _>("id"),
            user.get::<String, _>("name")
        );
    }
    Ok(())
}

async fn find_user_by_id(db: &sqlx::Pool<sqlx::Postgres>) -> result::Result<(), sqlx::Error> {
    let id = Uuid::parse_str("9b29622c-add1-42e5-b5e2-b6f9246939c5").unwrap();
    // let query = "SELECT id, name FROM public.members where id = $1";
    let users = sqlx::query!("SELECT id, name FROM public.members where id = $1", id)
        .fetch_all(db)
        .await?;

    for user in users {
        println!("id: {}, name: {}", user.id, user.name);
    }
    Ok(())
}

/// Read-model / DTO for an organization row.
///
/// All fields are public so [`sqlx::query_as!`] can populate them directly.
/// Use this instead of the domain [`Organization`] aggregate when you only
/// need to display data and do not need domain behaviour.
#[derive(Debug, Clone)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

/// Fetches all organizations into [`OrganizationResponse`] DTOs using
/// [`sqlx::query_as!`] (requires public struct fields).
async fn select_organization_response(db: &PgPool) -> result::Result<(), sqlx::Error> {
    let orgs: Vec<OrganizationResponse> = sqlx::query_as!(
        OrganizationResponse,
        "SELECT id, name, description, created_at, updated_at FROM public.organizations"
    )
    .fetch_all(db)
    .await?;

    for org in orgs {
        println!("id: {}, name: {}", org.id, org.name);
    }

    Ok(())
}

/// Fetches all organizations and maps each row into the domain [`Organization`]
/// aggregate via [`Organization::from_persistence`].
///
/// This is the DDD-style approach: the domain type keeps its fields private
/// and enforces invariants, so `query!` (not `query_as!`) is used and rows
/// are reconstructed through the factory method.
#[allow(non_snake_case)]
async fn select_organization_entity(db: &PgPool) -> result::Result<(), sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id, name, description, created_at, updated_at FROM public.organizations"
    )
    .fetch_all(db)
    .await?;

    let orgs: Vec<Organization> = rows
        .into_iter()
        .map(|r| {
            Organization::from_persistence(r.id, r.name, r.description, r.created_at, r.updated_at)
        })
        .collect();

    for org in orgs {
        println!("id: {}, name: {}", org.id(), org.name());
    }

    Ok(())
}

// async fn select_users_join_organizations(db: &PgPool) -> result::Result<(), sqlx::Error> {
//     // let query = "SELECT id, name FROM public.user";, public.organization.name
//     let users: Vec<UserAndOrgDto> = sqlx::query_as!(
//         UserAndOrgDto,
//         "SELECT public.user.id, public.user.name, public.organization.name as org_name
//         FROM public.user
//         JOIN public.organization
//             ON public.user.org_id = public.organization.id
//         WHERE public.organization.name = 'Acme Corp'"
//     )
//     .fetch_all(db)
//     .await?;

//     // println!("Users: {:?}", users);

//     for user in users {
//         println!(
//             "id: {}, name: {}, org_name: {}",
//             user.id, user.name, user.org_name
//         );
//     }

//     Ok(())
// }
