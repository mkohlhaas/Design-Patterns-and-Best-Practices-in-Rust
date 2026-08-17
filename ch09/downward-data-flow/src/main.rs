// This is very similar to the book code for Samsa.

use std::sync::{Arc, Mutex};

/// 1. Define the Downward Interface for Consumers
trait Subscriber: Send + Sync {
    fn on_message(&self, topic: &str, payload: &str);
}

/// 2. The Central Broker (Mediator / Source of Truth)
///
/// It holds a reference to the the subscribers.
#[derive(Default)]
struct Broker {
    subscribers: Mutex<Vec<Arc<dyn Subscriber>>>,
}

impl Broker {
    fn new() -> Self {
        Self::default()
    }

    /// Register a subscriber
    fn subscribe(&self, subscriber: Arc<dyn Subscriber>) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.push(subscriber);
    }

    /// Forward data downward to all targets
    fn publish(&self, topic: &str, payload: &str) {
        let subs = self.subscribers.lock().unwrap();
        for sub in subs.iter() {
            sub.on_message(topic, payload);
        }
    }
}

/// 3. The Producer Component
///
/// It holds a reference to the broker interface to push data down the pipeline.
struct SensorProducer {
    sensor_id: String,
    // Downward handle to the broker
    broker: Arc<Broker>,
}

impl SensorProducer {
    fn new(sensor_id: &str, broker: Arc<Broker>) -> Self {
        Self {
            sensor_id: sensor_id.to_string(),
            broker,
        }
    }

    /// Simulates a sensor reading and starts the downward flow
    fn read_telemetry(&self, temperature: f32) {
        let topic = format!("sensors/{}", self.sensor_id);
        let payload = format!("{} °C", temperature);

        // Data moves downward into the broker
        self.broker.publish(&topic, &payload);
    }
}

/// 4. The Subscriber Component
struct DisplayComponent {
    name: String,
}

impl Subscriber for DisplayComponent {
    fn on_message(&self, topic: &str, payload: &str) {
        println!(
            "[{}] Received update on '{}' -> {}",
            self.name, topic, payload
        );
    }
}

/// 5. Execution Topology
fn main() {
    // Initialize the central hub
    let broker = Arc::new(Broker::new());

    // Initialize and register subscribers
    let display_alpha = Arc::new(DisplayComponent {
        name: "Alpha Screen".to_string(),
    });
    let display_beta = Arc::new(DisplayComponent {
        name: "Gamma Logger".to_string(),
    });

    broker.subscribe(display_alpha);
    broker.subscribe(display_beta);

    // Initialize the producer, giving it a handle to the broker
    let thermal_sensor = SensorProducer::new("temp_room_1", Arc::clone(&broker));

    // Simulate events triggering the downward data cascade
    println!("=== Emitting First Telemetry ===");
    thermal_sensor.read_telemetry(21.5);

    println!("\n=== Emitting Second Telemetry ===");
    thermal_sensor.read_telemetry(22.1);
}
