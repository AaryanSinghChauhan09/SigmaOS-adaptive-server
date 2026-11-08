//! SigmaOS-adaptive-server — comprehensive adaptive server component.
//!
//! Provides adaptive server functionality for dynamic resource allocation,
//! load balancing, and performance optimization based on workload patterns.

use std::collections::HashMap;

/// Server type
#[derive(Debug, Clone, PartialEq)]
pub enum ServerType {
    /// Web server
    Web,
    /// Application server
    Application,
    /// Database server
    Database,
    /// File server
    File,
    /// Mail server
    Mail,
    /// Proxy server
    Proxy,
    /// Load balancer
    LoadBalancer,
    /// Cache server
    Cache,
}

/// Server state
#[derive(Debug, Clone, PartialEq)]
pub enum ServerState {
    /// Starting
    Starting,
    /// Running
    Running,
    /// Stopping
    Stopping,
    /// Stopped
    Stopped,
    /// Failed
    Failed,
    /// Maintenance
    Maintenance,
}

/// Resource allocation
#[derive(Debug, Clone)]
pub struct ResourceAllocation {
    /// CPU cores
    pub cpu_cores: u32,
    /// Memory in MB
    pub memory_mb: u64,
    /// Storage in GB
    pub storage_gb: u64,
    /// Network bandwidth in Mbps
    pub network_mbps: u32,
    /// GPU count
    pub gpu_count: u32,
}

/// Server metrics
#[derive(Debug, Clone)]
pub struct ServerMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Disk usage percentage
    pub disk_usage: f64,
    /// Network usage percentage
    pub network_usage: f64,
    /// Request rate (requests per second)
    pub request_rate: f64,
    /// Response time (milliseconds)
    pub response_time: f64,
    /// Error rate (percentage)
    pub error_rate: f64,
    /// Active connections
    pub active_connections: u64,
    /// Timestamp
    pub timestamp: u64,
}

/// Adaptive server
#[derive(Debug, Clone)]
pub struct AdaptiveServer {
    /// Server ID
    pub id: String,
    /// Server name
    pub name: String,
    /// Server type
    pub server_type: ServerType,
    /// Server state
    pub state: ServerState,
    /// Resource allocation
    pub resources: ResourceAllocation,
    /// Current metrics
    pub metrics: ServerMetrics,
    /// Target metrics
    pub target_metrics: TargetMetrics,
    /// Adaptation rules
    pub adaptation_rules: Vec<AdaptationRule>,
    /// Last adaptation timestamp
    pub last_adaptation: u64,
}

/// Target metrics
#[derive(Debug, Clone)]
pub struct TargetMetrics {
    /// Target CPU usage
    pub cpu_usage: f64,
    /// Target memory usage
    pub memory_usage: f64,
    /// Target response time
    pub response_time: f64,
    /// Target error rate
    pub error_rate: f64,
}

/// Adaptation rule
#[derive(Debug, Clone)]
pub struct AdaptationRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Condition
    pub condition: AdaptationCondition,
    /// Action
    pub action: AdaptationAction,
    /// Priority
    pub priority: u32,
    /// Enabled
    pub enabled: bool,
}

/// Adaptation condition
#[derive(Debug, Clone, PartialEq)]
pub enum AdaptationCondition {
    /// CPU usage above threshold
    CpuAbove(f64),
    /// CPU usage below threshold
    CpuBelow(f64),
    /// Memory usage above threshold
    MemoryAbove(f64),
    /// Memory usage below threshold
    MemoryBelow(f64),
    /// Response time above threshold
    ResponseTimeAbove(f64),
    /// Error rate above threshold
    ErrorRateAbove(f64),
    /// Request rate above threshold
    RequestRateAbove(f64),
    /// Always true
    Always,
}

/// Adaptation action
#[derive(Debug, Clone, PartialEq)]
pub enum AdaptationAction {
    /// Scale up resources
    ScaleUp,
    /// Scale down resources
    ScaleDown,
    /// Add instance
    AddInstance,
    /// Remove instance
    RemoveInstance,
    /// Adjust CPU allocation
    AdjustCpu(f64),
    /// Adjust memory allocation
    AdjustMemory(f64),
    /// Restart server
    Restart,
    /// Enable maintenance mode
    EnableMaintenance,
    /// Disable maintenance mode
    DisableMaintenance,
}

