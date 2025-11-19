use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RouteInfo {
    pub method: String,
    pub path: String,
    pub middleware: Vec<String>,
}

static ROUTES: Lazy<Mutex<Vec<RouteInfo>>> = Lazy::new(|| Mutex::new(Vec::new()));

pub fn track_route(method: &str, path: &str, middleware: Vec<String>) {
    let mut routes = ROUTES.lock().unwrap();
    routes.push(RouteInfo { method: method.to_uppercase(), path: path.to_string(), middleware });
}

pub fn log_all_routes() {
    let routes = ROUTES.lock().unwrap();

    if routes.is_empty() {
        tracing::warn!("⚠️  No routes registered!");
        return;
    }

    tracing::info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    tracing::info!("📋 Registered Routes ({} total)", routes.len());
    tracing::info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Group routes by prefix (e.g., auth, accounts, etc.)
    let mut grouped: HashMap<String, Vec<&RouteInfo>> = HashMap::new();

    for route in routes.iter() {
        let parts: Vec<&str> = route.path.split('/').filter(|s| !s.is_empty()).collect();
        let group = if parts.len() >= 3 { parts[2].to_string() } else { "root".to_string() };
        grouped.entry(group).or_default().push(route);
    }

    let mut groups: Vec<_> = grouped.keys().cloned().collect();
    groups.sort();

    for group in groups {
        let group_routes = grouped.get(&group).unwrap();
        tracing::info!("");
        tracing::info!("📁 {} ({} routes)", group.to_uppercase(), group_routes.len());

        for route in group_routes {
            let middleware_info = if !route.middleware.is_empty() {
                format!("- [{}]", route.middleware.join(", "))
            } else {
                String::new()
            };

            if middleware_info.is_empty() {
                tracing::info!("🌎 {:7} {}{}", route.method, route.path, middleware_info);
            } else {
                tracing::info!("🔐 {:7} {} {}", route.method, route.path, middleware_info);
            }
        }
    }

    tracing::info!("");
    tracing::info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
