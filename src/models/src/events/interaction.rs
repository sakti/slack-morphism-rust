use rsb_derive::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::blocks::*;
use crate::common::*;
use crate::messages::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackInteractionEvent {
    #[serde(rename = "block_actions")]
    BlockActions(SlackInteractionBlockActionsEvent),
    #[serde(rename = "dialog_submission")]
    DialogSubmission(SlackInteractionDialogueSubmissionEvent),
    #[serde(rename = "message_action")]
    MessageAction(SlackInteractionMessageActionEvent),
    #[serde(rename = "shortcut")]
    Shortcut(SlackInteractionShortcutEvent),
    #[serde(rename = "view_submission")]
    ViewSubmission(SlackInteractionViewSubmissionEvent),
    #[serde(rename = "view_closed")]
    ViewClosed(SlackInteractionViewClosedEvent),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionBlockActionsEvent {
    pub team: SlackBasicTeamInfo,
    pub user: Option<SlackBasicUserInfo>,
    pub api_app_id: SlackAppId,
    pub container: SlackInteractionActionContainer,
    pub trigger_id: SlackTriggerId,
    pub channel: Option<SlackBasicChannelInfo>,
    pub message: Option<SlackHistoryMessage>,
    pub view: Option<SlackView>,
    pub response_url: Option<SlackResponseUrl>,
    pub actions: Option<Vec<SlackInteractionActionInfo>>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackInteractionActionContainer {
    #[serde(rename = "message")]
    Message(SlackInteractionActionMessageContainer),
    #[serde(rename = "view")]
    View(SlackInteractionActionViewContainer),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionActionMessageContainer {
    pub message_ts: SlackTs,
    pub channel_id: Option<SlackChannelId>,
    pub is_ephemeral: Option<bool>,
    pub is_app_unfurl: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionActionViewContainer {
    pub view_id: SlackViewId,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionActionInfo {
    #[serde(rename = "type")]
    action_type: SlackActionType,
    action_id: SlackActionId,
    block_id: Option<SlackBlockId>,
    text: Option<SlackBlockText>,
    value: Option<String>,
    selected_option: Option<SlackBlockChoiceItem<SlackBlockText>>,
    action_ts: Option<SlackTs>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionDialogueSubmissionEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub channel: Option<SlackBasicChannelInfo>,
    #[serde(with = "serde_with::rust::string_empty_as_none")]
    pub callback_id: Option<SlackCallbackId>,
    state: Option<String>,
    submission: HashMap<String, String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionMessageActionEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub channel: Option<SlackBasicChannelInfo>,
    pub message: Option<SlackHistoryMessage>,
    pub callback_id: SlackCallbackId,
    pub trigger_id: SlackTriggerId,
    pub response_url: SlackResponseUrl,
    pub actions: Option<Vec<SlackInteractionActionInfo>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionShortcutEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub callback_id: SlackCallbackId,
    pub trigger_id: SlackTriggerId,
    pub actions: Option<Vec<SlackInteractionActionInfo>>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionViewSubmissionEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub view: SlackStatefulView,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Builder)]
pub struct SlackInteractionViewClosedEvent {
    pub team: SlackBasicTeamInfo,
    pub user: SlackBasicUserInfo,
    pub view: SlackStatefulView,
}
