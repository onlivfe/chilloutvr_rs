use serde::{Deserialize, Serialize};

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Deserialize,
	Serialize,
	strum::Display,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
#[non_exhaustive]
pub enum ResponseType {
	MenuPopup = 0,
	HudMessage = 1,
	PushNotification = 2,
	OnlineFriends = 10,
	Invites = 15,
	RequestInvites = 20,
	FriendRequest = 25,
}

#[derive(
	Debug,
	Clone,
	PartialEq,
	Eq,
	Hash,
	Deserialize,
	Serialize,
	strum::Display,
	strum::AsRefStr,
	strum::EnumVariantNames,
)]
#[non_exhaustive]
pub enum RequestType {
	SelfOnline = 0,
	FriendRequestSend = 5,
	FriendRequestAccept = 6,
	FriendRequestDecline = 7,
	UnFriend = 8,
	InviteSend = 10,
	InviteExpire = 11,
	RequestInvite = 15,
	RequestInviteAccept = 16,
	RequestInviteDecline = 17,
	BlockUser = 30,
	UnBlockUser = 31,
	GlobalMessage = 100,
	UserMessage = 101,
	SystemGlobalAnnouncement = 110,
	SystemUserAnnouncement = 111,
}
