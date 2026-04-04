use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::Organization;
use crate::domain::errors::{DomainError, Result};
use crate::domain::repository::OrganizationRepository;
use crate::domain::value_object::{OrganizationId, Pagination};

#[derive(Clone)]
pub struct PostgresOrganizationRepository {
    pool: PgPool,
}

impl PostgresOrganizationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationRepository for PostgresOrganizationRepository {
    async fn find_all(&self) -> Result<Vec<Organization>> {
        let rows = sqlx::query!(
            "SELECT id, name, description, created_at, updated_at FROM public.organizations"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(DomainError::custom_from_err)?;

        Ok(rows
            .into_iter()
            .map(|r| {
                Organization::from_persistence(r.id, r.name, r.description, r.created_at, r.updated_at)
            })
            .collect())
    }

    async fn find_by_id(&self, id: OrganizationId) -> Result<Option<Organization>> {
        let row = sqlx::query!(
            "SELECT id, name, description, created_at, updated_at FROM public.organizations WHERE id = $1",
            id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(DomainError::custom_from_err)?;

        match row {
            Some(r) => Ok(Some(Organization::from_persistence(r.id, r.name, r.description, r.created_at, r.updated_at))),
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Organization>> {
        let row = sqlx::query!(
            "SELECT id, name, description, created_at, updated_at FROM public.organizations WHERE name = $1",
            name
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(DomainError::custom_from_err)?;

        match row {
            Some(r) => Ok(Some(Organization::from_persistence(r.id, r.name, r.description, r.created_at, r.updated_at))),
            None => Ok(None),
        }
    }

    async fn find_paginated(&self, pagination: &Pagination) -> Result<Vec<Organization>> {
        let rows = sqlx::query!(
            "SELECT id, name, description, created_at, updated_at FROM public.organizations ORDER BY id LIMIT $1 OFFSET $2",
            pagination.limit() as i64,
            pagination.offset() as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(DomainError::custom_from_err)?;

        Ok(rows
            .into_iter()
            .map(|r| {
                Organization::from_persistence(r.id, r.name, r.description, r.created_at, r.updated_at)
            })
            .collect())
    }
}