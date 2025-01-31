# Trident Manifest Specification

1. Install [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
2. Save [trident-spec.json](../trident-spec.json)
3. `Ctrl + Shift + P` > `Open Remote Settings (JSON)`
4. Enter:

    ```json
    "evenBetterToml.schema.associations": {
        "^*Trident.toml$": "file://<ABSOLUTE_PATH>/trident-spec.json"
    },
    ```
