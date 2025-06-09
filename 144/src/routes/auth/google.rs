use warp::{Filter, Reply, Rejection};
use serde::{Deserialize, Serialize};
use oauth2::{
    AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    AuthUrl, TokenUrl, basic::BasicClient, reqwest::async_http_client,
    TokenResponse,
};
use serde_json::json;
use crate::{
    util::{AppResult, AppError, id, qdrant::{qdrant_path, qdrant_post, qdrant_put}},
    constants::SECRETS,
};
use std::time::Instant;
use once_cell::sync::Lazy;
use std::sync::Once;

// Initialize Google auth setup once
static INIT: Lazy<Once> = Lazy::new(|| Once::new());

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: String,
}

#[derive(Debug, Deserialize)]
pub struct AuthCallback {
    pub code: String,
    pub state: String,
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // Ensure indexes exist
    INIT.call_once(|| {
        tokio::spawn(async {
            if let Err(e) = create_required_indexes().await {
                log::error!("Failed to create indexes for Google auth: {:#?}", e);
            } else {
                log::info!("Google auth indexes initialized successfully");
            }
        });
    });
    
    log::info!("Initializing Google auth routes");
    login_route()
        .or(callback_route())
}

pub fn login_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("auth" / "google" / "login")
        .and(warp::get())
        .then(google_login)
}

pub fn callback_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("auth" / "google" / "callback")
        .and(warp::get())
        .and(warp::query::<AuthCallback>())
        .then(google_callback)
}

