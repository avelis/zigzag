//! A Windows bitmap (24-bit, uncompressed): the preview picture Bernina
//! (and bernette) machines show for an EXP, `design.bmp`.

/// `rgba` (`w` × `h`, premultiplied alpha, top row first) as a BMP, laid
/// over `background` where it isn't opaque.
pub fn write_bmp(w: u32, h: u32, rgba: &[u8], background: [u8; 3]) -> Vec<u8> {
    let row = (w as usize * 3).div_ceil(4) * 4;
    let pixels = row * h as usize;
    let mut out = Vec::with_capacity(54 + pixels);
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&((54 + pixels) as u32).to_le_bytes());
    out.extend_from_slice(&[0; 4]);
    out.extend_from_slice(&54u32.to_le_bytes());
    out.extend_from_slice(&40u32.to_le_bytes());
    out.extend_from_slice(&(w as i32).to_le_bytes());
    out.extend_from_slice(&(h as i32).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&24u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&(pixels as u32).to_le_bytes());
    // 72 dpi.
    out.extend_from_slice(&2835i32.to_le_bytes());
    out.extend_from_slice(&2835i32.to_le_bytes());
    out.extend_from_slice(&[0; 8]);
    // Bottom row first, blue-green-red, each row padded to 4 bytes.
    for y in (0..h as usize).rev() {
        let start = out.len();
        for x in 0..w as usize {
            let p = &rgba[(y * w as usize + x) * 4..][..4];
            let over =
                |c: u8, bg: u8| (c as u32 + (255 - p[3] as u32) * bg as u32 / 255).min(255) as u8;
            out.extend_from_slice(&[
                over(p[2], background[2]),
                over(p[1], background[1]),
                over(p[0], background[0]),
            ]);
        }
        out.resize(start + row, 0);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bitmap_is_bottom_row_first_bgr_with_padded_rows() {
        // 2 × 2: red, green on top; blue, transparent below.
        let rgba = [
            255, 0, 0, 255, 0, 255, 0, 255, //
            0, 0, 255, 255, 0, 0, 0, 0,
        ];
        let b = write_bmp(2, 2, &rgba, [10, 20, 30]);
        assert_eq!(&b[0..2], b"BM");
        assert_eq!(
            u32::from_le_bytes(b[2..6].try_into().unwrap()) as usize,
            b.len()
        );
        assert_eq!(u32::from_le_bytes(b[10..14].try_into().unwrap()), 54);
        assert_eq!(i32::from_le_bytes(b[18..22].try_into().unwrap()), 2);
        assert_eq!(u16::from_le_bytes([b[28], b[29]]), 24);
        // Rows of 6 bytes padded to 8: the bottom row (blue, background) first.
        assert_eq!(b.len(), 54 + 8 * 2);
        assert_eq!(&b[54..62], &[255, 0, 0, 30, 20, 10, 0, 0]);
        assert_eq!(&b[62..70], &[0, 0, 255, 0, 255, 0, 0, 0]);
    }
}
