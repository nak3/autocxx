# Callbacks into Rust

`autocxx` is primarily to allow calls from Rust to C++, but like `cxx` it also allows you to expose Rust APIs to C++.

You can:
* Declare that Rust types should be available to C++ using [`extern_rust_type`](https://docs.rs/autocxx/latest/autocxx/extern_rust/attr.extern_rust_type.html)
* Make Rust functions available to C++ using [`extern_rust_function`](https://docs.rs/autocxx/latest/autocxx/extern_rust/attr.extern_rust_function.html).
* Allow Rust subclasses of C++ classes.

This latter option is most commonly used for implementing "listeners" or ["observers"](https://en.wikipedia.org/wiki/Observer_pattern), so is often in practice how C++ will call into Rust. More details below.

# Subclasses

There is limited and experimental support for creating Rust subclasses of
C++ classes. (Yes, even more experimental than all the rest of this!)
See [`subclass::CppSubclass`](https://docs.rs/autocxx/latest/autocxx/subclass/trait.CppSubclass.html) for information about how you do this.
This is useful primarily if you want to listen out for messages broadcast
using the C++ observer/listener pattern.

```rust,ignore,autocxx,hidecpp
autocxx_integration_tests::doctest(
"
GoatObserver* obs = NULL;
int goat_feed = 0;

void register_observer(const GoatObserver& observer) {
    obs = const_cast<GoatObserver*>(&observer);
}
void deregister_observer() {
    obs = NULL;
};
void feed_goat() {
    goat_feed++;
    if (goat_feed > 2 && obs) {
        obs->goat_full();
    }
}
",
"#include <memory>
class GoatObserver {
public:
    virtual void goat_full() const = 0;
    virtual ~GoatObserver() {}
};

void register_observer(const GoatObserver& observer);
void deregister_observer();
void feed_goat();
",
{
use autocxx::prelude::*;
use autocxx::subclass::*;

include_cpp! {
    #include "input.h"
    safety!(unsafe_ffi)
    generate!("register_observer")
    generate!("deregister_observer")
    generate!("feed_goat")
    subclass!("GoatObserver", MyGoatObserver)
}

use ffi::*;

#[is_subclass(superclass("GoatObserver"))]
#[derive(Default)]
pub struct MyGoatObserver;

impl GoatObserver_methods for MyGoatObserver {
    fn goat_full(&self) {
        println!("BURP!");
    }
}

impl Drop for MyGoatObserver {
    fn drop(&mut self) {
        deregister_observer();
    }
}

fn main() {
    let goat_obs = MyGoatObserver::default_rust_owned();
    // Register a reference to the superclass &ffi::GoatObserver
    register_observer(goat_obs.as_ref().borrow().as_ref());
    feed_goat();
    feed_goat();
    feed_goat(); // prints BURP!
}
}
)
```