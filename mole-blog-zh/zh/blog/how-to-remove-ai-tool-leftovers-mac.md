# 如何安全删除 Ollama 与 LM Studio 模型

> 删除 LM Studio 模型并清理 Ollama 模型存储，同时保留聊天、微调结果、适配器、私有权重和共享数据块。

Published: 2026-07-16 | Updated: 2026-09-30

删除 LM Studio 模型或清理 Ollama 模型存储时，先用各自的模型列表确认对象，不要直接在 Finder 里删数据块。本地 AI 工具很容易占用几十 GB，因为模型权重本身就很大，公开模型通常能重新下载，但下载耗时，特定版本、许可或访问权限也可能变化，清理前先列出模型，并单独保存聊天、微调结果、适配器、提示词和项目数据。

## 在 LM Studio 中删除模型

1. 打开 LM Studio 的 **My Models**，按名称和大小找到目标模型。
2. 使用该模型自己的删除操作，不要去 Finder 里删除名称相近的文件夹。
3. 删除后运行 `lms ls`，确认它已不在 LM Studio 的模型列表中。

LM Studio 允许自行选择模型目录，所以照抄另一台 Mac 的路径并不可靠。如果改过目录，应以当前配置的位置为准，再核对实际释放的空间。

## 移除 Ollama 模型

先列出本机模型，再把准确名称交给 Ollama：

```
ollama ls
ollama rm MODEL_NAME
```

移除后再运行一次 `ollama ls`。这样 Ollama 会自行更新共享的 manifest 和数据块，不会让模型存储落入未知状态。

## 模型通常存在哪里

- **Ollama：** 默认保存在 `~/.ollama/models`，也可能由 `OLLAMA_MODELS` 指向其他目录，用 `ollama ls` 列出模型，用 `ollama rm <model>` 按名称移除，当前[命令行说明](https://docs.ollama.com/cli)记录了这两个命令
- **LM Studio：** 模型目录可由用户选择，通过应用的 My Models 或 [`lms ls`](https://lmstudio.ai/docs/cli/local-models/ls)可以查看模型和大小，再从 LM Studio 中移除，固定猜测某个隐藏路径并不可靠
- **Hugging Face：** 默认缓存位于 `~/.cache/huggingface/hub`，`HF_HOME` 等设置可以改变根目录，`hf cache ls` 用于列出仓库与修订，`hf cache rm model/<repo> --dry-run` 可预览指定模型的移除，`hf cache prune --dry-run` 可预览游离和未完成修订，详见 [Hugging Face 缓存说明](https://huggingface.co/docs/huggingface_hub/en/guides/manage-cache)

实际大小可以这样检查：

```
du -sh ~/.ollama/models ~/.cache/huggingface 2>/dev/null
```

## 模型、缓存和个人数据要分开

公开发布的模型也许能重新下载，本地微调、适配器、转换后的量化文件和私有检查点却可能没有副本。

桌面 AI 应用还可能在 Application Support 中保存对话与附件，或把部分历史同步到账号，因此导出和同步方式也值得一起确认，本地文件未必都可删除，对话也未必只在本机，具体取决于应用的实现。

## 完整卸载 AI 应用

应用包往往只占很小一部分空间，模型和支持文件才是大头，这与[彻底卸载普通应用](https://mole.fit/zh/blog/how-to-completely-uninstall-apps-on-mac)相同。

卸载 Ollama 前可以用 `ollama rm` 移除具名模型，LM Studio 通过应用管理已配置目录，聊天客户端则先导出需要的历史再按厂商说明卸载，Application Support 容器即使名称匹配，也可能混有仍需保留的数据，整目录删除的依据还不够。具体步骤见[在 Mac 上卸载 Ollama](https://mole.fit/zh/tested-apps/ollama)和[在 Mac 上卸载 LM Studio](https://mole.fit/zh/tested-apps/lm-studio)，两篇都先停掉正在运行的模型，再核对默认和自定义的模型位置。

## 为什么要用工具自己的删除命令

Ollama 把模型保存为按 sha256 哈希命名的内容寻址 blob，位于 `~/.ollama/models/blobs`，较小的 manifest 再把模型名和标签关联到所需 blob。

多个模型可以共享同一个 blob，`ollama rm` 会先移除 manifest，只有没有其他引用时才释放 blob，手动删除 blob 则可能破坏仍在使用它的模型。

模型体积不是附带开销，数十亿参数量化后仍需要数 GB，更大或精度更高的模型会占用更多，Hugging Face 也按修订保存不可变快照并对重复内容去重，让所属工具维护引用关系，才能避免损坏模型库或留下无效文件。

<figure class="blog-diagram">
  <img src="https://mole.fit/img/blog/ai-model-store.webp" width="1360" height="454" loading="lazy" alt="两个具名模型 manifest 指向一组按哈希命名的共享 blob，其中一个 blob 同时被两者引用，所以只有不再存在任何引用时，删除模型才会释放它。">
  <figcaption>多个模型可以共享 blob。只有最后一个引用消失时，空间才会释放。</figcaption>
</figure>

## 磁盘地图能看到什么

[Mole](https://mole.fit/zh/) 的「分析」页可以找出意外增长的模型目录，但模型名称、共享 blob 和修订关系仍应交给 Ollama、LM Studio 或 Hugging Face 管理，通用清理工具不该自行删除模型库或聊天记录。命令行 AI 工具卸掉以后留下的配置目录，以及 30 天没用的命令行工具，则可以在清理页远处的月亮，也就是 AI 清理与维护里逐项确认后移除。

## 安全的操作顺序

用所属工具列出模型，区分可下载权重与唯一的本地产物，导出需要保留的内容，停止使用目标模型的推理、训练和下载任务，再一次移除一个具名模型。随后检查磁盘大小，共享 blob 可能让实际回收量小于模型标称大小；只有确认不再保留任何数据，并退出相关应用和后台任务后，才在完整卸载中处理整个支持目录。

---

Canonical HTML page: https://mole.fit/zh/blog/how-to-remove-ai-tool-leftovers-mac
Blog index for agents: https://mole.fit/zh/blog/llms.txt
Site index for agents: https://mole.fit/llms.txt
