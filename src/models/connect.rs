use diesel::pg::PgConnection;
use lazy_static;
use std::env;
use std::{thread, time};

use r2d2;
use r2d2_diesel::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
lazy_static! {
	static ref CONNECTION_POOL: Pool = create_connection();
}

pub fn get_connection() -> Pool {
	CONNECTION_POOL.clone()
}

// get connection;
fn create_connection() -> Pool {
	let mut connect_counter = 0;
	loop {
		let conncet_url =
			env::var("DATABASE_URL").expect("can't get database connect url from env");

		let manager = ConnectionManager::<PgConnection>::new(conncet_url);
		match r2d2::Pool::builder().build(manager) {
			Ok(pool) => break pool,
			Err(_) => {
				// connect database with connection url in 30 min;
				if connect_counter > 30 {
					panic!("can't connect mysql database");
				}
				connect_counter += 1;
				thread::sleep(time::Duration::from_secs(1));
				continue;
			}
		}
	}
}
