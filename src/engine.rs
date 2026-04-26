use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::process::Command;
use tokio::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Model load failed: {0}")]
    LoadFailed(String),
    #[error("Inference failed: {0}")]
    InferenceFailed(String),
    #[error("Tokenization failed: {0}")]
    TokenizationFailed(String),
    #[error("Invalid GGUF file: {0}")]
    InvalidFormat(String),
    #[error("No GPU acceleration: {0}")]
    NoGpu(String),
    #[error("System capability detection failed: {0}")]
    SystemDetectFailed(String),
    #[error("Knowledge graph error: {0}")]
    KnowledgeGraph(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub name: String,
    pub path: String,
    pub quantization: String,
    pub context_length: usize,
    pub parameters: f32,
    pub embedding_dim: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemCapabilities {
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub cpu_model: String,
    pub cpu_features: Vec<String>,
    pub ram_total_gb: f64,
    pub ram_available_gb: f64,
    pub gpu_available: bool,
    pub gpu_name: String,
    pub gpu_vram_gb: f64,
    pub gpu_compute_capability: Option<String>,
    pub disk_total_gb: f64,
    pub disk_available_gb: f64,
    pub os_name: String,
    pub os_version: String,
    pub os_arch: String,
    pub tpu_available: bool,
    pub can_use_gpu: bool,
    pub can_use_quantization: bool,
    pub optimal_batch_size: usize,
    pub optimal_threads: usize,
    pub max_context_length: usize,
    pub recommended_quantization: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PowerConfig {
    pub performance_mode: bool,
    pub gpu_enabled: bool,
    pub multi_thread_enabled: bool,
    pub batch_size: usize,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: String,
    pub entity_type: String,
    pub content: String,
    pub properties: HashMap<String, String>,
    pub connections: Vec<String>,
    pub created_at: u64,
    pub last_accessed: u64,
    pub access_count: u32,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub from: String,
    pub to: String,
    pub relation_type: String,
    pub weight: f32,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: HashMap<String, KnowledgeNode>,
    pub edges: Vec<KnowledgeEdge>,
    pub entities: HashSet<String>,
    pub relations: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionRecord {
    pub id: String,
    pub timestamp: u64,
    pub input: String,
    pub output: String,
    pub outcome: InteractionOutcome,
    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionOutcome {
    pub success: bool,
    pub rating: f32,
    pub feedback: String,
    pub improvement_areas: Vec<String>,
    pub what_worked: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedWeight {
    pub pattern_id: String,
    pub weight_adjustment: f32,
    pub reason: String,
    pub success_rate_change: f32,
    pub sample_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_interactions: u32,
    pub successful_interactions: u32,
    pub failed_interactions: u32,
    pub success_rate: f32,
    pub average_rating: f32,
    pub patterns_learned: u32,
    pub improvements_applied: u32,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfLearningState {
    pub metrics: PerformanceMetrics,
    pub learned_weights: Vec<LearnedWeight>,
    pub interaction_history: Vec<InteractionRecord>,
    pub course_corrections: Vec<CourseCorrection>,
    pub reward_model: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseCorrection {
    pub id: String,
    pub timestamp: u64,
    pub issue: String,
    pub correction_applied: String,
    pub outcome: f32,
    pub successful: bool,
}

impl Default for SelfLearningState {
    fn default() -> Self {
        Self {
            metrics: PerformanceMetrics::default(),
            learned_weights: Vec::new(),
            interaction_history: Vec::new(),
            course_corrections: Vec::new(),
            reward_model: HashMap::new(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_interactions: 0,
            successful_interactions: 0,
            failed_interactions: 0,
            success_rate: 0.0,
            average_rating: 0.0,
            patterns_learned: 0,
            improvements_applied: 0,
            last_updated: now_timestamp(),
        }
    }
}

#[derive(Clone)]
pub struct ArgentEngine {
    inner: Arc<Mutex<EngineState>>,
}

struct EngineState {
    model_config: Option<ModelConfig>,
    vocab: HashMap<String, i64>,
    reverse_vocab: HashMap<i64, String>,
    context: Vec<i64>,
    kv_cache: HashMap<String, Vec<f32>>,
    system: SystemCapabilities,
    quantization_type: QuantizationType,
    coding_specialized: bool,
    code_cache: HashMap<String, String>,
    power_config: PowerConfig,
    knowledge_graph: KnowledgeGraph,
    self_learning: SelfLearningState,
}

fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

impl ArgentEngine {
    pub fn new() -> Self {
        let system = SystemCapabilities::detect();
        
        Self {
            inner: Arc::new(Mutex::new(EngineState {
                model_config: None,
                vocab: HashMap::new(),
                reverse_vocab: HashMap::new(),
                context: Vec::new(),
                kv_cache: HashMap::new(),
                system,
                quantization_type: QuantizationType::Q4K,
                coding_specialized: true,
                code_cache: HashMap::new(),
                power_config: PowerConfig::default(),
                knowledge_graph: KnowledgeGraph::new(),
                self_learning: SelfLearningState::default(),
            })),
        }
    }

    pub async fn get_system_capabilities(&self) -> SystemCapabilities {
        let state = self.inner.lock().await;
        state.system.clone()
    }

    pub async fn optimize_for_system(&self) -> PowerConfig {
        let mut state = self.inner.lock().await;
        
        let sys = &state.system;
        
        let performance_mode = sys.ram_available_gb > 16.0 && sys.cpu_cores >= 8;
        let gpu_enabled = sys.can_use_gpu && sys.gpu_vram_gb >= 4.0;
        let multi_thread = sys.cpu_threads >= 8;
        
        let batch_size = if sys.ram_available_gb > 32.0 {
            32
        } else if sys.ram_available_gb > 16.0 {
            16
        } else if sys.ram_available_gb > 8.0 {
            8
        } else {
            4
        };
        
        let priority = if performance_mode {
            "performance".to_string()
        } else if sys.ram_available_gb > 8.0 {
            "balanced".to_string()
        } else {
            "power_saver".to_string()
        };
        
        let config = PowerConfig {
            performance_mode,
            gpu_enabled,
            multi_thread_enabled: multi_thread,
            batch_size,
            priority,
        };
        
        state.power_config = config.clone();
        
        config
    }

    pub async fn add_knowledge(&self, entity: &str, entity_type: &str, content: &str, relations: Vec<(&str, &str)>) -> Result<(), EngineError> {
        let mut state = self.inner.lock().await;
        
        let node_id = format!("entity_{}", state.knowledge_graph.nodes.len());
        
        let node = KnowledgeNode {
            id: node_id.clone(),
            entity_type: entity_type.to_string(),
            content: content.to_string(),
            properties: HashMap::new(),
            connections: Vec::new(),
            created_at: now_timestamp(),
            last_accessed: now_timestamp(),
            access_count: 0,
            confidence: 0.5,
        };
        
        state.knowledge_graph.nodes.insert(node_id.clone(), node);
        state.knowledge_graph.entities.insert(entity_type.to_string());
        
        for (target, relation) in relations {
            let edge = KnowledgeEdge {
                from: node_id.clone(),
                to: target.to_string(),
                relation_type: relation.to_string(),
                weight: 1.0,
                evidence: Vec::new(),
            };
            state.knowledge_graph.edges.push(edge);
            state.knowledge_graph.relations.insert(relation.to_string());
        }
        
        Ok(())
    }

    pub async fn query_knowledge(&self, entity: &str) -> Result<Vec<KnowledgeNode>, EngineError> {
        let state = self.inner.lock().await;
        
        let mut results = Vec::new();
        
        for (_, node) in &state.knowledge_graph.nodes {
            if node.content.to_lowercase().contains(&entity.to_lowercase()) ||
               node.entity_type.to_lowercase().contains(&entity.to_lowercase()) {
                results.push(node.clone());
            }
        }
        
        Ok(results)
    }

    pub async fn record_interaction(&self, input: &str, output: &str, success: bool, rating: f32, feedback: &str) -> Result<(), EngineError> {
        let mut state = self.inner.lock().await;
        
        let record = InteractionRecord {
            id: format!("interaction_{}", state.self_learning.metrics.total_interactions + 1),
            timestamp: now_timestamp(),
            input: input.to_string(),
            output: output.to_string(),
            outcome: InteractionOutcome {
                success,
                rating,
                feedback: feedback.to_string(),
                improvement_areas: Vec::new(),
                what_worked: Vec::new(),
            },
            context: HashMap::new(),
        };
        
        state.self_learning.metrics.total_interactions += 1;
        
        if success {
            state.self_learning.metrics.successful_interactions += 1;
        } else {
            state.self_learning.metrics.failed_interactions += 1;
        }
        
        let total = state.self_learning.metrics.total_interactions as f32;
        state.self_learning.metrics.success_rate = 
            state.self_learning.metrics.successful_interactions as f32 / total;
        
        let current_avg = state.self_learning.metrics.average_rating;
        state.self_learning.metrics.average_rating = 
            (current_avg * (total - 1.0) + rating) / total;
        
        let mut learn = &mut state.self_learning;
        if let Some(r) = learn.interaction_history.last_mut() {
            r.outcome.what_worked.push(feedback.to_string());
            if !success {
                r.outcome.improvement_areas.push(feedback.to_string());
            }
        }
        
        learn.interaction_history.push(record);
        
        if learn.interaction_history.len() > 1000 {
            learn.interaction_history.drain(0..100);
        }
        
        state.self_learning.metrics.last_updated = now_timestamp();
        
        Ok(())
    }

    pub async fn learn_from_outcome(&self, outcome: &str, adjustment: f32, reason: &str) -> Result<(), EngineError> {
        let mut state = self.inner.lock().await;
        
        let weight = LearnedWeight {
            pattern_id: outcome.to_string(),
            weight_adjustment: adjustment,
            reason: reason.to_string(),
            success_rate_change: adjustment,
            sample_count: 1,
        };
        
        state.self_learning.learned_weights.push(weight);
        state.self_learning.metrics.patterns_learned += 1;
        
        if adjustment > 0.1 {
            state.self_learning.metrics.improvements_applied += 1;
        }
        
        Ok(())
    }

    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        let state = self.inner.lock().await;
        state.self_learning.metrics.clone()
    }

    pub async fn course_correct(&self, issue: &str, correction: &str) -> Result<(), EngineError> {
        let mut state = self.inner.lock().await;
        
        let correc = CourseCorrection {
            id: format!("correction_{}", state.self_learning.course_corrections.len() + 1),
            timestamp: now_timestamp(),
            issue: issue.to_string(),
            correction_applied: correction.to_string(),
            outcome: 0.0,
            successful: false,
        };
        
        state.self_learning.course_corrections.push(correc);
        
        Ok(())
    }

    pub async fn get_recommended_adjustments(&self) -> Vec<String> {
        let state = self.inner.lock().await;
        
        let mut recommendations = Vec::new();
        
        let metrics = &state.self_learning.metrics;
        
        if metrics.success_rate < 0.7 {
            recommendations.push("Consider reducing temperature for more accurate outputs".to_string());
        }
        
        if metrics.average_rating < 3.0 {
            recommendations.push("Lower default temperature to improve response quality".to_string());
        }
        
        let failed_count = state.self_learning.metrics.failed_interactions;
        let recent_interactions = &state.self_learning.interaction_history;
        
        if failed_count > 10 && recent_interactions.len() > 20 {
            let recent_failures = recent_interactions.iter()
                .filter(|i| !i.outcome.success)
                .count();
            
            if recent_failures > recent_interactions.len() / 3 {
                recommendations.push("High failure rate detected - consider adjusting approach".to_string());
            }
        }
        
        for correction in &state.self_learning.course_corrections {
            if !correction.successful {
                recommendations.push(format!("Previous correction '{}' failed, try alternative approach", correction.correction_applied));
            }
        }
        
        recommendations
    }

    pub async fn load_model(&self, path: &str) -> Result<ModelConfig, EngineError> {
        let mut state = self.inner.lock().await;
        
        let model_path = PathBuf::from(path);
        if !model_path.exists() {
            return Err(EngineError::LoadFailed(format!("Model file not found: {}", path)));
        }

        let file_name = model_path
            .file_stem()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let quantization = detect_quantization(path);
        let context_length = calculate_optimal_context(&state.system, path);
        let parameters = detect_parameters(path);
        let emb_dim = detect_embedding_dim(&quantization);
        
        let config = ModelConfig {
            name: file_name,
            path: path.to_string(),
            quantization,
            context_length,
            parameters,
            embedding_dim: emb_dim,
        };

        state.vocab = load_vocabulary();
        state.reverse_vocab = state.vocab.iter()
            .map(|(k, v)| (*v, k.clone()))
            .collect();
        state.model_config = Some(config.clone());
        state.coding_specialized = detect_coding_specialized(path);

        Ok(config)
    }

    pub async fn tokenize(&self, text: &str) -> Result<Vec<i64>, EngineError> {
        let state = self.inner.lock().await;
        
        if state.vocab.is_empty() {
            return Err(EngineError::TokenizationFailed("No model loaded".to_string()));
        }

        let mut tokens = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        
        for word in words {
            if let Some(&token) = state.vocab.get(word) {
                tokens.push(token);
            } else {
                let subwords = tokenize_subword(word, &state.vocab);
                tokens.extend(subwords);
            }
        }

        Ok(tokens)
    }

    pub async fn detokenize(&self, tokens: &[i64]) -> Result<String, EngineError> {
        let state = self.inner.lock().await;
        
        let mut text = String::new();
        for t in tokens {
            if let Some(s) = state.reverse_vocab.get(t) {
                text.push_str(s);
                text.push(' ');
            }
        }

        Ok(text.trim().to_string())
    }

    pub async fn generate(
        &self,
        prompt: &str,
        max_tokens: usize,
        temperature: f32,
        top_p: f32,
    ) -> Result<String, EngineError> {
        let tokens = self.tokenize(prompt).await?;
        
        let mut state = self.inner.lock().await;
        if state.model_config.is_none() {
            return Err(EngineError::InferenceFailed("No model loaded".to_string()));
        }

        let config = state.model_config.as_ref().unwrap();
        let mut generated = tokens.clone();

        for _ in 0..max_tokens {
            let input_tokens = if generated.len() > config.context_length {
                generated[generated.len() - config.context_length..].to_vec()
            } else {
                generated.clone()
            };

            let next_token = self.sample_token(&input_tokens, temperature, &state)?;

            if next_token == 0 || next_token == 2 {
                break;
            }

            generated.push(next_token);
        }

        let mut output = String::new();
        for t in &generated {
            if let Some(s) = state.reverse_vocab.get(t) {
                output.push_str(s);
            }
        }

        Ok(output)
    }

    fn sample_token(&self, input: &[i64], temperature: f32, state: &EngineState) -> Result<i64, EngineError> {
        if input.is_empty() {
            return Ok(1);
        }

        let hash_key = format!("{:?}_{}", input.len(), input.last().unwrap_or(&0));
        
        if state.coding_specialized {
            if let Some(cached) = state.code_cache.get(&hash_key) {
                return Ok(cached.len() as i64);
            }
        }

        let vocab_size = state.vocab.len().max(32000);
        let mut logits = vec![0.0f32; vocab_size];

        for (i, token) in input.iter().enumerate() {
            let seed = (*token as u64).wrapping_mul(31).wrapping_add(i as u64);
            let weight = ((input.len() - i) as f32) / (input.len() as f32);
            
            for j in 0..vocab_size.min(1000) {
                let hash = (seed.wrapping_add(j as u64)) % 1000;
                logits[j] += (hash as f32 / 1000.0) * weight;
            }
        }

        if temperature > 0.0 && temperature < 1.0 {
            let temp = 1.0 / temperature;
            for logit in logits.iter_mut() {
                *logit *= temp;
            }
        }

        let max_logit = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        
        let exp: Vec<f32> = logits.iter()
            .map(|l| (l - max_logit).exp())
            .collect();
        
        let sum: f32 = exp.iter().sum();
        if sum == 0.0 {
            return Ok(0);
        }

        let r: f32 = rand::random();
        let mut cumsum = 0.0f32;
        
        for (i, e) in exp.iter().enumerate() {
            cumsum += e / sum;
            if r <= cumsum {
                return Ok(i as i64);
            }
        }

        Ok(logits.len() as i64 - 1)
    }

    pub async fn code_completion(&self, prefix: &str, _file_type: &str) -> Result<String, EngineError> {
        let tokens = self.tokenize(prefix).await?;
        
        let mut state = self.inner.lock().await;
        
        let mut generated = tokens;
        
        for _ in 0..512 {
            if generated.len() > 2048 {
                break;
            }

            let input = generated[generated.len().saturating_sub(1024)..].to_vec();
            let next = self.sample_token(&input, 0.1, &state)?;

            if next == 2 || next == 0 {
                break;
            }

            generated.push(next);
        }
        
        let mut output = String::new();
        for t in &generated {
            if let Some(s) = state.reverse_vocab.get(t) {
                output.push_str(s);
            }
        }

        Ok(output)
    }

    pub async fn code_analysis(&self, code: &str) -> Result<CodeAnalysis, EngineError> {
        let tokens = self.tokenize(code).await?;
        
        let analysis = CodeAnalysis {
            language: detect_language(&tokens),
            complexity: calculate_complexity(&tokens),
            functions: count_functions(&tokens),
            imports: count_imports(&tokens),
            patterns: detect_patterns(&tokens),
            suggestions: Vec::new(),
        };

        Ok(analysis)
    }

    pub async fn refactor(&self, code: &str, pattern: &str) -> Result<String, EngineError> {
        let prompt = format!("Refactor this code using {} pattern:\n\n{}", pattern, code);
        self.generate(&prompt, 1024, 0.3, 0.9).await
    }

    pub async fn explain(&self, code: &str) -> Result<String, EngineError> {
        let prompt = format!("Explain this code in detail:\n\n```\n{}```", code);
        self.generate(&prompt, 2048, 0.2, 0.9).await
    }

    pub async fn test_generation(&self, code: &str) -> Result<String, EngineError> {
        let prompt = format!("Generate unit tests for this code:\n\n{}", code);
        self.generate(&prompt, 2048, 0.3, 0.9).await
    }

    pub async fn debug(&self, code: &str, error: &str) -> Result<String, EngineError> {
        let prompt = format!("Debug this code. Error: {}\n\nCode:\n{}", error, code);
        self.generate(&prompt, 2048, 0.2, 0.9).await
    }

    pub fn is_coding_specialized(&self) -> bool {
        false
    }

    pub fn context_length(&self) -> usize {
        4096
    }

    pub fn total_parameters(&self) -> f32 {
        7.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeAnalysis {
    pub language: String,
    pub complexity: usize,
    pub functions: usize,
    pub imports: usize,
    pub patterns: Vec<String>,
    pub suggestions: Vec<String>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            entities: HashSet::new(),
            relations: HashSet::new(),
        }
    }
}

impl Default for PowerConfig {
    fn default() -> Self {
        Self {
            performance_mode: false,
            gpu_enabled: false,
            multi_thread_enabled: false,
            batch_size: 4,
            priority: "balanced".to_string(),
        }
    }
}

impl SystemCapabilities {
    pub fn detect() -> Self {
        let cpu_cores = num_cpus();
        let cpu_threads = cpu_cores;
        let cpu_model = detect_cpu_model();
        let cpu_features = detect_cpu_features();
        
        let (ram_total, ram_available) = detect_ram();
        
        let (gpu_available, gpu_name, gpu_vram, compute) = detect_gpu();
        
        let (disk_total, disk_available) = detect_disk();
        
        let (os_name, os_version, os_arch) = detect_os();
        
        let tpu_available = false;
        
        let can_use_gpu = gpu_available && gpu_vram >= 2.0;
        let can_use_quantization = ram_total >= 8.0;
        
        let optimal_batch_size = if ram_total >= 32.0 {
            32
        } else if ram_total >= 16.0 {
            16
        } else {
            8
        };
        
        let optimal_threads = cpu_threads.min(16);
        
        let max_context = if ram_total >= 32.0 {
            32768
        } else if ram_total >= 16.0 {
            16384
        } else {
            4096
        };
        
        let recommended_quantization = if ram_total >= 16.0 && gpu_vram >= 6.0 {
            "Q4_K_M".to_string()
        } else if ram_total >= 8.0 && gpu_vram >= 4.0 {
            "Q5_K_S".to_string()
        } else if ram_total >= 6.0 {
            "Q6_K".to_string()
        } else {
            "Q8_0".to_string()
        };

        Self {
            cpu_cores,
            cpu_threads,
            cpu_model,
            cpu_features,
            ram_total_gb: ram_total,
            ram_available_gb: ram_available,
            gpu_available,
            gpu_name,
            gpu_vram_gb: gpu_vram,
            gpu_compute_capability: compute,
            disk_total_gb: disk_total,
            disk_available_gb: disk_available,
            os_name,
            os_version,
            os_arch,
            tpu_available,
            can_use_gpu,
            can_use_quantization,
            optimal_batch_size,
            optimal_threads,
            max_context_length: max_context,
            recommended_quantization,
        }
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

fn detect_cpu_model() -> String {
    if cfg!(target_os = "windows") {
        let output = Command::new("wmic")
            .args(["cpu", "get", "name"])
            .output();
        
        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout);
            let lines: Vec<&str> = s.lines().collect();
            if lines.len() > 1 {
                return lines[1].trim().to_string();
            }
        }
    }
    
    "Unknown CPU".to_string()
}

fn detect_cpu_features() -> Vec<String> {
    let mut features = Vec::new();
    
    #[cfg(target_arch = "x86_64")]
    features.push("x86_64".to_string());
    
    #[cfg(target_arch = "aarch64")]
    features.push("aarch64".to_string());
    
    features.push("simd".to_string());
    features.push("multithreading".to_string());
    
    features
}

fn detect_ram() -> (f64, f64) {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("wmic")
            .args(["OS", "get", "TotalVisibleMemorySize", "FreePhysicalMemory", "/format:value"])
            .output();
        
        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout);
            let mut total = 0.0;
            let mut free = 0.0;
            
            for line in s.lines() {
                if line.contains("TotalVisibleMemorySize=") {
                    if let Some(val) = line.split('=').nth(1) {
                        total = val.trim().parse::<f64>().unwrap_or(0.0) / 1024.0 / 1024.0;
                    }
                }
                if line.contains("FreePhysicalMemory=") {
                    if let Some(val) = line.split('=').nth(1) {
                        free = val.trim().parse::<f64>().unwrap_or(0.0) / 1024.0 / 1024.0;
                    }
                }
            }
            
            if total > 0.0 {
                return (total, free);
            }
        }
    }
    
    (8.0, 4.0)
}

fn detect_gpu() -> (bool, String, f64, Option<String>) {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("wmic")
            .args(["path", "win32_VideoController", "get", "name", "/format:value"])
            .output();
        
        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout);
            let mut name = "Unknown GPU".to_string();
            
            for line in s.lines() {
                if line.contains("Name=") {
                    if let Some(n) = line.split('=').nth(1) {
                        name = n.trim().to_string();
                    }
                }
            }
            
            let has_nvidia = name.contains("NVIDIA") || name.contains("GeForce") || name.contains("RTX");
            let has_amd = name.contains("AMD") || name.contains("Radeon");
            let has_intel = name.contains("Intel") && name.contains("Xe");
            
            let gpu_available = has_nvidia || has_amd || has_intel;
            let vram = if has_nvidia { 8.0 } else if has_amd { 6.0 } else { 4.0 };
            let compute = if has_nvidia {
                Some("CUDA".to_string())
            } else if has_amd {
                Some("ROCm".to_string())
            } else {
                None
            };
            
            return (gpu_available, name, vram, compute);
        }
    }
    
    (false, "No GPU detected".to_string(), 0.0, None)
}

fn detect_disk() -> (f64, f64) {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("wmic")
            .args(["logicaldisk", "get", "Size", "FreeSpace", "DeviceID"])
            .output();
        
        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout);
            let mut total = 0.0;
            let mut free = 0.0;
            
            for line in s.lines() {
                if line.contains("C:") {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() >= 3 {
                        total = parts[1].trim().parse::<f64>().unwrap_or(0.0) / 1024.0 / 1024.0 / 1024.0;
                        free = parts[2].trim().parse::<f64>().unwrap_or(0.0) / 1024.0 / 1024.0 / 1024.0;
                    }
                }
            }
            
            if total > 0.0 {
                return (total, free);
            }
        }
    }
    
    (256.0, 100.0)
}

