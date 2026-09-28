# backends/onnx

Handler file for choosing the correct version of ONNX Runtime, based on the environment:
  - When running in node, we use `onnxruntime-node` (`onnxruntime-web` is not bundled).
  - When running in the browser, we use `onnxruntime-web` (`onnxruntime-node` is not bundled).

This module is not directly exported, but can be accessed through the environment variables:
```javascript
import { env } from '@huggingface/transformers';
console.log(env.backends.onnx);
```

## Type Definitions

### ONNXExecutionProviders

_Type:_ `InferenceSession.ExecutionProviderConfig`
