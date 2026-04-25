Typesafe Traffic Light Protocol (TLP) label

```rust
use tlp::Label;

fn tlp_example() -> Result<(), Box<dyn std::error::Error>> {
    let label = Label::from_str("TLP:Amber+STRICT")?;

    assert_eq!(label, "TLP:AMBER+STRICT");
    assert_eq!(label, Label::AmberStrict);

    Ok(())
}
```
