# utils/audio

Helper module for audio processing.

These functions and classes are only used internally,
meaning an end-user shouldn't need to access anything here.

* [utils/audio](#module_utils/audio)
    * _static_
        * [.RawAudio](#module_utils/audio.RawAudio)
            * [`new RawAudio(audio, sampling_rate)`](#new_module_utils/audio.RawAudio_new)
            * [`.data`](#module_utils/audio.RawAudio+data) ⇒ Float32Array
            * [`.toBlob()`](#module_utils/audio.RawAudio+toBlob) ⇒ Blob
            * [`.save(path)`](#module_utils/audio.RawAudio+save) ⇒ Promise.&lt;void&gt;
        * [`.read_audio`](#module_utils/audio.read_audio)
        * [`.load_audio(url, sampling_rate)`](#module_utils/audio.load_audio) ⇒ Promise.&lt;Float32Array&gt;
            * [`~audio`](#module_utils/audio.load_audio..audio) : Float32Array
        * [`.hanning(M)`](#module_utils/audio.hanning) ⇒ Float64Array
        * [`.hamming(M)`](#module_utils/audio.hamming) ⇒ Float64Array
        * [`.mel_filter_bank(num_frequency_bins, num_mel_filters, min_frequency, max_frequency, sampling_rate, [norm], [mel_scale], [triangularize_in_mel_space])`](#module_utils/audio.mel_filter_bank) ⇒ Array
        * [`.spectrogram(waveform, window, frame_length, hop_length, options)`](#module_utils/audio.spectrogram) ⇒ [Promise.&lt;Tensor&gt;](#Tensor)
        * [`.window_function(window_length, name, options)`](#module_utils/audio.window_function) ⇒ Float64Array
    * _inner_
        * [`~generalized_cosine_window(M, a_0)`](#module_utils/audio..generalized_cosine_window) ⇒ Float64Array
        * [`~hertz_to_mel(freq, [mel_scale])`](#module_utils/audio..hertz_to_mel) ⇒ T
        * [`~mel_to_hertz(mels, [mel_scale])`](#module_utils/audio..mel_to_hertz) ⇒ T
        * [`~_create_triangular_filter_bank(fft_freqs, filter_freqs)`](#module_utils/audio.._create_triangular_filter_bank) ⇒ Array
        * [`~linspace(start, end, num)`](#module_utils/audio..linspace) ⇒
        * [`~padReflect(array, left, right)`](#module_utils/audio..padReflect) ⇒ T
        * [`~_db_conversion_helper(spectrogram, factor, reference, min_value, db_range)`](#module_utils/audio.._db_conversion_helper) ⇒ T
        * [`~amplitude_to_db(spectrogram, [reference], [min_value], [db_range])`](#module_utils/audio..amplitude_to_db) ⇒ T
        * [`~power_to_db(spectrogram, [reference], [min_value], [db_range])`](#module_utils/audio..power_to_db) ⇒ T
        * [`~encodeWAV(chunks, rate)`](#module_utils/audio..encodeWAV) ⇒ Blob

* * *

## utils/audio.RawAudio

**Kind**: static class of [utils/audio](#module_utils/audio)  

* [.RawAudio](#module_utils/audio.RawAudio)
    * [`new RawAudio(audio, sampling_rate)`](#new_module_utils/audio.RawAudio_new)
    * [`.data`](#module_utils/audio.RawAudio+data) ⇒ Float32Array
    * [`.toBlob()`](#module_utils/audio.RawAudio+toBlob) ⇒ Blob
    * [`.save(path)`](#module_utils/audio.RawAudio+save) ⇒ Promise.&lt;void&gt;

* * *

### `new RawAudio(audio, sampling_rate)`

Create a new `RawAudio` object.

  
    
      ParamTypeDescription
    
  
  

    audioFloat32Array | ArrayAudio data, either as a single Float32Array chunk or multiple Float32Array chunks.

    
    sampling_ratenumberSampling rate of the audio data

      

* * *

### `rawAudio.data` ⇒ Float32Array

Get the audio data, accumulating all chunks if necessary.

**Kind**: instance property of [RawAudio](#module_utils/audio.RawAudio)  
**Returns**: Float32Array - The audio data.  

* * *

### `rawAudio.toBlob()` ⇒ Blob

Convert the audio to a blob.

**Kind**: instance method of [RawAudio](#module_utils/audio.RawAudio)  

* * *

### `rawAudio.save(path)` ⇒ Promise.&lt;void&gt;

Save the audio to a wav file.

**Kind**: instance method of [RawAudio](#module_utils/audio.RawAudio)  

  
    
      ParamType
    
  
  

    pathstring
      

* * *

## `utils/audio.read_audio`

***Use [load_audio](load_audio) instead.***

**Kind**: static constant of [utils/audio](#module_utils/audio)  

* * *

## `utils/audio.load_audio(url, sampling_rate)` ⇒ Promise.&lt;Float32Array&gt;

Helper function to load audio from a path/URL.

**Kind**: static method of [utils/audio](#module_utils/audio)  
**Returns**: Promise.&lt;Float32Array&gt; - The decoded audio as a `Float32Array`.  

  
    
      ParamTypeDescription
    
  
  

    urlstring | URLThe path/URL to load the audio from.

    
    sampling_ratenumberThe sampling rate to use when decoding the audio.

      

* * *

### `load_audio~audio` : Float32Array

**Kind**: inner property of [load_audio](#module_utils/audio.load_audio)  

* * *

## `utils/audio.hanning(M)` ⇒ Float64Array

Generates a Hanning window of length M.
See https://numpy.org/doc/stable/reference/generated/numpy.hanning.html for more information.

**Kind**: static method of [utils/audio](#module_utils/audio)  
**Returns**: Float64Array - The generated Hanning window.  

  
    
      ParamTypeDescription
    
  
  

    MnumberThe length of the Hanning window to generate.

      

* * *

## `utils/audio.hamming(M)` ⇒ Float64Array

Generates a Hamming window of length M.
See https://numpy.org/doc/stable/reference/generated/numpy.hamming.html for more information.

**Kind**: static method of [utils/audio](#module_utils/audio)  
**Returns**: Float64Array - The generated Hamming window.  

  
    
      ParamTypeDescription
    
  
  

    MnumberThe length of the Hamming window to generate.

      

* * *

## `utils/audio.mel_filter_bank(num_frequency_bins, num_mel_filters, min_frequency, max_frequency, sampling_rate, [norm], [mel_scale], [triangularize_in_mel_space])` ⇒ Array

Creates a frequency bin conversion matrix used to obtain a mel spectrogram. This is called a *mel filter bank*, and
various implementation exist, which differ in the number of filters, the shape of the filters, the way the filters
are spaced, the bandwidth of the filters, and the manner in which the spectrum is warped. The goal of these
features is to approximate the non-linear human perception of the variation in pitch with respect to the frequency.

**Kind**: static method of [utils/audio](#module_utils/audio)  
**Returns**: Array - Triangular filter bank matrix, which is a 2D array of shape (`num_frequency_bins`, `num_mel_filters`).
This is a projection matrix to go from a spectrogram to a mel spectrogram.  

  
    
      ParamTypeDescription
    
  
  

    num_frequency_binsnumberNumber of frequency bins (should be the same as n_fft // 2 + 1
where n_fft is the size of the Fourier Transform used to compute the spectrogram).

    
    num_mel_filtersnumberNumber of mel filters to generate.

    
    min_frequencynumberLowest frequency of interest in Hz.

    
    max_frequencynumberHighest frequency of interest in Hz. This should not exceed sampling_rate / 2.

    
    sampling_ratenumberSample rate of the audio waveform.

    
    [norm]string | nullIf &quot;slaney&quot;, divide the triangular mel weights by the width of the mel band (area normalization).

    
    [mel_scale]stringThe mel frequency scale to use, &quot;htk&quot; or &quot;slaney&quot;.

    
    [triangularize_in_mel_space]booleanIf this option is enabled, the triangular filter is applied in mel space rather than frequency space.
This should be set to true in order to get the same results as torchaudio when computing mel filters.

      

* * *

## `utils/audio.spectrogram(waveform, window, frame_length, hop_length, options)` ⇒ [Promise.&lt;Tensor&gt;](#Tensor)

Calculates a spectrogram over one waveform using the Short-Time Fourier Transform.

This function can create the following kinds of spectrograms:
  - amplitude spectrogram (`power = 1.0`)
  - power spectrogram (`power = 2.0`)
  - complex-valued spectrogram (`power = None`)
  - log spectrogram (use `log_mel` argument)
  - mel spectrogram (provide `mel_filters`)
  - log-mel spectrogram (provide `mel_filters` and `log_mel`)

In this implementation, the window is assumed to be zero-padded to have the same size as the analysis frame.
A padded window can be obtained from `window_function()`. The FFT input buffer may be larger than the analysis frame,
typically the next power of two.

**Kind**: static method of [utils/audio](#module_utils/audio)  
**Returns**: [Promise.&lt;Tensor&gt;](#Tensor) - Spectrogram of shape `(num_frequency_bins, length)` (regular spectrogram) or shape `(num_mel_filters, length)` (mel spectrogram).  

  
    
      ParamTypeDefaultDescription
    
  
  

    waveformFloat32Array | Float64ArrayThe input waveform of shape (length,). This must be a single real-valued, mono waveform.

    
    windowFloat32Array | Float64ArrayThe windowing function to apply of shape (frame_length,), including zero-padding if necessary. The actual window length may be
shorter than frame_length, but we&#39;re assuming the array has already been zero-padded.

    
    frame_lengthnumberThe length of the analysis frames in samples (a.k.a., fft_length).

    
    hop_lengthnumberThe stride between successive analysis frames in samples.

    
    optionsObject
    
    [options.fft_length]numberThe size of the FFT buffer in samples. This determines how many frequency bins the spectrogram will have.
For optimal speed, this should be a power of two. If null, uses frame_length.

    
    [options.power]number1.0If 1.0, returns the amplitude spectrogram. If 2.0, returns the power spectrogram. If null, returns complex numbers.

    
    [options.center]booleantrueWhether to pad the waveform so that frame t is centered around time t * hop_length. If false, frame
t will start at time t * hop_length.

    
    [options.pad_mode]string&quot;&quot;reflect&quot;&quot;Padding mode used when center is true. Possible values are: &quot;constant&quot; (pad with zeros),
&quot;edge&quot; (pad with edge values), &quot;reflect&quot; (pads with mirrored values).

    
    [options.onesided]booleantrueIf true, only computes the positive frequencies and returns a spectrogram containing fft_length // 2 + 1
frequency bins. If false, also computes the negative frequencies and returns fft_length frequency bins.

    
    [options.preemphasis]numberCoefficient for a low-pass filter that applies pre-emphasis before the DFT.

    
    [options.preemphasis_htk_flavor]booleantrueWhether to apply the pre-emphasis filter in the HTK flavor.

    
    [options.mel_filters]ArrayThe mel filter bank of shape (num_freq_bins, num_mel_filters).
If supplied, applies this filter bank to create a mel spectrogram.

    
    [options.mel_floor]number1e-10Minimum value of mel frequency banks.

    
    [options.log_mel]stringnullHow to convert the spectrogram to log scale. Possible options are:
null (don&#39;t convert), &quot;log&quot; (take the natural logarithm), &quot;log10&quot; (take the base-10 logarithm), &quot;dB&quot; (convert to decibels),
&quot;log10_max_norm&quot; (take log10, then apply (max(x, maxVal - 8) + 4) / 4 normalization, where maxVal is computed from data or given by max_log_mel).
Can only be used when power is not null.

    
    [options.max_log_mel]numberWhen log_mel is &quot;log10_max_norm&quot;, use this fixed value as the max instead of computing from data.

    
    [options.reference]number1.0Sets the input spectrogram value that corresponds to 0 dB. For example, use max(spectrogram)[0] to set
the loudest part to 0 dB. Must be greater than zero.

    
    [options.min_value]number1e-10The spectrogram will be clipped to this minimum value before conversion to decibels, to avoid taking log(0).
For a power spectrogram, the default of 1e-10 corresponds to a minimum of -100 dB. For an amplitude spectrogram, the value 1e-5 corresponds to -100 dB.
Must be greater than zero.

    
    [options.db_range]numberSets the maximum dynamic range in decibels. For example, if db_range = 80, the difference between the
peak value and the smallest value will never be more than 80 dB. Must be greater than zero.

    
    [options.remove_dc_offset]booleanSubtract mean from waveform on each frame, applied before pre-emphasis. This should be set to true in
order to get the same results as torchaudio.compliance.kaldi.fbank when computing mel filters.

    
    [options.max_num_frames]numberIf provided, limits the number of frames to compute to this value.

    
    [options.min_num_frames]numberIf provided, ensures the number of frames to compute is at least this value.

    
    [options.do_pad]booleantrueIf true, pads the output spectrogram to have max_num_frames frames.

    
    [options.transpose]booleanfalseIf true, the returned spectrogram will have shape (num_frames, num_frequency_bins/num_mel_filters). If false, the returned spectrogram will have shape (num_frequency_bins/num_mel_filters, num_frames).

    
    [options.mel_offset]number0Offset to add to the mel spectrogram to avoid taking the log of zero.

    
    [options.mel_floor_mode]string&quot;&quot;clamp&quot;&quot;If mel_offset is provided, this option determines how to apply it. If &quot;clamp&quot;, the mel spectrogram will be clamped to have a minimum value of mel_offset. If &quot;add&quot;, mel_offset will be added to all values of the mel spectrogram.

      

* * *

## `utils/audio.window_function(window_length, name, options)` ⇒ Float64Array

Returns an array containing the specified window.

**Kind**: static method of [utils/audio](#module_utils/audio)  
**Returns**: Float64Array - The window of shape `(window_length,)` or `(frame_length,)`.  

  
    
      ParamTypeDefaultDescription
    
  
  

    window_lengthnumberThe length of the window in samples.

    
    namestringThe name of the window function.

    
    optionsObjectAdditional options.

    
    [options.periodic]booleantrueWhether the window is periodic or symmetric.

    
    [options.frame_length]numberThe length of the analysis frames in samples.
Provide a value for frame_length if the window is smaller than the frame length, so that it will be zero-padded.

    
    [options.center]booleantrueWhether to center the window inside the FFT buffer. Only used when frame_length is provided.

      

* * *

## `utils/audio~generalized_cosine_window(M, a_0)` ⇒ Float64Array

Helper function to generate windows that are special cases of the generalized cosine window.
See https://www.mathworks.com/help/signal/ug/generalized-cosine-windows.html for more information.

**Kind**: inner method of [utils/audio](#module_utils/audio)  
**Returns**: Float64Array - The generated window.  

  
    
      ParamTypeDescription
    
  
  

    MnumberNumber of points in the output window. If zero or less, an empty array is returned.

    
    a_0numberOffset for the generalized cosine window.

      

* * *

## `utils/audio~hertz_to_mel(freq, [mel_scale])` ⇒ T

**Kind**: inner method of [utils/audio](#module_utils/audio)  

  
    
      ParamTypeDefault
    
  
  

    freqT
    
    [mel_scale]string&quot;htk&quot;
      

* * *

## `utils/audio~mel_to_hertz(mels, [mel_scale])` ⇒ T

**Kind**: inner method of [utils/audio](#module_utils/audio)  

  
    
      ParamTypeDefault
    
  
  

    melsT
    
    [mel_scale]string&quot;htk&quot;
      

* * *

## `utils/audio~_create_triangular_filter_bank(fft_freqs, filter_freqs)` ⇒ Array

Creates a triangular filter bank.

Adapted from torchaudio and librosa.

**Kind**: inner method of [utils/audio](#module_utils/audio)  
**Returns**: Array - of shape `(num_frequency_bins, num_mel_filters)`.  

  
    
      ParamTypeDescription
    
  
  

    fft_freqsFloat64ArrayDiscrete frequencies of the FFT bins in Hz, of shape (num_frequency_bins,).

    
    filter_freqsFloat64ArrayCenter frequencies of the triangular filters to create, in Hz, of shape (num_mel_filters,).

      

* * *

## `utils/audio~linspace(start, end, num)` ⇒

Return evenly spaced numbers over a specified interval.

**Kind**: inner method of [utils/audio](#module_utils/audio)  
**Returns**: `num` evenly spaced samples, calculated over the interval `[start, stop]`.  

  
    
      ParamTypeDescription
    
  
  

    startnumberThe starting value of the sequence.

    
    endnumberThe end value of the sequence.

    
    numnumberNumber of samples to generate.

      

* * *

## `utils/audio~padReflect(array, left, right)` ⇒ T

**Kind**: inner method of [utils/audio](#module_utils/audio)  
**Returns**: T - The padded array.  

  
    
      ParamTypeDescription
    
  
  

    arrayTThe array to pad.

    
    leftnumberThe amount of padding to add to the left.

    
    rightnumberThe amount of padding to add to the right.

      

* * *

## `utils/audio~_db_conversion_helper(spectrogram, factor, reference, min_value, db_range)` ⇒ T

Helper function to compute `amplitude_to_db` and `power_to_db`.

**Kind**: inner method of [utils/audio](#module_utils/audio)  

  
    
      ParamType
    
  
  

    spectrogramT
    
    factornumber
    
    referencenumber
    
    min_valuenumber
    
    db_rangenumber
      

* * *

## `utils/audio~amplitude_to_db(spectrogram, [reference], [min_value], [db_range])` ⇒ T

Converts an amplitude spectrogram to the decibel scale. This computes `20 * log10(spectrogram / reference)`,
using basic logarithm properties for numerical stability. NOTE: Operates in-place.

The motivation behind applying the log function on the (mel) spectrogram is that humans do not hear loudness on a
linear scale. Generally to double the perceived volume of a sound we need to put 8 times as much energy into it.
This means that large variations in energy may not sound all that different if the sound is loud to begin with.
This compression operation makes the (mel) spectrogram features match more closely what humans actually hear.

**Kind**: inner method of [utils/audio](#module_utils/audio)  
**Returns**: T - The modified spectrogram in decibels.  

  
    
      ParamTypeDefaultDescription
    
  
  

    spectrogramTThe input amplitude (mel) spectrogram.

    
    [reference]number1.0Sets the input spectrogram value that corresponds to 0 dB.
For example, use np.max(spectrogram) to set the loudest part to 0 dB. Must be greater than zero.

    
    [min_value]number1e-5The spectrogram will be clipped to this minimum value before conversion to decibels,
to avoid taking log(0). The default of 1e-5 corresponds to a minimum of -100 dB. Must be greater than zero.

    
    [db_range]numberSets the maximum dynamic range in decibels. For example, if db_range = 80, the
difference between the peak value and the smallest value will never be more than 80 dB. Must be greater than zero.

      

* * *

## `utils/audio~power_to_db(spectrogram, [reference], [min_value], [db_range])` ⇒ T

Converts a power spectrogram to the decibel scale. This computes `10 * log10(spectrogram / reference)`,
using basic logarithm properties for numerical stability. NOTE: Operates in-place.

The motivation behind applying the log function on the (mel) spectrogram is that humans do not hear loudness on a
linear scale. Generally to double the perceived volume of a sound we need to put 8 times as much energy into it.
This means that large variations in energy may not sound all that different if the sound is loud to begin with.
This compression operation makes the (mel) spectrogram features match more closely what humans actually hear.

Based on the implementation of `librosa.power_to_db`.

**Kind**: inner method of [utils/audio](#module_utils/audio)  
**Returns**: T - The modified spectrogram in decibels.  

  
    
      ParamTypeDefaultDescription
    
  
  

    spectrogramTThe input power (mel) spectrogram. Note that a power spectrogram has the amplitudes squared!

    
    [reference]number1.0Sets the input spectrogram value that corresponds to 0 dB.
For example, use np.max(spectrogram) to set the loudest part to 0 dB. Must be greater than zero.

    
    [min_value]number1e-10The spectrogram will be clipped to this minimum value before conversion to decibels,
to avoid taking log(0). The default of 1e-10 corresponds to a minimum of -100 dB. Must be greater than zero.

    
    [db_range]numberSets the maximum dynamic range in decibels. For example, if db_range = 80, the
difference between the peak value and the smallest value will never be more than 80 dB. Must be greater than zero.

      

* * *

## `utils/audio~encodeWAV(chunks, rate)` ⇒ Blob

Efficiently encode audio data to a WAV file.
WAV file specs : https://en.wikipedia.org/wiki/WAV#WAV_File_header

Adapted from https://www.npmjs.com/package/audiobuffer-to-wav

**Kind**: inner method of [utils/audio](#module_utils/audio)  
**Returns**: Blob - The WAV file as a Blob.  

  
    
      ParamTypeDescription
    
  
  

    chunksArrayThe audio samples.

    
    ratenumberThe sample rate.

      

* * *
