use std::ops::BitOr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CastleRights(u8);

impl CastleRights {
    /// None.
    #[allow(non_upper_case_globals)]
    pub const None: Self = Self(0);
    /// White pieces kingside castling.
    #[allow(non_upper_case_globals)]
    pub const WhiteKS: Self = Self(0x01);
    /// White pieces queenside castling.
    #[allow(non_upper_case_globals)]
    pub const WhiteQS: Self = Self(0x02);
    /// Black pieces kingside castling.
    #[allow(non_upper_case_globals)]
    pub const BlackKS: Self = Self(0x04);
    /// Black pieces queenside castling.
    #[allow(non_upper_case_globals)]
    pub const BlackQS: Self = Self(0x08);
    /// Both colors have all castling rights.
    #[allow(non_upper_case_globals)]
    pub const All: Self = Self(0x0f);
}

impl BitOr for CastleRights {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl CastleRights {
    pub fn contains(self, flag: Self) -> bool {
        self.0 & flag.0 == flag.0
    }
}

#[test]
fn castle_rights_bitflags() {
    assert_eq!(
        CastleRights::None | CastleRights::WhiteKS | CastleRights::WhiteQS | CastleRights::BlackKS | CastleRights::BlackQS,
        CastleRights::All
    );
    assert!((CastleRights::WhiteKS | CastleRights::BlackQS).contains(CastleRights::WhiteKS));
    assert!(!CastleRights::WhiteKS.contains(CastleRights::All));
}
