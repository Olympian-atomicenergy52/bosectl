"""Bose QC35 / QC35 II device configuration.

Qualcomm CSR8670 platform.
No ECDH auth — all operators accessible without cloud auth.

This is a stub based on known information from based-connect and other
RE projects. Payload formats and feature availability need verification
against real hardware.

TODO: Connect QC35 via Bluetooth and verify/complete this config.
"""

from . import parsers

DEVICE_INFO = {
    "name": "Bose QuietComfort 35",
    "codename": "baywolf",  # Verify
    "platform": "CSR8670",
    "product_id": None,  # TODO: determine from device
    "variant": None,
}

# Feature map — addresses based on based-connect and community RE.
# Many of these need verification on real hardware.
FEATURES = {
    "battery": {
        "addr": (2, 2),
        "parser": parsers.parse_battery,
    },
    "firmware": {
        "addr": (0, 5),
        "parser": parsers.parse_firmware,
    },
    "product_name": {
        "addr": (1, 2),
        "parser": parsers.parse_product_name,
    },
    "voice_prompts": {
        "addr": (1, 3),
        "parser": parsers.parse_voice_prompts,
        "builder": parsers.build_voice_prompts,
    },
    # QC35 has 3-level ANC (high/low/off), not a 0-10 CNC slider.
    # The function block address and payload format may differ.
    "cnc": {
        "addr": (1, 5),  # TODO: verify
        "parser": parsers.parse_cnc,
    },
    "multipoint": {
        "addr": (1, 10),  # TODO: verify
        "parser": parsers.parse_multipoint,
        "builder": parsers.build_toggle,
    },
    "auto_pause": {
        "addr": (1, 24),  # TODO: verify
        "parser": parsers.parse_bool,
        "builder": parsers.build_toggle,
    },
    "pairing": {
        "addr": (4, 8),  # TODO: verify
    },
    "power": {
        "addr": (7, 4),  # TODO: verify
    },
    # QC35 does NOT have these features:
    # - eq (no EQ control)
    # - spatial (no spatial audio)
    # - sidetone
    # - auto_answer
    # - mode_config / AudioModes block 31 (uses different ANC approach)
    # - buttons (no configurable shortcut button)
}

PRESET_MODES = {
    "high":  {"idx": 0, "description": "High — full noise cancellation"},
    "low":   {"idx": 1, "description": "Low — reduced noise cancellation"},
    "off":   {"idx": 2, "description": "Off — no noise cancellation"},
}

MODE_BY_IDX = {m["idx"]: name for name, m in PRESET_MODES.items()}

EDITABLE_SLOTS = []  # QC35 has no editable mode slots

STATUS_OFFSETS = {}  # No mode config on QC35
