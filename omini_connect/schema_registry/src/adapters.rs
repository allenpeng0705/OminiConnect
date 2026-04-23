//! Connector adapters for the schema registry.

#![allow(dead_code)]
#![allow(unused_imports)]

use super::{Category, Platform, ToolSchema, ToolSchemaRegistry};

/// Feishu tool schema adapter
pub struct FeishuToolAdapter;

impl FeishuToolAdapter {
    pub fn new() -> Self {
        Self
    }

    fn categorize(name: &str) -> Category {
        match name {
            n if n.starts_with("calendar_") => Category::Calendar,
            n if n.starts_with("bitable_") => Category::Bitable,
            n if n.starts_with("message_") => Category::Messaging,
            _ => Category::Unknown,
        }
    }
}

impl Default for FeishuToolAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "feishu")]
impl ToolSchemaRegistry for FeishuToolAdapter {
    fn platform(&self) -> Platform {
        Platform::Feishu
    }

    fn all_tools(&self) -> Vec<ToolSchema> {
        use omini_connect_feishu::FeishuTool;

        FeishuTool::all_tools()
            .into_iter()
            .map(|t| {
                let category = Self::categorize(&t.name);
                ToolSchema {
                    name: t.name,
                    description: t.description,
                    input_schema: t.input_schema,
                    platform: Platform::Feishu,
                    category,
                }
            })
            .collect()
    }
}

/// DingTalk tool schema adapter
pub struct DingTalkToolAdapter;

impl DingTalkToolAdapter {
    pub fn new() -> Self {
        Self
    }

    fn categorize(name: &str) -> Category {
        match name {
            n if n.starts_with("workflow_") => Category::Workflow,
            n if n.starts_with("message_") => Category::Messaging,
            n if n.starts_with("user_") => Category::User,
            _ => Category::Unknown,
        }
    }
}

impl Default for DingTalkToolAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "dingtalk")]
impl ToolSchemaRegistry for DingTalkToolAdapter {
    fn platform(&self) -> Platform {
        Platform::DingTalk
    }

    fn all_tools(&self) -> Vec<ToolSchema> {
        use omini_connect_dingtalk::DingTalkTool;

        DingTalkTool::all_tools()
            .into_iter()
            .map(|t| {
                let category = Self::categorize(&t.name);
                ToolSchema {
                    name: t.name,
                    description: t.description,
                    input_schema: t.input_schema,
                    platform: Platform::DingTalk,
                    category,
                }
            })
            .collect()
    }
}

/// WeChat Work tool schema adapter
pub struct WeChatWorkToolAdapter;

impl WeChatWorkToolAdapter {
    pub fn new() -> Self {
        Self
    }

    fn categorize(name: &str) -> Category {
        match name {
            n if n.starts_with("external_contact_") => Category::Customer,
            n if n.starts_with("message_") => Category::Messaging,
            n if n.starts_with("user_") => Category::User,
            n if n.starts_with("department_") => Category::Department,
            _ => Category::Unknown,
        }
    }
}

impl Default for WeChatWorkToolAdapter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "wechatwork")]
impl ToolSchemaRegistry for WeChatWorkToolAdapter {
    fn platform(&self) -> Platform {
        Platform::WeChatWork
    }

    fn all_tools(&self) -> Vec<ToolSchema> {
        use omini_connect_wechatwork::WeChatWorkTool;

        WeChatWorkTool::all_tools()
            .into_iter()
            .map(|t| {
                let category = Self::categorize(&t.name);
                ToolSchema {
                    name: t.name,
                    description: t.description,
                    input_schema: t.input_schema,
                    platform: Platform::WeChatWork,
                    category,
                }
            })
            .collect()
    }
}
