use crate::{
    commands::{
        keyhive::{self, AddMemberToGroup, KeyhiveEntityId, MemberAccess, RemoveMemberFromGroup},
        Command,
    },
    io::{self, IoResult},
    Audience, CommandId, Commit, CommitBundle, DocumentId, EndpointId, EndpointResponse,
    OutboundRequestId, SignedMessage, StreamDirection, StreamId,
};

#[derive(Debug)]
pub struct Event(pub(super) EventInner);

impl Event {
    /// A storage task completed
    pub fn io_complete(result: IoResult) -> Event {
        Event(EventInner::IoComplete(result))
    }

    // Submit a new request to be handled
    pub fn handle_request(
        request: SignedMessage,
        receive_audience: Option<String>,
    ) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::HandleRequest {
                request: request.0,
                receive_audience,
            }),
        ));
        (command_id, event)
    }

    // Submit the response to an outgoing request
    pub fn handle_response(
        request_id: OutboundRequestId,
        response: EndpointResponse,
    ) -> (CommandId, Event) {
        let command_id = CommandId::new();
        (
            command_id,
            Event(EventInner::BeginCommand(
                command_id,
                Box::new(Command::HandleResponse {
                    request_id,
                    response: response.0,
                }),
            )),
        )
    }

    // Add some commits to a document
    #[tracing::instrument(skip(commits))]
    pub fn add_commits(root_id: DocumentId, commits: Vec<Commit>) -> (CommandId, Event) {
        let command_id = CommandId::new();
        (
            command_id,
            Event(EventInner::BeginCommand(
                command_id,
                Box::new(Command::AddCommits {
                    doc_id: root_id,
                    commits,
                }),
            )),
        )
    }

    // Create a new document
    pub fn create_doc(
        initial_commit: Commit,
        other_owners: Vec<KeyhiveEntityId>,
    ) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::CreateDoc {
                initial_commit,
                other_owners,
            }),
        ));
        (command_id, event)
    }

    // Load a document from storage
    pub fn load_doc(doc_id: DocumentId) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::LoadDoc {
                doc_id,
                decrypt: true,
            }),
        ));
        (command_id, event)
    }

    pub fn load_doc_encrypted(doc_id: DocumentId) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::LoadDoc {
                doc_id,
                decrypt: false,
            }),
        ));
        (command_id, event)
    }

    // Add a bundle of commits to a document
    pub fn add_bundle(doc: DocumentId, bundle: CommitBundle) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::AddBundle {
                doc_id: doc,
                bundle,
            }),
        ));
        (command_id, event)
    }

    pub fn create_stream(direction: StreamDirection) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::CreateStream(direction)),
        ));
        (command_id, event)
    }

    pub fn disconnect_stream(stream_id: StreamId) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::DisconnectStream { stream_id }),
        ));
        (command_id, event)
    }

    pub fn handle_message(stream_id: StreamId, message: Vec<u8>) -> Event {
        Event(EventInner::StreamMessage(stream_id, message))
    }

    pub fn register_endpoint(audience: Audience) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::RegisterEndpoint(audience)),
        ));
        (command_id, event)
    }

    pub fn unregister_endpoint(endpoint_id: EndpointId) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::UnregisterEndpoints(endpoint_id)),
        ));
        (command_id, event)
    }

    pub fn stop() -> Event {
        let command_id = CommandId::new();
        Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::Stop),
        ))
    }

    pub fn add_member_to_doc(
        doc_id: DocumentId,
        member: KeyhiveEntityId,
        access: MemberAccess,
    ) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::Keyhive(keyhive::KeyhiveCommand::AddMemberToDoc(
                doc_id, member, access,
            ))),
        ));
        (command_id, event)
    }

    pub fn remove_member_from_doc(
        doc_id: DocumentId,
        member: KeyhiveEntityId,
    ) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::Keyhive(
                keyhive::KeyhiveCommand::RemoveMemberFromDoc(doc_id, member),
            )),
        ));
        (command_id, event)
    }

    pub fn query_access(doc_id: DocumentId) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::Keyhive(keyhive::KeyhiveCommand::QueryAccess(
                doc_id,
            ))),
        ));
        (command_id, event)
    }

    pub fn create_group(other_owners: Vec<KeyhiveEntityId>) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::Keyhive(keyhive::KeyhiveCommand::CreateGroup(
                other_owners,
            ))),
        ));
        (command_id, event)
    }

    pub fn add_member_to_group(add: AddMemberToGroup) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::Keyhive(keyhive::KeyhiveCommand::AddMemberToGroup(
                add,
            ))),
        ));
        (command_id, event)
    }

    pub fn remove_member_from_group(remove: RemoveMemberFromGroup) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::Keyhive(
                keyhive::KeyhiveCommand::RemoveMemberFromGroup(remove),
            )),
        ));
        (command_id, event)
    }

    pub fn query_status(doc_id: DocumentId) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::QueryStatus(doc_id)),
        ));
        (command_id, event)
    }

    #[cfg(feature = "debug_events")]
    pub fn log_keyhive_events(
        nicknames: keyhive_core::debug_events::Nicknames,
    ) -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::Keyhive(keyhive::KeyhiveCommand::DebugEvents(
                nicknames,
            ))),
        ));
        (command_id, event)
    }

    pub fn create_contact_card() -> (CommandId, Event) {
        let command_id = CommandId::new();
        let event = Event(EventInner::BeginCommand(
            command_id,
            Box::new(Command::Keyhive(keyhive::KeyhiveCommand::CreateContactCard)),
        ));
        (command_id, event)
    }

    pub fn tick() -> Event {
        Event(EventInner::Tick)
    }
}

#[derive(Debug)]
pub(super) enum EventInner {
    IoComplete(io::IoResult),
    BeginCommand(CommandId, Box<Command>),
    StreamMessage(StreamId, Vec<u8>),
    Tick,
}
