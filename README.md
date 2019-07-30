# rocket_conditional_attach

Use this crate to conditionally attach a fairing.

```rust
// Include the relevant trait via the prelude.
use rocket_conditional_attach::prelude::*;

// Alternatively, you can also include the trait directly.
// use rocket_conditional_attach::ConditionalAttach;

rocket::ignite()
	...
	.attach_if(cfg!(feature = "telemetry"), Telemetry::default())
	...
```

