# Utilities for Trainer

This page lists all the utility functions used by [Trainer](/docs/transformers/v5.14.0/en/main_classes/trainer#transformers.Trainer).

Most of those are only useful if you are studying the code of the Trainer in the library.

## Utilities[[transformers.EvalPrediction]]

- **predictions** (`np.ndarray`) -- Predictions of the model.
- **label_ids** (`np.ndarray`) -- Targets to be matched.
- **inputs** (`np.ndarray`, *optional*) -- Input data passed to the model.
- **losses** (`np.ndarray`, *optional*) -- Loss values computed during evaluation.

Evaluation output (always contains labels), to be used to compute metrics.

An enumeration.

Helper function for reproducible behavior during distributed training. See
https://pytorch.org/docs/stable/notes/randomness.html for pytorch

- **seed** (`int`) --
  The seed to set.
- **deterministic** (`bool`, *optional*, defaults to `False`) --
  Whether to use deterministic algorithms where available. Can slow down training.

Helper function for reproducible behavior to set the seed in `random`, `numpy`, `torch` (if installed).

- **local_rank** (`int`) -- The rank of the local process.

Decorator to make all processes in distributed training wait for each local_master to do something.

## Callbacks internals[[transformers.trainer_callback.CallbackHandler]]

Internal class that just calls the list of callbacks in order.

## Trainer Argument Parser[[transformers.HfArgumentParser]]

- **dataclass_types** (`DataClassType` or `Iterable[DataClassType]`, *optional*) --
  Dataclass type, or list of dataclass types for which we will "fill" instances with the parsed args.
- **kwargs** (`dict[str, Any]`, *optional*) --
  Passed to `argparse.ArgumentParser()` in the regular way.

This subclass of `argparse.ArgumentParser` uses type hints on dataclasses to generate arguments.

The class is designed to play well with the native argparse. In particular, you can add more (non-dataclass backed)
arguments to the parser after initialization and you'll get the output back after parsing as an additional
namespace. Optional: To create sub argument groups use the `_argument_group_name` attribute in the dataclass.

- **args** --
  List of strings to parse. The default is taken from sys.argv. (same as argparse.ArgumentParser)
- **return_remaining_strings** --
  If true, also return a list of remaining argument strings.
- **look_for_args_file** --
  If true, will look for a ".args" file with the same base name as the entry point script for this
  process, and will append its potential content to the command line args.
- **args_filename** --
  If not None, will uses this file instead of the ".args" file specified in the previous argument.
- **args_file_flag** --
  If not None, will look for a file in the command-line args specified with this flag. The flag can be
  specified multiple times and precedence is determined by the order (last one wins).Tuple consisting of- the dataclass instances in the same order as they were passed to the initializer.abspath
- if applicable, an additional namespace for more (non-dataclass backed) arguments added to the parser
  after initialization.
- The potential list of remaining argument strings. (same as argparse.ArgumentParser.parse_known_args)

Parse command-line args into instances of the specified dataclass types.

This relies on argparse's `ArgumentParser.parse_known_args`. See the doc at:
docs.python.org/3/library/argparse.html#argparse.ArgumentParser.parse_args

- **args** (`dict`) --
  dict containing config values
- **allow_extra_keys** (`bool`, *optional*, defaults to `False`) --
  Defaults to False. If False, will raise an exception if the dict contains keys that are not parsed.Tuple consisting of- the dataclass instances in the same order as they were passed to the initializer.

Alternative helper method that does not use `argparse` at all, instead uses a dict and populating the dataclass
types.

- **json_file** (`str` or `os.PathLike`) --
  File name of the json file to parse
- **allow_extra_keys** (`bool`, *optional*, defaults to `False`) --
  Defaults to False. If False, will raise an exception if the json file contains keys that are not
  parsed.Tuple consisting of- the dataclass instances in the same order as they were passed to the initializer.

Alternative helper method that does not use `argparse` at all, instead loading a json file and populating the
dataclass types.

- **yaml_file** (`str` or `os.PathLike`) --
  File name of the yaml file to parse
- **allow_extra_keys** (`bool`, *optional*, defaults to `False`) --
  Defaults to False. If False, will raise an exception if the json file contains keys that are not
  parsed.Tuple consisting of- the dataclass instances in the same order as they were passed to the initializer.

Alternative helper method that does not use `argparse` at all, instead loading a yaml file and populating the
dataclass types.

## Debug Utilities[[transformers.debug_utils.DebugUnderflowOverflow]]

- **model** (`nn.Module`) --
  The model to debug.
- **max_frames_to_save** (`int`, *optional*, defaults to 21) --
  How many frames back to record
- **trace_batch_nums(`list[int]`,** *optional*, defaults to `[]`) --
  Which batch numbers to trace (turns detection off)
- **abort_after_batch_num**  (`int``, *optional*) --
  Whether to abort after a certain batch number has finished

This debug class helps detect and understand where the model starts getting very large or very small, and more
importantly `nan` or `inf` weight and activation elements.

There are 2 working modes:

1. Underflow/overflow detection (default)
2. Specific batch absolute min/max tracing without detection

Mode 1: Underflow/overflow detection

To activate the underflow/overflow detection, initialize the object with the model :

```python
debug_overflow = DebugUnderflowOverflow(model)
```

then run the training as normal and if `nan` or `inf` gets detected in at least one of the weight, input or output
elements this module will throw an exception and will print `max_frames_to_save` frames that lead to this event,
each frame reporting

1. the fully qualified module name plus the class name whose `forward` was run
2. the absolute min and max value of all elements for each module weights, and the inputs and output

For example, here is the header and the last few frames in detection report for `google/mt5-small` run in fp16

mixed precision :

```
Detected inf/nan during batch_number=0
Last 21 forward frames:
abs min  abs max  metadata
[...]
                  encoder.block.2.layer.1.DenseReluDense.wi_0 Linear
2.17e-07 4.50e+00 weight
1.79e-06 4.65e+00 input[0]
2.68e-06 3.70e+01 output
                  encoder.block.2.layer.1.DenseReluDense.wi_1 Linear
8.08e-07 2.66e+01 weight
1.79e-06 4.65e+00 input[0]
1.27e-04 2.37e+02 output
                  encoder.block.2.layer.1.DenseReluDense.wo Linear
1.01e-06 6.44e+00 weight
0.00e+00 9.74e+03 input[0]
3.18e-04 6.27e+04 output
                  encoder.block.2.layer.1.DenseReluDense T5DenseGatedGeluDense
1.79e-06 4.65e+00 input[0]
3.18e-04 6.27e+04 output
                  encoder.block.2.layer.1.dropout Dropout
3.18e-04 6.27e+04 input[0]
0.00e+00      inf output
```

You can see here, that `T5DenseGatedGeluDense.forward` resulted in output activations, whose absolute max value was
around 62.7K, which is very close to fp16's top limit of 64K. In the next frame we have `Dropout` which
renormalizes the weights, after it zeroed some of the elements, which pushes the absolute max value to more than
64K, and we get an overflow.

As you can see it's the previous frames that we need to look into when the numbers start going into very large for
fp16 numbers.

The tracking is done in a forward hook, which gets invoked immediately after `forward` has completed.

By default the last 21 frames are printed. You can change the default to adjust for your needs. For example :

```python
debug_overflow = DebugUnderflowOverflow(model, max_frames_to_save=100)
```

To validate that you have set up this debugging feature correctly, and you intend to use it in a training that
may take hours to complete, first run it with normal tracing enabled for one of a few batches as explained in
the next section.

Mode 2. Specific batch absolute min/max tracing without detection

The second work mode is per-batch tracing with the underflow/overflow detection feature turned off.

Let's say you want to watch the absolute min and max values for all the ingredients of each `forward` call of a

given batch, and only do that for batches 1 and 3. Then you instantiate this class as :

```python
debug_overflow = DebugUnderflowOverflow(model, trace_batch_nums=[1, 3])
```

And now full batches 1 and 3 will be traced using the same format as explained above. Batches are 0-indexed.

This is helpful if you know that the program starts misbehaving after a certain batch number, so you can
fast-forward right to that area.

Early stopping:

You can also specify the batch number after which to stop the training, with :

```python
debug_overflow = DebugUnderflowOverflow(model, trace_batch_nums=[1, 3], abort_after_batch_num=3)
```

This feature is mainly useful in the tracing mode, but you can use it for any mode.

**Performance**:

As this module measures absolute `min`/``max` of each weight of the model on every forward it'll slow the training
down. Therefore remember to turn it off once the debugging needs have been met.
