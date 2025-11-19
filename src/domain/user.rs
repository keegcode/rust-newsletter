use serde::Serialize;
use time::OffsetDateTime;

use crate::{api::ApiState, domain::email::Email};

#[derive(Debug, Serialize)]
pub struct UserEntity {
    pub id: i64,
    pub email: Email,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    pub verified_at: Option<OffsetDateTime>,
}

impl UserEntity {
    pub fn new(email: &Email) -> UserEntity {
        UserEntity {
            id: i64::default(),
            email: Email(email.0.clone()),
            created_at: OffsetDateTime::now_utc(),
            verified_at: None,
        }
    }

    pub async fn save(&mut self, state: &ApiState) -> Result<(), sqlx::Error> {
        let user = sqlx::query!(
            "INSERT INTO users (email) VALUES ($1) RETURNING id, email, created_at, verified_at",
            self.email.0
        )
        .fetch_one(&state.db)
        .await?;

        self.id = user.id;
        self.email = Email(user.email);
        self.created_at = user.created_at.assume_utc();
        self.verified_at = user.verified_at.map(|d| d.assume_utc());

        Ok(())
    }

    pub async fn get_by_email(
        state: &ApiState,
        email: &Email,
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, email, created_at, verified_at FROM users WHERE email = $1",
            email.0
        )
        .fetch_optional(&state.db)
        .await?;

        Ok(row.map(|r| UserEntity {
            id: r.id,
            email: Email(r.email),
            created_at: r.created_at.assume_utc(),
            verified_at: r.verified_at.map(|d| d.assume_utc()),
        }))
    }
}
