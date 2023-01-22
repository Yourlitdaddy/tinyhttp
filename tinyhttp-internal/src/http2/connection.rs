use super::frame::HTTP2Frame;

pub(crate) fn parse_data_frame(data_arr: &[u8]) -> Result<HTTP2Frame, Box<dyn std::error::Error>> {
    let mut data = data_arr.to_vec();
    log::trace!(
        "HTTP2 -> FIRST 24 bytes: {}",
        std::str::from_utf8(&data[0..=23]).unwrap()
    );
    data.drain(0..=23);
    log::trace!("HTTP2 -> ENTIRE FRAME IN u8: {:#?}", data);
    //let length: u32 = u32::from_be_bytes(data[0..2].try_into()?);
    let mut length_vec: Vec<u8> = data[0..=3].to_vec();
    length_vec.pop();
    let length: u32 = u32::from_be_bytes([vec![0u8], length_vec].concat().as_slice().try_into()?);
    log::trace!("HTTP2 -> LENGTH: {}", length);
    let frame_type = data[3];

    log::trace!("HTTP2 -> frame_type: {}", frame_type);

    let flags = data[4];
    let end_stream = flags & 0x01 != 0;
    let end_padding = flags & 0x08 != 0;
    log::debug!(
        "HTTP2 -> FLAGS: {}, len of preface: {}",
        flags,
        b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n".len()
    );

    let reserved = data[5] & (1 << 7);

    let stream_id = u32::from_be_bytes(data[5..=8].try_into()?);
    log::debug!("HTTP2 -> STREAM ID: {}", stream_id);
    // if length != data.len() as u32 - 9 {
    //     return Err("Invalid frame length".into());
    // }
    // Check if the frame type is valid
    if frame_type > 0x12 {
        return Err("Invalid frame type".into());
    }
    // Check if the stream id is valid
    // if (stream_id & 0x80000000) == 0 {
    //     return Err("Invalid stream id".into());
    // }

    let payload = &data[9..=length as usize];

    log::debug!("HTTP2 -> PAYLOAD:{}", std::str::from_utf8(payload).unwrap());

    Ok(HTTP2Frame::new()
        .frame_type(frame_type)
        .payload(payload.to_vec())
        .stream_id(stream_id))
}
