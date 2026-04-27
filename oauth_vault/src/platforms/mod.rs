//! Platform-specific OAuth2 implementations.

pub mod dingtalk;
pub mod facebook;
pub mod feishu;
pub mod linkedin;
pub mod wechatwork;
pub mod x;

pub use dingtalk::DingTalkPlatform;
pub use facebook::FacebookPlatform;
pub use feishu::FeishuPlatform;
pub use linkedin::LinkedInPlatform;
pub use wechatwork::WeChatWorkPlatform;
pub use x::XPlatform;
