//! Auto-detect paired BMAP devices via bluetoothctl (Linux).

use std::process::Command;

/// BMAP service UUID found in SDP records.
pub const BMAP_UUID: &str = "00000000-deca-fade-deca-deafdecacaff";

/// Auto-detect a paired BMAP-capable Bluetooth device.
///
/// Returns the MAC address string, or None if not found.
pub fn find_bmap_device() -> Option<String> {
    let output = Command::new("bluetoothctl")
        .args(["devices", "Paired"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() < 2 {
            continue;
        }
        let mac = parts[1];

        let info = Command::new("bluetoothctl")
            .args(["info", mac])
            .output()
            .ok()?;
        let info_str = String::from_utf8_lossy(&info.stdout);

        for info_line in info_str.lines() {
            let trimmed = info_line.trim();
            if trimmed.starts_with("Icon: audio-headset") && info_str.contains(BMAP_UUID) {
                return Some(mac.to_string());
            }
            if trimmed.to_lowercase().contains("bose") {
                return Some(mac.to_string());
            }
        }
    }
    None
}
