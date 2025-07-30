use crate::error::Error;
use axum::extract::Path;
use axum::extract::State;
use axum::http::header;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use notify::Config;
use notify::RecommendedWatcher;
use notify::RecursiveMode;
use notify::Watcher;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::SystemTime;
use tokio::fs;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[derive(Debug, Clone, Serialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub modified: SystemTime,
    pub size: u64,
    pub file_type: FileType,
}

#[derive(Debug, Clone, Serialize)]
pub enum FileType {
    Dashboard,
    Log,
}

#[derive(Clone)]
struct AppState {
    files: Arc<Mutex<HashMap<String, FileInfo>>>,
    base_path: PathBuf,
}

impl AppState {
    fn new(base_path: PathBuf) -> Self {
        Self {
            files: Arc::new(Mutex::new(HashMap::new())),
            base_path,
        }
    }

    fn update_files(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut files = self.files.lock().unwrap();
        files.clear();

        if !self.base_path.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Some(file_type) = determine_file_type(&path) {
                        let metadata = entry.metadata()?;

                        let info = FileInfo {
                            name: name.to_string(),
                            path: path.to_string_lossy().to_string(),
                            modified: metadata.modified()?,
                            size: metadata.len(),
                            file_type,
                        };
                        files.insert(name.to_string(), info);
                    }
                }
            }
        }

        Ok(())
    }

    fn get_files(&self) -> Vec<FileInfo> {
        let files = self.files.lock().unwrap();
        let mut sorted: Vec<_> = files.values().cloned().collect();
        sorted.sort_by(|a, b| b.modified.cmp(&a.modified));
        sorted
    }
}

fn determine_file_type(path: &std::path::Path) -> Option<FileType> {
    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
        match extension.to_lowercase().as_str() {
            "html" | "htm" => Some(FileType::Dashboard),
            "log" => Some(FileType::Log),
            _ => None,
        }
    } else {
        None
    }
}

async fn file_list(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let _ = state.update_files();
    let files = state.get_files();

    let html = generate_file_list_html(files);
    Ok(Html(html))
}

