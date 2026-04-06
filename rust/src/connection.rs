//! High-level BMAP device connection.

use crate::device::*;
use crate::error::{BmapError, BmapResult};
use crate::protocol::{Operator, BmapResponse, bmap_packet, parse_response, parse_all_responses};
use crate::transport::RfcommTransport;

/// High-level connection to a BMAP device.
pub struct BmapConnection {
    transport: RfcommTransport,
    config: DeviceConfig,
}

impl BmapConnection {
    /// Create a connection from an already-connected transport and device config.
    pub fn new(transport: RfcommTransport, config: DeviceConfig) -> Self {
        Self { transport, config }
    }

    /// Device configuration.
    pub fn config(&self) -> &DeviceConfig {
        &self.config
    }

    // ── Helpers ─────────────────────────────────────────────────────────────

    fn addr(&self, feature: Option<Addr>) -> BmapResult<Addr> {
        feature.ok_or_else(|| BmapError::Unsupported(
            format!("{} does not support this feature", self.config.info.name)
        ))
    }

    fn get(&self, addr: Addr) -> BmapResult<Vec<u8>> {
        let pkt = bmap_packet(addr.0, addr.1, Operator::Get, &[]);
        let data = self.transport.send_recv(&pkt)?;
        let resp = parse_response(&data)
            .ok_or_else(|| BmapError::Timeout("Empty response".into()))?;
        self.check_error(&resp)?;
        Ok(resp.payload)
    }

    fn setget(&self, addr: Addr, payload: &[u8]) -> BmapResult<BmapResponse> {
        let pkt = bmap_packet(addr.0, addr.1, Operator::SetGet, payload);
        let data = self.transport.send_recv(&pkt)?;
        let resp = parse_response(&data)
            .ok_or_else(|| BmapError::Timeout("Empty response".into()))?;
        self.check_error(&resp)?;
        Ok(resp)
    }

    fn start(&self, addr: Addr, payload: &[u8]) -> BmapResult<BmapResponse> {
        let pkt = bmap_packet(addr.0, addr.1, Operator::Start, payload);
        let data = self.transport.send_recv(&pkt)?;
        let resp = parse_response(&data)
            .ok_or_else(|| BmapError::Timeout("Empty response".into()))?;
        self.check_error(&resp)?;
        Ok(resp)
    }

    /// Send START and drain all async responses.
    pub fn start_drain(&self, addr: Addr, payload: &[u8]) -> BmapResult<Vec<BmapResponse>> {
        let pkt = bmap_packet(addr.0, addr.1, Operator::Start, payload);
        let data = self.transport.send_recv_drain(&pkt)?;
        Ok(parse_all_responses(&data))
    }

    fn check_error(&self, resp: &BmapResponse) -> BmapResult<()> {
        if resp.op == Operator::Error && !resp.payload.is_empty() {
            let code = resp.payload[0];
            if code == 5 {
                return Err(BmapError::Auth(resp.fmt()));
            }
            return Err(BmapError::Device {
                message: resp.fmt(),
                code,
            });
        }
        Ok(())
    }

    // ── Read Operations ─────────────────────────────────────────────────────

    /// Battery percentage.
    pub fn battery(&self) -> BmapResult<u8> {
        let addr = self.addr(self.config.battery)?;
        let payload = self.get(addr)?;
        parse_battery(&payload).ok_or_else(|| BmapError::Device {
            message: "Empty battery response".into(), code: 0,
        })
    }

    /// Firmware version string.
    pub fn firmware(&self) -> BmapResult<String> {
        let addr = self.addr(self.config.firmware)?;
        let payload = self.get(addr)?;
        Ok(parse_firmware(&payload))
    }

    /// Device Bluetooth name.
    pub fn name(&self) -> BmapResult<String> {
        let addr = self.addr(self.config.product_name)?;
        let payload = self.get(addr)?;
        Ok(parse_product_name(&payload))
    }

    /// Current audio mode index.
    pub fn mode_idx(&self) -> BmapResult<u8> {
        let addr = self.addr(self.config.current_mode)?;
        let payload = self.get(addr)?;
        payload.first().copied().ok_or_else(|| BmapError::Device {
            message: "Empty mode response".into(), code: 0,
        })
    }

    /// Current audio mode name.
    pub fn mode(&self) -> BmapResult<String> {
        let idx = self.mode_idx()?;
        for &(name, ref preset) in self.config.preset_modes {
            if preset.idx == idx {
                return Ok(name.to_string());
            }
        }
        Ok(format!("custom({})", idx))
    }

    /// Noise cancellation (current, max) tuple.
    pub fn cnc(&self) -> BmapResult<(u8, u8)> {
        let addr = self.addr(self.config.cnc)?;
        let payload = self.get(addr)?;
        Ok(parse_cnc(&payload))
    }

    /// EQ bands.
    pub fn eq(&self) -> BmapResult<Vec<EqBand>> {
        let addr = self.addr(self.config.eq)?;
        let payload = self.get(addr)?;
        Ok(parse_eq(&payload))
    }

