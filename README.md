# filler

A library and application for generating realisting, fake data.

## Data

- Names
- Addresses
- Phone numbers
- Emails
- Passwords

## Roadmap

- [ ] Allow extending data source?
- [ ] Output file formats
  - [ ] `txt`
  - [x] `json`
  - [ ] `yaml`
  - [ ] `toml`
  - [ ] `csv` 
- [ ] Data types
  - [x] Names
  - [ ] Emails
  - [x] Uuids
  - [ ] Phone numbers

## CLI

```bash
filler gen --count 2 uuids
```

will output

```json
{
    "data": [
        "497dcba3-ecbf-4587-a2dd-5eb0665e6880",
        "497dcba3-ecbf-4587-a2dd-5eb0665e6880"
    ]
}
```

## License

Licensed under either of

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.