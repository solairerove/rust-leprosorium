use std::{fs, path::Path};

use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool, sqlite::SqliteConnectOptions};
use uuid::Uuid;

use crate::{models::Note, util::now_utc};

#[derive(Clone)]
pub struct NotesStore {
    pool: SqlitePool,
}

impl NotesStore {
    pub async fn load_or_new(data_dir: &Path) -> sqlx::Result<Self> {
        let db_path = data_dir.join("notes.db");

        fs::create_dir_all(data_dir).map_err(sqlx::Error::Io)?;

        let options = SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn list_desc(&self) -> sqlx::Result<Vec<Note>> {
        let rows = sqlx::query_as::<_, NoteRow>(
            "SELECT id, title, body, created_at_unix FROM notes ORDER BY created_at_unix DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter().map(Note::try_from).collect()
    }

    pub async fn create_note(&self, title: String, body: String) -> sqlx::Result<()> {
        let now = now_utc();
        let id = Uuid::now_v7().to_string();
        sqlx::query("INSERT INTO notes (id, title, body, created_at_unix) VALUES (?1, ?2, ?3, ?4)")
            .bind(id)
            .bind(title)
            .bind(body)
            .bind(now.timestamp())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_note(&self, id: &str) -> sqlx::Result<()> {
        sqlx::query("DELETE FROM notes WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn update_note(
        &self,
        id: &str,
        title: String,
        body: String,
    ) -> sqlx::Result<Option<Note>> {
        let result = sqlx::query("UPDATE notes SET title = ?1, body = ?2 WHERE id = ?3")
            .bind(title)
            .bind(body)
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Ok(None);
        }

        self.get_note(id).await
    }

    pub async fn get_note(&self, id: &str) -> sqlx::Result<Option<Note>> {
        let row = sqlx::query_as::<_, NoteRow>(
            "SELECT id, title, body, created_at_unix FROM notes WHERE id = ?1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        row.map(Note::try_from).transpose()
    }
}

#[derive(FromRow)]
struct NoteRow {
    id: String,
    title: String,
    body: String,
    created_at_unix: i64,
}

#[derive(Debug, thiserror::Error)]
#[error("invalid unix timestamp in notes.created_at_unix: {0}")]
struct InvalidDbTimestamp(i64);

impl TryFrom<NoteRow> for Note {
    type Error = sqlx::Error;

    fn try_from(row: NoteRow) -> Result<Self, Self::Error> {
        let created_at = DateTime::from_timestamp(row.created_at_unix, 0).ok_or_else(|| {
            sqlx::Error::Decode(Box::new(InvalidDbTimestamp(row.created_at_unix)))
        })?;

        Ok(Self {
            id: row.id,
            title: row.title,
            body: row.body,
            created_at: created_at.with_timezone(&Utc),
        })
    }
}
