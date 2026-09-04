//! Popup VFX (visual effects) for the chat composer.
//! Handles wink (show) and poof (hide) animations when popups appear/disappear.

use std::time::Duration;

/// Duration of one animation frame.
const VFX_FRAME_INTERVAL: Duration = Duration::from_millis(33); // ~30fps

/// Number of frames for wink animation (show).
const WINK_FRAMES: u8 = 6; // ~200ms

/// Number of frames for poof animation (hide).
const POOF_FRAMES: u8 = 9; // ~300ms

/// VFX animation state for popup transitions.
#[derive(Debug, Clone, Default)]
pub(super) enum PopupVfx {
    #[default]
    None,
    /// White wink effect when popup appears.
    Wink {
        frame: u8,
        max_frames: u8,
    },
    /// Pink poof effect when popup disappears.
    Poof {
        frame: u8,
        max_frames: u8,
    },
}

impl PopupVfx {
    /// Create a new wink animation.
    pub(super) fn wink() -> Self {
        Self::Wink {
            frame: 0,
            max_frames: WINK_FRAMES,
        }
    }

    /// Create a new poof animation.
    pub(super) fn poof() -> Self {
        Self::Poof {
            frame: 0,
            max_frames: POOF_FRAMES,
        }
    }

    /// Advance the animation by one frame. Returns true if animation is still running.
    pub(super) fn tick(&mut self) -> bool {
        match self {
            Self::None => false,
            Self::Wink { frame, max_frames } => {
                *frame += 1;
                *frame < *max_frames
            }
            Self::Poof { frame, max_frames } => {
                *frame += 1;
                *frame < *max_frames
            }
        }
    }

    /// Returns true if animation is active.
    pub(super) fn is_active(&self) -> bool {
        !matches!(self, Self::None)
    }

    /// Get the frame interval.
    pub(super) fn frame_interval() -> Duration {
        VFX_FRAME_INTERVAL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vfx_none_is_not_active() {
        let vfx = PopupVfx::None;
        assert!(!vfx.is_active());
    }

    #[test]
    fn vfx_wink_animation() {
        let mut vfx = PopupVfx::wink();
        assert!(vfx.is_active());

        // Tick through all frames
        for i in 0..WINK_FRAMES {
            assert!(vfx.is_active());
            let running = vfx.tick();
            if i < WINK_FRAMES - 1 {
                assert!(running);
            } else {
                assert!(!running);
            }
        }

        assert!(!vfx.is_active());
    }

    #[test]
    fn vfx_poof_animation() {
        let mut vfx = PopupVfx::poof();
        assert!(vfx.is_active());

        // Tick through all frames
        for i in 0..POOF_FRAMES {
            assert!(vfx.is_active());
            let running = vfx.tick();
            if i < POOF_FRAMES - 1 {
                assert!(running);
            } else {
                assert!(!running);
            }
        }

        assert!(!vfx.is_active());
    }
}