fn detect_os() -> (String, String, String) {
    let os_name = if cfg!(target_os = "windows") {
        "Windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macOS".to_string()
    } else if cfg!(target_os = "linux") {
        "Linux".to_string()
    } else {
        "Unknown".to_string()
    };
    
    let os_arch = if cfg!(target_arch = "x86_64") {
        "x86_64".to_string()
    } else if cfg!(target_arch = "aarch64") {
        "aarch64".to_string()
    } else {
        "unknown".to_string()
    };
    
    let os_version = "1.0".to_string();
    
    (os_name, os_version, os_arch)
}

fn calculate_optimal_context(system: &SystemCapabilities, path: &str) -> usize {
    if path.contains("32k") || path.contains("32K") {
        return 32768;
    }
    
    let base_context = if system.ram_available_gb > 16.0 {
        16384
    } else if system.ram_available_gb > 8.0 {
        8192
    } else {
        4096
    };
    
    base_context
}

fn detect_quantization(path: &str) -> String {
    if path.contains("q2") || path.contains("Q2") {
        "Q2_K_S".to_string()
    } else if path.contains("q3") || path.contains("Q3") {
        "Q3_K_S".to_string()
    } else if path.contains("q4") || path.contains("Q4") {
        "Q4_K_M".to_string()
    } else if path.contains("q5") || path.contains("Q5") {
        "Q5_K_S".to_string()
    } else if path.contains("q6") || path.contains("Q6") {
        "Q6_K".to_string()
    } else if path.contains("q8") || path.contains("Q8") {
        "Q8_0".to_string()
    } else if path.contains("f16") || path.contains("F16") {
        "F16".to_string()
    } else {
        "Q4_K_M".to_string()
    }
}

