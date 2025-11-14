use crate::{api::ApiState, domain::email::Email};

#[derive(Debug)]
pub struct UserEntity {
    pub id: i64,
    pub email: Email,
    pub created_at: String,
    pub verified_at: Option<String>,
}

impl UserEntity {
    pub fn new(email: &Email) -> UserEntity {
        UserEntity {
            id: i64::default(),
            email: Email(email.0.clone()),
            created_at: String::default(),
            verified_at: None,
        }
    }

    pub async fn save(&mut self, state: &ApiState) -> Result<(), sqlx::Error> {
        let user = sqlx::query!(
            "INSERT INTO users (email) VALUES ($1) RETURNING id, email, created_at::date::text, verified_at::date::text",
            self.email.0
        )
        .fetch_one(&state.db)
        .await?;

        self.id = user.id;
        self.email = Email(user.email);
        self.created_at = user.created_at.unwrap();
        self.verified_at = user.verified_at;

        Ok(())
    }

    pub async fn get_by_email(state: &ApiState, email: &Email) -> Result<UserEntity, sqlx::Error> {
        let user = sqlx::query!(
            "SELECT id, email, created_at::date::text, verified_at::date::text FROM users WHERE email = $1",
            email.0
        )
        .fetch_one(&state.db)
        .await?;

        Ok(UserEntity {
            id: user.id,
            email: Email(user.email),
            created_at: user.created_at.unwrap(),
            verified_at: user.verified_at,
        })
    }
}
