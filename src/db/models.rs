use super::schema::assets;

#[derive(Serialize, Queryable)]
pub struct Asset {
    pub id: Option<i32>,
    pub ticker: String,
    pub fullname: String,
}

#[derive(Insertable)]
#[table_name = "assets"]
pub struct NewAsset<'a> {
    pub id: Option<i32>,
    pub ticker: &'a str,
    pub fullname: &'a str,
}
