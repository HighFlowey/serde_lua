# Serde Lua

## EXAMPLES

> [!NOTE]
> Read the ./examples directory to learn how to use the rust API

```lua
return 100
```

```lua
return {
    array = { "a", "b", "c" },
    dict = {
        a = "a",
        ["b"] = "b",
        c = "c",
    }
}
```

```lua
-- This will error because mixing array with dictionary is not allowed
return {
    "a",
    "b",
    c = "c"
}
```

## ISSUES

multi line comments dont account for how many equal signs you use at the opening and the closing

```lua
--[======[
    This won't throw syntax error
]===]
```

## TO-DO(s)

- [ ] impl serde::Deserializer
- [X] write tests
- [ ] impl serde::Deserialize
- [ ] documentation
- [ ] grammar rule for tuples