    /// Sidetone level name.
    pub fn sidetone(&self) -> BmapResult<&'static str> {
        let addr = self.addr(self.config.sidetone)?;
        let payload = self.get(addr)?;
        Ok(parse_sidetone(&payload))
    }

    /// Multipoint enabled.
    pub fn multipoint(&self) -> BmapResult<bool> {
        let addr = self.addr(self.config.multipoint)?;
        let payload = self.get(addr)?;
        Ok(parse_multipoint(&payload))
    }

    /// Auto play/pause enabled.
    pub fn auto_pause(&self) -> BmapResult<bool> {
        let addr = self.addr(self.config.auto_pause)?;
        let payload = self.get(addr)?;
        Ok(parse_bool(&payload))
    }

    /// Voice prompts (enabled, language_name).
    pub fn prompts(&self) -> BmapResult<(bool, &'static str)> {
        let addr = self.addr(self.config.voice_prompts)?;
        let payload = self.get(addr)?;
        Ok(parse_voice_prompts(&payload))
    }

    /// Button mapping.
    pub fn buttons(&self) -> BmapResult<ButtonMapping> {
        let addr = self.addr(self.config.buttons)?;
        let payload = self.get(addr)?;
        parse_buttons(&payload).ok_or_else(|| BmapError::Device {
            message: "Could not parse button config".into(), code: 0,
        })
    }

    /// Full device status.
    pub fn status(&self) -> BmapResult<DeviceStatus> {
        let (cnc_level, cnc_max) = self.cnc().unwrap_or((0, 10));
        let (prompts_enabled, prompts_language) = self.prompts().unwrap_or((false, "Unknown"));

        Ok(DeviceStatus {
            battery: self.battery()?,
            mode: self.mode()?,
            mode_idx: self.mode_idx()?,
            cnc_level,
            cnc_max,
            eq: self.eq().unwrap_or_default(),
            name: self.name().unwrap_or_default(),
            firmware: self.firmware().unwrap_or_default(),
            sidetone: self.sidetone().unwrap_or("off").to_string(),
            multipoint: self.multipoint().unwrap_or(false),
            auto_pause: self.auto_pause().unwrap_or(false),
            prompts_enabled,
            prompts_language: prompts_language.to_string(),
        })
    }

    // ── Write Operations ────────────────────────────────────────────────────

    /// Switch to a preset mode by name.
    pub fn set_mode(&self, name: &str) -> BmapResult<()> {
        let addr = self.addr(self.config.current_mode)?;
        let idx = self.config.preset_modes.iter()
            .find(|&&(n, _)| n.eq_ignore_ascii_case(name))
            .map(|&(_, ref m)| m.idx)
            .ok_or_else(|| BmapError::InvalidArg(format!("Unknown mode: {}", name)))?;

        let pkt = bmap_packet(addr.0, addr.1, Operator::Start, &[idx, 0]);
        let data = self.transport.send_recv(&pkt)?;
        let resp = parse_response(&data)
            .ok_or_else(|| BmapError::Timeout("No response".into()))?;
        self.check_error(&resp)?;
        if resp.op != Operator::Result {
            return Err(BmapError::Device { message: "Mode switch failed".into(), code: 0 });
        }
        Ok(())
    }

    /// Set EQ bands (-10 to +10 each).
    pub fn set_eq(&self, bass: i8, mid: i8, treble: i8) -> BmapResult<()> {
        let addr = self.addr(self.config.eq)?;
        for (band_id, val) in [(0u8, bass), (1, mid), (2, treble)] {
            let payload = [val as u8, band_id];
            let pkt = bmap_packet(addr.0, addr.1, Operator::SetGet, &payload);
            self.transport.send_recv(&pkt)?;
        }
        Ok(())
    }

    /// Set device name.
    pub fn set_name(&self, name: &str) -> BmapResult<()> {
        let addr = self.addr(self.config.product_name)?;
        self.setget(addr, name.as_bytes())?;
        Ok(())
    }

    /// Toggle multipoint.
    pub fn set_multipoint(&self, enabled: bool) -> BmapResult<()> {
        let addr = self.addr(self.config.multipoint)?;
        self.setget(addr, &[if enabled { 1 } else { 0 }])?;
        Ok(())
    }

    /// Toggle auto play/pause.
    pub fn set_auto_pause(&self, enabled: bool) -> BmapResult<()> {
        let addr = self.addr(self.config.auto_pause)?;
        self.setget(addr, &[if enabled { 1 } else { 0 }])?;
        Ok(())
    }

    /// Set sidetone level.
    pub fn set_sidetone(&self, level: &str) -> BmapResult<()> {
        let addr = self.addr(self.config.sidetone)?;
        let val = match level {
            "off" => 0u8,
            "high" => 1,
            "medium" | "med" => 2,
            "low" => 3,
            _ => return Err(BmapError::InvalidArg("Sidetone: off, low, medium, high".into())),
        };
        self.setget(addr, &[1, val])?;
        Ok(())
    }

    /// Power off device.
    pub fn power_off(&self) -> BmapResult<()> {
        let addr = self.addr(self.config.power)?;
        self.start(addr, &[0x00])?;
        Ok(())
    }

    /// Enter pairing mode.
    pub fn pair(&self) -> BmapResult<()> {
        let addr = self.addr(self.config.pairing)?;
        self.start(addr, &[0x01])?;
        Ok(())
    }

    /// Send raw hex bytes. Returns all responses.
    pub fn send_raw(&self, data: &[u8]) -> BmapResult<Vec<BmapResponse>> {
        let resp = self.transport.send_recv_drain(data)?;
        Ok(parse_all_responses(&resp))
    }
}
