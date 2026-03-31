use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::result;
// use time::OffsetDateTime;
use uuid::Uuid;

// use learning_sqlx_rs::Organization;
// use sqlx::mysql::MySqlPoolOptions;
// etc.
#[derive(sqlx::FromRow)]
#[allow(unused)]
pub struct User {
    id: Uuid,
    // org_id: Uuid,
    name: String,
    // username: String,
    // password_hash: String,
    // contact_info: serde_json::Value,
    // address: serde_json::Value,
    // status: String,
}
// }
// #[derive(sqlx::FromRow)]
pub struct OrganizationPRES {
    id: Uuid,
    name: String,
    //     description: Option<String>,
    //     created_at: OffsetDateTime,
    //     updated_at: OffsetDateTime,
}

#[tokio::main] // Requires the `attributes` feature of `async-std`
// or #[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = "postgres://postgres:default@localhost/sqlx_lrn";

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    println!("---------- Users: ");
    select_users(&db).await?;

    println!("---------- DDD style Organization: ");
    select_ddd_users(&db).await?;

    // println!("---------- Users join Organizations: ");
    // select_users_join_organizations(&db).await?;

    Ok(())
}

async fn select_users(db: &sqlx::Pool<sqlx::Postgres>) -> result::Result<(), sqlx::Error> {
    let query = "SELECT id, name FROM public.members";
    let users = sqlx::query(query).fetch_all(db).await?;

    // println!("Users: {:?}", users);

    for user in users {
        println!(
            "id: {}, name: {}",
            user.get::<Uuid, _>("id"),
            user.get::<String, _>("name")
        );
    }
    Ok(())
}

async fn select_ddd_users(db: &PgPool) -> result::Result<(), sqlx::Error> {
    // let query = "SELECT id, name FROM public.user";
    let orgs: Vec<OrganizationPRES> = sqlx::query_as!(
        OrganizationPRES,
        "SELECT id, name FROM public.organizations"
    )
    .fetch_all(db)
    .await?;

    // println!("Organization: {:?}", users);

    for org in orgs {
        println!("id: {}, name: {}", org.id, org.name);
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
