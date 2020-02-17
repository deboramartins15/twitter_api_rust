use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::tweets;

#[table_name = "tweets"]
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct Tweet {
    pub id: Option<i32>,
    pub user_id: Option<i32>,
    pub content: String
}

impl Tweet{
    pub fn create(tweet: Tweet, connection: &PgConnection) -> Tweet {
        diesel::insert_into(tweets::table)
            .values(&tweet)
            .execute(connection)
            .expect("Error creating new tweet");

            tweets::table.order(tweets::id.desc()).first(connection).unwrap()
    } 

    pub fn read(connection: &PgConnection) -> Vec<Tweet> {
        tweets::table.order(tweets::id).load::<Tweet>(connection).unwrap()
    }

    pub fn get(id: i32, connection: &PgConnection) -> Tweet {
        tweets::table.find(id).get_result::<Tweet>(connection)
    }
}