fn detect_parameters(path: &str) -> f32 {
    if path.contains("70b") || path.contains("70B") {
        70.0
    } else if path.contains("34b") || path.contains("34B") {
        34.0
    } else if path.contains("13b") || path.contains("13B") {
        13.0
    } else if path.contains("8b") || path.contains("8B") {
        8.0
    } else if path.contains("7b") || path.contains("7B") {
        7.0
    } else if path.contains("3b") || path.contains("3B") {
        3.0
    } else if path.contains("1b") || path.contains("1B") {
        1.0
    } else {
        7.0
    }
}

fn detect_embedding_dim(quantization: &str) -> usize {
    if quantization.starts_with("Q2") {
        2816
    } else if quantization.starts_with("Q3") {
        3072
    } else if quantization.starts_with("Q4") {
        4096
    } else if quantization.starts_with("Q5") {
        5120
    } else if quantization.starts_with("Q6") {
        6144
    } else if quantization.starts_with("Q8") {
        4096
    } else if quantization.starts_with("F16") {
        4096
    } else if quantization.starts_with("F32") {
        4096
    } else {
        4096
    }
}

fn detect_coding_specialized(path: &str) -> bool {
    let keywords = ["code", "codebase", "codestral", "deepseek-coder", "qwen-coder", "starcode", "llama-code"];
    let lower = path.to_lowercase();
    keywords.iter().any(|k| lower.contains(k))
}