pub async fn google_login() -> impl Reply {
    log::info!("Google login request received");
    let start_time = Instant::now();
    
    match create_auth_url().await {
        Ok(auth_url) => {
            let elapsed = start_time.elapsed();
            log::info!("Google auth URL created successfully in {:?}", elapsed);
            warp::reply::with_status(
                warp::reply::json(&json!({"auth_url": auth_url})),
                warp::http::StatusCode::OK,
            )
        }
        Err(e) => {
            let elapsed = start_time.elapsed();
            log::error!("Google login error after {:?}: {:#?}", elapsed, e);
            warp::reply::with_status(
                warp::reply::json(&json!({"error": "Failed to create auth URL"})),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

pub async fn google_callback(callback: AuthCallback) -> impl Reply {
    let state = &callback.state;
    // Log a masked version of the code for security
    let masked_code = if callback.code.len() > 8 {
        format!("{}...{}", &callback.code[0..4], &callback.code[callback.code.len()-4..])
    } else {
        "***".to_string()
    };
    
    log::info!("Google callback received with state: {}, code: {}", state, masked_code);
    let start_time = Instant::now();
    
    match handle_callback(callback).await {
        Ok(user_data) => {
            let user_id = user_data["user_id"].as_str().unwrap_or("unknown");
            let google_id = user_data["user"]["id"].as_str().unwrap_or("unknown");
            let email = user_data["user"]["email"].as_str().unwrap_or("unknown");
            
            // Mask email for privacy in logs
            let masked_email = if email.contains('@') {
                let parts: Vec<&str> = email.split('@').collect();
                if parts[0].len() > 2 {
                    format!("{}***{}@{}", &parts[0][0..1], &parts[0][parts[0].len()-1..], parts[1])
                } else {
                    "***@***".to_string()
                }
            } else {
                "***@***".to_string()
            };
            
            let elapsed = start_time.elapsed();
            log::info!(
                "Google auth successful for user_id: {}, google_id: {}***, email: {}, took: {:?}", 
                user_id,
                if google_id.len() > 6 { &google_id[0..6] } else { google_id },
                masked_email,
                elapsed
            );
            
            // Optimized verification using the index
            log::info!("Performing final verification");
            match qdrant_path("collections/i/points/scroll").await {
                Ok(path) => {
                    let verify_result = qdrant_post(
                        &path,
                        json!({
                            "filter": {
                                "must": [
                                    {
                                        "key": "google_id",
                                        "match": { "value": google_id }
                                    },
                                    {
                                        "key": "s",
                                        "match": { "value": "u" }
                                    }
                                ]
                            },
                            "with_payload": true,
                            "limit": 1
                        })
                    ).await;
                    
                    match verify_result {
                        Ok(result) => {
                            if let Some(points) = result["result"]["points"].as_array() {
                                if points.is_empty() {
                                    log::error!("VERIFICATION FAILED: User with Google ID {} not found!", 
                                              if google_id.len() > 6 { format!("{}***", &google_id[0..6]) } else { google_id.to_string() });
                                } else {
                                    log::info!("VERIFICATION SUCCESS: User found with Google ID");
                                }
                            }
                        },
                        Err(e) => {
                            log::error!("VERIFICATION ERROR: {}", e);
                            // Try fallback to ID query
                            log::info!("Trying verification by ID...");
                            let id_verify = qdrant_post(
                                &path,
                                json!({
                                    "filter": {
                                        "must": [
                                            {
                                                "key": "id",
                                                "match": { "value": user_id }
                                            }
                                        ]
                                    },
                                    "with_payload": true,
                                    "limit": 1
                                })
                            ).await;
                            
                            match id_verify {
                                Ok(id_result) => {
                                    if let Some(id_points) = id_result["result"]["points"].as_array() {
                                        if !id_points.is_empty() {
                                            log::info!("ID VERIFICATION SUCCESS: User found by ID");
                                        } else {
                                            log::error!("ID VERIFICATION FAILED: User not found by ID");
                                        }
                                    }
                                },
                                Err(e) => log::error!("ID verification error: {}", e)
                            }
                        }
                    }
                },
                Err(e) => log::error!("Failed to construct path for verification: {}", e)
            }
            
            warp::reply::with_status(
                warp::reply::json(&user_data),
                warp::http::StatusCode::OK,
            )
        }
        Err(e) => {
            let elapsed = start_time.elapsed();
            log::error!("Google callback error after {:?}: {:#?}", elapsed, e);
            
            // Enhanced error response with more context
            let mut error_response = json!({"error": "Authentication failed"});
            
            // Add some context about the error type without exposing sensitive details
            let error_str = e.to_string();
            if error_str.contains("database") || error_str.contains("qdrant") {
                error_response = json!({
                    "error": "Authentication failed due to database issue",
                    "code": "DB_ERROR"
                });
            } else if error_str.contains("token") || error_str.contains("exchange") {
                error_response = json!({
                    "error": "Authentication failed due to token validation issue",
                    "code": "TOKEN_ERROR"
                });
            } else if error_str.contains("Google") {
                error_response = json!({
                    "error": "Authentication failed due to Google API issue",
                    "code": "GOOGLE_API_ERROR"
                });
            }
            
            warp::reply::with_status(
                warp::reply::json(&error_response),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    }
}

async fn create_auth_url() -> AppResult<String> {
    log::debug!("Creating OAuth authorization URL");
    let start_time = Instant::now();
    
    let client = create_oauth_client().await?;
    
    log::debug!("OAuth client created in {:?}", start_time.elapsed());
    
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    log::debug!("Generated auth URL with CSRF token: {}", csrf_token.secret());
    
    Ok(auth_url.to_string())
}

async fn handle_callback(callback: AuthCallback) -> AppResult<serde_json::Value> {
    log::debug!("Handling OAuth callback");
    let start_time = Instant::now();
    
    // Exchange the code for a token
    log::debug!("Creating OAuth client for token exchange");
    let client = create_oauth_client().await?;
    
    log::debug!("Exchanging authorization code for token");
    let token_exchange_start = Instant::now();
    let token_result = client
        .exchange_code(AuthorizationCode::new(callback.code))
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            log::error!("Token exchange failed: {:?}", e);
            AppError::new("Failed to exchange code for token", e)
        })?;
    
    log::debug!("Token exchange completed in {:?}", token_exchange_start.elapsed());
    
    // Get user info from Google
    log::debug!("Fetching user info from Google");
    let user_info_start = Instant::now();
    let user_info = get_google_user_info(token_result.access_token().secret()).await?;
    
    // Mask sensitive info for logging
    let masked_id = if user_info.id.len() > 6 {
        format!("{}***", &user_info.id[0..6])
    } else {
        "***".to_string()
    };
    
    let masked_email = if user_info.email.contains('@') {
        let parts: Vec<&str> = user_info.email.split('@').collect();
        format!("{}***@{}", &parts[0][0..1], parts[1])
    } else {
        "***@***".to_string()
    };
    
    log::debug!(
        "Retrieved user info in {:?} - ID: {}, Email: {}, Name: {}",
        user_info_start.elapsed(),
        masked_id,
        masked_email,
        user_info.name
    );
    
    // Find or create user in Qdrant
    log::debug!("Finding or creating user in database");
    let db_start = Instant::now();
    let user_id = find_or_create_user(&user_info).await?;
    
    log::debug!("Database operation completed in {:?}", db_start.elapsed());
    
    // VERIFICATION: Query the database to verify the user exists
    log::info!("Verifying user was properly saved to database");
    let verify_start = Instant::now();
    
    // 1. Query specifically for this user by ID
    let user_query_result = qdrant_post(
        &qdrant_path("collections/i/points/scroll").await?,
        json!({
            "filter": {
                "must": [
                    {
                        "key": "s",
                        "match": { "value": "u" }
                    }
                ]
            },
            "with_payload": true,
            "with_vectors": false,
            "limit": 1,
            "scroll_filter": {
                "condition": {
                    "key": "id",
                    "match": { "value": user_id }
                }
            }
        })
    ).await;
    
    match user_query_result {
        Ok(result) => {
            if let Some(points) = result["result"]["points"].as_array() {
                if points.is_empty() {
                    log::error!("VERIFICATION FAILED: User with ID {} not found in database!", user_id);
                } else {
                    log::info!("VERIFICATION SUCCESS: User with ID {} found in database", user_id);
                    // Log the user data for verification
                    log::debug!("User data: {}", serde_json::to_string_pretty(&points[0]["payload"]).unwrap_or_default());
                }
            } else {
                log::error!("VERIFICATION FAILED: Unexpected response format from database");
            }
        },
        Err(e) => {
            log::error!("VERIFICATION ERROR: Failed to query user: {:#?}", e);
        }
    }
    
    // 2. Query for all users to diagnose potential issues
    let all_users_query = qdrant_post(
        &qdrant_path("collections/i/points/scroll").await?,
        json!({
            "filter": {
                "must": [
                    {
                        "key": "s",
                        "match": { "value": "u" }
                    }
                ]
            },
            "with_payload": false,
            "limit": 100
        })
    ).await;
    
    match all_users_query {
        Ok(result) => {
            if let Some(points) = result["result"]["points"].as_array() {
                log::info!("VERIFICATION: Found total of {} users in database", points.len());
                for (i, point) in points.iter().enumerate().take(10) {
                    if let Some(id) = point["id"].as_str() {
                        log::debug!("User #{}: ID {}", i+1, id);
                    }
                }
            } else {
                log::error!("VERIFICATION FAILED: Unexpected response format when querying all users");
            }
        },
        Err(e) => {
            log::error!("VERIFICATION ERROR: Failed to query all users: {:#?}", e);
        }
    }
    
    // 3. Query for all points in the collection to understand the data structure
    let all_points_query = qdrant_post(
        &qdrant_path("collections/i/points/scroll").await?,
        json!({
            "limit": 100,
            "with_payload": true
        })
    ).await;
    
    match all_points_query {
        Ok(result) => {
            if let Some(points) = result["result"]["points"].as_array() {
                log::info!("VERIFICATION: Found total of {} points in collection i", points.len());
                
                // Count points by tenant
                let mut tenant_counts = std::collections::HashMap::new();
                for point in points {
                    if let Some(tenant) = point["payload"]["s"].as_str() {
                        *tenant_counts.entry(tenant.to_string()).or_insert(0) += 1;
                    }
                }
                
                log::info!("VERIFICATION: Tenant distribution in collection:");
                for (tenant, count) in tenant_counts {
                    log::info!("  Tenant '{}': {} points", tenant, count);
                }
            } else {
                log::error!("VERIFICATION FAILED: Unexpected response format when querying all points");
            }
        },
        Err(e) => {
            log::error!("VERIFICATION ERROR: Failed to query all points: {:#?}", e);
        }
    }
    
    log::info!("Verification completed in {:?}", verify_start.elapsed());
    log::info!("Complete OAuth flow processed in {:?}", start_time.elapsed());
    
    Ok(json!({
        "user_id": user_id,
        "user": user_info
    }))
}

async fn create_oauth_client() -> AppResult<BasicClient> {
    log::debug!("Creating OAuth client");
    let start_time = Instant::now();
    
    let secrets = SECRETS.lock().await;
    
    let client_id = secrets
        .get("GOOGLE_CLIENT_ID")
        .ok_or_else(|| {
            log::error!("GOOGLE_CLIENT_ID not found in secrets");
            AppError::new_plain("GOOGLE_CLIENT_ID not found in secrets")
        })?;
    
    let client_secret = secrets
        .get("GOOGLE_CLIENT_SECRET")
        .ok_or_else(|| {
            log::error!("GOOGLE_CLIENT_SECRET not found in secrets");
            AppError::new_plain("GOOGLE_CLIENT_SECRET not found in secrets")
        })?;
    
    let redirect_url = secrets
        .get("GOOGLE_REDIRECT_URL")
        .ok_or_else(|| {
            log::error!("GOOGLE_REDIRECT_URL not found in secrets");
            AppError::new_plain("GOOGLE_REDIRECT_URL not found in secrets")
        })?;

    log::debug!("Using redirect URL: {}", redirect_url);

    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .map_err(|e| {
            log::error!("Invalid auth URL: {:?}", e);
            AppError::new("Invalid auth URL", e)
        })?;
    
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v4/token".to_string())
        .map_err(|e| {
            log::error!("Invalid token URL: {:?}", e);
            AppError::new("Invalid token URL", e)
        })?;

    log::debug!("OAuth client created in {:?}", start_time.elapsed());

    Ok(BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url)
            .map_err(|e| {
                log::error!("Invalid redirect URL: {:?}", e);
                AppError::new("Invalid redirect URL", e)
            })?,
    ))
}

async fn get_google_user_info(access_token: &str) -> AppResult<GoogleUser> {
    log::debug!("Fetching user info from Google API");
    let start_time = Instant::now();
    
    let response = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| {
            log::error!("Google API request failed: {:?}", e);
            AppError::new("Failed to get user info from Google", e)
        })?;

    let status = response.status();
    log::debug!("Google API response status: {}", status);
    
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        log::error!("Google API error: {} - {}", status, error_text);
        return Err(AppError::new_plain(&format!(
            "Failed to get user info from Google: Status {}", status
        )));
    }

    let user_info: GoogleUser = response
        .json()
        .await
        .map_err(|e| {
            log::error!("Failed to parse Google user info: {:?}", e);
            AppError::new("Failed to parse Google user info", e)
        })?;

    log::debug!(
        "Successfully retrieved user info in {:?} - Email: {}, Name: {}",
        start_time.elapsed(),
        if user_info.email.contains('@') { 
            format!("{}***@{}", 
                &user_info.email.split('@').next().unwrap_or("")[0..1],
                user_info.email.split('@').nth(1).unwrap_or("***")
            )
        } else { 
            "***@***".to_string() 
        },
        user_info.name
    );

    Ok(user_info)
}

