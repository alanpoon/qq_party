mod generated;
extern crate wapc_guest as guest;
pub use generated::*;
use guest::prelude::*;

fn gift_request(_request: GiftRequest) -> HandlerResult<GiftResponse> {
    Ok(GiftResponse::default()) // TODO: Provide implementation.
}

fn message_request(_request: MessageRequest) -> HandlerResult<MessageResponse> {
    Ok(MessageResponse::default()) // TODO: Provide implementation.
}

fn join_room_request(_request: JoinRoomRequest) -> HandlerResult<()> {
    Ok(()) // TODO: Provide implementation.
}
