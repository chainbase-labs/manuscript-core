use {
    super::MessageTransmitterV2Decoder,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod message_sent;
pub mod message_transmitter;
pub mod used_nonce;

pub enum MessageTransmitterV2Account {
    MessageSent(message_sent::MessageSent),
    MessageTransmitter(message_transmitter::MessageTransmitter),
    UsedNonce(used_nonce::UsedNonce),
}

impl AccountDecoder<'_> for MessageTransmitterV2Decoder {
    type AccountType = MessageTransmitterV2Account;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            message_sent::MessageSent::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MessageTransmitterV2Account::MessageSent(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            message_transmitter::MessageTransmitter::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MessageTransmitterV2Account::MessageTransmitter(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = used_nonce::UsedNonce::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MessageTransmitterV2Account::UsedNonce(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