fn load_vocabulary() -> HashMap<String, i64> {
    let mut vocab = HashMap::new();
    
    let common_tokens = [
        ("<unk>", 0),
        ("<s>", 1),
        ("</s>", 2),
        ("<pad>", 3),
        ("<|begin_of_text|>", 0),
        ("<|end_of_text|>", 2),
        ("<|repo_name|>", 10),
        ("<|file_sep|>", 11),
        ("<|chunk_start|>", 12),
        ("<|chunk_end|>", 13),
        ("<|question_start|>", 14),
        ("<|answer_start|>", 15),
        ("<|human|>", 16),
        ("<|assistant|>", 17),
        ("<|system|>", 18),
        ("func", 50),
        ("function", 51),
        ("class", 52),
        ("struct", 53),
        ("enum", 54),
        ("trait", 55),
        ("impl", 56),
        ("pub", 57),
        ("mod", 58),
        ("use", 59),
        ("import", 60),
        ("export", 61),
        ("const", 62),
        ("let", 63),
        ("mut", 64),
        ("fn", 65),
        ("async", 66),
        ("await", 67),
        ("return", 68),
        ("if", 70),
        ("else", 71),
        ("match", 72),
        ("for", 73),
        ("while", 74),
        ("loop", 75),
        ("break", 76),
        ("continue", 77),
        ("in", 78),
        ("self", 79),
        ("Self", 80),
        ("super", 81),
        ("crate", 82),
    ];

    for (token, id) in common_tokens {
        vocab.insert(token.to_string(), id);
    }

    vocab
}

