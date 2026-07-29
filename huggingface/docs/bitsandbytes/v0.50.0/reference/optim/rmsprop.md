# RMSprop

RMSprop is an adaptive learning rate optimizer that is very similar to `Adagrad`. RMSprop stores a *weighted average* of the squared past gradients for each parameter and uses it to scale their learning rate. This allows the learning rate to be automatically lower or higher depending on the magnitude of the gradient, and it prevents the learning rate from diminishing.

## RMSprop[[api-class]][[bitsandbytes.optim.RMSprop]]

## RMSprop8bit[[bitsandbytes.optim.RMSprop8bit]]

## RMSprop32bit[[bitsandbytes.optim.RMSprop32bit]]
