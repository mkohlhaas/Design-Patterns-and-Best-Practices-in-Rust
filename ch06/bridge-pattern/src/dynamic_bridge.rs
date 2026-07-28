// see first src/static_bridge.rs

// 1. The Implementation Trait (unchanged, but must be object-safe)
trait Device {
  fn is_enabled(&self) -> bool;
  fn enable(&mut self);
  fn disable(&mut self);
  fn set_volume(&mut self, volume: u8);
  fn status(&self);
}

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

// 2. The Abstraction Hierarchy (Now using Dynamic Dispatch)
struct UniversalRemote {
  // The "Bridge" is now a heap-allocated trait object.
  // No more `<D: Device>` generic parameters on the struct!
  device: Box<dyn Device>,
}

impl UniversalRemote {
  fn new(device: Box<dyn Device>) -> Self {
    Self { device }
  }

  fn toggle_power(&mut self) {
    if self.device.is_enabled() {
      self.device.disable();
    } else {
      self.device.enable();
    }
  }

  fn mute(&mut self) {
    self.device.set_volume(0);
  }

  fn print_status(&self) {
    self.device.status();
  }

  // NEW RUNTIME CAPABILITY: Swap the backend device instantly
  // Sets new device and returns old device.
  fn change_device(&mut self, new_device: Box<dyn Device>) -> Box<dyn Device> {
    std::mem::replace(&mut self.device, new_device)
  }
}

fn main() {
  // 1. Initialize the remote with a TV
  let tv = Box::new(Tv {
    on: false,
    volume: 30,
  });
  let mut remote = UniversalRemote::new(tv);

  println!("--- Controlling TV ---");
  remote.print_status();
  remote.toggle_power();
  remote.print_status();
  remote.mute();
  remote.print_status();

  // 2. Swap the TV for a Radio at runtime using the same remote instance
  let radio = Box::new(Radio {
    on: false,
    volume: 30,
  });

  let _old_tv = remote.change_device(radio); // The TV is returned here

  println!("\n--- Controlling Radio (Same Remote) ---");
  remote.print_status();
  remote.toggle_power();
  remote.print_status();
  remote.mute();
  remote.print_status();
}
