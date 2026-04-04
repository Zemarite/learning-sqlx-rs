use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::result;
use time::OffsetDateTime;
use uuid::Uuid;

use learning_sqlx_rs::Organization;
use learning_sqlx_rs::application::{GetMemberByIdQuery, get_members, handle_get_member_by_id, GetMembersPaginatedQuery, handle_get_members_paginated, GetOrganizationsPaginatedQuery, handle_get_organizations_paginated};
use learning_sqlx_rs::domain::errors::DomainError;
use learning_sqlx_rs::domain::value_object::Pagination;
use learning_sqlx_rs::infrastructure::{PostgresMemberRepository, PostgresOrganizationRepository};

#[tokio::main] // Requires the `attributes` feature of `async-std`
// or #[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = "postgres://postgres:default@localhost/sqlx_lrn";

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    //get_member_by_id(&db).await?;

    select_users(&db).await?;

    println!("---------- DDD style OrganizationResponse: ");
    select_organization_response(&db).await?;

    println!("---------- DDD style Organization Entity: ");
    select_organization_entity(&db).await?;

    // println!("---------- Users join Organizations: ");
    // select_users_join_organizations(&db).await?;

    Ok(())
}

/// Fetches paginated rows from `public.members` using the DDD approach.
async fn select_users(db: &sqlx::Pool<sqlx::Postgres>) -> result::Result<(), sqlx::Error> {
    let repo = PostgresMemberRepository::new(db.clone());
    let pagination = Pagination::new(1, 10).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
    let query = GetMembersPaginatedQuery { pagination };
    let users = handle_get_members_paginated(&repo, query).await.unwrap();

    for user in users {
        println!("id: {}, name: {}", user.id(), user.name());
    }
    Ok(())
}

async fn get_member_by_id(db: &sqlx::Pool<sqlx::Postgres>) -> result::Result<(), sqlx::Error> {
    let query = GetMemberByIdQuery {
        id: Uuid::parse_str("9b29622c-add1-42e5-b5e2-b6f9246939c5").unwrap(),
    };

    let repo = PostgresMemberRepository::new(db.clone());
    let member = handle_get_member_by_id(&repo, query).await.unwrap();

    println!("Member: {:?}", member);

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

/// Fetches paginated organizations into [`OrganizationResponse`] DTOs using the DDD approach.
async fn select_organization_response(db: &PgPool) -> result::Result<(), sqlx::Error> {
    let repo = PostgresOrganizationRepository::new(db.clone());
    let pagination = Pagination::new(1, 10).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
    let query = GetOrganizationsPaginatedQuery { pagination };
    let orgs = handle_get_organizations_paginated(&repo, query).await.unwrap();

    for org in orgs {
        println!("id: {}, name: {}", org.id(), org.name());
    }
    Ok(())
}

/// Fetches paginated organizations and maps to domain [`Organization`] entities (DDD-style).
#[allow(non_snake_case)]
async fn select_organization_entity(db: &PgPool) -> result::Result<(), sqlx::Error> {
    let repo = PostgresOrganizationRepository::new(db.clone());
    let pagination = Pagination::new(1, 10).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
    let query = GetOrganizationsPaginatedQuery { pagination };
    let orgs = handle_get_organizations_paginated(&repo, query).await.unwrap();

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
