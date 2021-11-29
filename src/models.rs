use super::schema::messages;

#[derive(Queryable)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub text: String,
}

#[derive(Insertable)]
#[table_name = "messages"]
pub struct NewMessage<'a> {
    pub user_id: &'a i64,
    pub text: &'a str,
}
