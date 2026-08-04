### The Memento Design Pattern

The Memento design pattern in Rust is a behavioral design pattern used to
capture and restore an object's internal state without exposing its
implementation details. It is primarily leveraged to build undo/redo systems,
transactional rollbacks, or checkpoint mechanics.

Because Rust enforces strict memory safety and ownership, implementing this
pattern differs slightly from traditional object-oriented languages. It relies
on value-based data structures rather than object reference pointers.

### Core Components

The pattern uses three main structural roles:

* Originator: The live application object that holds the current state. It creates snapshots of itself and consumes them to revert its state.
* Memento: A lightweight, immutable data structure that contains the saved state snapshot.
* Caretaker: The object responsible for tracking history (usually using a Vec stack). It stores and returns mementos but cannot view or modify their contents.
