// Code Example: Smart Home Integration
// The following code illustrates how to adapt a third-party legacy thermodynamic device interface
// into a unified, clean smart home system interface.

// 1. The Target Trait (The interface your application expects)
pub trait SmartDevice {
    fn get_status(&self) -> String;
}

// 2. The Adaptee (Third-party library code with an incompatible interface)
pub struct LegacyThermostat;

impl LegacyThermostat {
    pub fn current_temperature(&self) -> f32 {
        22.5
    }
}

// 3. The Adapter (Wraps the incompatible type)
pub struct ThermostatAdapter {
    pub legacy_device: LegacyThermostat,
}

// 4. Implement the Target Trait for the Adapter
impl SmartDevice for ThermostatAdapter {
    fn get_status(&self) -> String {
        // Translate the incompatible interface into the expected output
        let temp = self.legacy_device.current_temperature();
        format!("Temperature is stable at {}°C.", temp)
    }
}

fn main() {
    // Instantiate the incompatible third-party device
    let legacy_device = LegacyThermostat;

    // Wrap it inside our adapter
    let adapter = ThermostatAdapter { legacy_device };

    // The client interacts cleanly with the expected interface
    println!("{}", adapter.get_status());
    // Output: Temperature is stable at 22.5°C.
}
