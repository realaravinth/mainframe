/* Mainframe - manage infrastructure with a few button clicks
 * Copyright © 2021 Aravinth Manivannan <realaravinth@batsense.net>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */
use std::env;

use sqlx::sqlite::SqlitePool;

pub struct Data {
    pub db: SqlitePool,
}

impl Data {
    pub async fn new() -> Self {
        let db = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        Data { db }
    }
}
