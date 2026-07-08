use sqlx::PgPool;
use crate::modules::payment_gateway::model::payment_gateway::GatewayConfig;

pub struct PaymentGatewayRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> PaymentGatewayRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn list_gateways(&self) -> Result<Vec<GatewayConfig>, sqlx::Error> {
        sqlx::query_as::<_, GatewayConfig>("SELECT * FROM payment_gateways ORDER BY is_primary DESC")
            .fetch_all(self.pool)
            .await
    }

    pub async fn create_gateway(
        &self,
        gateway_id: &str,
        name: &str,
        is_primary: bool,
    ) -> Result<GatewayConfig, sqlx::Error> {
        sqlx::query_as::<_, GatewayConfig>(
            "INSERT INTO payment_gateways (gateway_id, name, is_primary) VALUES ($1,$2,$3) RETURNING *",
        )
        .bind(gateway_id)
        .bind(name)
        .bind(is_primary)
        .fetch_one(self.pool)
        .await
    }
}
