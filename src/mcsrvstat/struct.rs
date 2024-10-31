use serde::Deserialize;
#[derive(Clone)]
#[derive(Deserialize, Debug)]
pub struct MinecraftServerInfo {
    pub online: bool,
    pub ip: Option<String>,
    pub hostname: Option<String>,
    pub port : Option<i32>,
    pub version: Option<String>,
    pub players: Option<PlayerInfo>,
    pub motd: Option<MessageOfTheDay>,
    pub debug: Option<DebugInfo>,
}

#[derive(Clone)]
#[derive(Deserialize, Debug)]
pub struct PlayerInfo {
    pub online: u32,
    pub max: u32,
}
#[derive(Clone)]
#[derive(Deserialize, Debug)]
pub struct MessageOfTheDay {
   pub raw: Vec<String>,
}

#[derive(Clone)]
#[derive(Deserialize, Debug)]
pub struct DebugInfo {
    pub ping: bool,
    pub query: bool,
    pub srv: bool,
    pub cachetime : u32,
    pub cacheexpire : u32,
    pub apiversion : u32
}

impl DebugInfo { 
    pub fn to_strings(&self) -> Vec<String> {
        vec![
            self.ping.to_string(),
            self.query.to_string(),
            self.srv.to_string(),
            self.cachetime.to_string(),
            self.cacheexpire.to_string(),
            self.apiversion.to_string(),
        ]
    }
}