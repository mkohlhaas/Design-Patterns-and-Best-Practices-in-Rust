// ============================ //
// 1. Define the Observer Trait //
// ============================ //

pub trait Observer {
    fn update(&mut self, temp: f32);
}

// =============================== //
// 2. Implement Concrete Observers //
// =============================== //

// =============== //
// A. PhoneDisplay //
// =============== //

struct PhoneDisplay {
    phone_model: String,
}

impl Observer for PhoneDisplay {
    fn update(&mut self, temp: f32) {
        println!("📱 [{}] Display updated: {}°C", self.phone_model, temp);
    }
}

// ================ //
// B. WindowDisplay //
// ================ //

struct WindowDisplay {
    location: String,
}

impl Observer for WindowDisplay {
    fn update(&mut self, temp: f32) {
        println!(
            "🪟 [{}] Screen blinking! New temperature is {}°C",
            self.location, temp
        );
    }
}

// ======================== //
// 3. Implement the Subject //
// ======================== //

pub struct WeatherStation {
    temperature: f32,
    // The Subject owns the observers completely via Box
    observers: Vec<Box<dyn Observer>>,
}

impl Default for WeatherStation {
    fn default() -> Self {
        Self::new()
    }
}

impl WeatherStation {
    pub fn new() -> Self {
        Self {
            temperature: 0.0,
            observers: Vec::new(),
        }
    }

    // Pass ownership of the observer box directly into the vector
    pub fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    // Set new data and immediately trigger a notification loop
    pub fn set_temperature(&mut self, new_temp: f32) {
        println!(
            "\n🌡️ Weather Station: Temperature changed to {}°C",
            new_temp
        );
        self.temperature = new_temp;
        self.notify();
    }

    // Iterate through observers and mutate them directly
    fn notify(&mut self) {
        for observer in &mut self.observers {
            observer.update(self.temperature);
        }
    }
}

// ===== //
// Usage //
// ===== //

fn main() {
    let mut station = WeatherStation::new();

    // Create observers
    let pixel_phone = Box::new(PhoneDisplay {
        phone_model: String::from("Pixel 8"),
    });
    let kitchen_screen = Box::new(WindowDisplay {
        location: String::from("Kitchen"),
    });

    // Attach them to the station (ownership moves here)
    station.attach(pixel_phone);
    station.attach(kitchen_screen);

    // Simulate real-world weather changes
    station.set_temperature(22.5);
    station.set_temperature(24.1);
}
