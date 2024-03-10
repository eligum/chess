use crate::bitboard::Square;

// pub const slider_attack_offsets: &'static [Vec<i32>] = compute_offsets();

const DIRECTION_OFFSETS: &[i32] = &[1, 9, 8, 7, -1, -9, -8, -7];

pub fn compute_offsets() -> Vec<Vec<i32>> {
    let mut attacks = Vec::with_capacity(64);
    for idx in 0..64 {
        attacks[idx] = Vec::new();
        for offset in DIRECTION_OFFSETS {
            let r = idx as i32 + offset;
            if 0 <= r && r < 64{
                attacks[idx].push(r);
            }
        }
    }

    attacks
}
