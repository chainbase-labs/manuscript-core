use super::MessageTransmitterV2Decoder;
pub mod accept_ownership;
pub mod attester_disabled_event;
pub mod attester_enabled_event;
pub mod attester_manager_updated_event;
pub mod disable_attester;
pub mod enable_attester;
pub mod initialize;
pub mod is_nonce_used;
pub mod max_message_body_size_updated_event;
pub mod message_received_event;
pub mod ownership_transfer_started_event;
pub mod ownership_transferred_event;
pub mod pause;
pub mod pause_event;
pub mod pauser_changed_event;
pub mod receive_message;
pub mod reclaim_event_account;
pub mod send_message;
pub mod set_max_message_body_size;
pub mod set_signature_threshold;
pub mod signature_threshold_updated_event;
pub mod transfer_ownership;
pub mod unpause;
pub mod unpause_event;
pub mod update_attester_manager;
pub mod update_pauser;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum MessageTransmitterV2Instruction {
    AcceptOwnership(accept_ownership::AcceptOwnership),
    DisableAttester(disable_attester::DisableAttester),
    EnableAttester(enable_attester::EnableAttester),
    Initialize(initialize::Initialize),
    IsNonceUsed(is_nonce_used::IsNonceUsed),
    Pause(pause::Pause),
    ReceiveMessage(receive_message::ReceiveMessage),
    ReclaimEventAccount(reclaim_event_account::ReclaimEventAccount),
    SendMessage(send_message::SendMessage),
    SetMaxMessageBodySize(set_max_message_body_size::SetMaxMessageBodySize),
    SetSignatureThreshold(set_signature_threshold::SetSignatureThreshold),
    TransferOwnership(transfer_ownership::TransferOwnership),
    Unpause(unpause::Unpause),
    UpdateAttesterManager(update_attester_manager::UpdateAttesterManager),
    UpdatePauser(update_pauser::UpdatePauser),
    AttesterDisabledEvent(attester_disabled_event::AttesterDisabledEvent),
    AttesterEnabledEvent(attester_enabled_event::AttesterEnabledEvent),
    AttesterManagerUpdatedEvent(attester_manager_updated_event::AttesterManagerUpdatedEvent),
    MaxMessageBodySizeUpdatedEvent(
        max_message_body_size_updated_event::MaxMessageBodySizeUpdatedEvent,
    ),
    MessageReceivedEvent(message_received_event::MessageReceivedEvent),
    OwnershipTransferStartedEvent(ownership_transfer_started_event::OwnershipTransferStartedEvent),
    OwnershipTransferredEvent(ownership_transferred_event::OwnershipTransferredEvent),
    PauseEvent(pause_event::PauseEvent),
    PauserChangedEvent(pauser_changed_event::PauserChangedEvent),
    SignatureThresholdUpdatedEvent(
        signature_threshold_updated_event::SignatureThresholdUpdatedEvent,
    ),
    UnpauseEvent(unpause_event::UnpauseEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for MessageTransmitterV2Decoder {
    type InstructionType = MessageTransmitterV2Instruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            MessageTransmitterV2Instruction::AcceptOwnership => accept_ownership::AcceptOwnership,
            MessageTransmitterV2Instruction::DisableAttester => disable_attester::DisableAttester,
            MessageTransmitterV2Instruction::EnableAttester => enable_attester::EnableAttester,
            MessageTransmitterV2Instruction::Initialize => initialize::Initialize,
            MessageTransmitterV2Instruction::IsNonceUsed => is_nonce_used::IsNonceUsed,
            MessageTransmitterV2Instruction::Pause => pause::Pause,
            MessageTransmitterV2Instruction::ReceiveMessage => receive_message::ReceiveMessage,
            MessageTransmitterV2Instruction::ReclaimEventAccount => reclaim_event_account::ReclaimEventAccount,
            MessageTransmitterV2Instruction::SendMessage => send_message::SendMessage,
            MessageTransmitterV2Instruction::SetMaxMessageBodySize => set_max_message_body_size::SetMaxMessageBodySize,
            MessageTransmitterV2Instruction::SetSignatureThreshold => set_signature_threshold::SetSignatureThreshold,
            MessageTransmitterV2Instruction::TransferOwnership => transfer_ownership::TransferOwnership,
            MessageTransmitterV2Instruction::Unpause => unpause::Unpause,
            MessageTransmitterV2Instruction::UpdateAttesterManager => update_attester_manager::UpdateAttesterManager,
            MessageTransmitterV2Instruction::UpdatePauser => update_pauser::UpdatePauser,
            MessageTransmitterV2Instruction::AttesterDisabledEvent => attester_disabled_event::AttesterDisabledEvent,
            MessageTransmitterV2Instruction::AttesterEnabledEvent => attester_enabled_event::AttesterEnabledEvent,
            MessageTransmitterV2Instruction::AttesterManagerUpdatedEvent => attester_manager_updated_event::AttesterManagerUpdatedEvent,
            MessageTransmitterV2Instruction::MaxMessageBodySizeUpdatedEvent => max_message_body_size_updated_event::MaxMessageBodySizeUpdatedEvent,
            MessageTransmitterV2Instruction::MessageReceivedEvent => message_received_event::MessageReceivedEvent,
            MessageTransmitterV2Instruction::OwnershipTransferStartedEvent => ownership_transfer_started_event::OwnershipTransferStartedEvent,
            MessageTransmitterV2Instruction::OwnershipTransferredEvent => ownership_transferred_event::OwnershipTransferredEvent,
            MessageTransmitterV2Instruction::PauseEvent => pause_event::PauseEvent,
            MessageTransmitterV2Instruction::PauserChangedEvent => pauser_changed_event::PauserChangedEvent,
            MessageTransmitterV2Instruction::SignatureThresholdUpdatedEvent => signature_threshold_updated_event::SignatureThresholdUpdatedEvent,
            MessageTransmitterV2Instruction::UnpauseEvent => unpause_event::UnpauseEvent,
        )
    }
}
