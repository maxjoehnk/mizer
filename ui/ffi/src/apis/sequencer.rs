use std::sync::Arc;
use crate::types::{Array, drop_pointer, FFIFromPointer};
use mizer_sequencer::SequencerView;

pub struct Sequencer {
    pub view: SequencerView,
}

#[derive(Debug)]
#[repr(C)]
pub struct SequenceState {
    pub sequence_id: u32,
    pub active: u8,
    pub current_cue_id: u32
}

#[no_mangle]
pub extern fn read_sequencer_state(ptr: *const Sequencer) -> Array<SequenceState> {
    let ffi = Arc::from_pointer(ptr);
    let values = ffi.view.read();
    let values = values
        .into_iter()
        .map(|(id, sequence)| SequenceState {
            sequence_id: id,
            active: if sequence.active { 1 } else { 0 },
            current_cue_id: sequence.cue_id.unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    std::mem::forget(ffi);

    values.into()
}

#[no_mangle]
pub extern fn drop_sequencer_pointer(ptr: *const Sequencer) {
    drop_pointer(ptr);
}
