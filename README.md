# only_one

Value consumption helper.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## What is this?

This crate provides a wrapper type `One` that handles value consumption.

Internally, This type is super simple newtype of [`Option`]. However, it
sometimes makes code simpler than using `Option` directly. (Especially,
types that implements [`Drop`] are good [examples](#examples) of this.)

## Background

In implementation of [`Drop::drop`], let's say we want to consume the
field value but not reference it. There are several ways to achieve this,
as noted [later](#popular-ways). However, if we want the way that always
available and unsafe-free, there remains only one way. So, we normally
choose the way with [`Option::take`] method.

### Bad code smell

Using `Option` for this purpose has following drawbacks.

- In code writing, unwrapping processes are required in many places.
- In code reading, until reading drop code, wrapping reason is unclear.

### Popular ways

If mutable reference is provided,
field value of its target can be consumed by following ways.  
(About [`drop`][`Drop::drop`] method, we can use `&mut self`).

- [`Option::take`] - Always usable if filed type is wrapped by [`Option`]
- [`mem::take`] - Only usable if field type implements [`Default`]
- [`ptr::read`] - Aggressive binary data copy (required `unsafe`)
- [`mem::replace`] - Combine with [`mem::zeroed`] (required `unsafe`)

## Solution

`One` usually acts as a smart pointer to saved value. Therefore, unwrap
action is hidden by automatic dereference. And if value consumption is
required such as drop situation, you can use `One::take` method.
This mechanism shortens the code.

Also, `One` lacks generality like `Option`.
This clarifies wrapping purpose.

## Examples

Following code is a common example with double meaning 😓.

```rust
use only_one::prelude::*;

fn main() {
    let mut message_box = None;
    let mut worker = Worker::new(&mut message_box);
    assert_eq!(worker.message(), "I am a new worker!");

    worker.do_hard_work();
    assert_eq!(worker.message(), "I am buzy!");

    worker.do_bullshit_work();
    assert_eq!(message_box.unwrap(), "I am retired!");
}

struct Worker<'a> {
    message: One<String>,
    message_box: &'a mut Option<String>,
}

impl<'a> Worker<'a> {
    pub fn new(message_box: &'a mut Option<String>) -> Self {
        let message = One::new("I am a new worker!".to_string());
        Self {
            message,
            message_box,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn do_hard_work(&mut self) {
        *self.message = "I am buzy!".to_string();
    }

    pub fn do_bullshit_work(mut self) {
        *self.message = "I am retired!".to_string()
    }
}

impl Drop for Worker<'_> {
    fn drop(&mut self) {
        *self.message_box = Some(One::take(&mut self.message));
    }
}
```

## History

See [CHANGELOG](CHANGELOG.md).

<!-- Links -->

[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
[`Drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html
[`Drop::drop`]: https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop
[`Option`]: https://doc.rust-lang.org/std/option/enum.Option.html
[`Option::take`]: https://doc.rust-lang.org/std/option/enum.Option.html#method.take
[`mem::replace`]: https://doc.rust-lang.org/std/mem/fn.replace.html
[`mem::take`]: https://doc.rust-lang.org/std/mem/fn.take.html
[`mem::zeroed`]: https://doc.rust-lang.org/std/mem/fn.zeroed.html
[`ptr::read`]: https://doc.rust-lang.org/std/ptr/fn.read.html
