use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::Response;
use prost::Message;
use std::sync::Arc;

use crate::modules::customer::application::services::CustomerService;
use crate::shared::app_state::AppState;
use crate::shared::errors::AppError;
use crate::shared::middleware::auth::{require_permission, UserContext};
use crate::shared::protobuf::proto::customer::*;
use crate::shared::protobuf::proto as pb;
use crate::shared::protobuf::{proto_response, PROTOBUF_CONTENT_TYPE};

async fn decode_proto<T: Message + Default>(req: Request) -> Result<T, AppError> {
    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to read request body: {}", e)))?;
    T::decode(&body_bytes[..]).map_err(|e| AppError::Validation(format!("Invalid protobuf: {}", e)))
}

fn wrap_ok<T: Message>(data: &T) -> Result<Response<Body>, AppError> {
    let mut buf = bytes::BytesMut::with_capacity(data.encoded_len());
    data.encode(&mut buf).unwrap();
    let envelope = pb::Response {
        status: pb::ResponseStatus::Ok.into(),
        data: buf.freeze().to_vec(),
        error: None,
        meta: None,
    };
    Ok(proto_response(envelope))
}

fn wrap_created<T: Message>(data: &T) -> Result<Response<Body>, AppError> {
    let mut buf = bytes::BytesMut::with_capacity(data.encoded_len());
    data.encode(&mut buf).unwrap();
    let envelope = pb::Response {
        status: pb::ResponseStatus::Ok.into(),
        data: buf.freeze().to_vec(),
        error: None,
        meta: None,
    };
    let mut envelope_buf = bytes::BytesMut::with_capacity(envelope.encoded_len());
    envelope.encode(&mut envelope_buf).unwrap();
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("content-type", PROTOBUF_CONTENT_TYPE)
        .body(Body::from(envelope_buf.freeze()))
        .unwrap())
}

fn no_content_response() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap()
}

fn model_to_proto(c: &crate::modules::customer::domain::entities::customer::Model) -> Customer {
    Customer {
        id: c.id,
        customer_code: c.customer_code.clone(),
        branch_id: c.branch_id,
        name: c.name.clone(),
        email: c.email.clone(),
        phone: c.phone.clone(),
        status: c.status.clone(),
        created_at: c.created_at.to_rfc3339(),
        updated_at: c.updated_at.to_rfc3339(),
    }
}

fn address_to_proto(a: &crate::modules::customer::domain::entities::address::Model) -> CustomerAddress {
    CustomerAddress {
        id: a.id,
        customer_id: a.customer_id,
        address_type: a.address_type.clone(),
        line1: a.line1.clone(),
        line2: a.line2.clone(),
        city: a.city.clone(),
        state: a.state.clone(),
        pincode: a.pincode.clone(),
        is_primary: a.is_primary,
    }
}

/// POST /api/v1/customers/list
pub async fn list_customers(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    req: Request,
) -> Result<Response<Body>, AppError> {
    require_permission(&user, "customer.account.view").map_err(|e| AppError::Forbidden(e.1))?;
    let proto_req: ListCustomersRequest = decode_proto(req).await?;
    let branch_id = if user.is_company_wide {
        None
    } else {
        user.branch_id
    };
    let pagination = proto_req.pagination.unwrap_or_default();
    let page = if pagination.page == 0 { 1 } else { pagination.page as u64 };
    let limit = if pagination.page_size == 0 { 20 } else { pagination.page_size as u64 };
    let (customers, total) =
        CustomerService::list_customers(&state.db, branch_id, page, limit).await?;
    let proto_customers: Vec<Customer> = customers.iter().map(model_to_proto).collect();
    let resp = ListCustomersResponse {
        customers: proto_customers,
        total_count: total,
        page: page as u32,
        page_size: limit as u32,
    };
    wrap_ok(&resp)
}

