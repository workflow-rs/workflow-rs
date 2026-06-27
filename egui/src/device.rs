use crate::imports::*;

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
/// Screen orientation of the display area.
pub enum Orientation {
    /// Wider than tall.
    #[default]
    Landscape,
    /// Taller than wide (or narrow).
    Portrait,
}

/// Tracks device characteristics (mobile detection, orientation and screen
/// size) used to drive responsive layout decisions.
#[derive(Default, Clone)]
pub struct Device {
    /// `true` when the running platform is detected as a mobile device.
    pub mobile_device: bool,
    /// User/debug override forcing mobile behaviour regardless of detection.
    pub mobile_forced: bool,
    /// Orientation derived from the most recently observed screen size.
    pub orientation: Orientation,
    /// Override that pins the reported orientation when set.
    pub orientation_forced: Option<Orientation>,
    /// Most recently observed screen size, in points.
    pub screen_size: Vec2,
}

impl Device {
    /// Record the current screen size from `rect` and re-derive the orientation,
    /// classifying narrow or sub-540px-wide areas as portrait.
    pub fn set_screen_size(&mut self, rect: &Rect) {
        let size = rect.size();

        if size.x * 3. < size.y * 2. || size.x < 540.0 {
            self.orientation = Orientation::Portrait;
        } else {
            self.orientation = Orientation::Landscape;
        }

        self.screen_size = rect.size();
    }

    /// Returns the effective orientation, preferring a forced override when set
    /// and otherwise the orientation derived from the screen size.
    pub fn orientation(&self) -> Orientation {
        self.orientation_forced.unwrap_or(self.orientation)
    }

    /// Returns `true` if the device is detected as mobile or has been forced
    /// into mobile mode.
    pub fn is_mobile(&self) -> bool {
        self.mobile_device || self.mobile_forced
    }

    /// Toggle a forced portrait orientation: enables it when no override is set,
    /// otherwise clears the override and reverts to auto-detection.
    pub fn toggle_portrait(&mut self) {
        if self.orientation_forced.is_none() {
            self.orientation_forced = Some(Orientation::Portrait);
        } else {
            self.orientation_forced = None;
        }
    }

    /// Toggle the forced-mobile override on or off.
    pub fn toggle_mobile(&mut self) {
        self.mobile_forced = !self.mobile_forced;
    }

    /// Returns `true` when the UI should collapse to a single pane, such as on
    /// mobile (real or forced), in portrait orientation, or inside a Chrome
    /// extension popup.
    pub fn is_single_pane(&self) -> bool {
        workflow_core::runtime::is_chrome_extension()
            || self.mobile_forced
            || self.mobile_device
            || self.orientation() == Orientation::Portrait
    }

    /// Returns `true` when the device should use a multi-pane desktop layout
    /// (i.e. the inverse of [`Device::is_single_pane`]).
    #[inline]
    pub fn is_desktop(&self) -> bool {
        !self.is_single_pane()
    }

    /// Force the reported orientation to a specific value, or pass `None` to
    /// resume using the orientation derived from the screen size.
    pub fn force_orientation(&mut self, orientation: Option<Orientation>) {
        self.orientation_forced = orientation;
    }
}
