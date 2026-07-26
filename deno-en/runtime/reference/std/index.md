# Deno Standard Library (@std)

> The Deno Standard Library is a set of audited, dependency-free modules for common tasks, published on JSR under the @std scope.

The Deno Standard Library is a collection of high-quality, audited modules
for common tasks such as working with paths, dates, streams, and encodings.
Every module is reviewed by the Deno team, has no third-party dependencies,
and works across Deno, Node.js, Bun, Cloudflare Workers, and the browser
wherever the underlying platform APIs are available.

The library is published as a set of modular packages on
[JSR](https://jsr.io/@std) under the `@std` scope. You install only the
packages you need, and each one is versioned independently.

## Getting started

Add a package to your project with `deno add`:

```sh
deno add jsr:@std/path jsr:@std/fmt
```

This records the dependency in your `deno.json` import map, so you can
import it by its bare specifier:

```ts
import { join } from "@std/path";

console.log(join("dir", "file.txt"));
```

You can also import a package directly from JSR without adding it first,
which is handy for one-off scripts:

```ts
import { join } from "jsr:@std/path";
```

## Stability

Most `@std` packages are stable and follow semantic versioning. Some newer
or lower-level packages are marked as **unstable** (their API may change
without a major version bump) or **internal** (intended only for use by
other `@std` packages). Each package's page notes its stability, and
unstable packages carry a callout at the top.

## Packages

Follow a link for an overview, install instructions, and a link to the full
symbol reference on JSR.

<!-- packages:start -->
- [@std/assert](./assert/) – Common assertion functions, especially useful for testing
- [@std/async](./async/) – Utilities for asynchronous operations, like delays, debouncing, or pooling
- [@std/bytes](./bytes/) – Utilities to manipulate Uint8Arrays that are not built-in to JavaScript
- [@std/cache](./cache/) – UNSTABLE: Cache utilities
- [@std/cbor](./cbor/) – UNSTABLE: Utilities for parsing and serializing Concise Binary Object Representation (CBOR)
- [@std/cli](./cli/) – Tools for creating interactive command line tools
- [@std/collections](./collections/) – Pure functions for common tasks related to collection types like arrays and objects
- [@std/crypto](./crypto/) – Extensions to the Web Crypto API
- [@std/csv](./csv/) – Reading and writing of comma-separated values (CSV) files
- [@std/data-structures](./data-structures/) – Common data structures like red-black trees and binary heaps
- [@std/datetime](./datetime/) – UNSTABLE: Utilities for dealing with Date objects
- [@std/dotenv](./dotenv/) – UNSTABLE: Parsing and loading environment variables from a `.env` file
- [@std/encoding](./encoding/) – Utilities for encoding and decoding common formats like hex, base64, and varint
- [@std/expect](./expect/) – Jest compatible `expect` assertion functions
- [@std/fmt](./fmt/) – Utilities for formatting values, such as adding colors to text, formatting durations, printf utils, formatting byte numbers.
- [@std/front-matter](./front-matter/) – Extract front matter from strings
- [@std/fs](./fs/) – Helpers for working with the file system
- [@std/html](./html/) – Functions for HTML, such as escaping or unescaping HTML entities
- [@std/http](./http/) – Utilities for building HTTP servers
- [@std/ini](./ini/) – UNSTABLE: Parsing and serializing of INI files
- [@std/internal](./internal/) – INTERNAL: The internal package for @std. Do not use this directly.
- [@std/io](./io/) – UNSTABLE: The utilities for advanced I/O operations using Reader and Writer interfaces.
- [@std/json](./json/) – (Streaming) parsing and serializing of JSON files
- [@std/jsonc](./jsonc/) – Parsing and serializing of JSONC files
- [@std/log](./log/) – UNSTABLE: A customizable logger framework
- [@std/math](./math/) – Basic math utilities
- [@std/media-types](./media-types/) – Utility functions for media types (MIME types)
- [@std/msgpack](./msgpack/) – Encoding and decoding for the msgpack format
- [@std/net](./net/) – Utilities for working with the network
- [@std/path](./path/) – Utilities for working with file system paths
- [@std/random](./random/) – UNSTABLE: Various utilities using random number generators. The package also provides seeded pseudo-random number generator.
- [@std/regexp](./regexp/) – Utilities for working with RegExp
- [@std/semver](./semver/) – Parsing and comparing of semantic versions (SemVer)
- [@std/streams](./streams/) – Utilities for working with the Web Streams API
- [@std/tar](./tar/) – UNSTABLE: Streaming utilities for working with tar archives.
- [@std/testing](./testing/) – Tools for testing Deno code like snapshot testing, bdd testing, and time mocking
- [@std/text](./text/) – Utilities for working with text
- [@std/toml](./toml/) – Parsing and serializing of TOML files
- [@std/ulid](./ulid/) – Generation of Universally Unique Lexicographically Sortable Identifiers (ULIDs)
- [@std/uuid](./uuid/) – Generators and validators for UUIDs
- [@std/webgpu](./webgpu/) – UNSTABLE: Utilities for working with the Web GPU API
- [@std/xml](./xml/) – XML parsing and serialization for Deno.
- [@std/yaml](./yaml/) – Parsing and serializing of YAML files
<!-- packages:end -->

> This index and the individual package overview sections are generated. Add extra examples by creating files in `_overrides/<package>.md`.