/// POST /api/v1/customers/create
pub async fn create_customer(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    req: Request,
) -> Result<Response<Body>, AppError> {
    require_permission(&user, "customer.account.create").map_err(|e| AppError::Forbidden(e.1))?;
    let proto_req: CreateCustomerRequest = decode_proto(req).await?;
    let customer = CustomerService::create_customer(
        &state.db,
        proto_req.branch_id,
        proto_req.name,
        proto_req.email,
        proto_req.phone,
        proto_req.alternate_phone,
    )
    .await?;

    let payload = serde_json::json!({
        "customer_id": customer.id,
        "customer_code": customer.customer_code,
        "branch_id": customer.branch_id,
        "name": customer.name,
        "phone": customer.phone,
        "status": customer.status,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        "customer.created",
        "customer",
        customer.id,
        payload,
        None,
        None,
        Some(customer.branch_id),
    )
    .await
    {
        tracing::error!(customer_id = customer.id, error = %e, "Failed to publish customer.created event");
    }

    wrap_created(&model_to_proto(&customer))
}

/// POST /api/v1/customers/get
pub async fn get_customer(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    req: Request,
) -> Result<Response<Body>, AppError> {
    let proto_req: GetCustomerRequest = decode_proto(req).await?;
    let customer = CustomerService::get_customer(&state.db, proto_req.customer_id).await?;
    wrap_ok(&model_to_proto(&customer))
}

/// PATCH /api/v1/customers/update
pub async fn update_customer(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    req: Request,
) -> Result<Response<Body>, AppError> {
    require_permission(&user, "customer.account.update").map_err(|e| AppError::Forbidden(e.1))?;
    let proto_req: UpdateCustomerRequest = decode_proto(req).await?;
    let customer = CustomerService::update_customer(
        &state.db,
        proto_req.customer_id,
        proto_req.name,
        proto_req.email,
        proto_req.phone,
        proto_req.alternate_phone,
    )
    .await?;
    wrap_ok(&model_to_proto(&customer))
}

/// POST /api/v1/customers/update-status
pub async fn update_customer_status(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    req: Request,
) -> Result<Response<Body>, AppError> {
    require_permission(&user, "customer.account.update_status").map_err(|e| AppError::Forbidden(e.1))?;
    let proto_req: UpdateCustomerStatusRequest = decode_proto(req).await?;
    let id = proto_req.customer_id;
    let status = proto_req.status;
    let old_status = CustomerService::get_customer(&state.db, id)
        .await
        .map(|c| c.status)
        .unwrap_or_default();
    let customer = CustomerService::update_customer_status(&state.db, id, &status).await?;

    let event_type = match status.as_str() {
        "active" => "customer.activated",
        "suspended" => "customer.suspended",
        "terminated" => "customer.terminated",
        _ => "customer.status.changed",
    };
    let payload = serde_json::json!({
        "customer_id": customer.id,
        "old_status": old_status,
        "new_status": customer.status,
    });
    if let Err(e) = crate::infrastructure::messaging::outbox::insert_outbox_event(
        &state.db,
        event_type,
        "customer",
        customer.id,
        payload,
        None,
        None,
        Some(customer.branch_id),
    )
    .await
    {
        tracing::error!(customer_id = customer.id, error = %e, "Failed to publish customer status event");
    }

    wrap_ok(&model_to_proto(&customer))
}

/// DELETE /api/v1/customers/delete
pub async fn delete_customer(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    req: Request,
) -> Result<Response<Body>, AppError> {
    require_permission(&user, "customer.account.delete").map_err(|e| AppError::Forbidden(e.1))?;
    let proto_req: DeleteCustomerRequest = decode_proto(req).await?;
    CustomerService::delete_customer(&state.db, proto_req.customer_id).await?;
    Ok(no_content_response())
}

/// POST /api/v1/customers/search
pub async fn search_customers(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    req: Request,
) -> Result<Response<Body>, AppError> {
    let proto_req: SearchCustomersRequest = decode_proto(req).await?;
    let customers =
        CustomerService::search_customers(&state.db, proto_req.query, proto_req.status).await?;
    let proto_customers: Vec<Customer> = customers.iter().map(model_to_proto).collect();
    let resp = ListCustomersResponse {
        customers: proto_customers,
        total_count: 0,
        page: 1,
        page_size: 0,
    };
    wrap_ok(&resp)
}

/// POST /api/v1/customers/addresses/list
pub async fn list_addresses(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    req: Request,
) -> Result<Response<Body>, AppError> {
    let proto_req: ListAddressesRequest = decode_proto(req).await?;
    let addresses = CustomerService::get_addresses(&state.db, proto_req.customer_id).await?;
    let proto_addresses: Vec<CustomerAddress> = addresses.iter().map(address_to_proto).collect();
    let resp = ListAddressesResponse {
        addresses: proto_addresses,
    };
    wrap_ok(&resp)
}

/// POST /api/v1/customers/addresses/create
pub async fn add_address(
    State(state): State<Arc<AppState>>,
    user: UserContext,
    req: Request,
) -> Result<Response<Body>, AppError> {
    require_permission(&user, "customer.address.create").map_err(|e| AppError::Forbidden(e.1))?;
    let proto_req: AddAddressRequest = decode_proto(req).await?;
    let addr = CustomerService::add_address(
        &state.db,
        proto_req.customer_id,
        proto_req.address_type
            .unwrap_or_else(|| "installation".to_string()),
        proto_req.line1,
        proto_req.line2,
        proto_req.city,
        proto_req.state,
        proto_req.pincode,
        proto_req.landmark,
    )
    .await?;
    wrap_created(&address_to_proto(&addr))
}

/// POST /api/v1/customers/history
pub async fn get_customer_history(
    State(state): State<Arc<AppState>>,
    _user: UserContext,
    req: Request,
) -> Result<Response<Body>, AppError> {
    use crate::modules::audit::domain::entity_history::EntityHistoryService;

    let proto_req: GetCustomerHistoryRequest = decode_proto(req).await?;
    let page = proto_req.page.unwrap_or(1) as i64;
    let page_size = proto_req.page_size.unwrap_or(100) as i64;

    let result = EntityHistoryService::search_history(
        &state.db,
        "customers",
        Some(proto_req.customer_id.to_string()),
        None,
        None,
        None,
        None,
        page,
        page_size,
    )
    .await?;

    let items: Vec<CustomerHistoryEntry> = result
        .items
        .into_iter()
        .map(|h| CustomerHistoryEntry {
            id: h.id.parse::<i64>().unwrap_or(0),
            entity_id: h.entity_id,
            action: h.action,
            old_data: h.old_data.map(|v| v.to_string()),
            new_data: h.new_data.map(|v| v.to_string()),
            changed_fields: h.changed_fields.map(|v| v.join(",")),
            user_id: h.user_id,
            user_name: h.user_name,
            user_email: h.user_email,
            created_at: h.created_at.to_rfc3339(),
        })
        .collect();

    let resp = CustomerHistoryResponse {
        entity_type: "customers".to_string(),
        entity_id: proto_req.customer_id.to_string(),
        total_count: result.total as u64,
        items,
    };
    wrap_ok(&resp)
}
