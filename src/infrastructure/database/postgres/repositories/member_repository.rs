use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::errors::{DomainError, Result};
use crate::domain::repository::MemberRepository;
use crate::domain::value_object::Pagination;
use crate::domain::{Member, MemberRole};

#[derive(Clone)]
pub struct PostgresMemberRepository {
    pool: PgPool,
}

impl PostgresMemberRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn role_to_db(role: MemberRole) -> &'static str {
        role.as_str()
    }

    fn row_to_member(row: &sqlx::postgres::PgRow) -> Result<Member> {
        let organization_id = row
            .try_get::<Uuid, _>("organization_id")
            .map_err(DomainError::custom_from_err)?;
        let division_id = row
            .try_get::<Option<Uuid>, _>("division_id")
            .map_err(DomainError::custom_from_err)?;
        let name = row
            .try_get::<String, _>("name")
            .map_err(DomainError::custom_from_err)?;
        let email = row
            .try_get::<Option<String>, _>("email")
            .map_err(DomainError::custom_from_err)?;
        let role_str = row
            .try_get::<Option<String>, _>("role")
            .map_err(DomainError::custom_from_err)?;

        let role = match role_str {
            Some(raw) => MemberRole::from_str(&raw)?,
            None => MemberRole::OperationsManager,
        };

        Member::new(organization_id, name, email, role, division_id)
    }
}

#[async_trait]
impl MemberRepository for PostgresMemberRepository {
    async fn save(&self, member: &Member) -> Result<()> {
        sqlx::query(
            "INSERT INTO public.members (id, organization_id, division_id, name, email, role, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, NOW())
             ON CONFLICT (id)
             DO UPDATE SET
                organization_id = EXCLUDED.organization_id,
                division_id = EXCLUDED.division_id,
                name = EXCLUDED.name,
                email = EXCLUDED.email,
                role = EXCLUDED.role,
                updated_at = NOW()",
        )
        .bind(member.id())
        .bind(member.organization_id())
        .bind(member.division_id())
        .bind(member.name())
        .bind(member.email())
        .bind(Self::role_to_db(member.role()))
        .execute(&self.pool)
        .await
        .map_err(DomainError::custom_from_err)?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Member>> {
        let rows = sqlx::query(
            "SELECT id, organization_id, division_id, name, email, role
             FROM public.members",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(DomainError::custom_from_err)?;

        rows.into_iter()
            .map(|row| Self::row_to_member(&row))
            .collect()
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Member>> {
        let row = sqlx::query(
            "SELECT id, organization_id, division_id, name, email, role
             FROM public.members
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(DomainError::custom_from_err)?;

        match row {
            Some(row) => Ok(Some(Self::row_to_member(&row)?)),
            None => Ok(None),
        }
    }

    async fn find_by_organization(&self, org_id: Uuid) -> Result<Vec<Member>> {
        let rows = sqlx::query(
            "SELECT id, organization_id, division_id, name, email, role
             FROM public.members
             WHERE organization_id = $1",
        )
        .bind(org_id)
        .fetch_all(&self.pool)
        .await
        .map_err(DomainError::custom_from_err)?;

        rows.into_iter()
            .map(|row| Self::row_to_member(&row))
            .collect()
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM public.members WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(DomainError::custom_from_err)?;

        Ok(())
    }

    async fn find_paginated(&self, pagination: &Pagination) -> Result<Vec<Member>> {
        let rows = sqlx::query(
            "SELECT id, organization_id, division_id, name, email, role
             FROM public.members
             ORDER BY id
             LIMIT $1 OFFSET $2",
        )
        .bind(pagination.limit())
        .bind(pagination.offset())
        .fetch_all(&self.pool)
        .await
        .map_err(DomainError::custom_from_err)?;

        rows.into_iter()
            .map(|row| Self::row_to_member(&row))
            .collect()
    }
}
