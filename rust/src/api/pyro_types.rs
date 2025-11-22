// PYRO Fire Marshal API response types
// Implements standardized response format: {status, data, meta}

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Standard PYRO API response wrapper for successful responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyroSuccessResponse<T> {
    pub status: String,  // Always "success"
    pub data: T,
    pub meta: ResponseMeta,
}

/// Standard PYRO API response wrapper for error responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyroErrorResponse {
    pub status: String,  // Always "error"
    pub error: ErrorDetail,
    pub meta: ResponseMeta,
}

/// Error detail structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Response metadata included in all responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMeta {
    pub timestamp: DateTime<Utc>,
    pub request_id: Uuid,
    pub version: String,
}

impl ResponseMeta {
    /// Create new response metadata with current timestamp and generated request ID
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            request_id: Uuid::new_v4(),
            version: "v1".to_string(),
        }
    }

    /// Create response metadata with specific request ID (for request tracing)
    pub fn with_request_id(request_id: Uuid) -> Self {
        Self {
            timestamp: Utc::now(),
            request_id,
            version: "v1".to_string(),
        }
    }
}

impl Default for ResponseMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> PyroSuccessResponse<T> {
    /// Create a new success response with data
    pub fn new(data: T) -> Self {
        Self {
            status: "success".to_string(),
            data,
            meta: ResponseMeta::new(),
        }
    }

    /// Create a success response with specific request ID
    pub fn with_request_id(data: T, request_id: Uuid) -> Self {
        Self {
            status: "success".to_string(),
            data,
            meta: ResponseMeta::with_request_id(request_id),
        }
    }
}

impl PyroErrorResponse {
    /// Create a new error response
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            status: "error".to_string(),
            error: ErrorDetail {
                code: code.into(),
                message: message.into(),
                details: None,
            },
            meta: ResponseMeta::new(),
        }
    }

    /// Create error response with details
    pub fn with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: serde_json::Value,
    ) -> Self {
        Self {
            status: "error".to_string(),
            error: ErrorDetail {
                code: code.into(),
                message: message.into(),
                details: Some(details),
            },
            meta: ResponseMeta::new(),
        }
    }

    /// Create error response with specific request ID
    pub fn with_request_id(
        code: impl Into<String>,
        message: impl Into<String>,
        request_id: Uuid,
    ) -> Self {
        Self {
            status: "error".to_string(),
            error: ErrorDetail {
                code: code.into(),
                message: message.into(),
                details: None,
            },
            meta: ResponseMeta::with_request_id(request_id),
        }
    }
}

/// Standard error codes used in PYRO Fire Marshal
pub mod error_codes {
    pub const VALIDATION_ERROR: &str = "VALIDATION_ERROR";
    pub const AUTHENTICATION_ERROR: &str = "AUTHENTICATION_ERROR";
    pub const AUTHORIZATION_ERROR: &str = "AUTHORIZATION_ERROR";
    pub const NOT_FOUND: &str = "NOT_FOUND";
    pub const INTERNAL_ERROR: &str = "INTERNAL_ERROR";
    pub const RATE_LIMIT_EXCEEDED: &str = "RATE_LIMIT_EXCEEDED";
    pub const SCAN_IN_PROGRESS: &str = "SCAN_IN_PROGRESS";
    pub const SCAN_FAILED: &str = "SCAN_FAILED";
    pub const INVALID_TARGET: &str = "INVALID_TARGET";
    pub const EXPLOIT_NOT_FOUND: &str = "EXPLOIT_NOT_FOUND";
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_success_response_serialization() {
        let response = PyroSuccessResponse::new(json!({
            "scan_id": "test-123",
            "status": "completed"
        }));

        assert_eq!(response.status, "success");
        assert_eq!(response.meta.version, "v1");

        let json = serde_json::to_value(&response).unwrap();
        assert!(json.get("status").is_some());
        assert!(json.get("data").is_some());
        assert!(json.get("meta").is_some());
    }

    #[test]
    fn test_error_response_serialization() {
        let response = PyroErrorResponse::new(
            error_codes::VALIDATION_ERROR,
            "Invalid target range"
        );

        assert_eq!(response.status, "error");
        assert_eq!(response.error.code, error_codes::VALIDATION_ERROR);
        assert_eq!(response.error.message, "Invalid target range");

        let json = serde_json::to_value(&response).unwrap();
        assert!(json.get("status").is_some());
        assert!(json.get("error").is_some());
        assert!(json.get("meta").is_some());
    }

    #[test]
    fn test_error_response_with_details() {
        let details = json!({
            "field": "target_range",
            "provided": "invalid-ip",
            "expected": "valid CIDR or IP address"
        });

        let response = PyroErrorResponse::with_details(
            error_codes::VALIDATION_ERROR,
            "Invalid target range",
            details.clone()
        );

        assert!(response.error.details.is_some());
        assert_eq!(response.error.details.unwrap(), details);
    }
}
