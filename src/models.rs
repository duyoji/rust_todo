use schema::todos;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub completed: bool,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
    pub body: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct UpdateTodo {
    pub title: String,
    pub body: String,
    pub completed: bool,
}