// Function to find existing user by Google ID
async fn find_user_by_google_id(google_id: &str) -> AppResult<Option<String>> {
    let masked_id = if google_id.len() > 6 {
        format!("{}***", &google_id[0..6])
    } else {
        "***".to_string()
    };
    
    log::debug!("Searching for existing user with Google ID: {}", masked_id);
    let start_time = Instant::now();
    
    // Build the query using the index
    let query = json!({
        "filter": {
            "must": [
                {
                    "key": "google_id",
                    "match": {
                        "value": google_id
                    }
                },
                {
                    "key": "s",
                    "match": {
                        "value": "u"  // tenant id for users
                    }
                }
            ]
        },
        "limit": 1,
        "with_payload": true
    });
    
    log::debug!("Query for user: {}", serde_json::to_string_pretty(&query).unwrap_or_default());
    
    // Query Qdrant to find user with matching Google ID
    let query_path = match qdrant_path("collections/i/points/scroll").await {
        Ok(path) => path,
        Err(e) => {
            log::error!("Failed to construct Qdrant path: {:#?}", e);
            return Err(AppError::new("Failed to construct Qdrant path", e));
        }
    };
    
    log::debug!("Executing query to path: {}", query_path);
    
    let search_result = qdrant_post(&query_path, query)
        .await
        .map_err(|e| {
            log::error!("Database search error for Google ID {}: {:#?}", masked_id, e);
            AppError::new(&format!("Failed to search for user with Google ID: {}", masked_id), e)
        })?;
    
    log::debug!("Database search completed in {:?}", start_time.elapsed());
    
    // Debug log the response structure
    if log::log_enabled!(log::Level::Debug) {
        let response_keys = if let Some(obj) = search_result.as_object() {
            obj.keys().map(|k| k.to_string()).collect::<Vec<String>>().join(", ")
        } else {
            "not an object".to_string()
        };
        
        log::debug!("Response top-level keys: {}", response_keys);
        
        if let Some(result) = search_result.get("result") {
            let result_keys = if let Some(obj) = result.as_object() {
                obj.keys().map(|k| k.to_string()).collect::<Vec<String>>().join(", ")
            } else {
                "not an object".to_string()
            };
            log::debug!("Response 'result' keys: {}", result_keys);
        }
    }
    
    // Check if we found a user
    if let Some(points) = search_result.get("result").and_then(|r| r.get("points")).and_then(|p| p.as_array()) {
        log::debug!("Search returned {} points", points.len());
        
        if !points.is_empty() {
            // Extract the user ID of the existing user
            if let Some(user_id) = points[0].get("id").and_then(|id| id.as_str()) {
                let found_id = user_id.to_string();
                log::info!("Found existing user: {} for Google ID: {}", found_id, masked_id);
                
                // Log payload details
                if let Some(payload) = points[0].get("payload") {
                    log::debug!("User payload: {}", serde_json::to_string_pretty(payload).unwrap_or_default());
                }
                
                return Ok(Some(found_id));
            } else {
                log::error!("Invalid user ID format in response");
                return Err(AppError::new_plain("Invalid user ID format in response"));
            }
        }
    } else if !search_result.get("result").is_some() {
        log::error!("Unexpected response format: 'result' field missing");
        log::debug!("Raw response: {}", serde_json::to_string(&search_result).unwrap_or_default());
        return Err(AppError::new_plain("Unexpected response format from database"));
    }
    
    log::debug!("No existing user found for Google ID: {}", masked_id);
    // No user found
    Ok(None)
}

