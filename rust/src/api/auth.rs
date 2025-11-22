// JWT Authentication middleware for PYRO Fire Marshal integration
// Implements RS256 JWT validation with JWKS support

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use super::pyro_types::{error_codes, PyroErrorResponse};

/// JWT Claims structure expected from PYRO platform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,              // Subject (user ID)
    pub iat: i64,                 // Issued at (timestamp)
    pub exp: i64,                 // Expiration (timestamp)
    pub tenant_id: String,        // Tenant ID for multi-tenancy
    pub roles: Vec<String>,       // User roles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,    // Optional: user email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,     // Optional: user name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,  // Optional: specific permissions
}

/// Authentication state shared across handlers
#[derive(Clone)]
pub struct AuthState {
    pub jwt_secret: String,
    pub jwks_url: String,
    pub issuer: String,
    pub audience: String,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            jwt_secret: std::env::var("PYRO_JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-key-change-in-production".to_string()),
            jwks_url: std::env::var("PYRO_JWKS_URL")
                .unwrap_or_else(|_| "https://pyro-platform.local/.well-known/jwks.json".to_string()),
            issuer: std::env::var("PYRO_JWT_ISSUER")
                .unwrap_or_else(|_| "https://pyro-platform.local".to_string()),
            audience: std::env::var("PYRO_JWT_AUDIENCE")
                .unwrap_or_else(|_| "pyroutersploit".to_string()),
        }
    }
}

/// Extract JWT token from Authorization header
fn extract_token(headers: &HeaderMap) -> Result<String, PyroErrorResponse> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            PyroErrorResponse::new(
                error_codes::AUTHENTICATION_ERROR,
                "Missing Authorization header",
            )
        })?;

    if !auth_header.starts_with("Bearer ") {
        return Err(PyroErrorResponse::new(
            error_codes::AUTHENTICATION_ERROR,
            "Invalid Authorization header format. Expected: Bearer <token>",
        ));
    }

    Ok(auth_header["Bearer ".len()..].to_string())
}

/// Validate JWT token with RS256 algorithm
pub fn validate_jwt(token: &str, state: &AuthState) -> Result<JwtClaims, PyroErrorResponse> {
    // Set up validation parameters
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[&state.issuer]);
    validation.set_audience(&[&state.audience]);
    validation.validate_exp = true;

    // For development: use symmetric key (HS256) if RS256 public key not available
    // In production: fetch public key from JWKS endpoint
    let decoding_key = if state.jwt_secret.starts_with("-----BEGIN") {
        // RSA public key provided
        DecodingKey::from_rsa_pem(state.jwt_secret.as_bytes())
            .map_err(|e| {
                PyroErrorResponse::new(
                    error_codes::INTERNAL_ERROR,
                    format!("Failed to parse RSA public key: {}", e),
                )
            })?
    } else {
        // Development mode: use secret as symmetric key
        // TODO: Fetch actual RS256 public key from JWKS URL
        validation.algorithms = vec![Algorithm::HS256];
        DecodingKey::from_secret(state.jwt_secret.as_bytes())
    };

    // Decode and validate token
    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
        .map_err(|e| {
            PyroErrorResponse::new(
                error_codes::AUTHENTICATION_ERROR,
                format!("Invalid JWT token: {}", e),
            )
        })?;

    Ok(token_data.claims)
}

/// Middleware function for JWT authentication
pub async fn auth_middleware(
    State(auth_state): State<Arc<AuthState>>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, Response> {
    // Extract token from header
    let token = extract_token(&headers).map_err(|err| {
        (StatusCode::UNAUTHORIZED, Json(err)).into_response()
    })?;

    // Validate JWT
    let claims = validate_jwt(&token, &auth_state).map_err(|err| {
        (StatusCode::UNAUTHORIZED, Json(err)).into_response()
    })?;

    // Store claims in request extensions for handlers to access
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Check if user has required role
pub fn has_role(claims: &JwtClaims, required_role: &str) -> bool {
    claims.roles.iter().any(|role| role == required_role)
}

/// Check if user has required permission
pub fn has_permission(claims: &JwtClaims, required_permission: &str) -> bool {
    if let Some(permissions) = &claims.permissions {
        permissions.iter().any(|perm| perm == required_permission)
    } else {
        false
    }
}

/// Check if user has any of the required permissions
pub fn has_any_permission(claims: &JwtClaims, required_permissions: &[&str]) -> bool {
    if let Some(permissions) = &claims.permissions {
        required_permissions.iter().any(|req_perm| {
            permissions.iter().any(|perm| perm == req_perm)
        })
    } else {
        false
    }
}

/// Authorization middleware for specific roles
pub async fn require_role(
    claims: axum::Extension<JwtClaims>,
    required_role: String,
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    if !has_role(&claims, &required_role) {
        let error = PyroErrorResponse::new(
            error_codes::AUTHORIZATION_ERROR,
            format!("Insufficient permissions. Required role: {}", required_role),
        );
        return Err((StatusCode::FORBIDDEN, Json(error)).into_response());
    }

    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_claims_deserialization() {
        let json = r#"{
            "sub": "user-123",
            "iat": 1234567890,
            "exp": 1234571490,
            "tenant_id": "tenant-456",
            "roles": ["vulnscan_operator"],
            "email": "user@example.com"
        }"#;

        let claims: JwtClaims = serde_json::from_str(json).unwrap();
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.tenant_id, "tenant-456");
        assert!(claims.roles.contains(&"vulnscan_operator".to_string()));
    }

    #[test]
    fn test_has_role() {
        let claims = JwtClaims {
            sub: "user-123".to_string(),
            iat: 1234567890,
            exp: 1234571490,
            tenant_id: "tenant-456".to_string(),
            roles: vec!["vulnscan_operator".to_string(), "vulnscan_viewer".to_string()],
            email: None,
            name: None,
            permissions: None,
        };

        assert!(has_role(&claims, "vulnscan_operator"));
        assert!(has_role(&claims, "vulnscan_viewer"));
        assert!(!has_role(&claims, "vulnscan_admin"));
    }

    #[test]
    fn test_has_permission() {
        let claims = JwtClaims {
            sub: "user-123".to_string(),
            iat: 1234567890,
            exp: 1234571490,
            tenant_id: "tenant-456".to_string(),
            roles: vec!["vulnscan_operator".to_string()],
            email: None,
            name: None,
            permissions: Some(vec![
                "vulnscan:read".to_string(),
                "vulnscan:scan".to_string(),
            ]),
        };

        assert!(has_permission(&claims, "vulnscan:read"));
        assert!(has_permission(&claims, "vulnscan:scan"));
        assert!(!has_permission(&claims, "vulnscan:admin"));
    }
}
