# Optimization

This page contains the API reference documentation for learning rate optimizers included in `timm`.

## Optimizers

### Factory functions[[timm.optim.create_optimizer_v2]]

- **model_or_params** -- A PyTorch model or an iterable of parameters/parameter groups.
  If a model is provided, parameters will be automatically extracted and grouped
  based on the other arguments.
- **opt** -- Name of the optimizer to create (e.g., 'adam', 'adamw', 'sgd').
  Use list_optimizers() to see available options.
- **lr** -- Learning rate. If None, will use the optimizer's default.
- **weight_decay** -- Weight decay factor. Will be used to create param groups if model_or_params is a model.
- **momentum** -- Momentum factor for optimizers that support it. Only used if the
  chosen optimizer accepts a momentum parameter.
- **foreach** -- Enable/disable foreach (multi-tensor) implementation if available.
  If None, will use optimizer-specific defaults.
- **filter_bias_and_bn** -- If True, bias, norm layer parameters (all 1d params) will not have
  weight decay applied. Only used when model_or_params is a model and
  weight_decay > 0.
- **fallback_list** -- Collection of parameter name patterns to use fallback optimizer for
  hybrid optimizers (e.g., AdamW for Muon). Supports wildcard matching.
- **fallback_no_weight_decay** -- If True, params in model's no_weight_decay() list will use
  fallback optimizer for hybrid optimizers (e.g., AdamW for Muon).
- **layer_decay** -- Optional layer-wise learning rate decay factor. If provided,
  learning rates will be scaled by layer_decay^(max_depth - layer_depth).
  Only used when model_or_params is a model.
- **param_group_fn** -- Optional function to create custom parameter groups.
  If provided, other parameter grouping options will be ignored.
- ****kwargs** -- Additional optimizer-specific arguments (e.g., betas for Adam).Configured optimizer instance.
Create an optimizer instance via timm registry.

Creates and configures an optimizer with appropriate parameter groups and settings.
Supports automatic parameter group creation for weight decay and layer-wise learning
rates, as well as custom parameter grouping.

Examples:
>>> # Basic usage with a model
>>> optimizer = create_optimizer_v2(model, 'adamw', lr=1e-3)

>>> # SGD with momentum and weight decay
>>> optimizer = create_optimizer_v2(
...     model, 'sgd', lr=0.1, momentum=0.9, weight_decay=1e-4
... )

>>> # Adam with layer-wise learning rate decay
>>> optimizer = create_optimizer_v2(
...     model, 'adam', lr=1e-3, layer_decay=0.7
... )

>>> # Custom parameter groups
>>> def group_fn(model):
...     return [
...         {'params': model.backbone.parameters(), 'lr': 1e-4},
...         {'params': model.head.parameters(), 'lr': 1e-3}
...     ]
>>> optimizer = create_optimizer_v2(
...     model, 'sgd', param_group_fn=group_fn
... )

Note:
Parameter group handling precedence:
1. If param_group_fn is provided, it will be used exclusively
2. If layer_decay is provided, layer-wise groups will be created
3. If weight_decay > 0 and filter_bias_and_bn is True, weight decay groups will be created
4. Otherwise, all parameters will be in a single group

- **filter** -- Wildcard style filter string or list of filter strings
  (e.g., 'adam*' for all Adam variants, or ['adam*', '*8bit'] for
  Adam variants and 8-bit optimizers). Empty string means no filtering.
- **exclude_filters** -- Optional list of wildcard patterns to exclude. For example,
  ['*8bit', 'fused*'] would exclude 8-bit and fused implementations.
- **with_description** -- If True, returns tuples of (name, description) instead of
  just names. Descriptions provide brief explanations of optimizer characteristics.If with_description is FalseList of optimizer names as strings (e.g., ['adam', 'adamw', ...])
If with_description is True:
List of tuples of (name, description) (e.g., [('adam', 'Adaptive Moment...'), ...])
List available optimizer names, optionally filtered.

