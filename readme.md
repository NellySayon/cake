# Cake blueprint

Simple Scrypto blueprint that shows how to combine different other blueprints by using a cake as an example.

## How to install

1. Install the latest Scrypto version and prepare your IDE as described in the official documentation.

2. Clone this repository

3. Run ```Scrypto build```

4. Go to the developer console and publish the package.

### Bake your cake (instantiate your component)

```
CALL_FUNCTION
  Address("package_xxx...")
  "Cake"
  "instantiate_cake"
  true;
```
