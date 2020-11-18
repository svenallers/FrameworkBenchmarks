//! Db executor actor

use std::io;

use actix::prelude::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::models;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub struct DbExecutor {
    conn: PgPool ,
    rng: SmallRng,
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl DbExecutor {
    pub fn new(db_url: &str) -> DbExecutor {
        DbExecutor {
            conn: PgPoolOptions::new()
                .max_connections(5)
                .connect(db_url)
                .expect(&format!("Error connecting to {}", db_url)),
            rng: SmallRng::from_entropy(),
        }
    }
}

pub struct RandomWorld;

impl Message for RandomWorld {
    type Result = io::Result<models::World>;
}

impl Handler<RandomWorld> for DbExecutor {
    type Result = ResponseFuture<io::Result<models::World>>;

    fn handle(&mut self, _: RandomWorld, _: &mut Self::Context) -> Self::Result {
        let random_id = self.rng.gen_range(1, 10_001);
        Box::pin(
            async {
                sqlx::query_as::<_, models::World>("select * from world where id=$1").bind(random_id).fetch_one(&self.conn)
            }
        )
    }
}

pub struct RandomWorlds(pub u16);

impl Message for RandomWorlds {
    type Result = io::Result<Vec<models::World>>;
}

impl Handler<RandomWorlds> for DbExecutor {
    type Result = ResponseFuture<io::Result<Vec<models::World>>>;

    fn handle(&mut self, msg: RandomWorlds, _: &mut Self::Context) -> Self::Result {
        Box::pin(
            async {
                let mut worlds = Vec::with_capacity(msg.0 as usize);
                for _ in 0..msg.0 {
                    let w_id = self.rng.gen_range(1, 10_001);
                    let w = sqlx::<_, models::World>("select * from world where id=$1").bind(w_id).fetch_one(&self.conn)?;
                    worlds.push(w)
                }
                Ok(worlds)
            }
        )
    }
}

pub struct UpdateWorld(pub u16);

impl Message for UpdateWorld {
    type Result = io::Result<Vec<models::World>>;
}

impl Handler<UpdateWorld> for DbExecutor {
    type Result = ResponseFuture<io::Result<Vec<models::World>>>;

    fn handle(&mut self, msg: UpdateWorld, _: &mut Self::Context) -> Self::Result {
        Box::pin(
            async {
                let mut worlds = Vec::with_capacity(msg.0 as usize);
                for _ in 0..msg.0 {
                    let w_id: i32 = self.rng.gen_range(1, 10_001);
                    let mut w = sqlx::<_, models::World>("select * from world where id=$1").bind(w_id).fetch_one(&self.conn)?;
                    w.randomnumber = self.rng.gen_range(1, 10_001);
                    worlds.push(w);
                }
                worlds.sort_by_key(|w| w.id);

                let _ = self.conn.transaction::<(), io::Error, _>(|| {
                    for w in &worlds {
                        let _ = sqlx::query("update world set randomnumber=$1 where id=$2").bind(w.randomnumber).bind(w.id).execute(&self.conn).await?;
                    }
                    Ok(())
                });

                Ok(worlds)
            }
        )
    }
}

pub struct TellFortune;

impl Message for TellFortune {
    type Result = io::Result<Vec<models::Fortune>>;
}

impl Handler<TellFortune> for DbExecutor {
    type Result = ResponseFuture<io::Result<Vec<models::Fortune>>>;

    fn handle(&mut self, _: TellFortune, _: &mut Self::Context) -> Self::Result {
        Box::pin(
            async {
                let items = sqlx::query_as!(models::Fortune, "select * from fortune").fetch_all(&self.conn).await?;
                Ok(items)
            }
        )
    }
}