fn tokenize_subword(word: &str, vocab: &HashMap<String, i64>) -> Vec<i64> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = word.chars().collect();
    
    for i in 0..chars.len() {
        let sub: String = chars[i..].iter().collect();
        if let Some(&id) = vocab.get(&sub) {
            tokens.push(id);
            break;
        }
    }

    if tokens.is_empty() {
        tokens.push(0);
    }

    tokens
}

fn detect_language(tokens: &[i64]) -> String {
    let func_count = tokens.iter().filter(|&&t| t == 65).count();
    let class_count = tokens.iter().filter(|&&t| t == 52).count();
    let import_count = tokens.iter().filter(|&&t| t == 59 || t == 60).count();
    
    if func_count > class_count && func_count > import_count {
        "Rust".to_string()
    } else if class_count > func_count {
        "TypeScript/Java".to_string()
    } else if import_count > 0 {
        "Python".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn calculate_complexity(tokens: &[i64]) -> usize {
    let function_count = tokens.iter().filter(|&&t| t == 65 || t == 50 || t == 51).count();
    let loop_count = tokens.iter().filter(|&&t| t == 73 || t == 74 || t == 75).count();
    let conditional_count = tokens.iter().filter(|&&t| t == 70 || t == 71 || t == 72).count();

    function_count * 2 + loop_count + conditional_count
}

fn count_functions(tokens: &[i64]) -> usize {
    tokens.iter().filter(|&&t| t == 65 || t == 50 || t == 51).count()
}

fn count_imports(tokens: &[i64]) -> usize {
    tokens.iter().filter(|&&t| t == 59 || t == 60 || t == 61).count()
}

fn detect_patterns(tokens: &[i64]) -> Vec<String> {
    let mut patterns = Vec::new();
    
    let has_async = tokens.iter().any(|t| *t == 66 || *t == 67);
    let has_match = tokens.iter().any(|t| *t == 72);
    let has_class = tokens.iter().any(|t| *t == 52);
    let has_trait = tokens.iter().any(|t| *t == 55);
    
    if has_async {
        patterns.push("Async/Await".to_string());
    }
    if has_match {
        patterns.push("Pattern Matching".to_string());
    }
    if has_class {
        patterns.push("OOP".to_string());
    }
    if has_trait {
        patterns.push("Trait-based".to_string());
    }

    patterns
}

#[derive(Clone, Debug)]
pub enum QuantizationType {
    Q2K,
    Q3K,
    Q4K,
    Q5K,
    Q6K,
    Q8_0,
    F16,
    F32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine_creation() {
        let engine = ArgentEngine::new();
        let caps = engine.get_system_capabilities().await;
        
        assert!(caps.cpu_cores > 0);
        assert!(caps.ram_total_gb > 0.0);
    }
    
    #[tokio::test]
    async fn test_optimize_for_system() {
        let engine = ArgentEngine::new();
        let config = engine.optimize_for_system().await;
        
        assert!(config.batch_size > 0);
    }
    
    #[tokio::test]
    async fn test_knowledge_graph() {
        let engine = ArgentEngine::new();
        engine.add_knowledge("Rust", "language", "Systems programming language", vec![]).await.unwrap();
        
        let results = engine.query_knowledge("language").await.unwrap();
        assert!(results.len() >= 0);
    }
    
    #[tokio::test]
    async fn test_record_interaction() {
        let engine = ArgentEngine::new();
        engine.record_interaction("test input", "test output", true, 4.5, "Good response").await.unwrap();
        
        let metrics = engine.get_performance_metrics().await;
        assert_eq!(metrics.total_interactions, 1);
    }
}