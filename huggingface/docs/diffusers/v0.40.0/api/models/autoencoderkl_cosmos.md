# AutoencoderKLCosmos

[Cosmos Tokenizers](https://github.com/NVIDIA/Cosmos-Tokenizer).

Supported models:
- [nvidia/Cosmos-1.0-Tokenizer-CV8x8x8](https://huggingface.co/nvidia/Cosmos-1.0-Tokenizer-CV8x8x8)

The model can be loaded with the following code snippet.

```python
from diffusers import AutoencoderKLCosmos

vae = AutoencoderKLCosmos.from_pretrained("nvidia/Cosmos-1.0-Tokenizer-CV8x8x8", subfolder="vae")
```

## AutoencoderKLCosmos[[diffusers.AutoencoderKLCosmos]]

#### diffusers.AutoencoderKLCosmos[[diffusers.AutoencoderKLCosmos]]

```python
diffusers.AutoencoderKLCosmos(in_channels: int = 3, out_channels: int = 3, latent_channels: int = 16, encoder_block_out_channels: tuple[int, ...] = (128, 256, 512, 512), decode_block_out_channels: tuple[int, ...] = (256, 512, 512, 512), attention_resolutions: tuple[int, ...] = (32,), resolution: int = 1024, num_layers: int = 2, patch_size: int = 4, patch_type: str = 'haar', scaling_factor: float = 1.0, spatial_compression_ratio: int = 8, temporal_compression_ratio: int = 8, latents_mean: list[float] | None = [0.11362758, -0.0171717, 0.03071163, 0.02046862, 0.01931456, 0.02138567, 0.01999342, 0.02189187, 0.02011935, 0.01872694, 0.02168613, 0.02207148, 0.01986941, 0.01770413, 0.02067643, 0.02028245, 0.19125476, 0.04556972, 0.0595558, 0.05315534, 0.05496629, 0.05356264, 0.04856596, 0.05327453, 0.05410472, 0.05597149, 0.05524866, 0.05181874, 0.05071663, 0.05204537, 0.0564108, 0.05518042, 0.01306714, 0.03341161, 0.03847246, 0.02810185, 0.02790166, 0.02920026, 0.02823597, 0.02631033, 0.0278531, 0.02880507, 0.02977769, 0.03145441, 0.02888389, 0.03280773, 0.03484927, 0.03049198, -0.00197727, 0.07534957, 0.04963879, 0.05530893, 0.05410828, 0.05252541, 0.05029899, 0.05321025, 0.05149245, 0.0511921, 0.04643495, 0.04604527, 0.04631618, 0.04404101, 0.04403536, 0.04499495, -0.02994183, -0.04787003, -0.01064558, -0.01779824, -0.01490502, -0.02157517, -0.0204778, -0.02180816, -0.01945375, -0.02062863, -0.02192209, -0.02520639, -0.02246656, -0.02427533, -0.02683363, -0.02762006, 0.08019473, -0.13005368, -0.07568636, -0.06082374, -0.06036175, -0.05875364, -0.05921887, -0.05869788, -0.05273941, -0.052565, -0.05346428, -0.05456541, -0.053657, -0.05656897, -0.05728589, -0.05321847, 0.16718403, -0.00390146, 0.0379406, 0.0356561, 0.03554131, 0.03924074, 0.03873615, 0.04187329, 0.04226924, 0.04378717, 0.04684274, 0.05117614, 0.04547792, 0.05251586, 0.05048339, 0.04950784, 0.09564418, 0.0547128, 0.08183969, 0.07978633, 0.08076023, 0.08108605, 0.08011818, 0.07965573, 0.08187773, 0.08350263, 0.08101469, 0.0786941, 0.0774442, 0.07724521, 0.07830418, 0.07599796, -0.04987567, 0.05923908, -0.01058746, -0.01177603, -0.01116162, -0.01364149, -0.01546014, -0.0117213, -0.01780043, -0.01648314, -0.02100247, -0.02104417, -0.02482123, -0.02611689, -0.02561143, -0.02597336, -0.05364667, 0.08211684, 0.04686937, 0.04605641, 0.04304186, 0.0397355, 0.03686767, 0.04087112, 0.03704741, 0.03706401, 0.03120073, 0.03349091, 0.03319963, 0.03205781, 0.03195127, 0.03180481, 0.16427967, -0.11048453, -0.04595276, -0.04982893, -0.05213465, -0.04809378, -0.05080318, -0.04992863, -0.04493337, -0.0467619, -0.04884703, -0.04627892, -0.04913311, -0.04955709, -0.04533982, -0.04570218, -0.10612928, -0.05121198, -0.06761009, -0.07251801, -0.07265285, -0.07417855, -0.07202412, -0.07499027, -0.07625481, -0.07535747, -0.07638787, -0.07920305, -0.07596069, -0.07959418, -0.08265036, -0.07955471, -0.16888915, 0.0753242, 0.04062594, 0.03375093, 0.03337452, 0.03699376, 0.03651138, 0.03611023, 0.03555622, 0.03378554, 0.0300498, 0.03395559, 0.02941847, 0.03156432, 0.03431173, 0.03016853, -0.03415358, -0.01699573, -0.04029295, -0.04912157, -0.0498858, -0.04917918, -0.04918056, -0.0525189, -0.05325506, -0.05341973, -0.04983329, -0.04883146, -0.04985548, -0.04736718, -0.0462027, -0.04836091, 0.02055675, 0.03419799, -0.02907669, -0.04350509, -0.04156144, -0.04234421, -0.04446109, -0.04461774, -0.04882839, -0.04822346, -0.04502493, -0.0506244, -0.05146913, -0.04655267, -0.04862994, -0.04841615, 0.20312774, -0.07208502, -0.03635615, -0.03556088, -0.04246174, -0.04195838, -0.04293778, -0.04071276, -0.04240569, -0.04125213, -0.04395144, -0.03959096, -0.04044993, -0.04015875, -0.04088107, -0.03885176], latents_std: list[float] | None = [0.56700271, 0.65488982, 0.65589428, 0.66524369, 0.66619784, 0.6666382, 0.6720838, 0.66955978, 0.66928875, 0.67108786, 0.67092526, 0.67397463, 0.67894882, 0.67668313, 0.67769569, 0.67479557, 0.85245121, 0.8688373, 0.87348086, 0.88459337, 0.89135885, 0.8910504, 0.89714909, 0.89947474, 0.90201765, 0.90411824, 0.90692616, 0.90847772, 0.90648711, 0.91006982, 0.91033435, 0.90541548, 0.84960359, 0.85863352, 0.86895317, 0.88460612, 0.89245003, 0.89451706, 0.89931005, 0.90647358, 0.90338236, 0.90510076, 0.91008312, 0.90961218, 0.9123717, 0.91313171, 0.91435546, 0.91565102, 0.91877103, 0.85155135, 0.857804, 0.86998034, 0.87365264, 0.88161767, 0.88151032, 0.88758916, 0.89015514, 0.89245576, 0.89276224, 0.89450496, 0.90054202, 0.89994133, 0.90136105, 0.90114892, 0.77755755, 0.81456852, 0.81911844, 0.83137071, 0.83820474, 0.83890373, 0.84401101, 0.84425181, 0.84739357, 0.84798753, 0.85249585, 0.85114998, 0.85160935, 0.85626358, 0.85677862, 0.85641026, 0.69903517, 0.71697885, 0.71696913, 0.72583169, 0.72931731, 0.73254126, 0.73586977, 0.73734969, 0.73664582, 0.74084908, 0.74399322, 0.74471819, 0.74493188, 0.74824578, 0.75024873, 0.75274801, 0.8187142, 0.82251883, 0.82616025, 0.83164483, 0.84072375, 0.8396467, 0.84143305, 0.84880769, 0.8503468, 0.85196948, 0.85211051, 0.85386664, 0.85410017, 0.85439342, 0.85847849, 0.85385275, 0.67583984, 0.68259847, 0.69198853, 0.69928843, 0.70194328, 0.70467001, 0.70755547, 0.70917857, 0.71007699, 0.70963502, 0.71064079, 0.71027333, 0.71291167, 0.71537536, 0.71902508, 0.71604162, 0.72450989, 0.71979928, 0.72057378, 0.73035461, 0.73329622, 0.73660028, 0.73891461, 0.74279994, 0.74105692, 0.74002433, 0.74257588, 0.74416119, 0.74543899, 0.74694443, 0.74747062, 0.74586403, 0.90176988, 0.90990674, 0.91106802, 0.92163783, 0.92390233, 0.93056196, 0.93482202, 0.93642414, 0.93858379, 0.94064975, 0.94078934, 0.94325715, 0.94955301, 0.94814706, 0.95144123, 0.94923073, 0.49853548, 0.64968109, 0.6427654, 0.64966393, 0.6487664, 0.65203559, 0.6584242, 0.65351611, 0.65464371, 0.6574859, 0.65626335, 0.66123748, 0.66121179, 0.66077942, 0.66040152, 0.66474909, 0.61986589, 0.69138134, 0.6884557, 0.6955843, 0.69765401, 0.70015347, 0.70529598, 0.70468754, 0.70399523, 0.70479989, 0.70887572, 0.71126866, 0.7097227, 0.71249932, 0.71231949, 0.71175605, 0.35586974, 0.68723857, 0.68973219, 0.69958478, 0.6943453, 0.6995818, 0.70980215, 0.69899458, 0.70271689, 0.70095056, 0.69912851, 0.70522696, 0.70392174, 0.70916915, 0.70585734, 0.70373541, 0.98101336, 0.89024764, 0.89607251, 0.90678179, 0.91308665, 0.91812348, 0.91980827, 0.92480654, 0.92635667, 0.92887944, 0.93338072, 0.93468094, 0.93619436, 0.93906063, 0.94191772, 0.94471723, 0.83202779, 0.84106231, 0.84463632, 0.85829508, 0.86319661, 0.86751342, 0.86914337, 0.87085921, 0.87286359, 0.87537396, 0.87931138, 0.88054478, 0.8811838, 0.88872558, 0.88942474, 0.88934827, 0.44025335, 0.63061613, 0.63110614, 0.63601959, 0.6395812, 0.64104342, 0.65019929, 0.6502797, 0.64355946, 0.64657205, 0.64847094, 0.64728117, 0.64972943, 0.65162975, 0.65328044, 0.64914775])
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cosmos.py#L879)

**Parameters:**

in_channels (`int`, defaults to `3`) : Number of input channels.

out_channels (`int`, defaults to `3`) : Number of output channels.

latent_channels (`int`, defaults to `16`) : Number of latent channels.

encoder_block_out_channels (`tuple[int, ...]`, defaults to `(128, 256, 512, 512)`) : Number of output channels for each encoder down block.

decode_block_out_channels (`tuple[int, ...]`, defaults to `(256, 512, 512, 512)`) : Number of output channels for each decoder up block.

attention_resolutions (`tuple[int, ...]`, defaults to `(32,)`) : list of image/video resolutions at which to apply attention.

resolution (`int`, defaults to `1024`) : Base image/video resolution used for computing whether a block should have attention layers.

num_layers (`int`, defaults to `2`) : Number of resnet blocks in each encoder/decoder block.

patch_size (`int`, defaults to `4`) : Patch size used for patching the input image/video.

patch_type (`str`, defaults to `haar`) : Patch type used for patching the input image/video. Can be either `haar` or `rearrange`.

scaling_factor (`float`, defaults to `1.0`) : The component-wise standard deviation of the trained latent space computed using the first batch of the training set. This is used to scale the latent space to have unit variance when training the diffusion model. The latents are scaled with the formula `z = z * scaling_factor` before being passed to the diffusion model. When decoding, the latents are scaled back to the original scale with the formula: `z = 1 / scaling_factor * z`. For more details, refer to sections 4.3.2 and D.1 of the [High-Resolution Image Synthesis with Latent Diffusion Models](https://huggingface.co/papers/2112.10752) paper. Not applicable in Cosmos, but we default to 1.0 for consistency.

spatial_compression_ratio (`int`, defaults to `8`) : The spatial compression ratio to apply in the VAE. The number of downsample blocks is determined using this.

temporal_compression_ratio (`int`, defaults to `8`) : The temporal compression ratio to apply in the VAE. The number of downsample blocks is determined using this.

Autoencoder used in [Cosmos](https://huggingface.co/papers/2501.03575).

#### decode[[diffusers.AutoencoderKLCosmos.decode]]

```python
decode(z: torch.Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cosmos.py#L1062)

#### encode[[diffusers.AutoencoderKLCosmos.encode]]

```python
encode(x: torch.Tensor, return_dict: bool = True)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cosmos.py#L1040)

#### enable_tiling[[diffusers.AutoencoderKLCosmos.enable_tiling]]

```python
enable_tiling(tile_sample_min_height: int | None = None, tile_sample_min_width: int | None = None, tile_sample_min_num_frames: int | None = None, tile_sample_stride_height: float | None = None, tile_sample_stride_width: float | None = None, tile_sample_stride_num_frames: float | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cosmos.py#L1001)

**Parameters:**

tile_sample_min_height (`int`, *optional*) : The minimum height required for a sample to be separated into tiles across the height dimension.

tile_sample_min_width (`int`, *optional*) : The minimum width required for a sample to be separated into tiles across the width dimension.

tile_sample_stride_height (`int`, *optional*) : The minimum amount of overlap between two consecutive vertical tiles. This is to ensure that there are no tiling artifacts produced across the height dimension.

tile_sample_stride_width (`int`, *optional*) : The stride between two consecutive horizontal tiles. This is to ensure that there are no tiling artifacts produced across the width dimension.

Enable tiled VAE decoding. When this option is enabled, the VAE will split the input tensor into tiles to
compute decoding and encoding in several steps. This is useful for saving a large amount of memory and to allow
processing larger images.

#### forward[[diffusers.AutoencoderKLCosmos.forward]]

```python
forward(sample: torch.Tensor, sample_posterior: bool = False, return_dict: bool = True, generator: torch.Generator | None = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/autoencoder_kl_cosmos.py#L1074)

**Parameters:**

sample (`torch.Tensor`) : Input sample.

sample_posterior (`bool`, *optional*, defaults to `False`) : Whether to sample from the posterior.

return_dict (`bool`, *optional*, defaults to `True`) : Whether or not to return a `DecoderOutput` instead of a plain tuple.

generator (`torch.Generator`, *optional*) : A [`torch.Generator`](https://pytorch.org/docs/stable/generated/torch.Generator.html) to make sampling deterministic.

**Returns:** `~models.vae.DecoderOutput` or `tuple`

If `return_dict` is True, a `~models.vae.DecoderOutput` is returned, otherwise a plain `tuple` is
returned.

## AutoencoderKLOutput[[diffusers.models.modeling_outputs.AutoencoderKLOutput]]

#### diffusers.models.modeling_outputs.AutoencoderKLOutput[[diffusers.models.modeling_outputs.AutoencoderKLOutput]]

```python
diffusers.models.modeling_outputs.AutoencoderKLOutput(latent_dist: DiagonalGaussianDistribution)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/modeling_outputs.py#L7)

**Parameters:**

latent_dist (`DiagonalGaussianDistribution`) : Encoded outputs of `Encoder` represented as the mean and logvar of `DiagonalGaussianDistribution`. `DiagonalGaussianDistribution` allows for sampling latents from the distribution.

Output of AutoencoderKL encoding method.

## DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

#### diffusers.models.autoencoders.vae.DecoderOutput[[diffusers.models.autoencoders.vae.DecoderOutput]]

```python
diffusers.models.autoencoders.vae.DecoderOutput(sample: Tensor, commit_loss: typing.Optional[torch.FloatTensor] = None)
```

[Source](https://github.com/huggingface/diffusers/blob/v0.40.0/src/diffusers/models/autoencoders/vae.py#L46)

**Parameters:**

sample (`torch.Tensor` of shape `(batch_size, num_channels, height, width)`) : The decoded output sample from the last layer of the model.

Output of decoding method.
