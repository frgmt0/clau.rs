use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Session {
    pub id: SessionId,
    pub system_prompt: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Session {
    pub fn new(id: SessionId) -> Self {
        Self {
            id,
            system_prompt: None,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }
    
    pub fn id(&self) -> &SessionId {
        &self.id
    }
}

#[derive(Clone)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<SessionId, Session>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn builder() -> SessionBuilder {
        SessionBuilder::new()
    }
    
    pub async fn create_session(&self) -> SessionBuilder {
        SessionBuilder::with_manager(self.clone())
    }
    
    pub async fn get(&self, id: &SessionId) -> Result<Option<Session>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(id).cloned())
    }
    
    pub async fn resume(&self, id: &SessionId) -> Result<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(id).cloned()
            .ok_or_else(|| Error::SessionNotFound(id.to_string()))
    }
    
    pub async fn list(&self) -> Result<Vec<SessionId>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.keys().cloned().collect())
    }
    
    async fn store(&self, session: Session) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id.clone(), session);
        Ok(())
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SessionBuilder {
    session: Session,
    manager: Option<SessionManager>,
}

impl SessionBuilder {
    pub fn new() -> Self {
        let id = SessionId::new(uuid::Uuid::new_v4().to_string());
        Self {
            session: Session::new(id),
            manager: None,
        }
    }
    
    fn with_manager(manager: SessionManager) -> Self {
        let id = SessionId::new(uuid::Uuid::new_v4().to_string());
        Self {
            session: Session::new(id),
            manager: Some(manager),
        }
    }
    
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.session.system_prompt = Some(prompt.into());
        self
    }
    
    pub fn with_metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.session.metadata.insert(key.into(), value);
        self
    }
    
    pub async fn build(self) -> Result<Session> {
        if let Some(manager) = self.manager {
            manager.store(self.session.clone()).await?;
        }
        Ok(self.session)
    }
}