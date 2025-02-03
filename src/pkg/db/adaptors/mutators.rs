use sqlx::PgPool;
use uuid::Uuid;

use crate::pkg::db::models::User;
use crate::Result;

impl User{
    pub async fn create_or_update(pool: &PgPool, name: &str, pic_obj_name: Option<&str>) -> Result<Self>{
        let user = Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            pic_obj_name: pic_obj_name.map(|s| s.to_string()),
        };

        sqlx::query!(
            r#"
            INSERT INTO users (id, name, pic_obj_name)
            VALUES ($1, $2, $3)
            ON CONFLICT (id)
            DO UPDATE SET name = EXCLUDED.name, pic_obj_name = EXCLUDED.pic_obj_name
            "#,
            user.id,
            user.name,
            user.pic_obj_name
        )
        .execute(pool)
        .await?;

        Ok(user)
    }
}