// Renamed from store_user to create_user to be more explicit
async fn create_user(user_info: &GoogleUser) -> AppResult<String> {
    let masked_id = if user_info.id.len() > 6 {
        format!("{}***", &user_info.id[0..6])
    } else {
        "***".to_string()
    };
    
    let masked_email = if user_info.email.contains('@') {
        let parts: Vec<&str> = user_info.email.split('@').collect();
        format!("{}***@{}", &parts[0][0..1], parts[1])
    } else {
        "***@***".to_string()
    };
    
    log::info!("Creating new user - Google ID: {}, Email: {}, Name: {}", 
               masked_id, masked_email, user_info.name);
    let start_time = Instant::now();
    
    let user_id = id();
    log::debug!("Generated new user ID: {}", user_id);
    
    // Create a dummy vector with correct dimensions (768 for Gemini embeddings)
    let dummy_vector: Vec<f32> = vec![0.0; 768];
    
    // Prepare username from email
    let username = user_info.email.split('@').next().unwrap_or(&user_info.email);
    
    log::debug!("Storing user in database with payload - Name: {}, Username: {}", 
               user_info.name, username);
    
    // Prepare the user point payload - Include ids field as required by Qdrant API
    let request_body = json!({
        "points": [
            {
                "id": user_id,
                "vector": dummy_vector,
                "payload": {
                    "google_id": user_info.id,
                    "email": user_info.email,
                    "n": user_info.name,  // name
                    "i": [user_info.picture],  // images
                    "t": "",  // description (empty for now)
                    "l": "",  // location (empty for now)
                    "u": username,  // username from email
                    "z": null,  // zone (none initially)
                    "p": {"lat": 0.0, "lng": 0.0},  // position (default for now)
                    "s": "u"  // tenant id for users
                }
            }
        ]
    });
    
    // Log the exact payload being sent (with sensitive info masked)
    let debug_payload = json!({
        "ids": [user_id],
        "points": [
            {
                "id": user_id,
                "vector": "... dummy vector ...",
                "payload": {
                    "google_id": masked_id,
                    "email": masked_email,
                    "n": user_info.name,
                    "i": [user_info.picture],
                    "t": "",
                    "l": "",
                    "u": username,
                    "z": null,
                    "p": {"lat": 0.0, "lng": 0.0},
                    "s": "u"
                }
            }
        ]
    });
    
    log::debug!("User point payload: {}", serde_json::to_string_pretty(&debug_payload).unwrap_or_default());
    
    // Perform the database operation with enhanced error handling
    let db_operation_start = Instant::now();
    
    let result = qdrant_put(
        &qdrant_path("collections/i/points?wait=true").await?,
        request_body
    ).await;

    log::debug!("---add user result: {:#?}", result);    
    
    match &result {
        Ok(response) => {
            log::debug!("Qdrant insert response: {}", serde_json::to_string(response).unwrap_or_default());
            log::info!("User successfully inserted into database");
        },
        Err(e) => {
            log::error!("CRITICAL: Failed to store user in database: {:#?}", e);
            log::error!("Error context: User ID: {}, Google ID: {}", user_id, masked_id);
            
            // Check if the collection exists
            log::debug!("Checking if collection 'i' exists...");
            match qdrant_post(
                &qdrant_path("collections").await?,
                json!({})
            ).await {
                Ok(collections) => {
                    log::debug!("Collections response: {}", serde_json::to_string(&collections).unwrap_or_default());
                },
                Err(e) => {
                    log::error!("Failed to check collections: {:#?}", e);
                }
            }
        }
    }
    
    // Propagate any error from the database operation
    result.map_err(|e| {
        AppError::new("Failed to store user in database", e)
    })?;
    
    log::debug!("Database operation completed in {:?}", db_operation_start.elapsed());
    log::info!("User created successfully in {:?} - ID: {}", start_time.elapsed(), user_id);

    // Double-check the user was actually saved
    log::debug!("Verifying user was saved correctly...");
    let verify_start = Instant::now();
    
    match find_user_by_google_id(&user_info.id).await {
        Ok(Some(found_id)) => {
            if found_id == user_id {
                log::info!("User verification successful - User ID: {}", user_id);
            } else {
                log::warn!("User verification found different ID: {} (expected: {})", found_id, user_id);
            }
        },
        Ok(None) => {
            log::error!("CRITICAL: User verification failed - User with Google ID {} not found after creation!", masked_id);
        },
        Err(e) => {
            log::error!("User verification error: {:#?}", e);
        }
    }
    
    log::debug!("Verification completed in {:?}", verify_start.elapsed());
    
    Ok(user_id)
}

