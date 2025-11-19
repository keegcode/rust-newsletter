use serde::Serialize;
use time::OffsetDateTime;

use crate::api::ApiState;

#[derive(Debug, Serialize)]
pub struct Topic {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub template: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
}

pub struct NewTopicPayload {
    pub user_id: i64,
    pub name: String,
    pub template: String,
}

impl Topic {
    pub fn new(payload: &NewTopicPayload) -> Topic {
        Topic {
            id: i64::default(),
            name: String::from(&payload.name),
            template: String::from(&payload.template),
            user_id: payload.user_id,
            created_at: OffsetDateTime::now_utc(),
        }
    }

    pub async fn save(&mut self, state: &ApiState) -> Result<(), sqlx::Error> {
        let topic = sqlx::query!(
            "INSERT INTO topics (name, user_id, template) VALUES ($1, $2, $3) RETURNING id, created_at",
            self.name,
            self.user_id,
            self.template
        )
        .fetch_one(&state.db)
        .await?;

        self.id = topic.id;
        self.created_at = topic.created_at.assume_utc();

        Ok(())
    }

    pub async fn get_by_name(state: &ApiState, name: &str) -> Result<Option<Topic>, sqlx::Error> {
        let row = sqlx::query!(
            "SELECT id, user_id, name, template, created_at FROM topics where name = $1",
            name
        )
        .fetch_optional(&state.db)
        .await?;

        Ok(row.map(|r| Topic {
            id: r.id,
            user_id: r.user_id,
            name: r.name,
            template: r.template,
            created_at: r.created_at.assume_utc(),
        }))
    }

    pub async fn get_by_user_id(state: &ApiState, user_id: i64) -> Result<Vec<Topic>, sqlx::Error> {
        let rows = sqlx::query!(
            "SELECT id, user_id, name, template, created_at FROM topics where user_id = $1",
            user_id
        )
        .fetch_all(&state.db)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| Topic {
                id: r.id,
                user_id: r.user_id,
                name: r.name,
                template: r.template,
                created_at: r.created_at.assume_utc(),
            })
            .collect())
    }
}
