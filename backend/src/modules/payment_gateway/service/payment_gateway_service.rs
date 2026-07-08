use sqlx::PgPool;
use crate::common::errors::app_error::AppError;
use crate::modules::payment_gateway::repository::payment_gateway_repository::PaymentGatewayRepository;
use crate::modules::payment_gateway::request::payment_gateway_request::*;
use crate::modules::payment_gateway::response::payment_gateway_response::*;

pub struct PaymentGatewayService<'a> {
    repo: PaymentGatewayRepository<'a>,
}

impl<'a> PaymentGatewayService<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self {
            repo: PaymentGatewayRepository::new(pool),
        }
    }

    pub async fn list_gateways(&self) -> Result<Vec<GatewayConfigResponse>, AppError> {
        let g = self.repo.list_gateways().await?;
        Ok(g.iter()
            .map(|x| GatewayConfigResponse {
                id: x.id,
                gateway_id: x.gateway_id.clone(),
                name: x.name.clone(),
                is_primary: x.is_primary,
                is_active: x.is_active,
                created_at: x.created_at,
            })
            .collect())
    }

    pub async fn create_gateway(
        &self,
        req: CreateGatewayConfigRequest,
    ) -> Result<GatewayConfigResponse, AppError> {
        let g = self
            .repo
            .create_gateway(&req.gateway_id, &req.name, req.is_primary.unwrap_or(false))
            .await?;
        Ok(GatewayConfigResponse {
            id: g.id,
            gateway_id: g.gateway_id,
            name: g.name,
            is_primary: g.is_primary,
            is_active: g.is_active,
            created_at: g.created_at,
        })
    }

    pub async fn create_payment_link(
        &self,
        _req: CreatePaymentLinkRequest,
    ) -> Result<PaymentLinkResponse, AppError> {
        let url = format!("https://pay.aeroxe.in/{}", uuid::Uuid::new_v4());
        Ok(PaymentLinkResponse {
            payment_url: url,
            expires_in: 3600,
        })
    }
}
