# utils/random

Let there be order amidst the chaos.

This file implements Mersenne Twister 19937, matching Python's `random` module exactly for reproducibility.

```javascript
import { random } from '@huggingface/transformers';

random.seed(42);
random.random();           // 0.6394267984578837  (matches Python)
random.gauss(0, 1);        // normal-distributed value
random.choices(['a','b'], [3, 1]);  // weighted pick

const arr = [1, 2, 3, 4, 5];
random.shuffle(arr);       // in-place Fisher-Yates shuffle

// Use a separate instance to avoid affecting the global state:
const rng = new random.Random(42);
rng.random();              // 0.6394267984578837  (same seed, independent state)
```

**Note on Reproducibility:**
Similarly to the [Python random](https://docs.python.org/3/library/random.html#notes-on-reproducibility)
module, it is useful to be able to reproduce the sequences given by a pseudo-random number generator.
By reusing a seed value, the same sequence should be reproducible from run to run as long as multiple
threads or asynchronous operations are not running concurrently.

## Classes

### Random

Mersenne Twister 19937 PRNG, matching Python's `random.Random` class exactly.

Each instance has its own independent state, so seeding one instance does not
affect any other instance or the global helper functions.

```javascript
import { random } from '@huggingface/transformers';

const rng1 = new random.Random(42);
const rng2 = new random.Random(42);
rng1.random() === rng2.random(); // true (same seed, independent state)
```

#### `Random.seed([n])`

Seeds this instance's PRNG.

When called with a number, initializes the state deterministically from that value.
When called with no arguments (or `undefined`/`null`), seeds from OS entropy
via `crypto.getRandomValues`, matching Python's `random.seed()` behavior.

**Parameters**

- `n` (`number`) _optional_ — The seed value. Omit to seed from OS entropy.

#### `Random.random()`

Generates a random floating-point number in the half-open interval [0, 1).

Combines two 32-bit integers (using 53 bits of precision) to produce
a uniformly distributed double, matching Python's `random.random()`.

**Returns:** `number` — A random float in [0, 1).

#### `Random.gauss([mu], [sigma])`

Generates a random number from a Gaussian (normal) distribution.

Uses the Box-Muller transform with a cached spare value,
matching Python's `random.gauss()` output for the same seed.

**Parameters**

- `mu` (`number`) _optional_ — defaults to `0` — The mean of the distribution.
- `sigma` (`number`) _optional_ — defaults to `1` — The standard deviation of the distribution.

**Returns:** `number` — A normally distributed random value.

#### `Random.shuffle(arr)`

Shuffles an array in-place using the Fisher-Yates algorithm.

Uses rejection sampling via `getrandbits`-style bit masking to ensure
a uniform distribution, matching Python's `random.shuffle()`.

**Parameters**

- `arr` (`any[]`) — The array to shuffle in-place.

#### `Random.choices(population, weights)`

Selects a single element from a weighted population.

Matches Python's `random.choices(population, weights=weights, k=1)[0]`

**Parameters**

- `population` (`any[]`) — The array of items to choose from.
- `weights` (`number[]`) — An array of non-negative weights, one per population element.

**Returns:** `*` — A single randomly selected element from the population.

## Constants

### `random`

The default PRNG instance, mirroring Python's module-level `random` functions.
It shares a single global state, so if you want to generate independent sequences,
construct your own `new random.Random(seed)` instead.

```javascript
import { random } from '@huggingface/transformers';
random.seed(42);
random.random();     // 0.6394267984578837  (matches Python)
random.gauss(0, 1);  // normal-distributed value
random.choices(['a', 'b'], [3, 1]);  // weighted pick
```
