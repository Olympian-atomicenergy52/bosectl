"""Auto-detect paired BMAP devices via bluetoothctl (Linux)."""

import subprocess

# Bose BMAP service UUID found in SDP records.
BMAP_UUID = "00000000-deca-fade-deca-deafdecacaff"


def find_bmap_device():
    """Auto-detect a paired BMAP-capable Bluetooth device.

    Searches paired Bluetooth devices by checking for the audio-headset
    device class and the BMAP service UUID. Falls back to name matching
    for known manufacturers.

    Returns:
        MAC address string, or None if not found.
    """
    try:
        result = subprocess.run(
            ["bluetoothctl", "devices", "Paired"],
            capture_output=True, text=True, timeout=5,
        )
        for line in result.stdout.strip().splitlines():
            parts = line.split(None, 2)
            if len(parts) < 2:
                continue
            mac = parts[1]
            info = subprocess.run(
                ["bluetoothctl", "info", mac],
                capture_output=True, text=True, timeout=3,
            )
            for info_line in info.stdout.splitlines():
                info_line = info_line.strip()
                if info_line.startswith("Icon: audio-headset"):
                    if BMAP_UUID in info.stdout:
                        return mac
                if "bose" in info_line.lower():
                    return mac
    except (FileNotFoundError, subprocess.TimeoutExpired):
        pass
    return None