List all registered optimizers, with optional filtering using wildcard patterns.
Optimizers can be filtered using include and exclude patterns, and can optionally
return descriptions with each optimizer name.

Examples:
>>> list_optimizers()
['adam', 'adamw', 'sgd', ...]

>>> list_optimizers(['la*', 'nla*'])  # List lamb & lars
['lamb', 'lambc', 'larc', 'lars', 'nlarc', 'nlars']

>>> list_optimizers('*adam*', exclude_filters=['bnb*', 'fused*'])  # Exclude bnb & apex adam optimizers
['adam', 'adamax', 'adamp', 'adamw', 'nadam', 'nadamw', 'radam']

>>> list_optimizers(with_description=True)  # Get descriptions
[('adabelief', 'Adapts learning rate based on gradient prediction error'),
('adadelta', 'torch.optim Adadelta, Adapts learning rates based on running windows of gradients'),
('adafactor', 'Memory-efficient implementation of Adam with factored gradients'),
...]

- **name** -- Name of the optimizer to retrieve (e.g., 'adam', 'sgd')
- **bind_defaults** -- If True, returns a partial function with default arguments from OptimInfo bound.
  If False, returns the raw optimizer class.If bind_defaults is FalseThe optimizer class (e.g., torch.optim.Adam)
If bind_defaults is True:
A partial function with default arguments bound- ``ValueError`` -- If optimizer name is not found in registry</raises><raisederrors>``ValueError``
Get optimizer class by name with option to bind default arguments.

Retrieves the optimizer class or a partial function with default arguments bound.
This allows direct instantiation of optimizers with their default configurations
without going through the full factory.

Examples:
>>> # Get SGD with nesterov momentum default
>>> SGD = get_optimizer_class('sgd')  # nesterov=True bound
>>> opt = SGD(model.parameters(), lr=0.1, momentum=0.9)

>>> # Get raw optimizer class
>>> SGD = get_optimizer_class('sgd')
>>> opt = SGD(model.parameters(), lr=1e-3, momentum=0.9)

### Optimizer Classes[[timm.optim.AdaBelief]]

- **params** (iterable) -- iterable of parameters to optimize or dicts defining
  parameter groups
- **lr** (float, optional) -- learning rate (default: 1e-3)
- **betas** (Tuple[float, float], optional) -- coefficients used for computing
  running averages of gradient and its square (default: (0.9, 0.999))
- **eps** (float, optional) -- term added to the denominator to improve
  numerical stability (default: 1e-16)
- **weight_decay** (float, optional) -- weight decay (L2 penalty) (default: 0)
- **amsgrad** (boolean, optional) -- whether to use the AMSGrad variant of this
  algorithm from the paper `On the Convergence of Adam and Beyond`_
  (default: False)
- **decoupled_decay** (boolean, optional) -- (default: True) If set as True, then
  the optimizer uses decoupled weight decay as in AdamW
- **fixed_decay** (boolean, optional) -- (default: False) This is used when weight_decouple
  is set as True.
  When fixed_decay == True, the weight decay is performed as
  $W_{new} = W_{old} - W_{old} \times decay$.
  When fixed_decay == False, the weight decay is performed as
  $W_{new} = W_{old} - W_{old} \times decay \times lr$. Note that in this case, the
  weight decay ratio decreases with learning rate (lr).
- **rectify** (boolean, optional) -- (default: True) If set as True, then perform the rectified
  update similar to RAdam
- **degenerated_to_sgd** (boolean, optional) (default --True) If set as True, then perform SGD update
  when variance of gradient is high
Implements AdaBelief algorithm. Modified from Adam in PyTorch

reference: AdaBelief Optimizer, adapting stepsizes by the belief in observed gradients, NeurIPS 2020