// Function to find an existing user or create a new one
async fn find_or_create_user(user_info: &GoogleUser) -> AppResult<String> {
    let masked_id = if user_info.id.len() > 6 {
        format!("{}***", &user_info.id[0..6])
    } else {
        "***".to_string()
    };
    
    log::info!("Finding or creating user for Google ID: {}", masked_id);
    let start_time = Instant::now();
    
    // Try to find existing user by Google ID with enhanced error handling
    let find_result = find_user_by_google_id(&user_info.id).await;
    
    match find_result {
        Ok(Some(existing_user_id)) => {
            let elapsed = start_time.elapsed();
            log::info!("Using existing user: {} for Google ID: {} (found in {:?})",
                     existing_user_id, masked_id, elapsed);
            
            // Update the user's last login time (could be extended for other fields)
            log::debug!("Updating user's last login timestamp");
            
            // Add wait=true for the update operation
            let update_path = qdrant_path(&format!("collections/i/points/{}/payload?wait=true", existing_user_id)).await?;
            let update_result = qdrant_post(
                &update_path,
                json!({
                    "payload": {
                        "last_login": chrono::Utc::now().to_rfc3339()
                    }
                })
            ).await;
            
            if let Err(e) = update_result {
                log::warn!("Failed to update user's last login timestamp: {:#?}", e);
                // Continue anyway as this is not critical
            }
            
            return Ok(existing_user_id);
        },
        Ok(None) => {
            log::info!("No existing user found for Google ID: {} - creating new user", masked_id);
        },
        Err(e) => {
            log::error!("Error while searching for existing user: {:#?}", e);
            // Remove reference to ensure_indexes and simply continue
            log::info!("Continuing to user creation despite search error");
        }
    }
    
    // No existing user found, create a new one with enhanced error handling
    let create_result = create_user(user_info).await;
    
    match &create_result {
        Ok(user_id) => {
            let elapsed = start_time.elapsed();
            log::info!("Created new user: {} for Google ID: {} (took {:?})",
                     user_id, masked_id, elapsed);
        },
        Err(e) => {
            log::error!("Failed to create new user for Google ID {}: {:#?}", masked_id, e);
            
            // If create failed, do one final check to see if the user actually exists
            // This handles race conditions where the user might have been created by another request
            log::info!("Final check if user exists despite creation error");
            match find_user_by_google_id(&user_info.id).await {
                Ok(Some(existing_user_id)) => {
                    log::warn!("User actually exists despite creation error! ID: {}", existing_user_id);
                    return Ok(existing_user_id);
                },
                _ => {
                    // User truly doesn't exist, propagate the original error
                    return Err(AppError::new("Failed to create user", e.clone()));
                }
            }
        }
    }
    
    create_result
}

