// REST API implementation using axum

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use super::pyro_types::{error_codes, PyroErrorResponse, PyroSuccessResponse};
use super::auth::{AuthState, auth_middleware, JwtClaims};
use crate::db::models::*;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub auth: Arc<AuthState>,
}

/// Request body for initiating a vulnerability scan
#[derive(Debug, Clone, Deserialize)]
pub struct ScanRequest {
    pub target_range: String,
    pub scan_type: ScanType,
    #[serde(default)]
    pub verify_exploits: bool,
    #[serde(default = "default_max_threads")]
    pub max_threads: u32,
    #[serde(default = "default_timeout")]
    pub timeout_per_target: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<ScanCallbacks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ScanMetadata>,
}

fn default_max_threads() -> u32 { 10 }
fn default_timeout() -> u32 { 30 }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScanType {
    Quick,
    Standard,
    Comprehensive,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScanCallbacks {
    pub on_complete: Option<String>,
    pub on_vulnerability: Option<String>,
    pub on_error: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScanMetadata {
    pub environment: Option<String>,
    pub compliance_framework: Option<String>,
    pub requester: Option<String>,
}

/// Response for scan initiation
#[derive(Debug, Clone, Serialize)]
pub struct ScanInitResponse {
    pub scan_id: Uuid,
    pub status: String,
    pub estimated_completion: String,
    pub targets_count: u32,
}

/// Response for scan status query
#[derive(Debug, Clone, Serialize)]
pub struct ScanStatusResponse {
    pub scan_id: Uuid,
    pub status: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub targets_scanned: u32,
    pub vulnerabilities_found: u32,
    pub severity_breakdown: SeverityBreakdown,
    pub risk_score: f32,
    pub details: Vec<VulnerabilityDetail>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SeverityBreakdown {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
    pub info: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct VulnerabilityDetail {
    pub id: Uuid,
    pub target: String,
    pub vulnerability: String,
    pub severity: String,
    pub cvss_score: Option<f32>,
    pub exploit_available: bool,
    pub verified: bool,
}

/// Query parameters for listing scans
#[derive(Debug, Deserialize)]
pub struct ListScansQuery {
    pub status: Option<String>,
    pub environment: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 { 50 }

/// Query parameters for listing exploits
#[derive(Debug, Deserialize)]
pub struct ListExploitsQuery {
    pub category: Option<String>,
    pub target_platform: Option<String>,
    pub severity: Option<String>,
}

/// Create the main API router with all endpoints
pub async fn create_router() -> Router {
    let auth_state = Arc::new(AuthState::new());
    let app_state = AppState {
        auth: auth_state.clone(),
    };

    // Public routes (no authentication)
    let public_routes = Router::new()
        .route("/health", get(health_check));

    // Protected routes (require authentication)
    let protected_routes = Router::new()
        .route("/api/v1/vulnscan/scan", post(initiate_scan))
        .route("/api/v1/vulnscan/scans/:scan_id", get(get_scan_status))
        .route("/api/v1/vulnscan/scans", get(list_scans))
        .route("/api/v1/vulnscan/exploits", get(list_exploits))
        .route("/api/v1/vulnscan/exploits/:exploit_id", get(get_exploit))
        .route("/api/v1/vulnscan/cryptex", get(query_cryptex))
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            auth_middleware,
        ));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(app_state)
}

// ==================== Handler Functions ====================

async fn health_check() -> &'static str {
    "OK"
}

/// POST /api/v1/vulnscan/scan - Initiate a vulnerability scan
async fn initiate_scan(
    claims: axum::Extension<JwtClaims>,
    Json(request): Json<ScanRequest>,
) -> Result<Json<PyroSuccessResponse<ScanInitResponse>>, (StatusCode, Json<PyroErrorResponse>)> {
    // Validate input
    if request.target_range.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(PyroErrorResponse::new(
                error_codes::VALIDATION_ERROR,
                "target_range cannot be empty",
            )),
        ));
    }

    // TODO: Validate CIDR/IP format
    // TODO: Check user permissions (vulnscan:scan)
    // TODO: Create scan record in database
    // TODO: Queue scan job

    let scan_id = Uuid::new_v4();
    let response = ScanInitResponse {
        scan_id,
        status: "queued".to_string(),
        estimated_completion: chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(5))
            .unwrap()
            .to_rfc3339(),
        targets_count: 1, // TODO: Calculate from CIDR
    };

    Ok(Json(PyroSuccessResponse::new(response)))
}

