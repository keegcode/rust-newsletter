use crate::api::ApiState;

pub struct Topic {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub template: String,
    pub created_at: String,
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
            created_at: String::default(),
        }
    }

    pub async fn save(&mut self, state: &ApiState) -> Result<(), sqlx::Error> {
        let topic = sqlx::query!(
            "INSERT INTO topics (name, user_id, template) VALUES ($1, $2, $3) RETURNING id, created_at::date::text",
            self.name,
            self.user_id,
            self.template
        )
        .fetch_one(&state.db)
        .await?;

        self.id = topic.id;
        self.created_at = topic.created_at.unwrap();

        Ok(())
    }

    pub async fn get_by_name(state: &ApiState, name: &str) -> Result<Topic, sqlx::Error> {
        let topic = sqlx::query!(
            "SELECT id, user_id, name, template, created_at::date::text FROM topics where name = $1",
            name
        )
        .fetch_one(&state.db)
        .await?;

        Ok(Topic {
            id: topic.id,
            user_id: topic.user_id,
            name: topic.name,
            template: topic.template.unwrap(),
            created_at: topic.created_at.unwrap(),
        })
    }
}
