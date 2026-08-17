## A. Downward Dataflow Architecture

In Rust, a downward dataflow architecture (commonly referred to as
unidirectional data flow) is a design pattern where data moves strictly in one
direction—from a top-level parent component down to child components. It is
widely used in Rust UI frameworks like Dioxus,
[GPUI](https://mcpmarket.com/tools/skills/rust-ui-architecture), and Slint, as
well as game engines and server architectures.

Instead of allowing child components to directly mutate parent state (which
creates complex, cyclic borrow checker issues), the parent passes read-only
state down and child components pass events back up to trigger state updates.

### Core Mechanics of Downward Dataflow
The architecture is built around three distinct steps that prevent cyclic data
loops:

   1. State Ownership: A single, top-level struct or state manager holds the absolute source of truth.
   2. Downward Flow: State is passed down to sub-modules or UI view components as read-only references (&T) or lightweight reactive signals.
   3. Upward Events: If a deeply nested child wants to change data, it cannot mutate the state directly. It must emit an explicit event, callback, or message up to the owner.

      [ Parent State Store ] <---------+

               |                       |
     (Passes Read-Only State)    (Emits Event)
               v                       |
      [ Child Component ] -------------+

### Why Rust Strongly Prefers This Architecture

Object-Oriented Programming (OOP) often uses two-way data binding, where
multiple objects hold mutable references to each other. In Rust, this causes
severe friction with the borrow checker, which strictly enforces a rule of one
mutable reference (&mut) OR multiple immutable references (&) at any given
time.

By structuring data to only flow downwards, you achieve:

* Borrow Checker Compliance: No cyclic or overlapping mutable references are created.
* Thread Safety: Read-only data flowing downward can easily be shared across threads using Arc or parallel iterators like [Rayon](https://medium.com/@hadiyolworld007/dockerfile-security-for-data-science-teams-224f4bea6d9f).
* Predictable State: Because mutations only happen at the top level, tracking down bugs and writing unit tests becomes highly trivial.

### Code Example: Downward Flow with Events

Below is a simple architectural layout demonstrating how state flows down into an isolated component, while updates are handled via an upward callback closure.

```rust
// 1. Define the Global Statestruct
AppState {
    counter: i32,
}
// 2. A child component that receives read-only data "downwards"
struct CounterButton<'a> {
    // Read-only state passed down
    value: &'a i32, 
    // Upward event callback to notify the parent
    on_click: Box<dyn Fn() + 'a>, 
}

impl<'a> CounterButton<'a> {
    fn render(&self) {
        println!("Rendering button with value: {}", self.value);
    }

    fn simulate_user_click(&self) {
        // Child cannot mutate `self.value`. It passes an event UP.
        (self.on_click)(); 
    }
}

fn main() {
    // Parent owns the mutable state
    let mut state = AppState { counter: 0 };

    // Parent constructs the child, passing state DOWN and accepting events UP
    let button = CounterButton {
        value: &state.counter,
        on_click: Box::new(|| {
            // State mutation happens exclusively at the parent level
            state.counter += 1; 
        }),
    };

    button.render();
    button.simulate_user_click();
}
```

------------------------------

### Implementation Patterns in the Rust Ecosystem

* UI Frameworks (Dioxus / Leptos): Use a "Signal" pattern. A parent creates a Signal, passes a read-only handle down to nested components, and provides a setter function or closure to bubble up updates.
* Game Development (ECS): In Entity Component Systems (like [Bevy](https://medium.com/@theopinionatedev/the-data-oriented-rust-pattern-ecs-beyond-games-high-performance-backend-design-57596dbb24da)), data flows linearly through systems. Systems read components from a central world storage, process changes downstream, and write back results, avoiding cross-component interlocking.
* Compiler Architectures: rustc itself uses forward and backward dataflow analysis framework on its Mid-level Intermediate Representation (MIR) to pass code-safety metrics sequentially downwards through the compilation pipeline.


## B. Broker-Based Pub/Sub System

To build a broker-based pub/sub system using a downward dataflow architecture
in Rust, the broker must act as the absolute source of truth. Data flows
strictly downward from the broker to consumers, while producers and consumers
pass messages or registration intent back upward to the broker via asynchronous
channels.

To satisfy the borrow checker, this architecture relies on a central dispatch
loop where the broker completely owns the state (the list of subscribers) and
processes all incoming requests sequentially, eliminating mutable state
sharing.

### Core Architecture Patterns

* Upward Ingestion: Producers send payloads through the cloning end of a multi-producer, single-consumer (mpsc) channel.
* Downward Distribution: The broker broadcasts messages downward into individual single-producer, single-consumer (spsc or broadcast) channels owned by each subscriber.
* Ownership Isolation: Subscribers never access other subscribers or the broker's interior data. They only await their designated downward stream.

### Production-Ready Implementation

This implementation uses tokio for the asynchronous event loop and channel primitives.

```rust
use std::collections::HashMap;
use tokio::sync::{mpsc, broadcast};

/// 1. Define Upward Event Types
#[derive(Clone, Debug)]
enum BrokerEvent {
    Publish { topic: String, payload: String },
    Subscribe { topic: String, subscriber_id: String, tx: mpsc::Sender<String> },
}
/// 2. Central Broker (The Source of Truth)
struct PubSubBroker {
    // Upward channel receiver for incoming events
    event_rx: mpsc::Receiver<BrokerEvent>,
    // Downward routing map: Topic -> Map of Subscriber IDs to their specific downward channels
    subscribers: HashMap<String, HashMap<String, mpsc::Sender<String>>>,
}

impl PubSubBroker {
    fn new(event_rx: mpsc::Receiver<BrokerEvent>) -> Self {
        Self {
            event_rx,
            subscribers: HashMap::new(),
        }
    }

    /// Central event loop managing state linearly (Downward Dataflow)
    async fn run(mut self) {
        while let Some(event) = self.event_rx.recv().await {
            match event {
                // Handling Upward Subscription Intent
                BrokerEvent::Subscribe { topic, subscriber_id, tx } => {
                    self.subscribers
                        .entry(topic)
                        .or_insert_with(HashMap::new)
                        .insert(subscriber_id, tx);
                }
                // Handling Upward Message Publish -> Routing it DOWNWARD
                BrokerEvent::Publish { topic, payload } => {
                    if let Some(topic_subs) = self.subscribers.get(&topic) {
                        for (sub_id, tx) in topic_subs.iter() {
                            // Data flows strictly downward into the subscriber's private channel
                            if tx.send(payload.clone()).await.is_err() {
                                println!("Subscriber {} disconnected, routing failed.", sub_id);
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 3. Execution Topology
#[tokio::main]async fn main() {
    // The main entry pipeline to the broker
    let (event_tx, event_rx) = mpsc::channel::<BrokerEvent>(100);
    
    // Instantiate and spawn the broker state machine
    let broker = PubSubBroker::new(event_rx);
    tokio::spawn(broker.run());

    // Subscriber 1 setup
    let (sub1_tx, mut sub1_rx) = mpsc::channel::<String>(10);
    let tx_to_broker = event_tx.clone();
    
    // Sub1 sends intent UPWARD
    tx_to_broker.send(BrokerEvent::Subscribe {
        topic: "iot/sensors".to_string(),
        subscriber_id: "Sub_Alpha".to_string(),
        tx: sub1_tx,
    }).await.unwrap();

    // Spawn Sub1 worker awaiting DOWNWARD data
    tokio::spawn(async move {
        while let Some(message) = sub1_rx.recv().await {
            println!("[Sub_Alpha] Received downward data: {}", message);
        }
    });

    // Producer sends payload UPWARD to broker
    let producer_tx = event_tx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        producer_tx.send(BrokerEvent::Publish {
            topic: "iot/sensors".to_string(),
            payload: "Temperature: 22.4C".to_string(),
        }).await.unwrap();
    });

    // Keep main thread alive for demonstration
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
}
```

### Structural Advantages of this Layout

| Architectural Pillar | How It Resolves Rust Bottlenecks |
|---|---|
| No Mutex Locking | The broker state (HashMap) does not require an Arc<Mutex<T>>. It runs inside a single thread context driven by the channel loop. |
| Clean Lifetimes | Messages are owned by the channel during transit. There are zero cross-thread lifetime ('a) annotations required. |
| Isolated Backpressure | Since each subscriber possesses an independent downward channel, a slow consumer cannot block the broker unless its individual buffer fills up. |


## C. Implementation without Async Channels

To eliminate channels while keeping the borrow checker happy, you must shift
from an asynchronous push model to a synchronous, trait-based callback model
using shared ownership (Rc or Arc) and interior mutability (RefCell or Mutex).

Data still flows strictly downward: the Broker retains ownership of subscribers
and passes data down through a trait method execution, while subscribers have
zero visibility into the Broker or other subscribers.

### The Cost of Dropping Channels

When you remove channels, you lose the ability to queue or buffer messages. Instead:

* Immediate Execution: Publishing a message immediately executes the callback of every subscriber sequentially on the same thread.
* Reentrancy Risk: A subscriber cannot call a method on the broker during a callback, or it will cause a runtime panic (BorrowMutError). Data must flow strictly downward only.


### Thread-Safe Synchronous Implementation (No Channels)

This approach uses standard traits and smart pointers (Arc/Mutex) to pass message references downward safely across thread boundaries without channels.

```rust
use std::sync::{Arc, Mutex};

/// 1. Define the Downward Interface
/// Any struct implementing this can receive data flowing downward.
trait Subscriber: Send + Sync {
    fn on_message(&self, topic: &str, payload: &str);
}

/// 2. The Central Broker (Owns the Downward Paths)
#[derive(Default)]
struct Broker {
    // We use Arc<dyn Subscriber> so subscribers can be shared or managed outside.
    // Mutex allows adding/removing subscribers safely.
    subscribers: Mutex<Vec<Arc<dyn Subscriber>>>,
}

impl Broker {
    fn new() -> Self {
        Self::default()
    }

    /// Register a subscriber (Intent moves upward/inward)
    fn subscribe(&self, subscriber: Arc<dyn Subscriber>) {
        let mut subs = self.subscribers.lock().unwrap();
        subs.push(subscriber);
    }

    /// Publish data (Data flows strictly DOWNWARD through the trait boundary)
    fn publish(&self, topic: &str, payload: &str) {
        let subs = self.subscribers.lock().unwrap();
        for sub in subs.iter() {
            // Direct, synchronous downward function invocation
            sub.on_message(topic, payload);
        }
    }
}

/// 3. Concrete Subscriber Implementation
struct DisplayComponent {
    name: String,
}

impl Subscriber for DisplayComponent {
    fn on_message(&self, topic: &str, payload: &str) {
        // Data is processed instantly on the caller's thread
        println!("[{}] Downward data received on '{}': {}", self.name, topic, payload);
    }
}

/// 4. Execution Topology
fn main() {
    let broker = Arc::new(Broker::new());

    // Create isolated components
    let sub_alpha = Arc::new(DisplayComponent { name: "Alpha".to_string() });
    let sub_beta = Arc::new(DisplayComponent { name: "Beta".to_string() });

    // Pass components into the broker
    broker.subscribe(sub_alpha);
    broker.subscribe(sub_beta);

    // Publish data: Triggers immediate synchronous downward flow
    println!("--- Publishing Message ---");
    broker.publish("sensors/temp", "23.5 C");
}
```


### Channels vs. Traits Comparison

| Feature | Channel-Based (Previous Example) | Trait/Pointer-Based (This Example) |
|---|---|---|
| Execution | Asynchronous (Decoupled in time) | Synchronous (Immediate function call) |
| Memory Allocation | Medium (Heap allocations for buffers) | Low (Zero buffering overhead) |
| Blocking Behavior | Doesn't block producer (has a buffer) | Blocks producer until all callbacks finish |
| Thread Context | Messages cross thread boundaries safely | Executed directly on the producer's thread |


## D. Adding a Producer Component

To add a Producer component while maintaining a strict downward dataflow
architecture, the producer must remain entirely decoupled from the subscribers.

The producer only needs a downward dependency on the Broker. When the producer
triggers an action, it pushes data into the broker, which then cascades the
data downward to the registered subscribers.

### Structural Rule

* Producers point downward to the Broker.
* The Broker points downward to Subscribers.
* Subscribers have no knowledge of producers or the broker.

```
[ Producer ] 
     │  (Pushes data)
     ▼
[  Broker  ] 
     │  (Cascades data downward)
     ▼
[ Subscriber ]
```


### Synchronous Implementation with a Producer

Here is the complete, thread-safe implementation including a dedicated Producer component.

```rust
use std::sync::{Arc, Mutex};

/// 1. Define the Downward Interface for Consumers
trait Subscriber: Send + Sync {
    fn on_message(&self, topic: &str, payload: &str);
}

/// 2. The Central Broker (Mediator / Source of Truth)
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
        println!("[{}] Received update on '{}' -> {}", self.name, topic, payload);
    }
}

/// 5. Execution Topology
fn main() {
    // Initialize the central hub
    let broker = Arc::new(Broker::new());

    // Initialize and register subscribers
    let display_alpha = Arc::new(DisplayComponent { name: "Alpha Screen".to_string() });
    let display_beta = Arc::new(DisplayComponent { name: "Beta Logger".to_string() });
    
    broker.subscribe(display_alpha);
    broker.subscribe(display_beta);

    // Initialize the producer, giving it a handle to the broker
    let thermal_sensor = SensorProducer::new("temp_room_1", Arc::clone(&broker));

    // Simulate events triggering the downward data cascade
    println!("--- Emitting First Telemetry ---");
    thermal_sensor.read_telemetry(21.5);

    println!("\n--- Emitting Second Telemetry ---");
    thermal_sensor.read_telemetry(22.1);
}
```


### Architectural Highlights

* Ownership Isolation: SensorProducer and DisplayComponent do not know each other exist. They can be compiled, tested, and modified in completely separate crates.
* Synchronous Sequencing: When thermal_sensor.read_telemetry() is called, the execution flow strictly descends: SensorProducer -> Broker -> DisplayComponent sequentially on a single call stack.
* Thread Safety: Because everything uses Arc, you could effortlessly move thermal_sensor into a separate std::thread loop without altering the broker or subscriber code.