/// Adaptive server manager
#[derive(Debug, Clone)]
pub struct AdaptiveServerManager {
    /// Servers
    pub servers: HashMap<String, AdaptiveServer>,
    /// Load balancer
    pub load_balancer: LoadBalancer,
    /// Statistics
    pub stats: ServerManagerStatistics,
}

/// Load balancer
#[derive(Debug, Clone)]
pub struct LoadBalancer {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health check interval
    pub health_check_interval: u64,
    /// Healthy servers
    pub healthy_servers: Vec<String>,
    /// Unhealthy servers
    pub unhealthy_servers: Vec<String>,
}

/// Load balancing algorithm
#[derive(Debug, Clone, PartialEq)]
pub enum LoadBalancingAlgorithm {
    /// Round robin
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Least response time
    LeastResponseTime,
    /// Resource based
    ResourceBased,
    /// IP hash
    IPHash,
    /// Random
    Random,
}

/// Server manager statistics
#[derive(Debug, Clone, Default)]
pub struct ServerManagerStatistics {
    /// Total servers
    pub total_servers: u64,
    /// Running servers
    pub running_servers: u64,
    /// Stopped servers
    pub stopped_servers: u64,
    /// Failed servers
    pub failed_servers: u64,
    /// Total adaptations
    pub total_adaptations: u64,
    /// Last adaptation timestamp
    pub last_adaptation: u64,
}

impl Default for AdaptiveServerManager {
    fn default() -> Self {
        Self {
            servers: HashMap::new(),
            load_balancer: LoadBalancer {
                algorithm: LoadBalancingAlgorithm::LeastConnections,
                health_check_interval: 30,
                healthy_servers: vec![],
                unhealthy_servers: vec![],
            },
            stats: ServerManagerStatistics::default(),
        }
    }
}

impl AdaptiveServerManager {
    /// Create new server manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Initialize with default servers
    pub fn initialize(&mut self) {
        // Add default adaptation rules
        let default_rules = vec![
            AdaptationRule {
                id: "cpu_scale_up".into(),
                name: "Scale up on high CPU".into(),
                condition: AdaptationCondition::CpuAbove(80.0),
                action: AdaptationAction::ScaleUp,
                priority: 1,
                enabled: true,
            },
            AdaptationRule {
                id: "cpu_scale_down".into(),
                name: "Scale down on low CPU".into(),
                condition: AdaptationCondition::CpuBelow(20.0),
                action: AdaptationAction::ScaleDown,
                priority: 2,
                enabled: true,
            },
            AdaptationRule {
                id: "memory_scale_up".into(),
                name: "Scale up on high memory".into(),
                condition: AdaptationCondition::MemoryAbove(85.0),
                action: AdaptationAction::ScaleUp,
                priority: 3,
                enabled: true,
            },
            AdaptationRule {
                id: "error_restart".into(),
                name: "Restart on high errors".into(),
                condition: AdaptationCondition::ErrorRateAbove(5.0),
                action: AdaptationAction::Restart,
                priority: 1,
                enabled: true,
            },
        ];
        
        // Add sample servers
        for i in 0..3 {
            let server = AdaptiveServer {
                id: format!("server-{}", i + 1),
                name: format!("Web Server {}", i + 1),
                server_type: ServerType::Web,
                state: ServerState::Running,
                resources: ResourceAllocation {
                    cpu_cores: 4,
                    memory_mb: 8192,
                    storage_gb: 100,
                    network_mbps: 1000,
                    gpu_count: 0,
                },
                metrics: ServerMetrics {
                    cpu_usage: 45.0,
                    memory_usage: 60.0,
                    disk_usage: 30.0,
                    network_usage: 25.0,
                    request_rate: 100.0,
                    response_time: 150.0,
                    error_rate: 0.5,
                    active_connections: 50,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                },
                target_metrics: TargetMetrics {
                    cpu_usage: 70.0,
                    memory_usage: 80.0,
                    response_time: 200.0,
                    error_rate: 1.0,
                },
                adaptation_rules: default_rules.clone(),
                last_adaptation: 0,
            };
            
            let server_id = server.id.clone();
            self.servers.insert(server_id.clone(), server);
            self.load_balancer.healthy_servers.push(server_id);
        }
        
        self.update_statistics();
    }
    
    /// Add server
    pub fn add_server(&mut self, server: AdaptiveServer) -> String {
        let id = server.id.clone();
        self.servers.insert(id.clone(), server);
        self.update_statistics();
        id
    }
    
