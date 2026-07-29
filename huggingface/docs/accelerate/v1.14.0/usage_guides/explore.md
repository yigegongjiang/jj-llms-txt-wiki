# Start Here!

Please use the interactive tool below to help you get started with learning about a particular 
feature of Accelerate and how to utilize it! It will provide you with a code diff, an explanation
towards what is going on, as well as provide you with some useful links to explore more within
the documentation!

Most code examples start from the following python code before integrating Accelerate in some way:

```python
for batch in dataloader:
    optimizer.zero_grad()
    inputs, targets = batch
    inputs = inputs.to(device)
    targets = targets.to(device)
    outputs = model(inputs)
    loss = loss_function(outputs, targets)
    loss.backward()
    optimizer.step()
    scheduler.step()
```

	<iframe 
        src="https://hf-accelerate-accelerate-examples.hf.space?__theme=light"
        width="850"
        height="1600"
    >

    <iframe 
        src="https://hf-accelerate-accelerate-examples.hf.space?__theme=dark"
        width="850"
        height="1600"
    >