/// GET /api/v1/vulnscan/scans/:scan_id - Get scan status and results
async fn get_scan_status(
    claims: axum::Extension<JwtClaims>,
    Path(scan_id): Path<Uuid>,
) -> Result<Json<PyroSuccessResponse<ScanStatusResponse>>, (StatusCode, Json<PyroErrorResponse>)> {
    // TODO: Query database for scan
    // TODO: Check tenant_id matches claims.tenant_id
    // TODO: Return actual scan results

    // Placeholder response
    let response = ScanStatusResponse {
        scan_id,
        status: "completed".to_string(),
        started_at: chrono::Utc::now()
            .checked_sub_signed(chrono::Duration::minutes(10))
            .unwrap()
            .to_rfc3339(),
        completed_at: Some(chrono::Utc::now().to_rfc3339()),
        targets_scanned: 1,
        vulnerabilities_found: 0,
        severity_breakdown: SeverityBreakdown {
            critical: 0,
            high: 0,
            medium: 0,
            low: 0,
            info: 0,
        },
        risk_score: 0.0,
        details: vec![],
    };

    Ok(Json(PyroSuccessResponse::new(response)))
}

/// GET /api/v1/vulnscan/scans - List all scans with filtering
async fn list_scans(
    claims: axum::Extension<JwtClaims>,
    Query(params): Query<ListScansQuery>,
) -> Result<Json<PyroSuccessResponse<Vec<ScanStatusResponse>>>, (StatusCode, Json<PyroErrorResponse>)> {
    // TODO: Query database with filters
    // TODO: Apply tenant_id filter
    // TODO: Implement pagination

    Ok(Json(PyroSuccessResponse::new(vec![])))
}

/// GET /api/v1/vulnscan/exploits - List available exploits
async fn list_exploits(
    claims: axum::Extension<JwtClaims>,
    Query(params): Query<ListExploitsQuery>,
) -> Result<Json<PyroSuccessResponse<Vec<ExploitMetadata>>>, (StatusCode, Json<PyroErrorResponse>)> {
    // TODO: Query database for exploits
    // TODO: Apply filters

    Ok(Json(PyroSuccessResponse::new(vec![])))
}

/// GET /api/v1/vulnscan/exploits/:exploit_id - Get detailed exploit information
async fn get_exploit(
    claims: axum::Extension<JwtClaims>,
    Path(exploit_id): Path<Uuid>,
) -> Result<Json<PyroSuccessResponse<ExploitMetadata>>, (StatusCode, Json<PyroErrorResponse>)> {
    // TODO: Query database for specific exploit

    Err((
        StatusCode::NOT_FOUND,
        Json(PyroErrorResponse::new(
            error_codes::EXPLOIT_NOT_FOUND,
            format!("Exploit with ID {} not found", exploit_id),
        )),
    ))
}

/// GET /api/v1/vulnscan/cryptex - Query cryptex dictionary
async fn query_cryptex(
    claims: axum::Extension<JwtClaims>,
    Query(params): Query<serde_json::Value>,
) -> Result<Json<PyroSuccessResponse<Vec<CryptexEntry>>>, (StatusCode, Json<PyroErrorResponse>)> {
    // TODO: Query cryptex dictionary
    // TODO: Support search by function_name, branding_name, category

    Ok(Json(PyroSuccessResponse::new(vec![])))
}
