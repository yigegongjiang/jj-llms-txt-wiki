# utils/logger

Logger utility for Transformers.js with configurable log levels.

* * *

## `utils/logger.logger`

Logger that respects the configured log level in env.logLevel.

**Kind**: static constant of [utils/logger](#module_utils/logger)  
**Example**  
```js
import { logger } from './utils/logger.js';
logger.info('Model loaded successfully');
logger.warn('Deprecated method used');
logger.error('Failed to load model');
logger.debug('Token count:', tokens.length);
```

* * *
