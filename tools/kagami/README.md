# Kagami (Teacher and Exemplar and/or Looking glass)

Kagami is a tool used to generate and validate automatically generated data files that are shipped with Iroha.

## Build

From anywhere in the repository, run:

```bash
cargo build --bin kagami
```

This will place `kagami` inside the `target/debug/` directory (from the root of the repository).

## Usage

Run Kagami:

```
kagami <SUBCOMMAND>
```

### Subcommands

|        Command        |                             Description                              |
| --------------------- | -------------------------------------------------------------------- |
| [`crypto`](#crypto)   | Generate cryptographic key pairs                                     |
| [`docs`](#docs)       | Generate a Markdown reference of configuration parameters            |
| [`genesis`](#genesis) | Generate the default genesis block that is used in tests             |
| [`schema`](#schema)   | Generate the schema used for code generation in Iroha SDKs           |
| [`tokens`](#tokens)   | Generate a list of predefined permission tokens and their parameters |
| [`config`](#config)   | Generate the default configuration for the client or the peer        |
| `help`                | Print the help message for the tool or a subcommand                  |

## `crypto`

The `crypto` command generate cryptographic key pairs using the given algorithm and either private key or seed.

|     Option      |                                          Description                                           | Default value  |  Type  |
| --------------- | ---------------------------------------------------------------------------------------------- | -------------- | ------ |
| `--algorithm`   | The algorithm used to generate the key-pair: `ed25519`, `secp256k1`, `bls_normal`, `bls_small` | `ed25519`      | String |
| `--private_key` | The `private_key` used to generate the key-pair                                                | Not applicable | String |
| `--seed`        | The `seed` used to generate the key-pair                                                       | Not applicable | String |

You can also choose output format:

|   Flag      |                Description                                              |
| ----------- | ----------------------------------------------------------------------- |
| `--json`    | A flag to specify whether or not to output the key-pair in JSON format. |
| `--compact` | A flag to specify whether or not to output the key-pair compact format. |

### `crypto` usage examples

- Generate a key pair:

    ```bash
    ./kagami crypto
    ```

  <details> <summary>Expand to see the output</summary>

    ```bash
    Kagami. To see help run with `--help`.
    No flags specified, generating key-pair.
    Public key (multihash): ed0120232adec551bfa1856279ebccc3c3a09783c516478f4cbb2f42f342614bec7601
    Private key: a1e2c094496dd53ea103f1423b90ccb7d65ff25ab46f5fa1643c14e6010f7f75232adec551bfa1856279ebccc3c3a09783c516478f4cbb2f42f342614bec7601
    Digest function: ed25519
    ```
  </details>

- Generate a key pair from a given seed:

    ```bash
    ./kagami crypto --seed <seed>
    ```

- Generate a key with the `secp256k1` algorithm and a given private key (`b32129af69b829a88ab9bac60b2a33cc57f8843e93aae0478e93f2285059c236`):

    ```bash
    ./kagami crypto --algorithm secp256k1 --private-key "b32129af69b829a88ab9bac60b2a33cc57f8843e93aae0478e93f2285059c236"
    ```

  <details> <summary>Expand to see the output</summary>

    ```bash
    Public key (multihash): e70121031c59a9cabaf58f3b8a6157362b9f6feac3dd47ee947fbf2f335805e1a7f96bde
    Private key: b32129af69b829a88ab9bac60b2a33cc57f8843e93aae0478e93f2285059c236
    Digest function: secp256k1
    ```
  </details>

- Generate a key in JSON format:

    ```bash
    ./kagami crypto --json
    ```

  <details> <summary>Expand to see the output</summary>

    ```json
    {
        "public_key": "ed01203189e4982f98dc293ab9e32cf2b2d75fba49adbc345318a576377b75cc9e15c1",
        "private_key": {
            "digest_function": "ed25519",
            "payload": "d2162546e2025d28b680d062b91043a1e990de7da7861ee5e8039a6b39c9551f3189e4982f98dc293ab9e32cf2b2d75fba49adbc345318a576377b75cc9e15c1"
        }
    }
    ```
  </details>

- Generate a key in compact format:

    ```bash
    ./kagami crypto --compact
    ```

  <details> <summary>Expand to see the output</summary>

    ```bash
    ed01208c8a612f0d20f339a0ea8df21fea777cbbe3604281e5f52311e5c5602cd38d8e
    878f0fc05183857871a17605fe8f63b4aaf72ac9af4a5d8dd22536f6d016dff18c8a612f0d20f339a0ea8df21fea777cbbe3604281e5f52311e5c5602cd38d8e
    ed25519
    ```
  </details>

## `genesis`

- Generate a genesis block in JSON format:

    ```bash
    kagami genesis
    ```
- Generate a genesis block in JSON format and write the output to the specified file:

    ```bash
    kagami genesis >genesis.json
    ```
 - Generate a synthetic genesis block in JSON format and write the `n` domains, `m` accounts per domain and `p` assets per domain:

    ```bash
    kagami genesis --synthetic --domains n --accounts-per-domain m --assets-per-domain p
    ```

## `schema`

- Generate the schema in JSON format:

    ```bash
    kagami schema
    ```

- Generate the schema in JSON format and write the output to the specified file:

    ```bash
    kagami schema >schema.json
    ```

## `docs`

Generate peer configuration reference in a Markdown format:

```bash
kagami docs
```

The output should be identical to the [reference configuration](../../docs/source/references/config.md).

## `tokens`

- Generate a list of predefined permission tokens and their parameters:

    ```bash
    kagami tokens
    ```

- Generate a list of predefined permission tokens and their parameters and write the output to the specified JSON file:

    ```bash
    kagami tokens >tokens.json
    ```

## `config`

- Generate the default peer configuration:

    ```bash
    kagami config peer > peer-config.json
    ```

- Generate the default client configuration:

    ```bash
    kagami config client > client-config.json
    ```