# Experiment of ink! Smart Contract: Struct

This example smart contract is for testing purpose.

- Nested structs as parameter (`Inner`, `Outer`): Normal nested JSON object passed in through the API.
- Enum as parameter (`Shape::Circle`, `Shape::Rectangle`): Take `Shape::Rectangle` as example:

```
[
  {
    "Rectangle": [
      3,
      4
    ]
  }
]
```

or

```
[
  {
    "Rectangle": {
      "x": 3,
      "y": 4
    }
  }
]
```

**Explanation:** Specify the name of the enum variant in the JSON object's top-level entry. Then either pass in the fields in order or name the fields and specify the values in any order.
