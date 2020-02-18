#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;
use crate::schema::tweets;
use crate::tweets::Tweets;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Tweets>> {
    tweets::table.load::<Tweets>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Tweets> {
    tweets::table.find(id).get_result::<Tweets>(connection)
}

pub fn insert(tweets: Tweets, connection: &PgConnection) -> QueryResult<Tweets> {
    diesel::insert_into(tweets::table)
        .values(&InsertableTweet::from_tweet(tweets))
        .get_result(connection)
}

#[derive(Insertable)]
#[table_name = "tweets"]
struct InsertableTweet {
    pub id: i32,
    pub user_id: i32,
    pub content: String
}

impl InsertableTweet {

    fn from_tweet(tweets: Tweets) -> InsertableTweet {
        InsertableTweet {            
            id: tweets.id,
            user_id: tweets.user_id,
            content: tweets.content
        }
    }
}