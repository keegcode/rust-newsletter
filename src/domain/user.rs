use crate::{api::ApiState, domain::email::Email};

pub struct UserEntity {
    pub id: i64,
    pub email: Email,
    pub created_at: String,
    pub verified_at: Option<String>,
}

impl UserEntity {
    pub async fn get_by_email(state: &ApiState, email: &Email) -> Option<UserEntity> {
        let row =
            sqlx::query!("SELECT id, email, created_at, verified_at FROM users WHERE email = $1")
                .bind(email.into())
                .fetch_one(&state.db)
                .await
                .ok()?;

        Some(UserEntity {
            id: user.id,
            email: (),
            created_at: (),
            verified_at: (),
        })
    }
}
