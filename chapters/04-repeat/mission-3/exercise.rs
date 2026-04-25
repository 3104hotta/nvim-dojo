// Combined repeat practice
// A: yap + 4p to duplicate fn process, then rename
// B: macro to convert log::debug! -> tracing::debug! (8 lines)
// C: ci{ + dot to add Result to use imports (3 lines)

use crate::error::Error;
use crate::metrics::Counter;
use crate::trace::Span;

pub fn process(input: &[u8]) -> Result<Vec<u8>, Error> {
    if input.is_empty() {
        return Err(Error::empty_input());
    }
    let output: Vec<u8> = input.iter().map(|&b| b.wrapping_add(1)).collect();
    Ok(output)
}

pub fn emit_events(ids: &[u64]) {
    log::debug!("msg_1: processing {} events", ids.len());
    log::debug!("msg_2: first id={}", ids.first().unwrap_or(&0));
    log::debug!("msg_3: last id={}", ids.last().unwrap_or(&0));
    log::debug!("msg_4: span start");
    log::debug!("msg_5: acquiring lock");
    log::debug!("msg_6: lock acquired");
    log::debug!("msg_7: processing done");
    log::debug!("msg_8: span end");
}
