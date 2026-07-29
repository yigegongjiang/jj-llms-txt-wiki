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

* [utils/random](#module_utils/random)
    * _static_
        * [.Random](#module_utils/random.Random)
            * [`new Random()`](#new_module_utils/random.Random_new)
            * [`.seed([n])`](#module_utils/random.Random+seed)
            * [`._int32()`](#module_utils/random.Random+_int32) ⇒ number
            * [`.random()`](#module_utils/random.Random+random) ⇒ number
            * [`.gauss([mu], [sigma])`](#module_utils/random.Random+gauss) ⇒ number
            * [`.shuffle(arr)`](#module_utils/random.Random+shuffle)
            * [`.choices(population, weights)`](#module_utils/random.Random+choices) ⇒ *
    * _inner_
        * [`~_weightedIndexWith(randomFn, weights)`](#module_utils/random.._weightedIndexWith) ⇒ number

* * *

## utils/random.Random

Mersenne Twister 19937 PRNG, matching Python's `random.Random` class exactly.

Each instance has its own independent state, so seeding one instance does not
affect any other instance or the global helper functions.

**Kind**: static class of [utils/random](#module_utils/random)  

* [.Random](#module_utils/random.Random)
    * [`new Random()`](#new_module_utils/random.Random_new)
    * [`.seed([n])`](#module_utils/random.Random+seed)
    * [`._int32()`](#module_utils/random.Random+_int32) ⇒ number
    * [`.random()`](#module_utils/random.Random+random) ⇒ number
    * [`.gauss([mu], [sigma])`](#module_utils/random.Random+gauss) ⇒ number
    * [`.shuffle(arr)`](#module_utils/random.Random+shuffle)
    * [`.choices(population, weights)`](#module_utils/random.Random+choices) ⇒ *

* * *

### `new Random()`

**Example**  
```js
const rng1 = new Random(42);
const rng2 = new Random(42);
rng1.random() === rng2.random(); // true (same seed, independent state)
```

* * *

### `random.seed([n])`

Seeds this instance's PRNG.

When called with a number, initializes the state deterministically from that value.
When called with no arguments (or `undefined`/`null`), seeds from OS entropy
via `crypto.getRandomValues`, matching Python's `random.seed()` behaviour.

**Kind**: instance method of [Random](#module_utils/random.Random)  

  
    
      ParamTypeDescription
    
  
  

    [n]numberThe seed value. Omit to seed from OS entropy.

      

* * *

### `random._int32()` ⇒ number

Generates a random unsigned 32-bit integer.

Performs the "twist" step when the state buffer is exhausted,
then applies the standard MT19937 tempering transform.

**Kind**: instance method of [Random](#module_utils/random.Random)  
**Returns**: number - A random integer in the range [0, 2^32 - 1].  

* * *

### `random.random()` ⇒ number

Generates a random floating-point number in the half-open interval [0, 1).

Combines two 32-bit integers (using 53 bits of precision) to produce
a uniformly distributed double, matching Python's `random.random()`.

**Kind**: instance method of [Random](#module_utils/random.Random)  
**Returns**: number - A random float in [0, 1).  

* * *

### `random.gauss([mu], [sigma])` ⇒ number

Generates a random number from a Gaussian (normal) distribution.

Uses the Box-Muller transform with a cached spare value,
matching Python's `random.gauss()` output for the same seed.

**Kind**: instance method of [Random](#module_utils/random.Random)  
**Returns**: number - A normally distributed random value.  

  
    
      ParamTypeDefaultDescription
    
  
  

    [mu]number0The mean of the distribution.

    
    [sigma]number1The standard deviation of the distribution.

      

* * *

### `random.shuffle(arr)`

Shuffles an array in-place using the Fisher-Yates algorithm.

Uses rejection sampling via `getrandbits`-style bit masking to ensure
a uniform distribution, matching Python's `random.shuffle()`.

**Kind**: instance method of [Random](#module_utils/random.Random)  

  
    
      ParamTypeDescription
    
  
  

    arrArrayThe array to shuffle in-place.

      

* * *

### `random.choices(population, weights)` ⇒ *

Selects a single element from a weighted population.

Matches Python's `random.choices(population, weights=weights, k=1)[0]`

**Kind**: instance method of [Random](#module_utils/random.Random)  
**Returns**: * - A single randomly selected element from the population.  

  
    
      ParamTypeDescription
    
  
  

    populationArrayThe array of items to choose from.

    
    weightsArrayAn array of non-negative weights, one per population element.

      

* * *

## `utils/random~_weightedIndexWith(randomFn, weights)` ⇒ number

Returns a random index into `weights`, where each index's probability
is proportional to its weight. Uses a linear scan: O(n) time, O(1) memory.

**Kind**: inner method of [utils/random](#module_utils/random)  
**Returns**: number - A randomly selected index in `[0, weights.length)`.  

  
    
      ParamTypeDescription
    
  
  

    randomFnfunctionA function returning a uniform random float in [0, 1).

    
    weightsArrayLike.&lt;number&gt;Non-negative weights.

      

* * *
