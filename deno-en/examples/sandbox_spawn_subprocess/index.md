# Spawn a subprocess and get buffered output

> Learn how to spawn a subprocess, and get buffered output in a sandbox.

You can spawn subprocesses in a sandbox and get buffered output. For example, to
print the current working directory as seen below. This is useful for running
shell commands and scripts.

```ts
import { Sandbox } from "@deno/sandbox";

await using sandbox = await Sandbox.create();

const cwd = await sandbox.sh`pwd`;
```

For long‑running processes or large output, stream the stdout/stderr instead of
buffering it all in memory.
