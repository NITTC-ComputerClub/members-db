pub mod member;

use member::Member;
use serde::{Deserialize, Serialize};

use crate::filesystem;

const VERSION: u32 = 4;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Payload {
    version: Option<u32>,
    members: Vec<Member>,
}

#[derive(Debug)]
pub struct Database {
    path: String,
    payload: Payload,
}

impl Database {
    pub fn new(path: String) -> Self {
        let yaml = filesystem::open(path.as_str());
        let payload: Payload = serde_yaml::from_str(yaml.as_str()).expect("Failed to deserialize.");

        if payload.version.is_none() || payload.version.unwrap() != VERSION {
            panic!(
                "Only version {} is supported. Did you forget to migrate?",
                VERSION
            );
        }

        Database { path, payload }
    }

    pub fn save(&self) {
        let yaml = serde_yaml::to_string(&self.payload).expect("Failed to serialize.");

        filesystem::save(&self.path, yaml)
    }

    pub fn get_members(&self) -> &Vec<Member> {
        &self.payload.members
    }

    pub fn add_member(&mut self, member: Member) {
        self.payload.members.push(member)
    }
}
