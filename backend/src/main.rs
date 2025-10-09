use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

mod auth;
use auth::Auth0Config;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Auth0 config
    let auth_config = Auth0Config::from_env();

    // Build our application with routes
    let app = Router::new()
        .route("/", get(home_page))
        .route("/health", get(health_json))
        .route("/login", get(login))
        .route("/callback", get(callback))
        .layer(CorsLayer::permissive())
        .with_state((pool, auth_config));

    // Get port from environment or default to 8080
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Protettorato backend listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Home page with proper HTML
async fn home_page() -> impl IntoResponse {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Protettorato - Your Wisdom, Available 24/7</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }
        .container {
            background: white;
            border-radius: 20px;
            padding: 60px 40px;
            max-width: 600px;
            text-align: center;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
        }
        h1 {
            font-size: 2.5em;
            color: #111827;
            margin-bottom: 10px;
        }
        .tagline {
            font-size: 1.2em;
            color: #6B7280;
            margin-bottom: 40px;
        }
        .cta-button {
            display: inline-block;
            background: #2563EB;
            color: white;
            padding: 18px 48px;
            border-radius: 12px;
            text-decoration: none;
            font-size: 1.1em;
            font-weight: 600;
            transition: all 0.3s ease;
            box-shadow: 0 4px 14px rgba(37, 99, 235, 0.4);
        }
        .cta-button:hover {
            background: #1d4ed8;
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(37, 99, 235, 0.6);
        }
        .features {
            margin-top: 50px;
            display: grid;
            grid-template-columns: 1fr;
            gap: 20px;
            text-align: left;
        }
        .feature {
            padding: 20px;
            background: #F9FAFB;
            border-radius: 10px;
        }
        .feature-title {
            font-weight: 600;
            color: #111827;
            margin-bottom: 8px;
        }
        .feature-desc {
            color: #6B7280;
            font-size: 0.95em;
        }
        @media (max-width: 640px) {
            .container {
                padding: 40px 20px;
            }
            h1 {
                font-size: 2em;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🛡️ Protettorato v2.0</h1>
        <p class="tagline">Your wisdom, available 24/7</p>
        <p style="color: #6B7280; margin-bottom: 30px;">
            Transform your expertise into an AI assistant for your team in minutes.
        </p>
        
        <a href="/login" class="cta-button">Start with Google</a>
        
        <div class="features">
            <div class="feature">
                <div class="feature-title">📚 Knowledge Transfer</div>
                <div class="feature-desc">Share your expertise through natural conversation</div>
            </div>
            <div class="feature">
                <div class="feature-title">🤖 AI-Powered Bot</div>
                <div class="feature-desc">Deploy your digital twin on Line in minutes</div>
            </div>
            <div class="feature">
                <div class="feature-title">⚡ 24/7 Availability</div>
                <div class="feature-desc">Your team gets answers anytime, anywhere</div>
            </div>
        </div>
        
        <p style="margin-top: 40px; color: #9CA3AF; font-size: 0.9em;">
            Status: <a href="/health" style="color: #2563EB;">System Online</a>
        </p>
    </div>
</body>
</html>
    "#;

    (StatusCode::OK, Html(html))
}

// Health check as JSON
async fn health_json() -> impl IntoResponse {
    let response = HealthResponse {
        status: "ok".to_string(),
        service: "protettorato".to_string(),
        version: "0.1.0".to_string(),
    };
    
    (StatusCode::OK, axum::Json(response))
}

// Login handler
async fn login(
    State((_, auth_config)): State<(sqlx::PgPool, Auth0Config)>
) -> impl IntoResponse {
    Redirect::to(&auth_config.auth_url())
}

// Callback handler
async fn callback() -> impl IntoResponse {
    let html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Login Successful</title>
    <style>
        body {
            font-family: 'Inter', sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .success-box {
            background: white;
            padding: 60px;
            border-radius: 20px;
            text-align: center;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
        }
        h1 {
            color: #10B981;
            font-size: 2em;
            margin-bottom: 20px;
        }
        a {
            display: inline-block;
            margin-top: 30px;
            color: #2563EB;
            text-decoration: none;
            font-weight: 600;
        }
    </style>
</head>
<body>
    <div class="success-box">
        <h1>✅ Login Successful!</h1>
        <p>Welcome to Protettorato</p>
        <a href="/">← Go Home</a>
    </div>
</body>
</html>
    "#;

    (StatusCode::OK, Html(html))
}