// Create required indexes for Qdrant
async fn create_required_indexes() -> AppResult<()> {
    log::info!("Creating required indexes for Google authentication");
    let start_time = Instant::now();
    
    // Check if collection exists
    let collection_check = qdrant_post(
        &qdrant_path("collections/i").await?,
        json!({})
    ).await;
    
    match collection_check {
        Ok(_) => log::info!("Collection 'i' exists"),
        Err(e) => {
            log::info!("Collection 'i' doesn't exist or error checking: {}", e);
            
            // Create collection if it doesn't exist - add wait=true
            let create_result = qdrant_put(
                &qdrant_path("collections/i?wait=true").await?,
                json!({
                    "vectors": {
                        "size": 768,
                        "distance": "Cosine"
                    }
                })
            ).await;
            
            match create_result {
                Ok(_) => log::info!("Created collection 'i'"),
                Err(e) => {
                    log::error!("Failed to create collection: {}", e);
                    // Continue anyway to try creating indexes
                }
            }
        }
    }
    
    // Create index for google_id - add wait=true
    log::info!("Creating index for google_id");
    let google_id_result = qdrant_put(
        &qdrant_path("collections/i/index?wait=true").await?,
        json!({
            "field_name": "google_id",
            "field_schema": "keyword"
        })
    ).await;
    
    match google_id_result {
        Ok(_) => log::info!("Successfully created google_id index"),
        Err(e) => {
            if e.to_string().contains("already exists") {
                log::info!("Index for google_id already exists");
            } else {
                log::error!("Failed to create google_id index: {}", e);
                // Continue to create other indexes
            }
        }
    }
    
    // Create index for s (tenant) - add wait=true
    log::info!("Creating index for tenant field 's'");
    let tenant_result = qdrant_put(
        &qdrant_path("collections/i/index?wait=true").await?,
        json!({
            "field_name": "s",
            "field_schema": "keyword"
        })
    ).await;
    
    match tenant_result {
        Ok(_) => log::info!("Successfully created tenant 's' index"),
        Err(e) => {
            if e.to_string().contains("already exists") {
                log::info!("Index for tenant 's' already exists");
            } else {
                log::error!("Failed to create tenant index: {}", e);
                // Continue to create other indexes
            }
        }
    }
    
    // Create index for id field - add wait=true
    log::info!("Creating index for id field");
    let id_result = qdrant_put(
        &qdrant_path("collections/i/index?wait=true").await?,
        json!({
            "field_name": "id",
            "field_schema": "keyword"
        })
    ).await;
    
    match id_result {
        Ok(_) => log::info!("Successfully created id index"),
        Err(e) => {
            if e.to_string().contains("already exists") {
                log::info!("Index for id already exists");
            } else {
                log::error!("Failed to create id index: {}", e);
            }
        }
    }
    
    log::info!("Index creation completed in {:?}", start_time.elapsed());
    Ok(())
} 