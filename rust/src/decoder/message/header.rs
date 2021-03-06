//! Decoder for field headers

use super::state::State;
use crate::{
    decoder::{vint64, Event},
    error::{self, Error},
    field::{Header, Tag},
};

/// Decoder for field headers
#[derive(Default, Debug)]
pub(super) struct Decoder(vint64::Decoder);

impl Decoder {
    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning the new state.
    pub fn decode<'a>(
        mut self,
        input: &mut &'a [u8],
        last_tag: Option<Tag>,
    ) -> Result<(State, Option<Event<'a>>), Error> {
        if let Some(value) = self.0.decode(input)? {
            let header = Header::from(value);

            // Ensure field ordering is monotonically increasing
            if let Some(tag) = last_tag {
                if header.tag <= tag {
                    return Err(error::Kind::Order { tag: header.tag }.into());
                }
            }

            let event = Event::FieldHeader(header);
            let new_state = State::transition(&event);
            Ok((new_state, Some(event)))
        } else {
            Ok((State::Header(self), None))
        }
    }
}
