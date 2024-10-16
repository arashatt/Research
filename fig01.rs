
// `Poll` allows for polling of readiness events.
let poll = Poll::new()?;
// `Events` is collection of readiness `Event`s and can be filled by
// calling `Poll::poll`.
let events = Events::with_capacity(128);
