use std::{fs, path::Path};

use sqlx::{
    FromRow, SqlitePool,
    sqlite::SqliteConnectOptions,
};
use uuid::Uuid;

use crate::{models::Note, util::now_unix};

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

        Ok(rows.into_iter().map(Note::from).collect())
    }

    pub async fn create_note(&self, title: String, body: String) -> sqlx::Result<()> {
        let now = now_unix();
        let id = Uuid::now_v7().to_string();
        sqlx::query("INSERT INTO notes (id, title, body, created_at_unix) VALUES (?1, ?2, ?3, ?4)")
            .bind(id)
            .bind(title)
            .bind(body)
            .bind(now as i64)
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

        Ok(row.map(Note::from))
    }
}

#[derive(FromRow)]
struct NoteRow {
    id: String,
    title: String,
    body: String,
    created_at_unix: i64,
}

impl From<NoteRow> for Note {
    fn from(row: NoteRow) -> Self {
        Self {
            id: row.id,
            title: row.title,
            body: row.body,
            created_at_unix: row.created_at_unix as u64,
        }
    }
}
