# csver

Rust transaction processor.


## Design

__csver__ was designed with modulatiry in mind. Traits were defined for a __processor__ and __datastore__, so alternate implementations can be plugged in.

`Processor` module stipulates behavior of system as transactions are processed, and should contain all the business logic.

`DataStore` module should be a blanked implementation of a simple CRUD database for transactions and accounts.

![Design](/docs/version_1.jpg)

### **Domain**

In the `src/domain` module, we can find definitions for the `Processor` and `DataStore` traits, error modules and result wrappers.

### **Model**

In the `src/model` module, we can find basic model definitions for `Account` and `Transaction` objects, including serialization options.

## Implementation

I chose to keep it as simple as possible while still maintainig some structure beyond a script.

`MemStore` was implemented as a in memory store, without any support for multithreading/async.

`TxProcessor` was also implemented without multuthreading/async in mind.

## To Improve

Tradeoffs were made for sake of simplicity, but ideally with more time this is what I would immediately improve:

### 1. Error handling

Very basic error handling is done, I defined Error types but they could and should be expanded on and treated/propagated properly. I do some liberal use of `unwrap()` which I would not recommend in a production system.

### 2. Async support

Making use of `tokyo` here would have been my prefered choice, but I felt it would overcomplicate what should be a simple demonstration. Ideally the defined traits would be prepared for async support. E.g `MemStore` should make use of Mutexes and Arc's to be concurency safe, or just plugin an existing DBMS.

### 3. Input streams

Currently input stream process is very basic and not very flexible, I would provide a more sofisticated model where multiple streams could be consumed concurrently through some `InputCollector` and then feed these as one stream into the actual transaction processor.

### 4. Logging/Tracing

I do some crude `println!`, which I would not have in a production system. Some sort of tracing/logging library would be a better choice, with multiple level's of logs.

![Better Design](/docs/version_2.jpg)
