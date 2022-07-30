//!
//! * Implements most of the interfaces and functions of Node.JS Events.
//! * Specific work conditions may vary.
//!
//!
//! ## Example
//!
//! ```
//! extern crate events_emitter;
//! use events_emitter::EventEmitter;
//!
//! let events = EventEmitter::new();
//! events.on("hello", |x| { println!("{}", x) });
//! events.emit("hello", "word");
//! events.remove("hello");
//! ```
//!

// Event binding singleton.
pub struct Listener<T> {
    pub name: String,
    pub once: bool,
    pub listener: fn(&T),
}

// Event loop table.
pub struct EventEmitter<T> {
    pub listeners: Vec<Listener<T>>,
}

impl<T> EventEmitter<T> {
    /// # Create an instance.
    /// Create event loop bus.
    ///
    /// ## example
    /// ```
    /// let events = EventEmitter::new();
    /// ```
    pub fn new() -> Self {
        EventEmitter {
            listeners: Vec::new(),
        }
    }

    /// # Binding listener.
    /// Binding event.
    ///
    ///
    /// ## example
    /// ```
    /// let events = EventEmitter::new();
    /// events.on("hello", |x| { println!("{}", x) });
    /// ```
    pub fn on(&mut self, name: &str, listener: fn(&T)) {
        self.listeners.push(Listener {
            name: String::from(name),
            once: false,
            listener,
        });
    }

    /// # Trigger listener.
    /// Push event.
    ///
    ///
    /// ## example
    /// ```
    /// let events = EventEmitter::new();
    /// events.on("hello", |x| { println!("{:?}", x) });
    /// events.emit("hello", "word");
    /// ```
    pub fn emit(&mut self, name: &str, value: &T) {
        let mut removes = Vec::new();

        // Traversing the listener binding table.
        // Check the specified listener.
        // Check once binding.
        for (index, context) in self.listeners.iter().enumerate() {
            if context.name.as_str() == name {
                (context.listener)(value);
                if context.once == true {
                    removes.push(index);
                }
            }
        }

        // Delete once listener.
        for index in removes.iter() {
            self.listeners.remove(*index as usize);
        }
    }

    /// # Returns an array of event names for registered listeners.
    ///
    /// ## example
    /// ```
    /// let events = EventEmitter::new();
    /// events.on("hello", |x| { println!("{}", x) });
    /// println!("{:?}", events.event_names());
    /// ```
    pub fn event_names(&mut self) -> Vec<&str> {
        let mut names = Vec::new();
        for context in self.listeners.iter() {
            names.push(context.name.as_str());
        }

        names
    }

    /// # Returns the number of event listeners being listened to.
    ///
    /// ## example
    /// ```
    /// let events = EventEmitter::new();
    /// events.on("hello", |x| { println!("{}", x) });
    /// println!("{:?}", events.listener_count());
    /// ```
    pub fn listener_count(&mut self, name: &str) -> u8 {
        let mut count: u8 = 0;
        for context in self.listeners.iter() {
            if context.name.as_str() == name {
                count += 1;
            }
        }

        count
    }

    /// # Add once listener
    ///
    /// ## example
    /// ```
    /// let events = EventEmitter::new();
    /// events.once("hello", |x| { println!("{}", x) });
    /// ```
    pub fn once(&mut self, name: &str, listener: fn(&T)) {
        self.listeners.push(Listener {
            name: String::from(name),
            once: true,
            listener,
        });
    }

    /// # Remove listener.
    ///
    /// ## example
    /// ```
    /// let events = EventEmitter::new();
    /// events.on("hello", |x| { println!("{}", x) });
    /// events.remove("hello");
    /// ```
    pub fn remove(&mut self, name: &str) {
        let mut removes = Vec::new();

        // Traversing the listener binding table.
        // Check the specified listener.
        for (index, context) in self.listeners.iter().enumerate() {
            if context.name.as_str() == name {
                removes.push(index);
            }
        }

        // Delete once listener.
        for index in removes.iter() {
            self.listeners.remove(*index as usize);
        }
    }

    /// # Returns the specified listener list for the binding.
    ///
    /// ## example
    /// ```
    /// let events = EventEmitter::new();
    /// events.on("hello", |x| { println!("{}", x) });
    /// events.listeners("hello");
    /// ```
    pub fn listeners(&mut self, name: &str) -> Vec<&Listener<T>> {
        let mut listener = Vec::new();
        for context in self.listeners.iter() {
            if context.name.as_str() == name {
                listener.push(context);
            }
        }

        listener
    }
}
