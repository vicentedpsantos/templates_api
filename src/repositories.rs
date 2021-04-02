use super::models::*;
use super::schema::*;
use diesel::prelude::*;
use diesel::result::QueryResult;
pub struct TemplateRepository;

impl TemplateRepository {
    pub fn load_all(c: &SqliteConnection) -> QueryResult<Vec<Template>> {
        templates::table.limit(100).load::<Template>(c)
    }

    pub fn find_one(c: &SqliteConnection, id: i32) -> QueryResult<Template> {
        templates::table.find(id).get_result::<Template>(c)
    }

    pub fn create(c: &SqliteConnection, new_template: NewTemplate) -> QueryResult<Template> {
        diesel::insert_into(templates::table)
            .values(new_template)
            .execute(c)?;

        let last_id = Self::last_id(c)?;
        Self::find_one(c, last_id)
    }

    pub fn save(c: &SqliteConnection, template: Template) -> QueryResult<Template> {
        diesel::update(templates::table.find(template.id))
            .set((
                templates::name.eq(template.name.to_owned()),
                templates::content.eq(template.content.to_owned()),
            ))
            .execute(c)?;
        Self::find_one(c, template.id)
    }

    pub fn delete(c: &SqliteConnection, id: i32) -> bool {
        match Self::find_one(c, id) {
            Ok(_) => {
                let _ = diesel::delete(templates::table.find(id)).execute(c);
                true
            }
            Err(_) => false,
        }
    }

    fn last_id(c: &SqliteConnection) -> QueryResult<i32> {
        templates::table
            .select(templates::id)
            .order(templates::id.desc())
            .first(c)
    }
}
