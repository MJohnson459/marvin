use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodablePackage {
    pub id: String,
    pub name: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub downloads: i32,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
}