async fn serve_file(
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> Result<Response, StatusCode> {
    let file_path = state.base_path.join(&filename);

    if !file_path.exists() || !file_path.is_file() {
        return Err(StatusCode::NOT_FOUND);
    }

    if let Some(file_type) = determine_file_type(&file_path) {
        match fs::read_to_string(&file_path).await {
            Ok(content) => {
                let (content_type, formatted_content) = match file_type {
                    FileType::Dashboard => ("text/html; charset=utf-8", content),
                    FileType::Log => (
                        "text/html; charset=utf-8",
                        format_log_as_html(&content, &filename),
                    ),
                };

                let response = Response::builder()
                    .header(header::CONTENT_TYPE, content_type)
                    .body(formatted_content.into())
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                Ok(response)
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

fn generate_file_list_html(files: Vec<FileInfo>) -> String {
    // Load the template
    let template = include_str!("server/dashboard_list_template.html");

    // Separate files by type
    let mut dashboards: Vec<&FileInfo> = files
        .iter()
        .filter(|f| matches!(f.file_type, FileType::Dashboard))
        .collect();
    let mut logs: Vec<&FileInfo> = files
        .iter()
        .filter(|f| matches!(f.file_type, FileType::Log))
        .collect();

    // Sort each type by modified time
    dashboards.sort_by(|a, b| b.modified.cmp(&a.modified));
    logs.sort_by(|a, b| b.modified.cmp(&a.modified));

    let sections = if files.is_empty() {
        r#"
        <div class="no-dashboards">
            <div class="no-dashboards-icon">📊</div>
            <h3>No Files Available</h3>
            <p>No fuzzing files found in the monitored directory.</p>
            <p style="font-size: 0.9em; margin-top: 8px;">Start fuzzing to see results here.</p>
        </div>
        "#
        .to_string()
    } else {
        let mut sections = String::new();

        // Dashboard section
        if !dashboards.is_empty() {
            sections.push_str(&format!(
                r#"
                <div class="section-header">
                    <h2>📊 Fuzzing Dashboards <span class="count">({} files)</span></h2>
                </div>
                <div class="dashboards-grid">
                "#,
                dashboards.len()
            ));

            for dashboard in dashboards {
                let time_str = format_time(dashboard.modified);
                sections.push_str(&format!(
                    r#"
                    <div class="file-item dashboard-type">
                        <div class="file-header">
                            <h3 class="file-title">📊 {}</h3>
                            <div class="file-meta">
                                <span class="file-time">{}</span>
                            </div>
                        </div>
                        <div class="file-actions">
                            <a href="/file/{}" class="btn btn-primary">
                                View Dashboard
                            </a>
                        </div>
                    </div>
                    "#,
                    dashboard.name, time_str, dashboard.name
                ));
            }

            sections.push_str("</div>");
        }

        // Debug logs section
        if !logs.is_empty() {
            sections.push_str(&format!(
                r#"
                <div class="section-header">
                    <h2>📋 Debug Logs <span class="count">({} files)</span></h2>
                </div>
                <div class="dashboards-grid">
                "#,
                logs.len()
            ));

            for log in logs {
                let time_str = format_time(log.modified);
                sections.push_str(&format!(
                    r#"
                    <div class="file-item log-type">
                        <div class="file-header">
                            <h3 class="file-title">📋 {}</h3>
                            <div class="file-meta">
                                <span class="file-time">{}</span>
                            </div>
                        </div>
                        <div class="file-actions">
                            <a href="/file/{}" class="btn btn-secondary">
                                View Log
                            </a>
                        </div>
                    </div>
                    "#,
                    log.name, time_str, log.name
                ));
            }

            sections.push_str("</div>");
        }

        sections
    };

    // Replace template variables
    template.replace("{{DASHBOARD_ITEMS}}", &sections)
}

fn format_time(time: SystemTime) -> String {
    use std::time::UNIX_EPOCH;

    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let now_secs = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();

            let diff = now_secs.saturating_sub(secs);

            if diff < 60 {
                "Just now".to_string()
            } else if diff < 3600 {
                format!("{} min ago", diff / 60)
            } else if diff < 86400 {
                format!("{} hr ago", diff / 3600)
            } else {
                format!("{} days ago", diff / 86400)
            }
        }
        Err(_) => "Unknown".to_string(),
    }
}

pub struct DashboardServer {
    directory: PathBuf,
    host: String,
    port: u16,
}

impl DashboardServer {
    pub fn new(directory: impl Into<PathBuf>, host: String, port: u16) -> Self {
        Self {
            directory: directory.into(),
            host,
            port,
        }
    }

    pub async fn start(&self) -> Result<(), Error> {
        let base_path = self.directory.clone();

        // Create directory if it doesn't exist
        if !base_path.exists() {
            tokio::fs::create_dir_all(&base_path).await?;
        }

        println!("🚀 Starting Trident Dashboard Server");
        println!("📁 Serving dashboards from: {}", base_path.display());
        println!("🌐 Server running at: http://{}:{}", self.host, self.port);
        println!("📊 Dashboard list: http://{}:{}/", self.host, self.port);
        println!("🔄 Web page auto-refreshes every 3 seconds");
        println!();

        let state = AppState::new(base_path.clone());

        // Initial scan for files
        let _ = state.update_files();

        // Set up file watcher for real-time updates
        let watch_state = state.clone();
        let watch_path = base_path.clone();
        tokio::spawn(async move {
            let (tx, mut rx) = tokio::sync::mpsc::channel(100);

            let mut watcher = RecommendedWatcher::new(
                move |res| {
                    let _ = tx.blocking_send(res);
                },
                Config::default(),
            )
            .unwrap();

            let _ = watcher.watch(&watch_path, RecursiveMode::NonRecursive);

            while let Some(_event) = rx.recv().await {
                let _ = watch_state.update_files();
            }
        });

        // Build the router
        let app = Router::new()
            .route("/", get(file_list))
            .route("/file/:filename", get(serve_file))
            .route("/dashboard/:filename", get(serve_file)) // Backward compatibility
            .nest_service("/static", ServeDir::new(&base_path))
            .layer(CorsLayer::permissive())
            .with_state(state);

        // Start the server
        let addr = format!("{}:{}", self.host, self.port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;

        println!("✅ Server started successfully!");
        println!("Press Ctrl+C to stop the server");
        println!();

        axum::serve(listener, app).await?;

        Ok(())
    }
}

fn format_log_as_html(content: &str, filename: &str) -> String {
    let highlighted_content = content
        .lines()
        .map(|line| {
            let escaped = html_escape(line);
            if line.contains("ERROR") {
                format!("<div class=\"log-error\">{}</div>", escaped)
            } else if line.contains("DEBUG") {
                format!("<div class=\"log-debug\">{}</div>", escaped)
            } else if line.contains("Program") && line.contains("invoke") {
                format!("<div class=\"log-invoke\">{}</div>", escaped)
            } else if line.contains("Program") && line.contains("success") {
                format!("<div class=\"log-success\">{}</div>", escaped)
            } else if line.contains("Program") && line.contains("failed") {
                format!("<div class=\"log-failed\">{}</div>", escaped)
            } else if line.contains("PANICKED") {
                format!("<div class=\"log-panic\">{}</div>", escaped)
            } else {
                format!("<div class=\"log-line\">{}</div>", escaped)
            }
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - Trident Debug Log</title>
    <style>
        body {{
            font-family: 'Courier New', monospace;
            margin: 0;
            padding: 20px;
            background: #0f172a;
            color: #e2e8f0;
            font-size: 14px;
            line-height: 1.4;
        }}
        .header {{
            background: #1e293b;
            padding: 20px;
            border-radius: 8px;
            margin-bottom: 20px;
            border-left: 4px solid #3b82f6;
        }}
        .content {{
            background: #1e293b;
            padding: 20px;
            border-radius: 8px;
            max-height: 80vh;
            overflow-y: auto;
            border: 1px solid #334155;
        }}
        .log-line {{ color: #cbd5e1; }}
        .log-error {{ color: #f87171; font-weight: bold; }}
        .log-debug {{ color: #94a3b8; }}
        .log-invoke {{ color: #22c55e; }}
        .log-success {{ color: #22c55e; font-weight: bold; }}
        .log-failed {{ color: #f87171; font-weight: bold; }}
        .log-panic {{ color: #fbbf24; font-weight: bold; background: rgba(251, 191, 36, 0.1); padding: 2px 4px; border-radius: 4px; }}
        .nav {{ margin-bottom: 20px; }}
        .nav a {{ color: #38bdf8; text-decoration: none; }}
        .nav a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="nav">
        <a href="/">← Back to File List</a>
    </div>
    <div class="header">
        <h1>📋 Debug Log: {}</h1>
        <p>Trident SVM execution log with syntax highlighting</p>
    </div>
    <div class="content">
        {}
    </div>
</body>
</html>"#,
        filename, filename, highlighted_content
    )
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}