For a complete table of recommended hyperparameters, see https://github.com/juntang-zhuang/Adabelief-Optimizer'
For example train/args for EfficientNet see these gists
- link to train_script: https://gist.github.com/juntang-zhuang/0a501dd51c02278d952cf159bc233037
- link to args.yaml: https://gist.github.com/juntang-zhuang/517ce3c27022b908bb93f78e4f786dc3

- **closure** (callable, optional) -- A closure that reevaluates the model
  and returns the loss.
Performs a single optimization step.

Implements Adafactor algorithm.

This implementation is based on: `Adafactor: Adaptive Learning Rates with Sublinear Memory Cost`
(see https://arxiv.org/abs/1804.04235)

Note that this optimizer internally adjusts the learning rate depending on the
*scale_parameter*, *relative_step* and *warmup_init* options.

To use a manual (external) learning rate schedule you should set `scale_parameter=False` and
`relative_step=False`.

Ags:
params: iterable of parameters to optimize or dicts defining parameter groups
lr: external learning rate
eps: regularization constants for square gradient and parameter scale respectively
eps_scale: regularization constants for parameter scale respectively
clip_threshold: threshold of root-mean-square of final gradient update
decay_rate: coefficient used to compute running averages of square gradient
beta1: coefficient used for computing running averages of gradient
weight_decay: weight decay
scale_parameter: if True, learning rate is scaled by root-mean-square of parameter
warmup_init: time-dependent learning rate computation depends on whether warm-up initialization is being used

- **closure** (callable, optional) -- A closure that reevaluates the model and returns the loss.
Performs a single optimization step.

PyTorch implementation of BigVision's Adafactor variant with both single and multi tensor implementations.

Adapted from https://github.com/google-research/big_vision by Ross Wightman

- **params** (iterable) -- iterable of parameters to optimize or dicts defining parameter groups
- **lr** (float, optional) -- learning rate (default: 0.1)
- **betas** ((float, float), optional) -- coefficients used for computing running averages of gradient and the
  squared hessian trace (default: (0.9, 0.999))
- **eps** (float, optional) -- term added to the denominator to improve numerical stability (default: 1e-8)
- **weight_decay** (float, optional) -- weight decay (L2 penalty) (default: 0.0)
- **hessian_power** (float, optional) -- exponent of the hessian trace (default: 1.0)
- **update_each** (int, optional) -- compute the hessian trace approximation only after *this* number of steps
  (to save time) (default: 1)
- **n_samples** (int, optional) -- how many times to sample `z` for the approximation of the hessian trace (default: 1)

Implements the AdaHessian algorithm from "ADAHESSIAN: An Adaptive Second OrderOptimizer for Machine Learning"

Gets all parameters in all param_groups with gradients

Computes the Hutchinson approximation of the hessian trace and accumulates it for each trainable parameter.

- **closure** (callable, optional) -- a closure that reevaluates the model and returns the loss (default -- None)

Performs a single optimization step.

Zeros out the accumulated hessian traces.

- **params** -- Iterable of parameters to optimize or dicts defining parameter groups.
- **lr** -- Learning rate.
- **betas** -- Coefficients used for first- and second-order moments.
- **eps** -- Term added to the denominator to improve numerical stability.
- **weight_decay** -- Decoupled weight decay (L2 penalty)
- **no_prox** -- How to perform the weight decay
- **caution** -- Enable caution from 'Cautious Optimizers'
- **foreach** -- If True would use torch._foreach implementation. Faster but uses slightly more memory.
Implements a pytorch variant of Adan.

Adan was proposed in Adan: Adaptive Nesterov Momentum Algorithm for Faster Optimizing Deep Models
https://arxiv.org/abs/2208.06677

Performs a single optimization step.

ADOPT: Modified Adam Can Converge with Any β2 with the Optimal Rate: https://arxiv.org/abs/2411.02853

- **closure** (Callable, optional) -- A closure that reevaluates the model
  and returns the loss.
Perform a single optimization step.

- **params** -- Iterable of parameters to optimize or dicts defining parameter groups.
- **lr** -- Learning rate
- **betas** -- Coefficients used for computing running averages of gradient and its norm.
- **eps** -- Term added to the denominator to improve numerical stability.
- **weight_decay** -- Weight decay
- **grad_averaging** -- Whether apply (1-beta2) to grad when calculating running averages of gradient.
- **max_grad_norm** -- Value used to clip global grad norm.
- **trust_clip** -- Enable LAMBC trust ratio clipping.
- **always_adapt** -- Apply adaptive learning rate to 0.0 weight decay parameter.
- **caution** -- Apply caution.
- **decoupled** -- apply decoupled weight decay
- **corrected_weight_decay** -- apply corrected weight decay (lr**2 / max_lr) when using decoupled_decay
Implements a pure pytorch variant of FuseLAMB (NvLamb variant) optimizer from apex.optimizers.FusedLAMB
reference: https://github.com/NVIDIA/DeepLearningExamples/blob/master/PyTorch/LanguageModeling/Transformer-XL/pytorch/lamb.py

LAMB was proposed in:
- Large Batch Optimization for Deep Learning - Training BERT in 76 minutes:  https://arxiv.org/abs/1904.00962
- On the Convergence of Adam and Beyond: https://openreview.net/forum?id=ryQu7f-RZ

- **closure** (callable, optional) -- A closure that reevaluates the model
  and returns the loss.
Performs a single optimization step.

LaProp Optimizer

Paper: LaProp: Separating Momentum and Adaptivity in Adam, https://arxiv.org/abs/2002.04839

- **closure** (callable, optional) -- A closure that reevaluates the model
  and returns the loss.
Performs a single optimization step.

- **params** (iterable) -- iterable of parameters to optimize or dicts defining parameter groups.
- **lr** (float, optional) -- learning rate (default: 1.0).
- **momentum** (float, optional) -- momentum factor (default: 0)
- **weight_decay** (float, optional) -- weight decay (L2 penalty) (default: 0)
- **dampening** (float, optional) -- dampening for momentum (default: 0)
- **nesterov** (bool, optional) -- enables Nesterov momentum (default: False)
- **trust_coeff** (float) -- trust coefficient for computing adaptive lr / trust_ratio (default: 0.001)
- **eps** (float) -- eps for division denominator (default: 1e-8)
- **trust_clip** (bool) -- enable LARC trust ratio clipping (default: False)
- **always_adapt** (bool) -- always apply LARS LR adapt, otherwise only when group weight_decay != 0 (default: False)
LARS for PyTorch

Paper: `Large batch training of Convolutional Networks` - https://arxiv.org/pdf/1708.03888.pdf

- **closure** (callable, optional) -- A closure that reevaluates the model and returns the loss.
Performs a single optimization step.

Implements Lion algorithm.

- **closure** -- A closure that reevaluates the model and returns the loss.the loss.
Performs a single optimization step.

- **params** (iterable) --
  Iterable of parameters to optimize or dicts defining parameter groups.
- **lr** (float) --
  Learning rate (default: 1e-2).
- **momentum** (float) --
  Momentum value in  the range [0,1) (default: 0.9).
- **weight_decay** (float) --
  Weight decay, i.e. a L2 penalty (default: 0).
- **eps** (float) --
  Term added to the denominator outside of the root operation to improve numerical stability. (default: 1e-6).

MADGRAD_: A Momentumized, Adaptive, Dual Averaged Gradient Method for Stochastic
Optimization.

.. _MADGRAD: https://arxiv.org/abs/2101.11075

MADGRAD is a general purpose optimizer that can be used in place of SGD or
Adam may converge faster and generalize better. Currently GPU-only.
Typically, the same learning rate schedule that is used for SGD or Adam may
be used. The overall learning rate is not comparable to either method and
should be determined by a hyper-parameter sweep.

MADGRAD requires less weight decay than other methods, often as little as
zero. Momentum values used for SGD or Adam's beta1 should work here also.

On sparse problems both weight_decay and momentum should be set to 0.

- **closure** (callable, optional) -- A closure that reevaluates the model and returns the loss.
Performs a single optimization step.

MARS Optimizer

Paper: MARS: Unleashing the Power of Variance Reduction for Training Large Models
https://arxiv.org/abs/2411.10438

- **closure** (callable, optional) -- A closure that reevaluates the model
  and returns the loss.
Performs a single optimization step.

- **params** -- iterable of parameters to optimize or dicts defining parameter groups
- **lr** -- learning rate
- **betas** -- coefficients used for computing running averages of gradient and its square
- **eps** -- term added to the denominator to improve numerical stability
- **weight_decay** -- weight decay coefficient
- **caution** -- enable caution
- **corrected_weight_decay** -- apply corrected weight decay (lr**2 / max_lr)
Implements NAdamW algorithm.

See Table 1 in https://arxiv.org/abs/1910.05446 for the implementation of
the NAdam algorithm (there is also a comment in the code which highlights
the only difference of NAdamW and AdamW).

For further details regarding the algorithm we refer to
- Decoupled Weight Decay Regularization: https://arxiv.org/abs/1711.05101
- On the Convergence of Adam and Beyond: https://openreview.net/forum?id=ryQu7f-RZ

- **closure** (callable, optional) -- A closure that reevaluates the model
  and returns the loss.
Performs a single optimization step.

- **params** (iterable) -- iterable of parameters to optimize or dicts defining
  parameter groups
- **lr** (float, optional) -- learning rate (default: 1e-3)
- **betas** (Tuple[float, float], optional) -- coefficients used for computing
  running averages of gradient and its square (default: (0.95, 0.98))
- **eps** (float, optional) -- term added to the denominator to improve
  numerical stability (default: 1e-8)
- **weight_decay** (float, optional) -- weight decay (L2 penalty) (default: 0)
- **grad_averaging** -- gradient averaging
- **amsgrad** (boolean, optional) -- whether to use the AMSGrad variant of this
  algorithm from the paper `On the Convergence of Adam and Beyond`_
  (default: False)

Implements Novograd algorithm.

- **closure** (callable, optional) -- A closure that reevaluates the model
- **and** returns the loss. --
Performs a single optimization step.

- **params** -- iterable of parameters to optimize or dicts defining parameter groups
- **lr** -- learning rate
- **momentum** -- momentum factor
- **alpha** -- smoothing (decay) constant
- **eps** -- term added to the denominator to improve numerical stability
- **centered** -- if `True`, compute the centered RMSProp, the gradient is normalized by an estimation of its variance
- **weight_decay** -- weight decay (L2 penalty) (default: 0)
- **decoupled_decay** -- decoupled weight decay as per https://arxiv.org/abs/1711.05101
- **corrected_weight_decay** -- apply corrected weight decay (lr**2 / max_lr) when decoupled_decay is True
- **lr_in_momentum** -- learning rate scaling is included in the momentum buffer update as per defaults in Tensorflow
- **caution** -- apply caution
Implements RMSprop algorithm (TensorFlow style epsilon)

NOTE: This is a direct cut-and-paste of PyTorch RMSprop with eps applied before sqrt
and a few other modifications to closer match Tensorflow for matching hyper-params.

Noteworthy changes include:
1. Epsilon applied inside square-root
2. square_avg initialized to ones
3. LR scaling of update accumulated in momentum buffer

Proposed by G. Hinton in his
[course](http://www.cs.toronto.edu/~tijmen/csc321/slides/lecture_slides_lec6.pdf).

The centered version first appears in [Generating Sequences
With Recurrent Neural Networks](https://arxiv.org/pdf/1308.0850v5.pdf).

- **closure** (callable, optional) -- A closure that reevaluates the model
  and returns the loss.
Performs a single optimization step.

- **closure** (Callable, optional) -- A closure that reevaluates the model
  and returns the loss.
Performs a single optimization step.
