use std::fmt::{Display, Formatter};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Bookmark {
    pub id: uuid::Uuid,
    pub url: String,
    pub title: String,
    pub about: String,
    pub star: bool,
    pub archive: bool,
    pub trash: bool,
}

impl Display for Bookmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bookmark: {}, {}, {}, {}", self.id, self.url, self.title, self.about)
    }
}

pub struct Bookmarks(pub Vec<Bookmark>);

impl Display for Bookmarks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))
    }
}

#[cfg(feature = "ssr")]
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Bookmark {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        let id: &str = row.try_get("id")?;
        let id = uuid::Uuid::try_parse(id);
        // if let Err(uuid_parsing_error) = id {
        //     let e = sqlx::Error::ColumnDecode { index: "id".into(), source: Box::new(uuid_parsing_error) };
        //     return Err(e)
        // }
        let id = id.unwrap_or_default(); // TODO: remove
        let url: String = row.try_get("url")?;
        let title: String = row.try_get("title")?;
        let about: String = row.try_get("about")?;
        let star = if row.try_get::<i32, _>("star")? == 0 { false } else { true };
        let archive = if row.try_get::<i32, _>("archive")? == 0 { false } else { true };
        let trash = if row.try_get::<i32, _>("trash")? == 0 { false } else { true };
        Ok(Self { id, url, title, about, star, archive, trash })
    }
}
