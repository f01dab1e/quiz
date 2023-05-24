# Testing

## Snapshot testing

Snapshot testing is a technique to capture and compare expected and actual outputs during testing. It's useful for testing complex or dynamic code outputs. To update snapshots during testing, use the command `env UPDATE_EXPECT=1 cargo test`.

## Best practics

* Avoid using `#[should_panic]` tests. Instead, explicitly check for None, Err, etc. This is because `#[should_panic]` is meant for library authors to prevent silent failures when misused. In `lt_quiz`, we handle user input without panics, and panic messages from `#[should_panic]` tests can be confusing.

* Avoid `#[ignore]` tests. If a test doesn't work, assert the wrong behavior and add a fixme comment explaining why it's incorrect. This approach helps in noticing when the behavior is fixed and ensures that even wrong behavior is accounted for (not causing panics).

# Code Style

* Use type ascription instead of the turbofish (`::<Type>`) syntax whenever possible.
* Avoid creating single-use helper functions.
