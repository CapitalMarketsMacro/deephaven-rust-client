//! Port of `Dh_NetClient/ticking/BarrageProcessor.cs::CreateSubscriptionRequest`.
//!
//! Builds the nested Barrage flatbuffers used as the DoExchange priming
//! `app_metadata`: an inner `BarrageSubscriptionRequest` wrapped in a
//! `BarrageMessageWrapper` carrying the `dphn` magic and the subscription
//! message type. Field values mirror the C# reference exactly.

use crate::barrage_generated::io::deephaven::barrage::flatbuf as fb;

/// `0x6E687064` — the numerical representation of ASCII "dphn". Port of
/// `BarrageProcessor.DeephavenMagicNumber`.
pub const DEEPHAVEN_MAGIC: u32 = 0x6E68_7064;

/// flatbuffers `[byte]` is signed (`i8`); our ticket/payload bytes are `u8`.
fn to_i8(bytes: &[u8]) -> Vec<i8> {
    bytes.iter().map(|&b| b as i8).collect()
}

// Used by tests now; used by the update-metadata decode path in Phase 5.
#[allow(dead_code)]
fn to_u8(bytes: &[i8]) -> Vec<u8> {
    bytes.iter().map(|&b| b as u8).collect()
}

/// Build the `BarrageMessageWrapper` bytes subscribing to the whole table named
/// by `ticket_bytes`. Omitting `columns`/`viewport` requests all columns and
/// the entire table. Port of `CreateSubscriptionRequest`.
pub fn create_subscription_request(ticket_bytes: &[u8]) -> Vec<u8> {
    // Inner request. Options match the C# call:
    // (use_deephaven_nulls=true, min_update_interval_ms=0, batch_size=4096,
    //  max_message_size=0, columns_as_list=true). The deprecated
    // column_conversion_mode field is dropped by planus.
    let options = fb::BarrageSubscriptionOptions {
        use_deephaven_nulls: true,
        min_update_interval_ms: 0,
        batch_size: 4096,
        max_message_size: 0,
        columns_as_list: true,
        preview_list_length_limit: 0,
    };
    let request = fb::BarrageSubscriptionRequest {
        ticket: Some(to_i8(ticket_bytes)),
        columns: None,
        viewport: None,
        subscription_options: Some(Box::new(options)),
        reverse_viewport: false,
        subscription_token: None,
    };

    let mut payload_builder = planus::Builder::new();
    let payload = payload_builder.finish(&request, None).to_vec();

    // Outer wrapper carrying the payload bytes.
    let wrapper = fb::BarrageMessageWrapper {
        magic: DEEPHAVEN_MAGIC,
        msg_type: fb::BarrageMessageType::BarrageSubscriptionRequest,
        msg_payload: Some(to_i8(&payload)),
    };

    let mut wrapper_builder = planus::Builder::new();
    wrapper_builder.finish(&wrapper, None).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use planus::ReadAsRoot;

    #[test]
    fn subscription_request_round_trips() {
        // A representative export ticket: 'e' + little-endian id 1.
        let ticket = b"e\x01\x00\x00\x00";
        let bytes = create_subscription_request(ticket);

        // Outer wrapper parses back with the right magic and message type.
        let wrapper = fb::BarrageMessageWrapperRef::read_as_root(&bytes)
            .expect("wrapper should parse");
        assert_eq!(wrapper.magic().expect("magic"), DEEPHAVEN_MAGIC);
        assert_eq!(
            wrapper.msg_type().expect("msg_type"),
            fb::BarrageMessageType::BarrageSubscriptionRequest
        );

        // Inner payload parses back and the ticket round-trips.
        let payload = wrapper
            .msg_payload()
            .expect("payload result")
            .expect("payload present");
        let payload_u8 = to_u8(&payload.to_vec());

        let inner = fb::BarrageSubscriptionRequestRef::read_as_root(&payload_u8)
            .expect("inner should parse");
        let ticket_back = inner
            .ticket()
            .expect("ticket result")
            .expect("ticket present");
        assert_eq!(to_u8(&ticket_back.to_vec()), ticket);

        // Subscription options carried the ported values.
        let opts = inner
            .subscription_options()
            .expect("options result")
            .expect("options present");
        assert!(opts.use_deephaven_nulls().expect("nulls"));
        assert_eq!(opts.batch_size().expect("batch"), 4096);
        assert!(opts.columns_as_list().expect("cols"));
    }
}