    /// Remove server
    pub fn remove_server(&mut self, server_id: &str) -> bool {
        if self.servers.remove(server_id).is_some() {
            self.load_balancer.healthy_servers.retain(|s| s != server_id);
            self.load_balancer.unhealthy_servers.retain(|s| s != server_id);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get server
    pub fn get_server(&self, server_id: &str) -> Option<&AdaptiveServer> {
        self.servers.get(server_id)
    }
    
    /// Get server mutably
    pub fn get_server_mut(&mut self, server_id: &str) -> Option<&mut AdaptiveServer> {
        self.servers.get_mut(server_id)
    }
    
    /// Get all servers
    pub fn get_all_servers(&self) -> Vec<&AdaptiveServer> {
        self.servers.values().collect()
    }
    
    /// Get servers by type
    pub fn get_servers_by_type(&self, server_type: &ServerType) -> Vec<&AdaptiveServer> {
        self.servers.values()
            .filter(|s| s.server_type == *server_type)
            .collect()
    }
    
    /// Get servers by state
    pub fn get_servers_by_state(&self, state: &ServerState) -> Vec<&AdaptiveServer> {
        self.servers.values()
            .filter(|s| s.state == *state)
            .collect()
    }
    
    /// Update server metrics
    pub fn update_server_metrics(&mut self, server_id: &str, metrics: ServerMetrics) -> bool {
        if let Some(server) = self.servers.get_mut(server_id) {
            server.metrics = metrics;
            return true;
        }
        false
    }
    
    /// Start server
    pub fn start_server(&mut self, server_id: &str) -> bool {
        if let Some(server) = self.servers.get_mut(server_id) {
            if server.state == ServerState::Stopped || server.state == ServerState::Failed {
                server.state = ServerState::Starting;
                server.state = ServerState::Running;
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Stop server
    pub fn stop_server(&mut self, server_id: &str) -> bool {
        if let Some(server) = self.servers.get_mut(server_id) {
            if server.state == ServerState::Running {
                server.state = ServerState::Stopping;
                server.state = ServerState::Stopped;
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Restart server
    pub fn restart_server(&mut self, server_id: &str) -> bool {
        if let Some(server) = self.servers.get_mut(server_id) {
            server.state = ServerState::Starting;
            server.state = ServerState::Running;
            server.last_adaptation = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            self.stats.total_adaptations += 1;
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Scale server resources
    pub fn scale_server(&mut self, server_id: &str, cpu_cores: u32, memory_mb: u64) -> bool {
        if let Some(server) = self.servers.get_mut(server_id) {
            server.resources.cpu_cores = cpu_cores;
            server.resources.memory_mb = memory_mb;
            server.last_adaptation = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            self.stats.total_adaptations += 1;
            return true;
        }
        false
    }
    
    /// Check adaptation rules for server
    pub fn check_adaptation_rules(&mut self, server_id: &str) -> Vec<AdaptationAction> {
        let mut actions = vec![];
        
        if let Some(server) = self.servers.get(server_id) {
            for rule in &server.adaptation_rules {
                if !rule.enabled {
                    continue;
                }
                
                let should_apply = match &rule.condition {
                    AdaptationCondition::CpuAbove(threshold) => server.metrics.cpu_usage > *threshold,
                    AdaptationCondition::CpuBelow(threshold) => server.metrics.cpu_usage < *threshold,
                    AdaptationCondition::MemoryAbove(threshold) => server.metrics.memory_usage > *threshold,
                    AdaptationCondition::MemoryBelow(threshold) => server.metrics.memory_usage < *threshold,
                    AdaptationCondition::ResponseTimeAbove(threshold) => server.metrics.response_time > *threshold,
                    AdaptationCondition::ErrorRateAbove(threshold) => server.metrics.error_rate > *threshold,
                    AdaptationCondition::RequestRateAbove(threshold) => server.metrics.request_rate > *threshold,
                    AdaptationCondition::Always => true,
                };
                
                if should_apply {
                    actions.push(rule.action.clone());
                }
            }
        }
        
        // Sort by priority (lower number = higher priority)
        actions.sort_by(|a, b| {
            let priority_a = self.servers.get(server_id)
                .and_then(|s| s.adaptation_rules.iter()
                    .find(|r| r.action == *a)
                    .map(|r| r.priority))
                .unwrap_or(u32::MAX);
            let priority_b = self.servers.get(server_id)
                .and_then(|s| s.adaptation_rules.iter()
                    .find(|r| r.action == *b)
                    .map(|r| r.priority))
                .unwrap_or(u32::MAX);
            priority_a.cmp(&priority_b)
        });
        
        actions
    }
    
    /// Apply adaptation actions
    pub fn apply_adaptation_actions(&mut self, server_id: &str, actions: Vec<AdaptationAction>) -> bool {
        let mut applied = false;
        
        for action in actions {
            match action {
                AdaptationAction::ScaleUp => {
                    if self.scale_server(server_id, 
                        self.servers.get(server_id).map(|s| s.resources.cpu_cores * 2).unwrap_or(8),
                        self.servers.get(server_id).map(|s| s.resources.memory_mb * 2).unwrap_or(16384)) {
                        applied = true;
                    }
                }
                AdaptationAction::ScaleDown => {
                    if self.scale_server(server_id,
                        self.servers.get(server_id).map(|s| s.resources.cpu_cores / 2).unwrap_or(2),
                        self.servers.get(server_id).map(|s| s.resources.memory_mb / 2).unwrap_or(4096)) {
                        applied = true;
                    }
                }
                AdaptationAction::Restart => {
                    if self.restart_server(server_id) {
                        applied = true;
                    }
                }
                AdaptationAction::EnableMaintenance => {
                    if let Some(server) = self.servers.get_mut(server_id) {
                        server.state = ServerState::Maintenance;
                        applied = true;
                    }
                }
                AdaptationAction::DisableMaintenance => {
                    if let Some(server) = self.servers.get_mut(server_id) {
                        if server.state == ServerState::Maintenance {
                            server.state = ServerState::Running;
                            applied = true;
                        }
                    }
                }
                _ => {}
            }
        }
        
        applied
    }
    
    /// Perform health check
    pub fn perform_health_check(&mut self) -> Vec<String> {
        let mut unhealthy = vec![];
        
        for (server_id, server) in &self.servers {
            // Check if server is responding
            let is_healthy = server.state == ServerState::Running && 
                           server.metrics.error_rate < 10.0 &&
                           server.metrics.response_time < 1000.0;
            
            if !is_healthy {
                unhealthy.push(server_id.clone());
            }
        }
        
        // Update load balancer
        self.load_balancer.unhealthy_servers = unhealthy.clone();
        self.load_balancer.healthy_servers = self.servers.keys()
            .filter(|id| !unhealthy.contains(id))
            .cloned()
            .collect();
        
        unhealthy
    }
    
    /// Get next server from load balancer
    pub fn get_next_server(&mut self) -> Option<String> {
        match self.load_balancer.algorithm {
            LoadBalancingAlgorithm::RoundRobin => {
                // Simple round robin - return first healthy server
                self.load_balancer.healthy_servers.first().cloned()
            }
            LoadBalancingAlgorithm::LeastConnections => {
                // Return server with least active connections
                self.load_balancer.healthy_servers.iter()
                    .filter_map(|id| self.servers.get(id))
                    .min_by_key(|s| s.metrics.active_connections)
                    .map(|s| s.id.clone())
            }
            LoadBalancingAlgorithm::LeastResponseTime => {
                // Return server with least response time
                self.load_balancer.healthy_servers.iter()
                    .filter_map(|id| self.servers.get(id))
                    .min_by(|a, b| a.metrics.response_time.partial_cmp(&b.metrics.response_time).unwrap())
                    .map(|s| s.id.clone())
            }
            LoadBalancingAlgorithm::Random => {
                // Simple random selection
                if self.load_balancer.healthy_servers.is_empty() {
                    None
                } else {
                    let index = (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as usize) % self.load_balancer.healthy_servers.len();
                    Some(self.load_balancer.healthy_servers[index].clone())
                }
            }
            _ => {
                // Default to first healthy server
                self.load_balancer.healthy_servers.first().cloned()
            }
        }
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.stats.total_servers = self.servers.len() as u64;
        self.stats.running_servers = self.servers.values()
            .filter(|s| s.state == ServerState::Running)
            .count() as u64;
        self.stats.stopped_servers = self.servers.values()
            .filter(|s| s.state == ServerState::Stopped)
            .count() as u64;
        self.stats.failed_servers = self.servers.values()
            .filter(|s| s.state == ServerState::Failed)
            .count() as u64;
    }
    
    /// Get server manager statistics
    pub fn get_statistics(&self) -> &ServerManagerStatistics {
        &self.stats
    }
    
    /// Export server data
    pub fn export_data(&self) -> ServerExport {
        ServerExport {
            servers: self.servers.values().cloned().collect(),
            load_balancer: self.load_balancer.clone(),
        }
    }
    
    /// Import server data
    pub fn import_data(&mut self, export: ServerExport) {
        self.servers.clear();
        for server in export.servers {
            self.servers.insert(server.id.clone(), server);
        }
        self.load_balancer = export.load_balancer;
        self.update_statistics();
    }
}

/// Server export data
#[derive(Debug, Clone)]
pub struct ServerExport {
    /// Servers
    pub servers: Vec<AdaptiveServer>,
    /// Load balancer
    pub load_balancer: LoadBalancer,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_server_type() {
        let types = vec![
            ServerType::Web,
            ServerType::Application,
            ServerType::Database,
            ServerType::File,
            ServerType::Mail,
            ServerType::Proxy,
            ServerType::LoadBalancer,
            ServerType::Cache,
        ];
        assert_eq!(types.len(), 8);
    }
    
    #[test]
    fn test_server_state() {
        let states = vec![
            ServerState::Starting,
            ServerState::Running,
            ServerState::Stopping,
            ServerState::Stopped,
            ServerState::Failed,
            ServerState::Maintenance,
        ];
        assert_eq!(states.len(), 6);
    }
    
    #[test]
    fn test_adaptation_condition() {
        let conditions = vec![
            AdaptationCondition::CpuAbove(80.0),
            AdaptationCondition::CpuBelow(20.0),
            AdaptationCondition::MemoryAbove(85.0),
            AdaptationCondition::MemoryBelow(15.0),
            AdaptationCondition::ResponseTimeAbove(500.0),
            AdaptationCondition::ErrorRateAbove(5.0),
            AdaptationCondition::RequestRateAbove(1000.0),
            AdaptationCondition::Always,
        ];
        assert_eq!(conditions.len(), 8);
    }
    
    #[test]
    fn test_adaptation_action() {
        let actions = vec![
            AdaptationAction::ScaleUp,
            AdaptationAction::ScaleDown,
            AdaptationAction::AddInstance,
            AdaptationAction::RemoveInstance,
            AdaptationAction::AdjustCpu(0.5),
            AdaptationAction::AdjustMemory(0.5),
            AdaptationAction::Restart,
            AdaptationAction::EnableMaintenance,
            AdaptationAction::DisableMaintenance,
        ];
        assert_eq!(actions.len(), 9);
    }
    
    #[test]
    fn test_load_balancing_algorithm() {
        let algorithms = vec![
            LoadBalancingAlgorithm::RoundRobin,
            LoadBalancingAlgorithm::LeastConnections,
            LoadBalancingAlgorithm::LeastResponseTime,
            LoadBalancingAlgorithm::ResourceBased,
            LoadBalancingAlgorithm::IPHash,
            LoadBalancingAlgorithm::Random,
        ];
        assert_eq!(algorithms.len(), 6);
    }
    
    #[test]
    fn test_manager_creation() {
        let manager = AdaptiveServerManager::new();
        assert_eq!(manager.get_statistics().total_servers, 0);
    }
    
    #[test]
    fn test_initialize() {
        let mut manager = AdaptiveServerManager::new();
        manager.initialize();
        
        assert_eq!(manager.get_statistics().total_servers, 3);
        assert_eq!(manager.get_statistics().running_servers, 3);
        assert!(!manager.load_balancer.healthy_servers.is_empty());
    }
    
    #[test]
    fn test_add_server() {
        let mut manager = AdaptiveServerManager::new();
        
        let server = AdaptiveServer {
            id: "test-server".into(),
            name: "Test Server".into(),
            server_type: ServerType::Web,
            state: ServerState::Running,
            resources: ResourceAllocation {
                cpu_cores: 4,
                memory_mb: 8192,
                storage_gb: 100,
                network_mbps: 1000,
                gpu_count: 0,
            },
            metrics: ServerMetrics {
                cpu_usage: 50.0,
                memory_usage: 60.0,
                disk_usage: 30.0,
                network_usage: 25.0,
                request_rate: 100.0,
                response_time: 150.0,
                error_rate: 0.5,
                active_connections: 50,
                timestamp: 0,
            },
            target_metrics: TargetMetrics {
                cpu_usage: 70.0,
                memory_usage: 80.0,
                response_time: 200.0,
                error_rate: 1.0,
            },
            adaptation_rules: vec![],
            last_adaptation: 0,
        };
        
        manager.add_server(server);
        assert!(manager.get_server("test-server").is_some());
        assert_eq!(manager.get_statistics().total_servers, 1);
    }
    
    #[test]
    fn test_start_stop_server() {
        let mut manager = AdaptiveServerManager::new();
        manager.initialize();
        
        // Stop a server
        assert!(manager.stop_server("server-1"));
        assert_eq!(manager.get_server("server-1").unwrap().state, ServerState::Stopped);
        
        // Start it again
        assert!(manager.start_server("server-1"));
        assert_eq!(manager.get_server("server-1").unwrap().state, ServerState::Running);
    }
    
    #[test]
    fn test_scale_server() {
        let mut manager = AdaptiveServerManager::new();
        manager.initialize();
        
        assert!(manager.scale_server("server-1", 8, 16384));
        
        let server = manager.get_server("server-1").unwrap();
        assert_eq!(server.resources.cpu_cores, 8);
        assert_eq!(server.resources.memory_mb, 16384);
    }
    
    #[test]
    fn test_check_adaptation_rules() {
        let mut manager = AdaptiveServerManager::new();
        manager.initialize();
        
        // Set high CPU usage to trigger scale up rule
        manager.update_server_metrics("server-1", ServerMetrics {
            cpu_usage: 90.0,
            memory_usage: 60.0,
            disk_usage: 30.0,
            network_usage: 25.0,
            request_rate: 100.0,
            response_time: 150.0,
            error_rate: 0.5,
            active_connections: 50,
            timestamp: 0,
        });
        
        let actions = manager.check_adaptation_rules("server-1");
        assert!(!actions.is_empty());
        assert!(actions.contains(&AdaptationAction::ScaleUp));
    }
    
    #[test]
    fn test_apply_adaptation_actions() {
        let mut manager = AdaptiveServerManager::new();
        manager.initialize();
        
        let actions = vec![AdaptationAction::ScaleUp];
        assert!(manager.apply_adaptation_actions("server-1", actions));
        
        let server = manager.get_server("server-1").unwrap();
        assert_eq!(server.resources.cpu_cores, 8); // 4 * 2
        assert_eq!(server.resources.memory_mb, 16384); // 8192 * 2
    }
    
    #[test]
    fn test_health_check() {
        let mut manager = AdaptiveServerManager::new();
        manager.initialize();
        
        // Make one server unhealthy
        manager.update_server_metrics("server-1", ServerMetrics {
            cpu_usage: 90.0,
            memory_usage: 95.0, // High memory
            disk_usage: 30.0,
            network_usage: 25.0,
            request_rate: 100.0,
            response_time: 2000.0, // High response time
            error_rate: 15.0, // High error rate
            active_connections: 50,
            timestamp: 0,
        });
        
        let unhealthy = manager.perform_health_check();
        assert!(unhealthy.contains(&"server-1".to_string()));
        assert!(!manager.load_balancer.unhealthy_servers.is_empty());
    }
    
    #[test]
    fn test_get_next_server() {
        let mut manager = AdaptiveServerManager::new();
        manager.initialize();
        
        let server = manager.get_next_server();
        assert!(server.is_some());
        assert!(manager.servers.contains_key(&server.unwrap()));
    }
    
    #[test]
    fn test_get_servers_by_type() {
        let mut manager = AdaptiveServerManager::new();
        manager.initialize();
        
        let web_servers = manager.get_servers_by_type(&ServerType::Web);
        assert_eq!(web_servers.len(), 3);
    }
    
    #[test]
    fn test_get_servers_by_state() {
        let mut manager = AdaptiveServerManager::new();
        manager.initialize();
        
        let running_servers = manager.get_servers_by_state(&ServerState::Running);
        assert_eq!(running_servers.len(), 3);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = AdaptiveServerManager::new();
        manager.initialize();
        
        let export = manager.export_data();
        
        let mut manager2 = AdaptiveServerManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.get_statistics().total_servers, manager.get_statistics().total_servers);
    }
}
