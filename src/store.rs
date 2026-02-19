use std::{fs, path::Path};

use sqlx::{Row, SqlitePool, sqlite::SqliteConnectOptions};

use crate::{models::Note, util::now_unix};

#[derive(Clone)]
pub struct NotesStore {
    pool: SqlitePool,
}

impl NotesStore {
    pub async fn load_or_new(data_dir: &Path) -> Self {
        let db_path = data_dir.join("notes.db");

        if let Err(err) = fs::create_dir_all(data_dir) {
            eprintln!("Failed to create data directory: {err}");
        }

        let options = SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options)
            .await
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to connect SQLite database {}: {err}",
                    db_path.display()
                )
            });

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                body TEXT NOT NULL,
                created_at_unix INTEGER NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await
        .unwrap_or_else(|err| panic!("Failed to initialize notes table: {err}"));

        Self { pool }
    }

    pub async fn list_desc(&self) -> Vec<Note> {
        let rows = sqlx::query(
            "SELECT id, title, body, created_at_unix FROM notes ORDER BY created_at_unix DESC",
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_else(|err| panic!("Failed to list notes: {err}"));

        rows.into_iter()
            .map(|row| Note {
                id: row.get::<String, _>("id"),
                title: row.get::<String, _>("title"),
                body: row.get::<String, _>("body"),
                created_at_unix: row.get::<i64, _>("created_at_unix") as u64,
            })
            .collect()
    }

    pub async fn create_note(&self, title: String, body: String) {
        let now = now_unix();
        let id = format!("{now}");
        sqlx::query("INSERT INTO notes (id, title, body, created_at_unix) VALUES (?1, ?2, ?3, ?4)")
            .bind(id)
            .bind(title)
            .bind(body)
            .bind(now as i64)
            .execute(&self.pool)
            .await
            .unwrap_or_else(|err| panic!("Failed to create note: {err}"));
    }

    pub async fn delete_note(&self, id: &str) {
        sqlx::query("DELETE FROM notes WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap_or_else(|err| panic!("Failed to delete note: {err}"));
    }

    pub async fn update_note(&self, id: &str, title: String, body: String) -> Option<Note> {
        let result = sqlx::query("UPDATE notes SET title = ?1, body = ?2 WHERE id = ?3")
            .bind(title)
            .bind(body)
            .bind(id)
            .execute(&self.pool)
            .await
            .unwrap_or_else(|err| panic!("Failed to update note: {err}"));

        if result.rows_affected() == 0 {
            return None;
        }

        self.get_note(id).await
    }

    pub async fn get_note(&self, id: &str) -> Option<Note> {
        let row = sqlx::query("SELECT id, title, body, created_at_unix FROM notes WHERE id = ?1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .unwrap_or_else(|err| panic!("Failed to get note: {err}"))?;

        Some(Note {
            id: row.get::<String, _>("id"),
            title: row.get::<String, _>("title"),
            body: row.get::<String, _>("body"),
            created_at_unix: row.get::<i64, _>("created_at_unix") as u64,
        })
    }
}
