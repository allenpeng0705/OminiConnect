//! Platform-specific OAuth2 implementations.

pub mod dingtalk;
pub mod feishu;
pub mod wechatwork;

pub use dingtalk::DingTalkPlatform;
pub use feishu::FeishuPlatform;
pub use wechatwork::WeChatWorkPlatform;
