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
use chrono::offset::Utc;
use tokio::net::TcpListener;

use crate::Data;

#[derive(Debug, Clone)]
pub struct Port {
    port: u16,
    created_at: i64,
}

impl Port {
    pub async fn new(data: &Data) -> Self {
        let port = Self::get_port().await;
        sqlx::query!(
            "INSERT into mainframe_port_lock (port, created_at) VALUES ($1, $2)",
            port.port,
            port.created_at
        )
        .execute(&data.db)
        .await
        .unwrap();
        port
    }

    async fn get_port() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let socket = listener.local_addr().unwrap();
        let port = socket.port();
        let created_at = Utc::now().timestamp();
        Self { port, created_at }
    }

    pub async fn delete_port(&self, data: &Data) {
        sqlx::query!("DELETE FROM mainframe_port_lock where port = $1", self.port,)
            .execute(&data.db)
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    #[test]
    async fn port_works() {
        let data = Data::new().await;
        let port = Port::new(&data).await;

        //        let res = sqlx::query!(
        //            "SELECT EXISTS (SELECT 1 from mainframe_port_lock WHERE port = $1)",
        //            &payload.val,
        //        )
        //        .fetch_one(&data.db)
        //        .await
        //        .unwrap();
        //
        //        assert!(res.exists.as_ref().unwrap());

        port.delete_port(&data).await;
    }
}
