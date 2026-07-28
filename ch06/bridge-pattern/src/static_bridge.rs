// Remotes and Devices
//
// Imagine a scenario where you have multiple types of remote controls (Basic, Advanced) and
// multiple types of devices (TV, Radio). Instead of creating a matrix of classes like TvBasicRemote
// and RadioAdvancedRemote, the Bridge pattern keeps them separate.

// 1. The Implementation Trait (Low-level interface)
trait Device {
  fn is_enabled(&self) -> bool;
  fn enable(&mut self);
  fn disable(&mut self);
  fn set_volume(&mut self, volume: u8);
  fn status(&self); // Added for easy demonstration
}

// Concrete Implementation A: TV
struct Tv {
  on: bool,
  volume: u8,
}

impl Device for Tv {
  fn is_enabled(&self) -> bool {
    self.on
  }
  fn enable(&mut self) {
    self.on = true;
  }
  fn disable(&mut self) {
    self.on = false;
  }
  fn set_volume(&mut self, volume: u8) {
    self.volume = volume;
  }
  fn status(&self) {
    println!(
      "TV is {}, volume: {}",
      if self.on { "ON" } else { "OFF" },
      self.volume
    );
  }
}

// Concrete Implementation B: Radio (NEW)
struct Radio {
  on: bool,
  volume: u8,
}

impl Device for Radio {
  fn is_enabled(&self) -> bool {
    self.on
  }
  fn enable(&mut self) {
    self.on = true;
  }
  fn disable(&mut self) {
    self.on = false;
  }
  fn set_volume(&mut self, volume: u8) {
    self.volume = volume;
  }
  fn status(&self) {
    println!(
      "Radio is {}, volume: {}",
      if self.on { "ON" } else { "OFF" },
      self.volume
    );
  }
}

// 2. The Abstraction Hierarchy (High-level interface using composition)
// NOTE: `Device` is a trait!
// A bridge is built with a generic struct with a trait bound!
struct RemoteControl<D: Device> {
  // It bridges two independent hierarchies: On one side, you have the Abstraction (the
  // RemoteControl layer, which defines how users interact with things). On the other side, you have
  // the Implementation (the Device trait, which defines how hardware executes commands like Tv or
  // Radio)
  device: D, // NOTE: This is the "Bridge"!
}

impl<D: Device> RemoteControl<D> {
  fn new(device: D) -> Self {
    Self { device }
  }

  fn toggle_power(&mut self) {
    if self.device.is_enabled() {
      self.device.disable();
    } else {
      self.device.enable();
    }
  }
}

// An Extended Abstraction that inherits functionality through the bridge
struct AdvancedRemoteControl<D: Device> {
  remote: RemoteControl<D>,
}

impl<D: Device> AdvancedRemoteControl<D> {
  fn new(device: D) -> Self {
    Self {
      remote: RemoteControl::new(device),
    }
  }

  fn mute(&mut self) {
    self.remote.device.set_volume(0);
  }

  fn print_status(&self) {
    self.remote.device.status();
  }
}

fn main() {
  {
    // ---- TV Setup ----
    let tv = Tv {
      on: false,
      volume: 30,
    };
    let mut tv_remote = AdvancedRemoteControl::new(tv);

    println!("--- Testing TV ---");
    tv_remote.print_status();
    tv_remote.remote.toggle_power();
    tv_remote.print_status();
    tv_remote.mute();
    tv_remote.print_status();
  }

  {
    // ---- Radio Setup ----
    let radio = Radio {
      on: false,
      volume: 30,
    };
    // The exact same remote logic works seamlessly with the Radio
    let mut radio_remote = AdvancedRemoteControl::new(radio);

    println!("\n--- Testing Radio ---");
    radio_remote.print_status();
    radio_remote.remote.toggle_power();
    radio_remote.print_status();
    radio_remote.mute();
    radio_remote.print_status();
  }
}
