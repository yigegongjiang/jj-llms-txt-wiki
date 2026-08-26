> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência do Agent SDK - Python

> Referência completa da API para o Python Agent SDK, incluindo todas as funções, tipos e classes.

<h2 id="installation">
  Instalação
</h2>

Instale o pacote em um ambiente virtual. Em instalações recentes do Debian, Ubuntu e Homebrew Python, executar `pip install` contra o Python do sistema falha com `error: externally-managed-environment`.

```bash theme={null}
python3 -m venv .venv
source .venv/bin/activate
pip install claude-agent-sdk
```

Para uv, Windows PowerShell e configuração de chave de API, consulte [Comece no visão geral do Agent SDK](/docs/pt/agent-sdk/overview#get-started).

<h2 id="choosing-between-query-and-claudesdkclient">
  Escolhendo entre `query()` e `ClaudeSDKClient`
</h2>

O SDK Python fornece duas maneiras de interagir com Claude Code:

<h3 id="quick-comparison">
  Comparação rápida
</h3>

| Recurso                        | `query()`                                      | `ClaudeSDKClient`                  |
| :----------------------------- | :--------------------------------------------- | :--------------------------------- |
| **Sessão**                     | Cria uma nova sessão por padrão                | Reutiliza a mesma sessão           |
| **Conversa**                   | Troca única                                    | Múltiplas trocas no mesmo contexto |
| **Conexão**                    | Gerenciada automaticamente                     | Controle manual                    |
| **Entrada em Streaming**       | ✅ Suportado                                    | ✅ Suportado                        |
| **Interrupções**               | ❌ Não suportado                                | ✅ Suportado                        |
| **hooks**                      | ✅ Suportado                                    | ✅ Suportado                        |
| **Ferramentas Personalizadas** | ✅ Suportado                                    | ✅ Suportado                        |
| **Continuar Chat**             | Manual via `continue_conversation` ou `resume` | ✅ Automático                       |
| **Caso de Uso**                | Tarefas únicas                                 | Conversas contínuas                |

<h3 id="when-to-use-query-one-off-tasks">
  Quando usar `query()` (tarefas únicas)
</h3>

**Melhor para:**

* Perguntas únicas onde você não precisa do histórico de conversa
* Tarefas independentes que não requerem contexto de trocas anteriores
* Scripts de automação simples
* Quando você quer um novo começo cada vez

<h3 id="when-to-use-claudesdkclient-continuous-conversation">
  Quando usar `ClaudeSDKClient` (conversa contínua)
</h3>

**Melhor para:**

* **Continuando conversas** - Quando você precisa que Claude se lembre do contexto
* **Perguntas de acompanhamento** - Construindo sobre respostas anteriores
* **Aplicações interativas** - Interfaces de chat, REPLs
* **Lógica orientada por resposta** - Quando a próxima ação depende da resposta de Claude
* **Controle de sessão** - Gerenciando o ciclo de vida da conversa explicitamente

<h2 id="functions">
  Funções
</h2>

<h3 id="query">
  `query()`
</h3>

Cria uma nova sessão para cada interação com Claude Code por padrão. Retorna um iterador assíncrono que produz mensagens conforme chegam. Cada chamada para `query()` começa do zero sem memória de interações anteriores, a menos que você passe `continue_conversation=True` ou `resume` em [`ClaudeAgentOptions`](#claudeagentoptions). Veja [Sessions](/docs/pt/agent-sdk/sessions).

```python theme={null}
async def query(
    *,
    prompt: str | AsyncIterable[dict[str, Any]],
    options: ClaudeAgentOptions | None = None,
    transport: Transport | None = None
) -> AsyncIterator[Message]
```

<h4 id="parameters">
  Parâmetros
</h4>

| Parâmetro   | Tipo                         | Descrição                                                                         |
| :---------- | :--------------------------- | :-------------------------------------------------------------------------------- |
| `prompt`    | `str \| AsyncIterable[dict]` | O prompt de entrada como uma string ou iterável assíncrono para modo de streaming |
| `options`   | `ClaudeAgentOptions \| None` | Objeto de configuração opcional (padrão para `ClaudeAgentOptions()` se None)      |
| `transport` | `Transport \| None`          | Transport personalizado opcional para comunicação com o processo CLI              |

<h4 id="returns">
  Retorna
</h4>

Retorna um `AsyncIterator[Message]` que produz mensagens da conversa.

<h4 id="example-with-options">
  Exemplo - Com opções
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    options = ClaudeAgentOptions(
        system_prompt="You are an expert Python developer",
        permission_mode="acceptEdits",
        cwd="/home/user/project",
    )

    async for message in query(prompt="Create a Python web server", options=options):
        print(message)


asyncio.run(main())
```

<h3 id="tool">
  `tool()`
</h3>

Decorador para definir ferramentas MCP com segurança de tipo.

```python theme={null}
def tool(
    name: str,
    description: str,
    input_schema: type | dict[str, Any],
    annotations: ToolAnnotations | None = None
) -> Callable[[Callable[[Any], Awaitable[dict[str, Any]]]], SdkMcpTool[Any]]
```

<h4 id="parameters-2">
  Parâmetros
</h4>

| Parâmetro      | Tipo                                            | Descrição                                                              |
| :------------- | :---------------------------------------------- | :--------------------------------------------------------------------- |
| `name`         | `str`                                           | Identificador único para a ferramenta                                  |
| `description`  | `str`                                           | Descrição legível por humanos do que a ferramenta faz                  |
| `input_schema` | `type \| dict[str, Any]`                        | Schema definindo os parâmetros de entrada da ferramenta (veja abaixo)  |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Anotações MCP opcionais fornecendo dicas de comportamento aos clientes |

<h4 id="input-schema-options">
  Opções de schema de entrada
</h4>

1. **Mapeamento de tipo simples** (recomendado):

   ```python theme={null}
   {"text": str, "count": int, "enabled": bool}
   ```

2. **Formato JSON Schema** (para validação complexa):
   ```python theme={null}
   {
       "type": "object",
       "properties": {
           "text": {"type": "string"},
           "count": {"type": "integer", "minimum": 0},
       },
       "required": ["text"],
   }
   ```

<h4 id="returns-2">
  Retorna
</h4>

Uma função decoradora que envolve a implementação da ferramenta e retorna uma instância `SdkMcpTool`.

<h4 id="example">
  Exemplo
</h4>

```python theme={null}
from claude_agent_sdk import tool
from typing import Any


@tool("greet", "Greet a user", {"name": str})
async def greet(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Hello, {args['name']}!"}]}
```

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Re-exportado de `mcp.types` (também disponível como `from claude_agent_sdk import ToolAnnotations`). Todos os campos são dicas opcionais; clientes não devem confiar neles para decisões de segurança.

| Campo             | Tipo           | Padrão  | Descrição                                                                                                                                                                   |
| :---------------- | :------------- | :------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `str \| None`  | `None`  | Título legível por humanos para a ferramenta                                                                                                                                |
| `readOnlyHint`    | `bool \| None` | `False` | Se `True`, a ferramenta não modifica seu ambiente                                                                                                                           |
| `destructiveHint` | `bool \| None` | `True`  | Se `True`, a ferramenta pode realizar atualizações destrutivas (apenas significativo quando `readOnlyHint` é `False`)                                                       |
| `idempotentHint`  | `bool \| None` | `False` | Se `True`, chamadas repetidas com os mesmos argumentos não têm efeito adicional (apenas significativo quando `readOnlyHint` é `False`)                                      |
| `openWorldHint`   | `bool \| None` | `True`  | Se `True`, a ferramenta interage com entidades externas (por exemplo, busca na web). Se `False`, o domínio da ferramenta é fechado (por exemplo, uma ferramenta de memória) |

```python theme={null}
from claude_agent_sdk import tool, ToolAnnotations
from typing import Any


@tool(
    "search",
    "Search the web",
    {"query": str},
    annotations=ToolAnnotations(readOnlyHint=True, openWorldHint=True),
)
async def search(args: dict[str, Any]) -> dict[str, Any]:
    return {"content": [{"type": "text", "text": f"Results for: {args['query']}"}]}
```

<h3 id="create_sdk_mcp_server">
  `create_sdk_mcp_server()`
</h3>

Cria um servidor MCP em processo que é executado dentro de sua aplicação Python.

```python theme={null}
def create_sdk_mcp_server(
    name: str,
    version: str = "1.0.0",
    tools: list[SdkMcpTool[Any]] | None = None
) -> McpSdkServerConfig
```

<h4 id="parameters-3">
  Parâmetros
</h4>

| Parâmetro | Tipo                            | Padrão    | Descrição                                                    |
| :-------- | :------------------------------ | :-------- | :----------------------------------------------------------- |
| `name`    | `str`                           | -         | Identificador único para o servidor                          |
| `version` | `str`                           | `"1.0.0"` | String de versão do servidor                                 |
| `tools`   | `list[SdkMcpTool[Any]] \| None` | `None`    | Lista de funções de ferramenta criadas com decorador `@tool` |

<h4 id="returns-3">
  Retorna
</h4>

Retorna um objeto `McpSdkServerConfig` que pode ser passado para `ClaudeAgentOptions.mcp_servers`.

<h4 id="example-2">
  Exemplo
</h4>

```python theme={null}
from claude_agent_sdk import tool, create_sdk_mcp_server


@tool("add", "Add two numbers", {"a": float, "b": float})
async def add(args):
    return {"content": [{"type": "text", "text": f"Sum: {args['a'] + args['b']}"}]}


@tool("multiply", "Multiply two numbers", {"a": float, "b": float})
async def multiply(args):
    return {"content": [{"type": "text", "text": f"Product: {args['a'] * args['b']}"}]}


calculator = create_sdk_mcp_server(
    name="calculator",
    version="2.0.0",
    tools=[add, multiply],  # Pass decorated functions
)

# Use with Claude
options = ClaudeAgentOptions(
    mcp_servers={"calc": calculator},
    allowed_tools=["mcp__calc__add", "mcp__calc__multiply"],
)
```

<h3 id="list_sessions">
  `list_sessions()`
</h3>

Lista sessões passadas com metadados. Filtre por diretório de projeto ou liste sessões em todos os projetos. Síncrono; retorna imediatamente.

```python theme={null}
def list_sessions(
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0,
    include_worktrees: bool = True
) -> list[SDKSessionInfo]
```

<h4 id="parameters-4">
  Parâmetros
</h4>

| Parâmetro           | Tipo          | Padrão | Descrição                                                                                             |
| :------------------ | :------------ | :----- | :---------------------------------------------------------------------------------------------------- |
| `directory`         | `str \| None` | `None` | Diretório para listar sessões. Quando omitido, retorna sessões em todos os projetos                   |
| `limit`             | `int \| None` | `None` | Número máximo de sessões a retornar                                                                   |
| `offset`            | `int`         | `0`    | Número de sessões a pular do início dos resultados classificados. Use com `limit` para paginação      |
| `include_worktrees` | `bool`        | `True` | Quando `directory` está dentro de um repositório git, inclua sessões de todos os caminhos de worktree |

<h4 id="return-type-sdksessioninfo">
  Tipo de retorno: `SDKSessionInfo`
</h4>

| Propriedade     | Tipo          | Descrição                                                                                  |
| :-------------- | :------------ | :----------------------------------------------------------------------------------------- |
| `session_id`    | `str`         | Identificador único de sessão                                                              |
| `summary`       | `str`         | Título de exibição: título personalizado, resumo gerado automaticamente ou primeiro prompt |
| `last_modified` | `int`         | Hora da última modificação em milissegundos desde a época                                  |
| `file_size`     | `int \| None` | Tamanho do arquivo de sessão em bytes (`None` para backends de armazenamento remoto)       |
| `custom_title`  | `str \| None` | Título de sessão definido pelo usuário                                                     |
| `first_prompt`  | `str \| None` | Primeiro prompt de usuário significativo na sessão                                         |
| `git_branch`    | `str \| None` | Branch Git no final da sessão                                                              |
| `cwd`           | `str \| None` | Diretório de trabalho para a sessão                                                        |
| `tag`           | `str \| None` | Tag de sessão definida pelo usuário (veja [`tag_session()`](#tag_session))                 |
| `created_at`    | `int \| None` | Hora de criação da sessão em milissegundos desde a época                                   |

<h4 id="example-3">
  Exemplo
</h4>

Imprima as 10 sessões mais recentes para um projeto. Os resultados são classificados por `last_modified` descendente, então o primeiro item é o mais novo. Omita `directory` para pesquisar em todos os projetos.

```python theme={null}
from claude_agent_sdk import list_sessions

for session in list_sessions(directory="/path/to/project", limit=10):
    print(f"{session.summary} ({session.session_id})")
```

<h3 id="get_session_messages">
  `get_session_messages()`
</h3>

Recupera mensagens de uma sessão passada. Síncrono; retorna imediatamente.

```python theme={null}
def get_session_messages(
    session_id: str,
    directory: str | None = None,
    limit: int | None = None,
    offset: int = 0
) -> list[SessionMessage]
```

<h4 id="parameters-5">
  Parâmetros
</h4>

| Parâmetro    | Tipo          | Padrão      | Descrição                                                                      |
| :----------- | :------------ | :---------- | :----------------------------------------------------------------------------- |
| `session_id` | `str`         | obrigatório | O ID da sessão para recuperar mensagens                                        |
| `directory`  | `str \| None` | `None`      | Diretório do projeto para procurar. Quando omitido, pesquisa todos os projetos |
| `limit`      | `int \| None` | `None`      | Número máximo de mensagens a retornar                                          |
| `offset`     | `int`         | `0`         | Número de mensagens a pular do início                                          |

<h4 id="return-type-sessionmessage">
  Tipo de retorno: `SessionMessage`
</h4>

| Propriedade          | Tipo                           | Descrição                       |
| :------------------- | :----------------------------- | :------------------------------ |
| `type`               | `Literal["user", "assistant"]` | Papel da mensagem               |
| `uuid`               | `str`                          | Identificador único de mensagem |
| `session_id`         | `str`                          | Identificador de sessão         |
| `message`            | `Any`                          | Conteúdo bruto da mensagem      |
| `parent_tool_use_id` | `None`                         | Reservado para uso futuro       |

<h4 id="example-4">
  Exemplo
</h4>

```python theme={null}
from claude_agent_sdk import list_sessions, get_session_messages

sessions = list_sessions(limit=1)
if sessions:
    messages = get_session_messages(sessions[0].session_id)
    for msg in messages:
        print(f"[{msg.type}] {msg.uuid}")
```

<h3 id="get_session_info">
  `get_session_info()`
</h3>

Lê metadados para uma única sessão por ID sem verificar o diretório do projeto completo. Síncrono; retorna imediatamente.

```python theme={null}
def get_session_info(
    session_id: str,
    directory: str | None = None,
) -> SDKSessionInfo | None
```

<h4 id="parameters-6">
  Parâmetros
</h4>

| Parâmetro    | Tipo          | Padrão      | Descrição                                                                                |
| :----------- | :------------ | :---------- | :--------------------------------------------------------------------------------------- |
| `session_id` | `str`         | obrigatório | UUID da sessão a procurar                                                                |
| `directory`  | `str \| None` | `None`      | Caminho do diretório do projeto. Quando omitido, pesquisa todos os diretórios de projeto |

Retorna [`SDKSessionInfo`](#return-type-sdksessioninfo), ou `None` se a sessão não for encontrada.

<h4 id="example-5">
  Exemplo
</h4>

Procure os metadados de uma única sessão sem verificar o diretório do projeto. Útil quando você já tem um ID de sessão de uma execução anterior.

```python theme={null}
from claude_agent_sdk import get_session_info

info = get_session_info("550e8400-e29b-41d4-a716-446655440000")
if info:
    print(f"{info.summary} (branch: {info.git_branch}, tag: {info.tag})")
```

<h3 id="rename_session">
  `rename_session()`
</h3>

Renomeia uma sessão anexando uma entrada de título personalizado. Chamadas repetidas são seguras; o título mais recente vence. Síncrono.

```python theme={null}
def rename_session(
    session_id: str,
    title: str,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-7">
  Parâmetros
</h4>

| Parâmetro    | Tipo          | Padrão      | Descrição                                                                                |
| :----------- | :------------ | :---------- | :--------------------------------------------------------------------------------------- |
| `session_id` | `str`         | obrigatório | UUID da sessão a renomear                                                                |
| `title`      | `str`         | obrigatório | Novo título. Deve ser não vazio após remover espaços em branco                           |
| `directory`  | `str \| None` | `None`      | Caminho do diretório do projeto. Quando omitido, pesquisa todos os diretórios de projeto |

Lança `ValueError` se `session_id` não for um UUID válido ou `title` estiver vazio; `FileNotFoundError` se a sessão não puder ser encontrada.

<h4 id="example-6">
  Exemplo
</h4>

Renomeie a sessão mais recente para que seja mais fácil encontrá-la depois. O novo título aparece em [`SDKSessionInfo.custom_title`](#return-type-sdksessioninfo) em leituras subsequentes.

```python theme={null}
from claude_agent_sdk import list_sessions, rename_session

sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    rename_session(sessions[0].session_id, "Refactor auth module")
```

<h3 id="tag_session">
  `tag_session()`
</h3>

Marca uma sessão. Passe `None` para limpar a tag. Chamadas repetidas são seguras; a tag mais recente vence. Síncrono.

```python theme={null}
def tag_session(
    session_id: str,
    tag: str | None,
    directory: str | None = None,
) -> None
```

<h4 id="parameters-8">
  Parâmetros
</h4>

| Parâmetro    | Tipo          | Padrão      | Descrição                                                                                |
| :----------- | :------------ | :---------- | :--------------------------------------------------------------------------------------- |
| `session_id` | `str`         | obrigatório | UUID da sessão a marcar                                                                  |
| `tag`        | `str \| None` | obrigatório | String de tag, ou `None` para limpar. Unicode-sanitizado antes de armazenar              |
| `directory`  | `str \| None` | `None`      | Caminho do diretório do projeto. Quando omitido, pesquisa todos os diretórios de projeto |

Lança `ValueError` se `session_id` não for um UUID válido ou `tag` estiver vazio após sanitização; `FileNotFoundError` se a sessão não puder ser encontrada.

<h4 id="example-7">
  Exemplo
</h4>

Marque uma sessão e depois filtre por essa tag em uma leitura posterior. Passe `None` para limpar uma tag existente.

```python theme={null}
from claude_agent_sdk import list_sessions, tag_session

# Tag a session
tag_session("550e8400-e29b-41d4-a716-446655440000", "needs-review")

# Later: find all sessions with that tag
for session in list_sessions(directory="/path/to/project"):
    if session.tag == "needs-review":
        print(session.summary)
```

<h2 id="classes">
  Classes
</h2>

<h3 id="claudesdkclient">
  `ClaudeSDKClient`
</h3>

**Mantém uma sessão de conversa em múltiplas trocas.** Este é o equivalente Python de como a função `query()` do SDK TypeScript funciona internamente - cria um objeto cliente que pode continuar conversas.

<h4 id="key-features">
  Recursos principais
</h4>

* **Continuidade de sessão**: Mantém contexto de conversa em múltiplas chamadas `query()`
* **Mesma conversa**: A sessão retém mensagens anteriores
* **Suporte a interrupção**: Pode parar a execução no meio da tarefa
* **Ciclo de vida explícito**: Você controla quando a sessão começa e termina
* **Fluxo orientado por resposta**: Pode reagir a respostas e enviar acompanhamentos
* **Ferramentas e hooks personalizados**: Suporta ferramentas personalizadas (criadas com decorador `@tool`) e hooks

```python theme={null}
class ClaudeSDKClient:
    def __init__(self, options: ClaudeAgentOptions | None = None, transport: Transport | None = None)
    async def connect(self, prompt: str | AsyncIterable[dict] | None = None) -> None
    async def query(self, prompt: str | AsyncIterable[dict], session_id: str = "default") -> None
    async def receive_messages(self) -> AsyncIterator[Message]
    async def receive_response(self) -> AsyncIterator[Message]
    async def interrupt(self) -> None
    async def set_permission_mode(self, mode: str) -> None
    async def set_model(self, model: str | None = None) -> None
    async def rewind_files(self, user_message_id: str) -> None
    async def get_mcp_status(self) -> McpStatusResponse
    async def reconnect_mcp_server(self, server_name: str) -> None
    async def toggle_mcp_server(self, server_name: str, enabled: bool) -> None
    async def stop_task(self, task_id: str) -> None
    async def get_server_info(self) -> dict[str, Any] | None
    async def disconnect(self) -> None
```

<h4 id="methods">
  Métodos
</h4>

| Método                                    | Descrição                                                                                                                                                                   |
| :---------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `__init__(options)`                       | Inicializa o cliente com configuração opcional                                                                                                                              |
| `connect(prompt)`                         | Conecta a Claude com um prompt inicial opcional ou fluxo de mensagem                                                                                                        |
| `query(prompt, session_id)`               | Envia uma nova solicitação em modo de streaming                                                                                                                             |
| `receive_messages()`                      | Recebe todas as mensagens de Claude como um iterador assíncrono                                                                                                             |
| `receive_response()`                      | Recebe mensagens até e incluindo uma ResultMessage                                                                                                                          |
| `interrupt()`                             | Envia sinal de interrupção (funciona apenas em modo de streaming)                                                                                                           |
| `set_permission_mode(mode)`               | Altera o modo de permissão para a sessão atual                                                                                                                              |
| `set_model(model)`                        | Altera o modelo para a sessão atual. Passe `None` para redefinir para padrão                                                                                                |
| `rewind_files(user_message_id)`           | Restaura arquivos para seu estado na mensagem de usuário especificada. Requer `enable_file_checkpointing=True`. Veja [File checkpointing](/docs/pt/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Obtém o status de todos os servidores MCP configurados. Retorna [`McpStatusResponse`](#mcpstatusresponse)                                                                   |
| `reconnect_mcp_server(server_name)`       | Tenta reconectar a um servidor MCP que falhou ou foi desconectado                                                                                                           |
| `toggle_mcp_server(server_name, enabled)` | Ativa ou desativa um servidor MCP no meio da sessão. Desativar remove suas ferramentas                                                                                      |
| `stop_task(task_id)`                      | Para uma tarefa de fundo em execução. Uma [`TaskNotificationMessage`](#tasknotificationmessage) com status `"stopped"` segue no fluxo de mensagens                          |
| `get_server_info()`                       | Obtém informações do servidor incluindo ID de sessão e capacidades                                                                                                          |
| `disconnect()`                            | Desconecta de Claude                                                                                                                                                        |

<h4 id="context-manager-support">
  Suporte a Gerenciador de Contexto
</h4>

O cliente pode ser usado como um gerenciador de contexto assíncrono para gerenciamento automático de conexão:

```python theme={null}
async with ClaudeSDKClient() as client:
    await client.query("Hello Claude")
    async for message in client.receive_response():
        print(message)
```

> **Importante:** Ao iterar sobre mensagens, evite usar `break` para sair cedo, pois isso pode causar problemas de limpeza do asyncio. Em vez disso, deixe a iteração ser concluída naturalmente ou use sinalizadores para rastrear quando você encontrou o que precisa.

<h4 id="example-continuing-a-conversation">
  Exemplo - Continuando uma conversa
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, AssistantMessage, TextBlock, ResultMessage


async def main():
    async with ClaudeSDKClient() as client:
        # First question
        await client.query("What's the capital of France?")

        # Process response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Follow-up question - the session retains the previous context
        await client.query("What's the population of that city?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")

        # Another follow-up - still in the same conversation
        await client.query("What are some famous landmarks there?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Claude: {block.text}")


asyncio.run(main())
```

<h4 id="example-streaming-input-with-claudesdkclient">
  Exemplo - Entrada em streaming com ClaudeSDKClient
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient


async def message_stream():
    """Generate messages dynamically."""
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Analyze the following data:"},
    }
    await asyncio.sleep(0.5)
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Temperature: 25°C, Humidity: 60%"},
    }
    await asyncio.sleep(0.5)
    yield {
        "type": "user",
        "message": {"role": "user", "content": "What patterns do you see?"},
    }


async def main():
    async with ClaudeSDKClient() as client:
        # Stream input to Claude
        await client.query(message_stream())

        # Process response
        async for message in client.receive_response():
            print(message)

        # Follow-up in same session
        await client.query("Should we be concerned about these readings?")

        async for message in client.receive_response():
            print(message)


asyncio.run(main())
```

<h4 id="example-using-interrupts">
  Exemplo - Usando interrupções
</h4>

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions, ResultMessage


async def interruptible_task():
    options = ClaudeAgentOptions(allowed_tools=["Bash"], permission_mode="acceptEdits")

    async with ClaudeSDKClient(options=options) as client:
        # Start a long-running task
        await client.query("Count from 1 to 100 slowly, using the bash sleep command")

        # Let it run for a bit
        await asyncio.sleep(2)

        # Interrupt the task
        await client.interrupt()
        print("Task interrupted!")

        # Drain the interrupted task's messages (including its ResultMessage)
        async for message in client.receive_response():
            if isinstance(message, ResultMessage):
                print(f"Interrupted task finished with subtype={message.subtype!r}")
                # subtype is "error_during_execution" for interrupted tasks

        # Send a new command
        await client.query("Just say hello instead")

        # Now receive the new response
        async for message in client.receive_response():
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(f"New result: {message.result}")


asyncio.run(interruptible_task())
```

<Note>
  **Comportamento do buffer após interrupção:** `interrupt()` envia um sinal de parada mas não limpa o buffer de mensagens. Mensagens já produzidas pela tarefa interrompida, incluindo sua `ResultMessage` (com `subtype="error_during_execution"`), permanecem no fluxo. Você deve drená-las com `receive_response()` antes de ler a resposta a uma nova consulta. Se você enviar uma nova consulta imediatamente após `interrupt()` e chamar `receive_response()` apenas uma vez, você receberá as mensagens da tarefa interrompida, não a resposta da nova consulta.
</Note>

<h4 id="example-advanced-permission-control">
  Exemplo - Controle avançado de permissão
</h4>

```python theme={null}
from claude_agent_sdk import ClaudeSDKClient, ClaudeAgentOptions
from claude_agent_sdk.types import (
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def custom_permission_handler(
    tool_name: str, input_data: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    """Custom logic for tool permissions."""

    # Block writes to system directories
    if tool_name == "Write" and input_data.get("file_path", "").startswith("/system/"):
        return PermissionResultDeny(
            message="System directory write not allowed", interrupt=True
        )

    # Redirect sensitive file operations
    if tool_name in ["Write", "Edit"] and "config" in input_data.get("file_path", ""):
        safe_path = f"./sandbox/{input_data['file_path']}"
        return PermissionResultAllow(
            updated_input={**input_data, "file_path": safe_path}
        )

    # Allow everything else
    return PermissionResultAllow(updated_input=input_data)


async def main():
    # Não liste também as ferramentas controladas em allowed_tools: as regras de permissão aprovam chamadas antes que can_use_tool seja executado
    options = ClaudeAgentOptions(can_use_tool=custom_permission_handler)

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Update the system config file")

        async for message in client.receive_response():
            # Will use sandbox path instead
            print(message)


asyncio.run(main())
```

<h2 id="types">
  Tipos
</h2>

<Note>
  **`@dataclass` vs `TypedDict`:** Este SDK usa dois tipos de tipos. Classes decoradas com `@dataclass` (como `ResultMessage`, `AgentDefinition`, `TextBlock`) são instâncias de objeto em tempo de execução e suportam acesso a atributos: `msg.result`. Classes definidas com `TypedDict` (como `ThinkingConfigEnabled`, `McpStdioServerConfig`, `SyncHookJSONOutput`) são **dicts simples em tempo de execução** e requerem acesso a chave: `config["budget_tokens"]`, não `config.budget_tokens`. A sintaxe de chamada `ClassName(field=value)` funciona para ambos, mas apenas dataclasses produzem objetos com atributos.
</Note>

<h3 id="sdkmcptool">
  `SdkMcpTool`
</h3>

Definição para uma ferramenta SDK MCP criada com o decorador `@tool`.

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| Propriedade    | Tipo                                       | Descrição                                                                                                 |
| :------------- | :----------------------------------------- | :-------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                      | Identificador único para a ferramenta                                                                     |
| `description`  | `str`                                      | Descrição legível por humanos                                                                             |
| `input_schema` | `type[T] \| dict[str, Any]`                | Schema para validação de entrada                                                                          |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]` | Função assíncrona que manipula a execução da ferramenta                                                   |
| `annotations`  | `ToolAnnotations \| None`                  | Anotações MCP opcionais (por exemplo, `readOnlyHint`, `destructiveHint`, `openWorldHint`). De `mcp.types` |

<h3 id="transport">
  `Transport`
</h3>

Classe base abstrata para implementações de transport personalizado. Use isso para comunicar com o processo Claude sobre um canal personalizado (por exemplo, uma conexão remota em vez de um subprocess local).

<Warning>
  Esta é uma API interna de baixo nível. A interface pode mudar em versões futuras. Implementações personalizadas devem ser atualizadas para corresponder a qualquer mudança de interface.
</Warning>

```python theme={null}
from abc import ABC, abstractmethod
from collections.abc import AsyncIterator
from typing import Any


class Transport(ABC):
    @abstractmethod
    async def connect(self) -> None: ...

    @abstractmethod
    async def write(self, data: str) -> None: ...

    @abstractmethod
    def read_messages(self) -> AsyncIterator[dict[str, Any]]: ...

    @abstractmethod
    async def close(self) -> None: ...

    @abstractmethod
    def is_ready(self) -> bool: ...

    @abstractmethod
    async def end_input(self) -> None: ...
```

| Método            | Descrição                                                                          |
| :---------------- | :--------------------------------------------------------------------------------- |
| `connect()`       | Conecta o transport e prepara para comunicação                                     |
| `write(data)`     | Escreve dados brutos (JSON + nova linha) para o transport                          |
| `read_messages()` | Iterador assíncrono que produz mensagens JSON analisadas                           |
| `close()`         | Fecha a conexão e limpa recursos                                                   |
| `is_ready()`      | Retorna `True` se o transport pode enviar e receber                                |
| `end_input()`     | Fecha o fluxo de entrada (por exemplo, fechar stdin para transports de subprocess) |

Importação: `from claude_agent_sdk import Transport`

<h3 id="claudeagentoptions">
  `ClaudeAgentOptions`
</h3>

Dataclass de configuração para consultas Claude Code.

```python theme={null}
@dataclass
class ClaudeAgentOptions:
    tools: list[str] | ToolsPreset | None = None
    allowed_tools: list[str] = field(default_factory=list)
    system_prompt: str | SystemPromptPreset | SystemPromptFile | None = None
    mcp_servers: dict[str, McpServerConfig] | str | Path = field(default_factory=dict)
    strict_mcp_config: bool = False
    permission_mode: PermissionMode | None = None
    continue_conversation: bool = False
    resume: str | None = None
    max_turns: int | None = None
    max_budget_usd: float | None = None
    disallowed_tools: list[str] = field(default_factory=list)
    model: str | None = None
    fallback_model: str | None = None
    betas: list[SdkBeta] = field(default_factory=list)
    output_format: dict[str, Any] | None = None
    permission_prompt_tool_name: str | None = None
    cwd: str | Path | None = None
    cli_path: str | Path | None = None
    settings: str | None = None
    add_dirs: list[str | Path] = field(default_factory=list)
    env: dict[str, str] = field(default_factory=dict)
    extra_args: dict[str, str | None] = field(default_factory=dict)
    max_buffer_size: int | None = None
    debug_stderr: Any = sys.stderr  # Deprecated
    stderr: Callable[[str], None] | None = None
    can_use_tool: CanUseTool | None = None
    hooks: dict[HookEvent, list[HookMatcher]] | None = None
    user: str | None = None
    include_partial_messages: bool = False
    include_hook_events: bool = False
    fork_session: bool = False
    agents: dict[str, AgentDefinition] | None = None
    setting_sources: list[SettingSource] | None = None
    skills: list[str] | Literal["all"] | None = None
    sandbox: SandboxSettings | None = None
    plugins: list[SdkPluginConfig] = field(default_factory=list)
    max_thinking_tokens: int | None = None  # Deprecated: use thinking instead
    thinking: ThinkingConfig | None = None
    effort: EffortLevel | None = None
    enable_file_checkpointing: bool = False
    session_store: SessionStore | None = None
    session_store_flush: SessionStoreFlushMode = "batched"
```

| Propriedade                   | Tipo                                                                                  | Padrão                             | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| :---------------------------- | :------------------------------------------------------------------------------------ | :--------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                             | Configuração de ferramentas. Use `{"type": "preset", "preset": "claude_code"}` para as ferramentas padrão do Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                               | Ferramentas para auto-aprovar sem solicitar. Isso não restringe Claude apenas a essas ferramentas; ferramentas não listadas caem através de `permission_mode` e `can_use_tool`. Use `disallowed_tools` para bloquear ferramentas. Veja [Permissions](/docs/pt/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                     |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptFile \| None`                               | `None`                             | Configuração de prompt do sistema. Passe uma string para um prompt personalizado, `{"type": "preset", "preset": "claude_code"}` para o prompt do sistema do Claude Code com `"append"` opcional, ou `{"type": "file", "path": "..."}` para carregar um prompt grande do disco. Veja [`SystemPromptPreset`](#systempromptpreset) e [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                                                                                                |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                               | Configurações de servidor MCP ou caminho para arquivo de configuração                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `strict_mcp_config`           | `bool`                                                                                | `False`                            | Quando `True`, use apenas os servidores passados em `mcp_servers` e ignore o projeto `.mcp.json`, configurações do usuário, servidores MCP fornecidos por plugins e [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai). Mapeia para o sinalizador CLI `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                                  |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                             | Modo de permissão para uso de ferramentas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `continue_conversation`       | `bool`                                                                                | `False`                            | Continua a conversa mais recente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `resume`                      | `str \| None`                                                                         | `None`                             | ID de sessão para retomar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `max_turns`                   | `int \| None`                                                                         | `None`                             | Número máximo de turnos agênticos (rodadas de uso de ferramenta)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                             | Para a consulta quando a estimativa de custo do lado do cliente atinge este valor em USD. Comparado com a mesma estimativa que `total_cost_usd`; veja [Track cost and usage](/docs/pt/agent-sdk/cost-tracking) para ressalvas de precisão                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                               | Ferramentas para negar. Um nome simples como `"Bash"` remove a ferramenta do contexto do Claude. Uma regra com escopo como `"Bash(rm *)"` deixa a ferramenta disponível e nega chamadas correspondentes em todos os modos de permissão, incluindo `bypassPermissions`. Veja [Permissions](/docs/pt/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                            | Ativa rastreamento de mudança de arquivo para retrocesso. Veja [File checkpointing](/docs/pt/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `model`                       | `str \| None`                                                                         | `None`                             | Alias de modelo Claude ou nome de modelo completo. Veja [valores aceitos e IDs específicos do provedor](/docs/pt/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `fallback_model`              | `str \| None`                                                                         | `None`                             | Modelo de fallback a usar se o modelo primário falhar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                               | Recursos beta para ativar. Veja [`SdkBeta`](#sdkbeta) para opções disponíveis                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                             | Formato de saída para respostas estruturadas (por exemplo, `{"type": "json_schema", "schema": {...}}`). Veja [Structured outputs](/docs/pt/agent-sdk/structured-outputs) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                             | Nome da ferramenta MCP para prompts de permissão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                             | Diretório de trabalho atual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                             | Caminho personalizado para o executável CLI do Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `settings`                    | `str \| None`                                                                         | `None`                             | Caminho para arquivo de configurações                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                               | Diretórios adicionais que Claude pode acessar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `env`                         | `dict[str, str]`                                                                      | `{}`                               | Variáveis de ambiente mescladas no topo do ambiente de processo herdado. Veja [Environment variables](/docs/pt/env-vars) para variáveis que o CLI subjacente lê, e [Handle slow or stalled API responses](#handle-slow-or-stalled-api-responses) para variáveis relacionadas a timeout                                                                                                                                                                                                                                                                                                                                                                                        |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                               | Argumentos CLI adicionais a passar diretamente para o CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                             | Bytes máximos ao fazer buffer da saída padrão do CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                       | *Deprecated* - Objeto semelhante a arquivo para saída de depuração. Use callback `stderr` em vez disso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                             | Função de callback para saída stderr do CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                             | Função de callback de permissão de ferramenta, invocada apenas quando o [fluxo de permissão](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) cai através de um prompt. Não invocada para chamadas auto-aprovadas por `allowed_tools`, regras de permissão ou `permission_mode`. `AskUserQuestion`, ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools), e ferramentas MCP marcadas [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool) a alcançam mesmo se você as permitiu; em modo `dontAsk` essas são negadas em vez disso. Veja [`CanUseTool`](#canusetool) para detalhes |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                             | Configurações de hook para interceptar eventos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `user`                        | `str \| None`                                                                         | `None`                             | Identificador de usuário                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `include_partial_messages`    | `bool`                                                                                | `False`                            | Inclua eventos de streaming de mensagem parcial. Quando ativado, mensagens [`StreamEvent`](#streamevent) são produzidas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `include_hook_events`         | `bool`                                                                                | `False`                            | Inclua eventos de ciclo de vida de hook no fluxo de mensagens como objetos `HookEventMessage`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `fork_session`                | `bool`                                                                                | `False`                            | Ao retomar com `resume`, bifurque para um novo ID de sessão em vez de continuar a sessão original                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                             | Subagentes definidos programaticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                               | Carregue plugins personalizados de caminhos locais. Veja [Plugins](/docs/pt/agent-sdk/plugins) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                             | Configure o comportamento do sandbox programaticamente. Veja [Sandbox settings](#sandboxsettings) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (CLI defaults: all sources) | Controle quais configurações do sistema de arquivos carregar. Passe `[]` para desabilitar configurações de usuário, projeto e local. Configurações de política gerenciada carregam independentemente; configurações gerenciadas pelo servidor são buscadas quando a sessão se autentica com uma credencial de organização em uma [configuração elegível](/docs/pt/server-managed-settings#platform-availability). Veja [Use Claude Code features](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                    |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                             | Skills disponíveis para a sessão. Passe `"all"` para ativar cada skill descoberto, ou uma lista de nomes de skills. Quando definido, o SDK adiciona a ferramenta Skill a `allowed_tools` automaticamente. Se você também passar `tools`, inclua `"Skill"` nessa lista. Veja [Skills](/docs/pt/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                               |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                             | *Deprecated* - Tokens máximos para blocos de pensamento. Use `thinking` em vez disso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                             | Controla o comportamento de pensamento estendido. Tem precedência sobre `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                             | Nível de esforço para profundidade de pensamento. Veja [ajustar o nível de esforço](/docs/pt/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `session_store`               | [`SessionStore`](/docs/pt/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                             | Espelhe transcrições de sessão para um backend externo para que qualquer host possa retomá-las. Veja [Persist sessions to external storage](/docs/pt/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                        | Quando fazer flush das entradas de transcrição espelhadas para `session_store`. `"batched"` faz flush uma vez por turno ou quando o buffer enche; `"eager"` dispara um flush de fundo após cada frame. Ignorado quando `session_store` é `None`                                                                                                                                                                                                                                                                                                                                                                                                                          |

<h4 id="handle-slow-or-stalled-api-responses">
  Lidar com respostas de API lentas ou travadas
</h4>

O subprocess CLI lê várias variáveis de ambiente que controlam timeouts de API e detecção de travamento. Passe-as através de `ClaudeAgentOptions.env`:

```python theme={null}
options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS`: timeout por solicitação no cliente Anthropic, em milissegundos. Padrão `600000`. Aplica-se ao loop principal e a todos os subagentes.
* `CLAUDE_CODE_MAX_RETRIES`: máximo de tentativas de API. Padrão `10`, limitado a `15`. Cada tentativa obtém sua própria janela `API_TIMEOUT_MS`, então o tempo de parede no pior caso é aproximadamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` mais backoff. Para execuções autônomas que precisam aguardar interrupções mais longas, defina `CLAUDE_CODE_RETRY_WATCHDOG=1`: ele tenta erros de capacidade indefinidamente, e a partir do Claude Code v2.1.199 aumenta o padrão para outros erros transitórios para `300` e remove o limite nesta variável.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: watchdog de travamento para subagentes lançados com `run_in_background`. Padrão `600000`. Redefine em cada evento de stream; em caso de travamento, aborta o subagente, marca a tarefa como falha e expõe o erro ao pai com qualquer resultado parcial. Não se aplica a subagentes síncronos.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` com `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: aborta a solicitação quando os cabeçalhos chegaram mas o corpo da resposta para de fazer stream. O watchdog está ativado por padrão para todos os provedores; defina `CLAUDE_ENABLE_STREAM_WATCHDOG=0` para desabilitá-lo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` padrão é `300000` e é fixado nesse mínimo. A solicitação abortada passa pelo caminho de tentativa normal.

<h3 id="outputformat">
  `OutputFormat`
</h3>

Configuração para validação de saída estruturada. Passe isso como um `dict` para o campo `output_format` em `ClaudeAgentOptions`:

```python theme={null}
# Expected dict shape for output_format
{
    "type": "json_schema",
    "schema": {...},  # Your JSON Schema definition
}
```

| Campo    | Obrigatório | Descrição                                           |
| :------- | :---------- | :-------------------------------------------------- |
| `type`   | Sim         | Deve ser `"json_schema"` para validação JSON Schema |
| `schema` | Sim         | Definição JSON Schema para validação de saída       |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

Configuração para usar o prompt do sistema preset do Claude Code com adições opcionais.

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
```

| Campo                      | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                       |
| :------------------------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`                     | Sim         | Deve ser `"preset"` para usar um prompt do sistema preset                                                                                                                                                                                                                                                                                                       |
| `preset`                   | Sim         | Deve ser `"claude_code"` para usar o prompt do sistema do Claude Code                                                                                                                                                                                                                                                                                           |
| `append`                   | Não         | Instruções adicionais para anexar ao prompt do sistema preset                                                                                                                                                                                                                                                                                                   |
| `exclude_dynamic_sections` | Não         | Mova contexto por sessão como diretório de trabalho, sinalizador git-repo e caminhos de memória automática do prompt do sistema para a primeira mensagem do usuário. Melhora a reutilização de cache de prompt entre usuários e máquinas. Veja [Modify system prompts](/docs/pt/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

Configuração para carregar um prompt do sistema personalizado de um arquivo em vez de passá-lo como uma string. O SDK mapeia isso para o sinalizador CLI [`--system-prompt-file`](/docs/pt/cli-reference#system-prompt-flags). Use a forma de arquivo quando o prompt é grande: o SDK passa um `system_prompt` string no argv do subprocess CLI, que está sujeito aos limites de comprimento de linha de comando do SO antes do SDK enviar qualquer solicitação de API. No Linux, um único argumento mais longo que aproximadamente 128 KB falha no spawn do processo com `Argument list too long`. No Windows, toda a linha de comando é limitada a aproximadamente 32 KB, então a forma de string falha em um limite inferior.

```python theme={null}
class SystemPromptFile(TypedDict):
    type: Literal["file"]
    path: str
```

| Campo  | Obrigatório | Descrição                                            |
| :----- | :---------- | :--------------------------------------------------- |
| `type` | Sim         | Deve ser `"file"` para carregar o prompt do disco    |
| `path` | Sim         | Caminho para um arquivo contendo o prompt do sistema |

<h3 id="settingsource">
  `SettingSource`
</h3>

Controla quais fontes de configuração baseadas em sistema de arquivos o SDK carrega configurações.

```python theme={null}
SettingSource = Literal["user", "project", "local"]
```

| Valor       | Descrição                                                        | Localização                   |
| :---------- | :--------------------------------------------------------------- | :---------------------------- |
| `"user"`    | Configurações globais do usuário                                 | `~/.claude/settings.json`     |
| `"project"` | Configurações de projeto compartilhadas (controladas por versão) | `.claude/settings.json`       |
| `"local"`   | Configurações de projeto local (não controladas por versão)      | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamento padrão
</h4>

Quando `setting_sources` é omitido ou `None`, `query()` carrega as mesmas configurações do sistema de arquivos que o CLI do Claude Code: usuário, projeto e local. Configurações de política gerenciada são carregadas em todos os casos; configurações gerenciadas pelo servidor são buscadas quando a sessão se autentica com uma credencial de organização em uma [configuração elegível](/docs/pt/server-managed-settings#platform-availability). Veja [What settingSources does not control](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control) para entradas que são lidas independentemente desta opção, e como desabilitá-las.

<h4 id="why-use-setting_sources">
  Por que usar setting\_sources
</h4>

**Desabilitar configurações do sistema de arquivos:**

```python theme={null}
# Do not load user, project, or local settings from disk
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=[]
    ),
):
    print(message)
```

<Note>
  No Python SDK 0.1.59 e anterior, uma lista vazia era tratada da mesma forma que omitir a opção, então `setting_sources=[]` não desabilitava configurações do sistema de arquivos. Atualize para uma versão mais recente se você precisar que uma lista vazia tenha efeito. O SDK TypeScript não é afetado.
</Note>

**Carregue todas as configurações do sistema de arquivos explicitamente:**

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions

async for message in query(
    prompt="Analyze this code",
    options=ClaudeAgentOptions(
        setting_sources=["user", "project", "local"]
    ),
):
    print(message)
```

**Carregue apenas fontes de configuração específicas:**

```python theme={null}
# Load only project settings, ignore user and local
async for message in query(
    prompt="Run CI checks",
    options=ClaudeAgentOptions(
        setting_sources=["project"]  # Only .claude/settings.json
    ),
):
    print(message)
```

**Ambientes de teste e CI:**

```python theme={null}
# Ensure consistent behavior in CI by excluding local settings
async for message in query(
    prompt="Run tests",
    options=ClaudeAgentOptions(
        setting_sources=["project"],  # Only team-shared settings
        permission_mode="bypassPermissions",
    ),
):
    print(message)
```

**Aplicações apenas SDK:**

```python theme={null}
# Define everything programmatically.
# Pass [] to opt out of filesystem setting sources.
async for message in query(
    prompt="Review this PR",
    options=ClaudeAgentOptions(
        setting_sources=[],
        agents={...},
        mcp_servers={...},
        allowed_tools=["Read", "Grep", "Glob"],
    ),
):
    print(message)
```

**Carregando instruções de projeto CLAUDE.md:**

```python theme={null}
# Load project settings to include CLAUDE.md files
async for message in query(
    prompt="Add a new feature following project conventions",
    options=ClaudeAgentOptions(
        system_prompt={
            "type": "preset",
            "preset": "claude_code",  # Use Claude Code's system prompt
        },
        setting_sources=["project"],  # Loads CLAUDE.md from project
        allowed_tools=["Read", "Write", "Edit"],
    ),
):
    print(message)
```

<h4 id="settings-precedence">
  Precedência de configurações
</h4>

Quando múltiplas fontes são carregadas, as configurações são mescladas com esta precedência (maior para menor):

1. Configurações locais (`.claude/settings.local.json`)
2. Configurações de projeto (`.claude/settings.json`)
3. Configurações de usuário (`~/.claude/settings.json`)

Opções programáticas como `agents` e `allowed_tools` substituem configurações do sistema de arquivos de usuário, projeto e local. Configurações de política gerenciada têm precedência sobre opções programáticas.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Configuração para um subagente definido programaticamente.

```python theme={null}
@dataclass
class AgentDefinition:
    description: str
    prompt: str
    tools: list[str] | None = None
    disallowedTools: list[str] | None = None
    model: str | None = None
    skills: list[str] | None = None
    memory: Literal["user", "project", "local"] | None = None
    mcpServers: list[str | dict[str, Any]] | None = None
    initialPrompt: str | None = None
    maxTurns: int | None = None
    background: bool | None = None
    effort: EffortLevel | int | None = None
    permissionMode: PermissionMode | None = None
```

| Campo             | Obrigatório | Descrição                                                                                                                                                                                                                                                              |
| :---------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`     | Sim         | Descrição em linguagem natural de quando usar este agente                                                                                                                                                                                                              |
| `prompt`          | Sim         | O prompt do sistema do agente                                                                                                                                                                                                                                          |
| `tools`           | Não         | Array de nomes de ferramentas permitidas. Se omitido, herda todas as ferramentas                                                                                                                                                                                       |
| `disallowedTools` | Não         | Array de nomes de ferramentas a remover do conjunto de ferramentas do agente. Padrões de nível de servidor MCP também são aceitos: `mcp__server` ou `mcp__server__*` remove cada ferramenta desse servidor, e `mcp__*` remove cada ferramenta MCP de qualquer servidor |
| `model`           | Não         | Substituição de modelo para este agente. Aceita um alias como `"sonnet"`, `"opus"`, `"haiku"`, ou `"inherit"`, ou um ID de modelo completo. Se omitido, usa o modelo principal                                                                                         |
| `skills`          | Não         | Lista de nomes de skills a pré-carregar no contexto do agente na inicialização. Skills não listados permanecem invocáveis através da ferramenta Skill                                                                                                                  |
| `memory`          | Não         | Fonte de memória para este agente: `"user"`, `"project"`, ou `"local"`                                                                                                                                                                                                 |
| `mcpServers`      | Não         | Servidores MCP disponíveis para este agente. Cada entrada é um nome de servidor ou um dict `{name: config}` inline                                                                                                                                                     |
| `initialPrompt`   | Não         | Auto-enviado como o primeiro turno de usuário quando este agente é executado como o agente de thread principal                                                                                                                                                         |
| `maxTurns`        | Não         | Número máximo de turnos agênticos antes do agente parar                                                                                                                                                                                                                |
| `background`      | Não         | Execute este agente como uma tarefa de fundo não bloqueante quando invocado                                                                                                                                                                                            |
| `effort`          | Não         | Nível de esforço de raciocínio para este agente. Aceita um nível nomeado ou um inteiro. Veja [`EffortLevel`](#effortlevel)                                                                                                                                             |
| `permissionMode`  | Não         | Modo de permissão para execução de ferramenta dentro deste agente. Veja [`PermissionMode`](#permissionmode)                                                                                                                                                            |

<Note>
  Os nomes de campo `AgentDefinition` usam camelCase, como `disallowedTools`, `permissionMode` e `maxTurns`. Esses nomes mapeiam diretamente para o formato de fio compartilhado com o SDK TypeScript. Isso difere de `ClaudeAgentOptions`, que usa snake\_case Python para campos de nível superior equivalentes como `disallowed_tools` e `permission_mode`. Como `AgentDefinition` é uma dataclass, passar uma palavra-chave snake\_case levanta um `TypeError` no tempo de construção.
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

Modos de permissão para controlar a execução de ferramentas.

```python theme={null}
PermissionMode = Literal[
    "default",  # Standard permission behavior
    "acceptEdits",  # Auto-accept file edits
    "plan",  # Planning mode - explore without editing
    "dontAsk",  # Deny anything not pre-approved instead of prompting
    "bypassPermissions",  # Bypass permission checks; explicit ask rules still prompt (use with caution)
    "auto",  # A model classifier approves or denies each tool call
]
```

<h3 id="effortlevel">
  `EffortLevel`
</h3>

Níveis de esforço para guiar a profundidade de pensamento.

```python theme={null}
EffortLevel = Literal[
    "low",  # Minimal thinking, fastest responses
    "medium",  # Moderate thinking
    "high",  # Deep reasoning
    "xhigh",  # Extended reasoning; falls back to "high" on models that don't support it
    "max",  # Maximum effort
]
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Alias de tipo para funções de callback de permissão de ferramenta.

```python theme={null}
CanUseTool = Callable[
    [str, dict[str, Any], ToolPermissionContext], Awaitable[PermissionResult]
]
```

O callback recebe:

* `tool_name`: Nome da ferramenta sendo chamada
* `input_data`: Os parâmetros de entrada da ferramenta
* `context`: Um `ToolPermissionContext` com informações adicionais

Retorna um `PermissionResult` (ou `PermissionResultAllow` ou `PermissionResultDeny`).

O callback é a substituição do SDK para o prompt de permissão interativo: é invocado apenas quando o [fluxo de avaliação de permissão](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) se resolve para um prompt. Chamadas de ferramenta já aprovadas por uma entrada `allowed_tools`, uma regra de permissão de configurações ou o modo de permissão, como `acceptEdits` ou `bypassPermissions`, nunca o invocam. Para controlar cada chamada de ferramenta, use um [hook `PreToolUse`](/docs/pt/agent-sdk/hooks) em vez disso.

`AskUserQuestion`, ferramentas MCP marcadas [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool), e ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) a alcançam mesmo quando uma regra de permissão corresponde. Em modo `dontAsk` essas chamadas são negadas em vez disso, sem invocar o callback.

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Informações de contexto passadas para callbacks de permissão de ferramenta.

```python theme={null}
@dataclass
class ToolPermissionContext:
    signal: Any | None = None  # Future: abort signal support
    suggestions: list[PermissionUpdate] = field(default_factory=list)
    blocked_path: str | None = None
    decision_reason: str | None = None
    title: str | None = None
    display_name: str | None = None
    description: str | None = None
```

| Campo             | Tipo                     | Descrição                                                                                                                                                                                                                           |
| :---------------- | :----------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`          | `Any \| None`            | Reservado para suporte futuro de sinal de aborto                                                                                                                                                                                    |
| `suggestions`     | `list[PermissionUpdate]` | Sugestões de atualização de permissão do CLI. Prompts Bash incluem uma sugestão com o destino `localSettings`, então retorná-la em `updated_permissions` escreve a regra em `.claude/settings.local.json` e persiste entre sessões. |
| `blocked_path`    | `str \| None`            | Caminho de arquivo que acionou a solicitação de permissão, quando aplicável. Por exemplo, quando um comando Bash tenta acessar um caminho fora dos diretórios permitidos                                                            |
| `decision_reason` | `str \| None`            | Razão pela qual esta solicitação de permissão foi acionada. Encaminhada de um hook PreToolUse `permissionDecisionReason` quando o hook retornou `"ask"`                                                                             |
| `title`           | `str \| None`            | Sentença completa do prompt de permissão, como `Claude wants to read foo.txt`. Use como o texto do prompt principal quando presente                                                                                                 |
| `display_name`    | `str \| None`            | Frase de substantivo curta para a ação da ferramenta, como `Read file`, adequada para rótulos de botão                                                                                                                              |
| `description`     | `str \| None`            | Subtítulo legível por humanos para a UI de permissão                                                                                                                                                                                |

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Tipo de união para resultados de callback de permissão.

```python theme={null}
PermissionResult = PermissionResultAllow | PermissionResultDeny
```

<h3 id="permissionresultallow">
  `PermissionResultAllow`
</h3>

Resultado indicando que a chamada de ferramenta deve ser permitida.

```python theme={null}
@dataclass
class PermissionResultAllow:
    behavior: Literal["allow"] = "allow"
    updated_input: dict[str, Any] | None = None
    updated_permissions: list[PermissionUpdate] | None = None
```

| Campo                 | Tipo                             | Padrão    | Descrição                                    |
| :-------------------- | :------------------------------- | :-------- | :------------------------------------------- |
| `behavior`            | `Literal["allow"]`               | `"allow"` | Deve ser "allow"                             |
| `updated_input`       | `dict[str, Any] \| None`         | `None`    | Entrada modificada a usar em vez da original |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None`    | Atualizações de permissão a aplicar          |

<h3 id="permissionresultdeny">
  `PermissionResultDeny`
</h3>

Resultado indicando que a chamada de ferramenta deve ser negada.

```python theme={null}
@dataclass
class PermissionResultDeny:
    behavior: Literal["deny"] = "deny"
    message: str = ""
    interrupt: bool = False
```

| Campo       | Tipo              | Padrão   | Descrição                                           |
| :---------- | :---------------- | :------- | :-------------------------------------------------- |
| `behavior`  | `Literal["deny"]` | `"deny"` | Deve ser "deny"                                     |
| `message`   | `str`             | `""`     | Mensagem explicando por que a ferramenta foi negada |
| `interrupt` | `bool`            | `False`  | Se deve interromper a execução atual                |

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Configuração para atualizar permissões programaticamente.

```python theme={null}
@dataclass
class PermissionUpdate:
    type: Literal[
        "addRules",
        "replaceRules",
        "removeRules",
        "setMode",
        "addDirectories",
        "removeDirectories",
    ]
    rules: list[PermissionRuleValue] | None = None
    behavior: Literal["allow", "deny", "ask"] | None = None
    mode: PermissionMode | None = None
    directories: list[str] | None = None
    destination: (
        Literal["userSettings", "projectSettings", "localSettings", "session"] | None
    ) = None
```

| Campo         | Tipo                                      | Descrição                                                |
| :------------ | :---------------------------------------- | :------------------------------------------------------- |
| `type`        | `Literal[...]`                            | O tipo de operação de atualização de permissão           |
| `rules`       | `list[PermissionRuleValue] \| None`       | Regras para operações de adicionar/substituir/remover    |
| `behavior`    | `Literal["allow", "deny", "ask"] \| None` | Comportamento para operações baseadas em regras          |
| `mode`        | `PermissionMode \| None`                  | Modo para operação setMode                               |
| `directories` | `list[str] \| None`                       | Diretórios para operações de adicionar/remover diretório |
| `destination` | `Literal[...] \| None`                    | Onde aplicar a atualização de permissão                  |

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

Uma regra a adicionar, substituir ou remover em uma atualização de permissão.

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

<h3 id="toolspreset">
  `ToolsPreset`
</h3>

Configuração de ferramentas preset para usar o conjunto de ferramentas padrão do Claude Code.

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Controla o comportamento de pensamento estendido. Uma união de três configurações:

```python theme={null}
ThinkingDisplay = Literal["summarized", "omitted"]


class ThinkingConfigAdaptive(TypedDict):
    type: Literal["adaptive"]
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigEnabled(TypedDict):
    type: Literal["enabled"]
    budget_tokens: int
    display: NotRequired[ThinkingDisplay]


class ThinkingConfigDisabled(TypedDict):
    type: Literal["disabled"]


ThinkingConfig = ThinkingConfigAdaptive | ThinkingConfigEnabled | ThinkingConfigDisabled
```

| Variante   | Campos                             | Descrição                                             |
| :--------- | :--------------------------------- | :---------------------------------------------------- |
| `adaptive` | `type`, `display`                  | Claude decide adaptativamente quando pensar           |
| `enabled`  | `type`, `budget_tokens`, `display` | Ativa pensamento com um orçamento de token específico |
| `disabled` | `type`                             | Desativa pensamento                                   |

O campo opcional `display` controla se o texto de pensamento é retornado `"summarized"` ou `"omitted"`. No Claude Opus 4.7 e posterior, o padrão da API é `"omitted"`, então defina `"summarized"` para receber conteúdo de pensamento em saídas [`ThinkingBlock`](#thinkingblock).

Como estas são classes `TypedDict`, são dicts simples em tempo de execução. Construa-as como literais de dict ou chame a classe como um construtor; ambos produzem um `dict`. Acesse campos com `config["budget_tokens"]`, não `config.budget_tokens`:

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions, ThinkingConfigEnabled

# Option 1: dict literal (recommended, no import needed)
options = ClaudeAgentOptions(thinking={"type": "enabled", "budget_tokens": 20000})

# Option 2: constructor-style (returns a plain dict)
config = ThinkingConfigEnabled(type="enabled", budget_tokens=20000)
print(config["budget_tokens"])  # 20000
# config.budget_tokens would raise AttributeError
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Tipo literal para recursos beta do SDK.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Use com o campo `betas` em `ClaudeAgentOptions` para ativar recursos beta.

<Warning>
  O beta `context-1m-2025-08-07` foi descontinuado a partir de 30 de abril de 2026. Passar este cabeçalho com Claude Sonnet 4.5 ou Sonnet 4 não tem efeito, e solicitações que excedem a janela de contexto padrão de 200k-token retornam um erro. Para usar uma janela de contexto de 1M-token, migre para [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, ou Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), que incluem contexto de 1M a preços padrão sem cabeçalho beta necessário.
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

Configuração para servidores MCP do SDK criados com `create_sdk_mcp_server()`.

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # MCP Server instance
```

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Tipo de união para configurações de servidor MCP.

```python theme={null}
McpServerConfig = (
    McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig
)
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```python theme={null}
class McpStdioServerConfig(TypedDict):
    type: NotRequired[Literal["stdio"]]  # Optional for backwards compatibility
    command: str
    args: NotRequired[list[str]]
    env: NotRequired[dict[str, str]]
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```python theme={null}
class McpSSEServerConfig(TypedDict):
    type: Literal["sse"]
    url: str
    headers: NotRequired[dict[str, str]]
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```python theme={null}
class McpHttpServerConfig(TypedDict):
    type: Literal["http"]
    url: str
    headers: NotRequired[dict[str, str]]
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

A configuração de um servidor MCP conforme relatado por [`get_mcp_status()`](#methods). Esta é a união de todas as variantes de transporte [`McpServerConfig`](#mcpserverconfig) mais uma variante de saída apenas `claudeai-proxy` para servidores proxied através de claude.ai.

```python theme={null}
McpServerStatusConfig = (
    McpStdioServerConfig
    | McpSSEServerConfig
    | McpHttpServerConfig
    | McpSdkServerConfigStatus
    | McpClaudeAIProxyServerConfig
)
```

`McpSdkServerConfigStatus` é a forma serializável de [`McpSdkServerConfig`](#mcpsdkserverconfig) com apenas campos `type` (`"sdk"`) e `name` (`str`); a `instance` em processo é omitida. `McpClaudeAIProxyServerConfig` tem campos `type` (`"claudeai-proxy"`), `url` (`str`), e `id` (`str`).

<h3 id="mcpstatusresponse">
  `McpStatusResponse`
</h3>

Resposta de [`ClaudeSDKClient.get_mcp_status()`](#methods). Envolve a lista de status de servidor sob a chave `mcpServers`.

```python theme={null}
class McpStatusResponse(TypedDict):
    mcpServers: list[McpServerStatus]
```

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Status de um servidor MCP conectado, contido em [`McpStatusResponse`](#mcpstatusresponse).

```python theme={null}
class McpServerStatus(TypedDict):
    name: str
    status: McpServerConnectionStatus  # "connected" | "failed" | "needs-auth" | "pending" | "disabled"
    serverInfo: NotRequired[McpServerInfo]
    error: NotRequired[str]
    config: NotRequired[McpServerStatusConfig]
    scope: NotRequired[str]
    tools: NotRequired[list[McpToolInfo]]
```

| Campo        | Tipo                                                         | Descrição                                                                                                                                                                                      |
| :----------- | :----------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`       | `str`                                                        | Nome do servidor                                                                                                                                                                               |
| `status`     | `str`                                                        | Um de `"connected"`, `"failed"`, `"needs-auth"`, `"pending"`, ou `"disabled"`                                                                                                                  |
| `serverInfo` | `dict` (opcional)                                            | Nome e versão do servidor (`{"name": str, "version": str}`)                                                                                                                                    |
| `error`      | `str` (opcional)                                             | Mensagem de erro se o servidor falhou ao conectar                                                                                                                                              |
| `config`     | [`McpServerStatusConfig`](#mcpserverstatusconfig) (opcional) | Configuração do servidor. Mesma forma que [`McpServerConfig`](#mcpserverconfig) (stdio, SSE, HTTP, ou SDK), mais uma variante `claudeai-proxy` para servidores conectados através de claude.ai |
| `scope`      | `str` (opcional)                                             | Escopo de configuração                                                                                                                                                                         |
| `tools`      | `list` (opcional)                                            | Ferramentas fornecidas por este servidor, cada uma com campos `name`, `description`, e `annotations`                                                                                           |

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Configuração para carregar plugins no SDK.

```python theme={null}
class SdkPluginConfig(TypedDict):
    type: Literal["local"]
    path: str
```

| Campo  | Tipo               | Descrição                                                        |
| :----- | :----------------- | :--------------------------------------------------------------- |
| `type` | `Literal["local"]` | Deve ser `"local"` (apenas plugins locais atualmente suportados) |
| `path` | `str`              | Caminho absoluto ou relativo para o diretório do plugin          |

**Exemplo:**

```python theme={null}
plugins = [
    {"type": "local", "path": "./my-plugin"},
    {"type": "local", "path": "/absolute/path/to/plugin"},
]
```

Para informações completas sobre criação e uso de plugins, veja [Plugins](/docs/pt/agent-sdk/plugins).

<h2 id="message-types">
  Tipos de Mensagem
</h2>

<h3 id="message">
  `Message`
</h3>

Tipo de união de todas as mensagens possíveis.

```python theme={null}
Message = (
    UserMessage
    | AssistantMessage
    | SystemMessage
    | ResultMessage
    | StreamEvent
    | RateLimitEvent
)
```

<h3 id="usermessage">
  `UserMessage`
</h3>

Mensagem de entrada do usuário.

```python theme={null}
@dataclass
class UserMessage:
    content: str | list[ContentBlock]
    uuid: str | None = None
    parent_tool_use_id: str | None = None
    tool_use_result: dict[str, Any] | None = None
```

| Campo                | Tipo                        | Descrição                                                                          |
| :------------------- | :-------------------------- | :--------------------------------------------------------------------------------- |
| `content`            | `str \| list[ContentBlock]` | Conteúdo da mensagem como texto ou blocos de conteúdo                              |
| `uuid`               | `str \| None`               | Identificador único de mensagem                                                    |
| `parent_tool_use_id` | `str \| None`               | ID de uso de ferramenta se esta mensagem é uma resposta de resultado de ferramenta |
| `tool_use_result`    | `dict[str, Any] \| None`    | Dados de resultado de ferramenta se aplicável                                      |

<h3 id="assistantmessage">
  `AssistantMessage`
</h3>

Mensagem de resposta do assistente com blocos de conteúdo.

```python theme={null}
@dataclass
class AssistantMessage:
    content: list[ContentBlock]
    model: str
    parent_tool_use_id: str | None = None
    error: AssistantMessageError | None = None
    usage: dict[str, Any] | None = None
    message_id: str | None = None
```

| Campo                | Tipo                                                         | Descrição                                                                             |
| :------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------ |
| `content`            | `list[ContentBlock]`                                         | Lista de blocos de conteúdo na resposta                                               |
| `model`              | `str`                                                        | Modelo que gerou a resposta                                                           |
| `parent_tool_use_id` | `str \| None`                                                | ID de uso de ferramenta se esta é uma resposta aninhada                               |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Tipo de erro se a resposta encontrou um erro                                          |
| `usage`              | `dict[str, Any] \| None`                                     | Uso de token por mensagem (mesmas chaves que [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | ID de mensagem da API. Múltiplas mensagens de um turno compartilham o mesmo ID        |

<h3 id="assistantmessageerror">
  `AssistantMessageError`
</h3>

Possíveis tipos de erro para mensagens do assistente.

```python theme={null}
AssistantMessageError = Literal[
    "authentication_failed",
    "billing_error",
    "rate_limit",
    "invalid_request",
    "server_error",
    "max_output_tokens",
    "unknown",
]
```

<h3 id="systemmessage">
  `SystemMessage`
</h3>

Mensagem do sistema com metadados.

```python theme={null}
@dataclass
class SystemMessage:
    subtype: str
    data: dict[str, Any]
```

<h3 id="resultmessage">
  `ResultMessage`
</h3>

Mensagem de resultado final com informações de custo e uso.

```python theme={null}
@dataclass
class ResultMessage:
    subtype: str
    duration_ms: int
    duration_api_ms: int
    is_error: bool
    num_turns: int
    session_id: str
    stop_reason: str | None = None
    total_cost_usd: float | None = None
    usage: dict[str, Any] | None = None
    result: str | None = None
    structured_output: Any = None
    model_usage: dict[str, Any] | None = None
    permission_denials: list[Any] | None = None
    deferred_tool_use: DeferredToolUse | None = None
    errors: list[str] | None = None
    api_error_status: int | None = None
    uuid: str | None = None
```

O campo `subtype` determina quais outros campos são preenchidos. É um de `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"` ou `"error_max_structured_output_retries"`. A dataclass Python achata todas as variantes em uma forma, portanto campos que não se aplicam ao subtipo retornado são `None`.

Vários campos carregam detalhes de diagnóstico quando a conversa termina em um erro:

* `is_error`: `True` quando a conversa terminou em um estado de erro. Sempre `True` nos subtipos `error_*`. Em `subtype="success"` é `True` quando a solicitação final do modelo falhou, significando que o loop do agente foi concluído mas a última chamada da API retornou um erro.
* `api_error_status`: o código de status HTTP do erro de API de encerramento. `None` quando o turno terminou sem um. Preenchido apenas em `subtype="success"`.
* `result`: texto da mensagem final do assistente em `subtype="success"`, ou `None` nos subtipos `error_*`. Quando `subtype="success"` e `is_error=True`, isso contém a string de erro da API se uma estiver disponível mas pode estar vazio, então verifique `api_error_status` e o conteúdo anterior de `AssistantMessage` para detalhes.
* `errors`: strings de erro no nível do loop, como a mensagem de máximo de turnos. Preenchido apenas nos subtipos `error_*`.

O dict `usage` contém as seguintes chaves quando presentes:

| Chave                         | Tipo  | Descrição                                                                                                                                                                                                                          |
| ----------------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | Tokens de entrada consumidos pelo loop do agente de nível superior. [Tokens de subagente não estão incluídos](/docs/pt/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); use `model_usage` para contabilidade de árvore completa. |
| `output_tokens`               | `int` | Tokens de saída gerados pelo loop do agente de nível superior. Tokens de subagente não estão incluídos.                                                                                                                            |
| `cache_creation_input_tokens` | `int` | Tokens usados para criar novas entradas de cache.                                                                                                                                                                                  |
| `cache_read_input_tokens`     | `int` | Tokens lidos de entradas de cache existentes.                                                                                                                                                                                      |

O dict `model_usage` mapeia nomes de modelo para uso por modelo. As chaves do dict interno usam camelCase porque o valor é passado sem modificação do processo CLI subjacente, correspondendo ao tipo TypeScript [`ModelUsage`](/docs/pt/agent-sdk/typescript#modelusage):

| Chave                      | Tipo    | Descrição                                                                                                                                                     |
| -------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | Tokens de entrada para este modelo.                                                                                                                           |
| `outputTokens`             | `int`   | Tokens de saída para este modelo.                                                                                                                             |
| `cacheReadInputTokens`     | `int`   | Tokens de leitura de cache para este modelo.                                                                                                                  |
| `cacheCreationInputTokens` | `int`   | Tokens de criação de cache para este modelo.                                                                                                                  |
| `webSearchRequests`        | `int`   | Solicitações de busca na web feitas por este modelo.                                                                                                          |
| `costUSD`                  | `float` | Custo estimado em USD para este modelo, computado no lado do cliente. Veja [Rastrear custo e uso](/docs/pt/agent-sdk/cost-tracking) para ressalvas de faturamento. |
| `contextWindow`            | `int`   | Tamanho da janela de contexto para este modelo.                                                                                                               |
| `maxOutputTokens`          | `int`   | Limite máximo de token de saída para este modelo.                                                                                                             |

<h3 id="streamevent">
  `StreamEvent`
</h3>

Evento de fluxo para atualizações de mensagem parcial durante streaming. Apenas recebido quando `include_partial_messages=True` em `ClaudeAgentOptions`. Importe via `from claude_agent_sdk.types import StreamEvent`.

```python theme={null}
@dataclass
class StreamEvent:
    uuid: str
    session_id: str
    event: dict[str, Any]  # The raw Claude API stream event
    parent_tool_use_id: str | None = None
```

| Campo                | Tipo             | Descrição                                                                                                                                                                       |
| :------------------- | :--------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `uuid`               | `str`            | Identificador único para este evento                                                                                                                                            |
| `session_id`         | `str`            | Identificador de sessão                                                                                                                                                         |
| `event`              | `dict[str, Any]` | Os dados brutos do evento de fluxo da API Claude                                                                                                                                |
| `parent_tool_use_id` | `str \| None`    | Sempre `None`. Eventos de fluxo são emitidos apenas para a sessão principal. Para atribuição de subagente, use mensagens completas como [`AssistantMessage`](#assistantmessage) |

<h3 id="ratelimitevent">
  `RateLimitEvent`
</h3>

Emitido quando o status do limite de taxa muda (por exemplo, de `"allowed"` para `"allowed_warning"`). Use isso para avisar usuários antes de atingirem um limite rígido, ou para recuar quando o status é `"rejected"`.

```python theme={null}
@dataclass
class RateLimitEvent:
    rate_limit_info: RateLimitInfo
    uuid: str
    session_id: str
```

| Campo             | Tipo                              | Descrição                      |
| :---------------- | :-------------------------------- | :----------------------------- |
| `rate_limit_info` | [`RateLimitInfo`](#ratelimitinfo) | Estado de limite de taxa atual |
| `uuid`            | `str`                             | Identificador único de evento  |
| `session_id`      | `str`                             | Identificador de sessão        |

<h3 id="ratelimitinfo">
  `RateLimitInfo`
</h3>

Estado de limite de taxa carregado por [`RateLimitEvent`](#ratelimitevent).

```python theme={null}
RateLimitStatus = Literal["allowed", "allowed_warning", "rejected"]
RateLimitType = Literal[
    "five_hour", "seven_day", "seven_day_opus", "seven_day_sonnet", "overage"
]


@dataclass
class RateLimitInfo:
    status: RateLimitStatus
    resets_at: int | None = None
    rate_limit_type: RateLimitType | None = None
    utilization: float | None = None
    overage_status: RateLimitStatus | None = None
    overage_resets_at: int | None = None
    overage_disabled_reason: str | None = None
    raw: dict[str, Any] = field(default_factory=dict)
```

| Campo                     | Tipo                      | Descrição                                                                                                              |
| :------------------------ | :------------------------ | :--------------------------------------------------------------------------------------------------------------------- |
| `status`                  | `RateLimitStatus`         | Status atual. `"allowed_warning"` significa aproximando-se do limite; `"rejected"` significa que o limite foi atingido |
| `resets_at`               | `int \| None`             | Timestamp Unix quando a janela de limite de taxa é redefinida                                                          |
| `rate_limit_type`         | `RateLimitType \| None`   | Qual janela de limite de taxa se aplica                                                                                |
| `utilization`             | `float \| None`           | Fração do limite de taxa consumido (0.0 a 1.0)                                                                         |
| `overage_status`          | `RateLimitStatus \| None` | Status do uso de excedente pré-pago, se aplicável                                                                      |
| `overage_resets_at`       | `int \| None`             | Timestamp Unix quando a janela de excedente é redefinida                                                               |
| `overage_disabled_reason` | `str \| None`             | Por que o excedente está indisponível, se o status é `"rejected"`                                                      |
| `raw`                     | `dict[str, Any]`          | Dict bruto completo do CLI, incluindo campos não modelados acima                                                       |

<h3 id="taskstartedmessage">
  `TaskStartedMessage`
</h3>

Emitido quando uma tarefa de fundo começa. Uma tarefa de fundo é qualquer coisa rastreada fora do turno principal: um comando Bash em fundo, um watch de [Monitor](#monitor), um subagente gerado via ferramenta Agent, ou um agente remoto. O campo `task_type` diz qual. Esta nomenclatura não está relacionada à renomeação de ferramenta `Task`-para-`Agent`.

```python theme={null}
@dataclass
class TaskStartedMessage(SystemMessage):
    task_id: str
    description: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    task_type: str | None = None
```

| Campo         | Tipo          | Descrição                                                                                                                 |
| :------------ | :------------ | :------------------------------------------------------------------------------------------------------------------------ |
| `task_id`     | `str`         | Identificador único para a tarefa                                                                                         |
| `description` | `str`         | Descrição da tarefa                                                                                                       |
| `uuid`        | `str`         | Identificador único de mensagem                                                                                           |
| `session_id`  | `str`         | Identificador de sessão                                                                                                   |
| `tool_use_id` | `str \| None` | ID de uso de ferramenta associado                                                                                         |
| `task_type`   | `str \| None` | Que tipo de tarefa de fundo: `"local_bash"` para Bash em fundo e watches de Monitor, `"local_agent"`, ou `"remote_agent"` |

<h3 id="taskusage">
  `TaskUsage`
</h3>

Dados de token e tempo para uma tarefa de fundo.

```python theme={null}
class TaskUsage(TypedDict):
    total_tokens: int
    tool_uses: int
    duration_ms: int
```

<h3 id="taskprogressmessage">
  `TaskProgressMessage`
</h3>

Emitido periodicamente com atualizações de progresso para uma tarefa de fundo em execução.

```python theme={null}
@dataclass
class TaskProgressMessage(SystemMessage):
    task_id: str
    description: str
    usage: TaskUsage
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    last_tool_name: str | None = None
```

| Campo            | Tipo          | Descrição                                   |
| :--------------- | :------------ | :------------------------------------------ |
| `task_id`        | `str`         | Identificador único para a tarefa           |
| `description`    | `str`         | Descrição de status atual                   |
| `usage`          | `TaskUsage`   | Uso de token para esta tarefa até agora     |
| `uuid`           | `str`         | Identificador único de mensagem             |
| `session_id`     | `str`         | Identificador de sessão                     |
| `tool_use_id`    | `str \| None` | ID de uso de ferramenta associado           |
| `last_tool_name` | `str \| None` | Nome da última ferramenta que a tarefa usou |

<h3 id="tasknotificationmessage">
  `TaskNotificationMessage`
</h3>

Emitido quando uma tarefa de fundo é concluída, falha ou é parada. Tarefas de fundo incluem comandos Bash `run_in_background`, watches de Monitor e subagentes em fundo.

```python theme={null}
@dataclass
class TaskNotificationMessage(SystemMessage):
    task_id: str
    status: TaskNotificationStatus  # "completed" | "failed" | "stopped"
    output_file: str
    summary: str
    uuid: str
    session_id: str
    tool_use_id: str | None = None
    usage: TaskUsage | None = None
```

| Campo         | Tipo                     | Descrição                                       |
| :------------ | :----------------------- | :---------------------------------------------- |
| `task_id`     | `str`                    | Identificador único para a tarefa               |
| `status`      | `TaskNotificationStatus` | Um de `"completed"`, `"failed"`, ou `"stopped"` |
| `output_file` | `str`                    | Caminho para o arquivo de saída da tarefa       |
| `summary`     | `str`                    | Resumo do resultado da tarefa                   |
| `uuid`        | `str`                    | Identificador único de mensagem                 |
| `session_id`  | `str`                    | Identificador de sessão                         |
| `tool_use_id` | `str \| None`            | ID de uso de ferramenta associado               |
| `usage`       | `TaskUsage \| None`      | Uso de token final para a tarefa                |

<h2 id="content-block-types">
  Tipos de Bloco de Conteúdo
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Tipo de união de todos os blocos de conteúdo.

```python theme={null}
ContentBlock = TextBlock | ThinkingBlock | ToolUseBlock | ToolResultBlock
```

<h3 id="textblock">
  `TextBlock`
</h3>

Bloco de conteúdo de texto.

```python theme={null}
@dataclass
class TextBlock:
    text: str
```

<h3 id="thinkingblock">
  `ThinkingBlock`
</h3>

Bloco de conteúdo de pensamento (para modelos com capacidade de pensamento).

```python theme={null}
@dataclass
class ThinkingBlock:
    thinking: str
    signature: str
```

<h3 id="tooluseblock">
  `ToolUseBlock`
</h3>

Bloco de solicitação de uso de ferramenta.

```python theme={null}
@dataclass
class ToolUseBlock:
    id: str
    name: str
    input: dict[str, Any]
```

<h3 id="toolresultblock">
  `ToolResultBlock`
</h3>

Bloco de resultado de execução de ferramenta.

```python theme={null}
@dataclass
class ToolResultBlock:
    tool_use_id: str
    content: str | list[dict[str, Any]] | None = None
    is_error: bool | None = None
```

<h2 id="error-types">
  Tipos de Erro
</h2>

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Classe de exceção base para todos os erros do SDK.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

<h3 id="clinotfounderror">
  `CLINotFoundError`
</h3>

Levantado quando Claude Code CLI não está instalado ou não é encontrado.

```python theme={null}
class CLINotFoundError(CLIConnectionError):
    def __init__(
        self, message: str = "Claude Code not found", cli_path: str | None = None
    ):
        """
        Args:
            message: Error message (default: "Claude Code not found")
            cli_path: Optional path to the CLI that was not found
        """
```

<h3 id="cliconnectionerror">
  `CLIConnectionError`
</h3>

Levantado quando a conexão com Claude Code falha.

```python theme={null}
class CLIConnectionError(ClaudeSDKError):
    """Failed to connect to Claude Code."""
```

<h3 id="processerror">
  `ProcessError`
</h3>

Levantado quando o processo Claude Code falha.

```python theme={null}
class ProcessError(ClaudeSDKError):
    def __init__(
        self, message: str, exit_code: int | None = None, stderr: str | None = None
    ):
        self.exit_code = exit_code
        self.stderr = stderr
```

<h3 id="clijsondecodeerror">
  `CLIJSONDecodeError`
</h3>

Levantado quando a análise JSON falha.

```python theme={null}
class CLIJSONDecodeError(ClaudeSDKError):
    def __init__(self, line: str, original_error: Exception):
        """
        Args:
            line: The line that failed to parse
            original_error: The original JSON decode exception
        """
        self.line = line
        self.original_error = original_error
```

<h2 id="hook-types">
  Tipos de Hook
</h2>

Para um guia abrangente sobre o uso de hooks com exemplos e padrões comuns, veja o [Guia de Hooks](/docs/pt/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Tipos de evento de hook suportados.

```python theme={null}
HookEvent = Literal[
    "PreToolUse",  # Called before tool execution
    "PostToolUse",  # Called after tool execution
    "PostToolUseFailure",  # Called when a tool execution fails
    "UserPromptSubmit",  # Called when user submits a prompt
    "Stop",  # Called when stopping execution
    "SubagentStop",  # Called when a subagent stops
    "PreCompact",  # Called before message compaction
    "Notification",  # Called for notification events
    "SubagentStart",  # Called when a subagent starts
    "PermissionRequest",  # Called when a permission decision is needed
]
```

<Note>
  O SDK TypeScript suporta eventos de hook adicionais não disponíveis ainda em Python: `SessionStart`, `SessionEnd`, `Setup`, `TeammateIdle`, `TaskCompleted`, `ConfigChange`, `WorktreeCreate`, `WorktreeRemove`, `PostToolBatch` e `MessageDisplay`.
</Note>

<h3 id="hookcallback">
  `HookCallback`
</h3>

Definição de tipo para funções de callback de hook.

```python theme={null}
HookCallback = Callable[[HookInput, str | None, HookContext], Awaitable[HookJSONOutput]]
```

Parâmetros:

* `input`: Entrada de hook fortemente tipada com uniões discriminadas baseadas em `hook_event_name` (veja [`HookInput`](#hookinput))
* `tool_use_id`: Identificador de uso de ferramenta opcional (para hooks relacionados a ferramentas)
* `context`: Contexto de hook com informações adicionais

Retorna um [`HookJSONOutput`](#hookjsonoutput) que pode conter:

* `decision`: `"block"` para bloquear a ação
* `systemMessage`: Mensagem de aviso mostrada ao usuário
* `hookSpecificOutput`: Dados de saída específicos do hook

<h3 id="hookcontext">
  `HookContext`
</h3>

Informações de contexto passadas para callbacks de hook.

```python theme={null}
class HookContext(TypedDict):
    signal: Any | None  # Future: abort signal support
```

<h3 id="hookmatcher">
  `HookMatcher`
</h3>

Configuração para corresponder hooks a eventos ou ferramentas específicas.

```python theme={null}
@dataclass
class HookMatcher:
    matcher: str | None = (
        None  # Tool name or pattern to match (e.g., "Bash", "Write|Edit")
    )
    hooks: list[HookCallback] = field(
        default_factory=list
    )  # List of callbacks to execute
    timeout: float | None = (
        None  # Timeout in seconds for all hooks in this matcher (default: 60)
    )
```

<h3 id="hookinput">
  `HookInput`
</h3>

Tipo de união de todos os tipos de entrada de hook. O tipo real depende do campo `hook_event_name`.

```python theme={null}
HookInput = (
    PreToolUseHookInput
    | PostToolUseHookInput
    | PostToolUseFailureHookInput
    | UserPromptSubmitHookInput
    | StopHookInput
    | SubagentStopHookInput
    | PreCompactHookInput
    | NotificationHookInput
    | SubagentStartHookInput
    | PermissionRequestHookInput
)
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

Campos base presentes em todos os tipos de entrada de hook.

```python theme={null}
class BaseHookInput(TypedDict):
    session_id: str
    transcript_path: str
    cwd: str
    permission_mode: NotRequired[str]
```

| Campo             | Tipo             | Descrição                                       |
| :---------------- | :--------------- | :---------------------------------------------- |
| `session_id`      | `str`            | Identificador de sessão atual                   |
| `transcript_path` | `str`            | Caminho para o arquivo de transcrição da sessão |
| `cwd`             | `str`            | Diretório de trabalho atual                     |
| `permission_mode` | `str` (opcional) | Modo de permissão atual                         |

<h3 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h3>

Dados de entrada para eventos de hook `PreToolUse`.

```python theme={null}
class PreToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PreToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Campo             | Tipo                    | Descrição                                                                         |
| :---------------- | :---------------------- | :-------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PreToolUse"]` | Sempre "PreToolUse"                                                               |
| `tool_name`       | `str`                   | Nome da ferramenta prestes a ser executada                                        |
| `tool_input`      | `dict[str, Any]`        | Parâmetros de entrada para a ferramenta                                           |
| `tool_use_id`     | `str`                   | Identificador único para este uso de ferramenta                                   |
| `agent_id`        | `str` (opcional)        | Identificador de subagente, presente quando o hook dispara dentro de um subagente |
| `agent_type`      | `str` (opcional)        | Tipo de subagente, presente quando o hook dispara dentro de um subagente          |

<h3 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h3>

Dados de entrada para eventos de hook `PostToolUse`.

```python theme={null}
class PostToolUseHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUse"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_response: Any
    tool_use_id: str
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Campo             | Tipo                     | Descrição                                                                         |
| :---------------- | :----------------------- | :-------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUse"]` | Sempre "PostToolUse"                                                              |
| `tool_name`       | `str`                    | Nome da ferramenta que foi executada                                              |
| `tool_input`      | `dict[str, Any]`         | Parâmetros de entrada que foram usados                                            |
| `tool_response`   | `Any`                    | Resposta da execução da ferramenta                                                |
| `tool_use_id`     | `str`                    | Identificador único para este uso de ferramenta                                   |
| `agent_id`        | `str` (opcional)         | Identificador de subagente, presente quando o hook dispara dentro de um subagente |
| `agent_type`      | `str` (opcional)         | Tipo de subagente, presente quando o hook dispara dentro de um subagente          |

<h3 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h3>

Dados de entrada para eventos de hook `PostToolUseFailure`. Chamado quando uma execução de ferramenta falha.

```python theme={null}
class PostToolUseFailureHookInput(BaseHookInput):
    hook_event_name: Literal["PostToolUseFailure"]
    tool_name: str
    tool_input: dict[str, Any]
    tool_use_id: str
    error: str
    is_interrupt: NotRequired[bool]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Campo             | Tipo                            | Descrição                                                                         |
| :---------------- | :------------------------------ | :-------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Sempre "PostToolUseFailure"                                                       |
| `tool_name`       | `str`                           | Nome da ferramenta que falhou                                                     |
| `tool_input`      | `dict[str, Any]`                | Parâmetros de entrada que foram usados                                            |
| `tool_use_id`     | `str`                           | Identificador único para este uso de ferramenta                                   |
| `error`           | `str`                           | Mensagem de erro da execução falhada                                              |
| `is_interrupt`    | `bool` (opcional)               | Se a falha foi causada por uma interrupção                                        |
| `agent_id`        | `str` (opcional)                | Identificador de subagente, presente quando o hook dispara dentro de um subagente |
| `agent_type`      | `str` (opcional)                | Tipo de subagente, presente quando o hook dispara dentro de um subagente          |

<h3 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h3>

Dados de entrada para eventos de hook `UserPromptSubmit`.

```python theme={null}
class UserPromptSubmitHookInput(BaseHookInput):
    hook_event_name: Literal["UserPromptSubmit"]
    prompt: str
```

| Campo             | Tipo                          | Descrição                     |
| :---------------- | :---------------------------- | :---------------------------- |
| `hook_event_name` | `Literal["UserPromptSubmit"]` | Sempre "UserPromptSubmit"     |
| `prompt`          | `str`                         | O prompt enviado pelo usuário |

<h3 id="stophookinput">
  `StopHookInput`
</h3>

Dados de entrada para eventos de hook `Stop`.

```python theme={null}
class StopHookInput(BaseHookInput):
    hook_event_name: Literal["Stop"]
    stop_hook_active: bool
```

| Campo              | Tipo              | Descrição                      |
| :----------------- | :---------------- | :----------------------------- |
| `hook_event_name`  | `Literal["Stop"]` | Sempre "Stop"                  |
| `stop_hook_active` | `bool`            | Se o hook de parada está ativo |

<h3 id="subagentstophookinput">
  `SubagentStopHookInput`
</h3>

Dados de entrada para eventos de hook `SubagentStop`.

```python theme={null}
class SubagentStopHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStop"]
    stop_hook_active: bool
    agent_id: str
    agent_transcript_path: str
    agent_type: str
```

| Campo                   | Tipo                      | Descrição                                          |
| :---------------------- | :------------------------ | :------------------------------------------------- |
| `hook_event_name`       | `Literal["SubagentStop"]` | Sempre "SubagentStop"                              |
| `stop_hook_active`      | `bool`                    | Se o hook de parada está ativo                     |
| `agent_id`              | `str`                     | Identificador único para o subagente               |
| `agent_transcript_path` | `str`                     | Caminho para o arquivo de transcrição do subagente |
| `agent_type`            | `str`                     | Tipo do subagente                                  |

<h3 id="precompacthookinput">
  `PreCompactHookInput`
</h3>

Dados de entrada para eventos de hook `PreCompact`.

```python theme={null}
class PreCompactHookInput(BaseHookInput):
    hook_event_name: Literal["PreCompact"]
    trigger: Literal["manual", "auto"]
    custom_instructions: str | None
```

| Campo                 | Tipo                        | Descrição                                  |
| :-------------------- | :-------------------------- | :----------------------------------------- |
| `hook_event_name`     | `Literal["PreCompact"]`     | Sempre "PreCompact"                        |
| `trigger`             | `Literal["manual", "auto"]` | O que acionou a compactação                |
| `custom_instructions` | `str \| None`               | Instruções personalizadas para compactação |

<h3 id="notificationhookinput">
  `NotificationHookInput`
</h3>

Dados de entrada para eventos de hook `Notification`.

```python theme={null}
class NotificationHookInput(BaseHookInput):
    hook_event_name: Literal["Notification"]
    message: str
    title: NotRequired[str]
    notification_type: str
```

| Campo               | Tipo                      | Descrição                           |
| :------------------ | :------------------------ | :---------------------------------- |
| `hook_event_name`   | `Literal["Notification"]` | Sempre "Notification"               |
| `message`           | `str`                     | Conteúdo da mensagem de notificação |
| `title`             | `str` (opcional)          | Título da notificação               |
| `notification_type` | `str`                     | Tipo de notificação                 |

<h3 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h3>

Dados de entrada para eventos de hook `SubagentStart`.

```python theme={null}
class SubagentStartHookInput(BaseHookInput):
    hook_event_name: Literal["SubagentStart"]
    agent_id: str
    agent_type: str
```

| Campo             | Tipo                       | Descrição                            |
| :---------------- | :------------------------- | :----------------------------------- |
| `hook_event_name` | `Literal["SubagentStart"]` | Sempre "SubagentStart"               |
| `agent_id`        | `str`                      | Identificador único para o subagente |
| `agent_type`      | `str`                      | Tipo do subagente                    |

<h3 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h3>

Dados de entrada para eventos de hook `PermissionRequest`. Permite que hooks manipulem decisões de permissão programaticamente.

```python theme={null}
class PermissionRequestHookInput(BaseHookInput):
    hook_event_name: Literal["PermissionRequest"]
    tool_name: str
    tool_input: dict[str, Any]
    permission_suggestions: NotRequired[list[Any]]
    agent_id: NotRequired[str]
    agent_type: NotRequired[str]
```

| Campo                    | Tipo                           | Descrição                                                                         |
| :----------------------- | :----------------------------- | :-------------------------------------------------------------------------------- |
| `hook_event_name`        | `Literal["PermissionRequest"]` | Sempre "PermissionRequest"                                                        |
| `tool_name`              | `str`                          | Nome da ferramenta solicitando permissão                                          |
| `tool_input`             | `dict[str, Any]`               | Parâmetros de entrada para a ferramenta                                           |
| `permission_suggestions` | `list[Any]` (opcional)         | Atualizações de permissão sugeridas do CLI                                        |
| `agent_id`               | `str` (opcional)               | Identificador de subagente, presente quando o hook dispara dentro de um subagente |
| `agent_type`             | `str` (opcional)               | Tipo de subagente, presente quando o hook dispara dentro de um subagente          |

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Tipo de união para valores de retorno de callback de hook.

```python theme={null}
HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

Saída de hook síncrona com campos de controle e decisão.

```python theme={null}
class SyncHookJSONOutput(TypedDict):
    # Control fields
    continue_: NotRequired[bool]  # Whether to proceed (default: True)
    suppressOutput: NotRequired[bool]  # Hide stdout from transcript
    stopReason: NotRequired[str]  # Message when continue is False

    # Decision fields
    decision: NotRequired[Literal["block"]]
    systemMessage: NotRequired[str]  # Warning message for user
    reason: NotRequired[str]  # Feedback for Claude

    # Hook-specific output
    hookSpecificOutput: NotRequired[HookSpecificOutput]
```

<Note>
  Use `continue_` (com underscore) no código Python. É automaticamente convertido para `continue` quando enviado para o CLI.
</Note>

<h4 id="hookspecificoutput">
  `HookSpecificOutput`
</h4>

Um `TypedDict` contendo o nome do evento de hook e campos específicos do evento. A forma depende do valor `hookEventName`. Para detalhes completos sobre campos disponíveis por evento de hook, veja [Controlar execução com hooks](/docs/pt/agent-sdk/hooks#outputs).

Uma união discriminada de tipos de saída específicos do evento. O campo `hookEventName` determina quais campos são válidos.

```python theme={null}
class PreToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PreToolUse"]
    permissionDecision: NotRequired[Literal["allow", "deny", "ask", "defer"]]
    permissionDecisionReason: NotRequired[str]
    updatedInput: NotRequired[dict[str, Any]]
    additionalContext: NotRequired[str]


class PostToolUseHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUse"]
    additionalContext: NotRequired[str]
    updatedToolOutput: NotRequired[Any]
    updatedMCPToolOutput: NotRequired[Any]  # Deprecated: use updatedToolOutput, which works for all tools


class PostToolUseFailureHookSpecificOutput(TypedDict):
    hookEventName: Literal["PostToolUseFailure"]
    additionalContext: NotRequired[str]


class UserPromptSubmitHookSpecificOutput(TypedDict):
    hookEventName: Literal["UserPromptSubmit"]
    additionalContext: NotRequired[str]


class NotificationHookSpecificOutput(TypedDict):
    hookEventName: Literal["Notification"]
    additionalContext: NotRequired[str]


class SubagentStartHookSpecificOutput(TypedDict):
    hookEventName: Literal["SubagentStart"]
    additionalContext: NotRequired[str]


class PermissionRequestHookSpecificOutput(TypedDict):
    hookEventName: Literal["PermissionRequest"]
    decision: dict[str, Any]


HookSpecificOutput = (
    PreToolUseHookSpecificOutput
    | PostToolUseHookSpecificOutput
    | PostToolUseFailureHookSpecificOutput
    | UserPromptSubmitHookSpecificOutput
    | NotificationHookSpecificOutput
    | SubagentStartHookSpecificOutput
    | PermissionRequestHookSpecificOutput
)
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

Saída de hook assíncrona que adia a execução do hook.

```python theme={null}
class AsyncHookJSONOutput(TypedDict):
    async_: Literal[True]  # Set to True to defer execution
    asyncTimeout: NotRequired[int]  # Timeout in milliseconds
```

<Note>
  Use `async_` (com underscore) no código Python. É automaticamente convertido para `async` quando enviado para o CLI.
</Note>

<h3 id="hook-usage-example">
  Exemplo de Uso de Hook
</h3>

Este exemplo registra dois hooks: um que bloqueia comandos bash perigosos como `rm -rf /`, e outro que registra todo o uso de ferramenta para auditoria. O hook de segurança funciona apenas em comandos Bash (via `matcher`), enquanto o hook de registro funciona em todas as ferramentas.

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, HookMatcher, HookContext
from typing import Any


async def validate_bash_command(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Validate and potentially block dangerous bash commands."""
    if input_data["tool_name"] == "Bash":
        command = input_data["tool_input"].get("command", "")
        if "rm -rf /" in command:
            return {
                "hookSpecificOutput": {
                    "hookEventName": "PreToolUse",
                    "permissionDecision": "deny",
                    "permissionDecisionReason": "Dangerous command blocked",
                }
            }
    return {}


async def log_tool_use(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage for auditing."""
    print(f"Tool used: {input_data.get('tool_name')}")
    return {}


options = ClaudeAgentOptions(
    hooks={
        "PreToolUse": [
            HookMatcher(
                matcher="Bash", hooks=[validate_bash_command], timeout=120
            ),  # 2 min for validation
            HookMatcher(
                hooks=[log_tool_use]
            ),  # Applies to all tools (default 60s timeout)
        ],
        "PostToolUse": [HookMatcher(hooks=[log_tool_use])],
    }
)

async for message in query(prompt="Analyze this codebase", options=options):
    print(message)
```

<h2 id="tool-input/output-types">
  Tipos de Entrada/Saída de Ferramenta
</h2>

Documentação de schemas de entrada/saída para todas as ferramentas Claude Code integradas. Embora o SDK Python não exporte esses como tipos, eles representam a estrutura de entradas e saídas de ferramenta em mensagens.

<h3 id="agent">
  Agent
</h3>

**Nome da ferramenta:** `Agent` (anteriormente `Task`, que ainda é aceito como alias)

**Entrada:**

```python theme={null}
{
    "description": str,  # Uma descrição breve (3-5 palavras) da tarefa
    "prompt": str,  # A tarefa para o agente executar
    "subagent_type": str,  # O tipo de agente especializado a usar
}
```

**Saída:**

```python theme={null}
{
    "result": str,  # Resultado final do subagente
    "usage": dict | None,  # Estatísticas de uso de tokens
    "total_cost_usd": float | None,  # Custo total estimado em USD
    "duration_ms": int | None,  # Duração da execução em milissegundos
}
```

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Nome da ferramenta:** `AskUserQuestion`

Faz perguntas de esclarecimento ao usuário durante a execução. Veja [Lidar com aprovações e entrada do usuário](/docs/pt/agent-sdk/user-input#handle-clarifying-questions) para detalhes de uso.

**Entrada:**

```python theme={null}
{
    "questions": [  # Perguntas a fazer ao usuário (1-4 perguntas)
        {
            "question": str,  # A pergunta completa a fazer ao usuário
            "header": str,  # Rótulo muito breve exibido como chip/tag (máx 12 caracteres)
            "options": [  # As escolhas disponíveis (2-4 opções)
                {
                    "label": str,  # Texto de exibição para esta opção (1-5 palavras)
                    "description": str,  # Explicação do que esta opção significa
                }
            ],
            "multiSelect": bool,  # Defina como true para permitir múltiplas seleções
        }
    ],
    "answers": dict[str, str | list[str]] | None,
    # Respostas do usuário preenchidas pelo sistema de permissões. Respostas
    # de múltipla seleção podem ser uma lista de rótulos ou uma string separada por vírgula
}
```

**Saída:**

```python theme={null}
{
    "questions": [  # As perguntas que foram feitas
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Mapeia texto da pergunta para string de resposta
    # Respostas de múltipla seleção são separadas por vírgula
}
```

<h3 id="bash">
  Bash
</h3>

**Nome da ferramenta:** `Bash`

**Entrada:**

```python theme={null}
{
    "command": str,  # O comando a executar
    "timeout": int | None,  # Tempo limite opcional em milissegundos (máx 600000; valores maiores são limitados ao máximo)
    "description": str | None,  # Descrição clara e concisa (5-10 palavras)
    "run_in_background": bool | None,  # Defina como true para executar em segundo plano
}
```

**Saída:**

```python theme={null}
{
    "output": str,  # Saída combinada de stdout e stderr
    "exitCode": int,  # Código de saída do comando
    "killed": bool | None,  # Se o comando foi interrompido devido ao tempo limite
    "shellId": str | None,  # ID do shell para processos em segundo plano
}
```

<h3 id="monitor">
  Monitor
</h3>

**Nome da ferramenta:** `Monitor`

Executa uma fonte de fundo e entrega cada evento para Claude para que ele possa reagir sem polling: `command` executa um script e emite um evento por linha stdout, e `ws` abre um WebSocket e emite um evento por frame de texto. Forneça exatamente um de `command` ou `ws`.

Quando Monitor executa um comando, ele segue as mesmas regras de permissão que Bash; uma observação de WebSocket solicita aprovação separadamente. A fonte `ws` requer Claude Code v2.1.195 ou posterior. Veja a [referência da ferramenta Monitor](/docs/pt/tools-reference#monitor-tool) para comportamento e disponibilidade de provedor.

**Entrada:**

```python theme={null}
{
    "command": str | None,  # Script de shell; cada linha stdout é um evento, exit encerra a observação
    "ws": dict | None,  # Fonte WebSocket: {"url": str, "protocols": list[str] | None}; cada frame de texto é um evento
    "description": str,  # Descrição breve mostrada em notificações
    "timeout_ms": int | None,  # Interromper após este prazo (padrão 300000, máx 3600000)
    "persistent": bool | None,  # Executar pela vida útil da sessão; parar com TaskStop
}
```

**Saída:**

```python theme={null}
{
    "taskId": str,  # ID da tarefa de monitor de fundo
    "timeoutMs": int,  # Prazo de tempo limite em milissegundos (0 quando persistente)
    "persistent": bool | None,  # True quando executando até TaskStop ou fim da sessão
}
```

<h3 id="edit">
  Edit
</h3>

**Nome da ferramenta:** `Edit`

**Entrada:**

```python theme={null}
{
    "file_path": str,  # O caminho absoluto do arquivo a modificar
    "old_string": str,  # O texto a substituir
    "new_string": str,  # O texto para substituir por
    "replace_all": bool | None,  # Substituir todas as ocorrências (padrão False)
}
```

**Saída:**

```python theme={null}
{
    "message": str,  # Mensagem de confirmação
    "replacements": int,  # Número de substituições realizadas
    "file_path": str,  # Caminho do arquivo que foi editado
}
```

<h3 id="read">
  Read
</h3>

**Nome da ferramenta:** `Read`

**Entrada:**

```python theme={null}
{
    "file_path": str,  # O caminho absoluto do arquivo a ler
    "offset": int | None,  # O número da linha para começar a ler
    "limit": int | None,  # O número de linhas a ler
}
```

**Saída (Arquivos de texto):**

```python theme={null}
{
    "content": str,  # Conteúdo do arquivo com números de linha
    "total_lines": int,  # Número total de linhas no arquivo
    "lines_returned": int,  # Linhas realmente retornadas
}
```

**Saída (Imagens):**

```python theme={null}
{
    "image": str,  # Dados de imagem codificados em Base64
    "mime_type": str,  # Tipo MIME da imagem
    "file_size": int,  # Tamanho do arquivo em bytes
}
```

<h3 id="write">
  Write
</h3>

**Nome da ferramenta:** `Write`

**Entrada:**

```python theme={null}
{
    "file_path": str,  # O caminho absoluto do arquivo a escrever
    "content": str,  # O conteúdo a escrever no arquivo
}
```

**Saída:**

```python theme={null}
{
    "message": str,  # Mensagem de sucesso
    "bytes_written": int,  # Número de bytes escritos
    "file_path": str,  # Caminho do arquivo que foi escrito
}
```

<h3 id="glob">
  Glob
</h3>

**Nome da ferramenta:** `Glob`

**Entrada:**

```python theme={null}
{
    "pattern": str,  # O padrão glob para corresponder arquivos
    "path": str | None,  # O diretório a pesquisar (padrão cwd)
}
```

**Saída:**

```python theme={null}
{
    "matches": list[str],  # Array de caminhos de arquivo correspondentes
    "count": int,  # Número de correspondências encontradas
    "search_path": str,  # Diretório de pesquisa usado
}
```

<h3 id="grep">
  Grep
</h3>

**Nome da ferramenta:** `Grep`

**Entrada:**

```python theme={null}
{
    "pattern": str,  # O padrão de expressão regular
    "path": str | None,  # Arquivo ou diretório a pesquisar
    "glob": str | None,  # Padrão glob para filtrar arquivos
    "type": str | None,  # Tipo de arquivo a pesquisar
    "output_mode": str | None,  # "content", "files_with_matches", ou "count"
    "-i": bool | None,  # Pesquisa insensível a maiúsculas/minúsculas
    "-n": bool | None,  # Mostrar números de linha
    "-B": int | None,  # Linhas a mostrar antes de cada correspondência
    "-A": int | None,  # Linhas a mostrar após cada correspondência
    "-C": int | None,  # Linhas a mostrar antes e depois
    "head_limit": int | None,  # Limitar saída às primeiras N linhas/entradas
    "multiline": bool | None,  # Ativar modo multilinha
}
```

**Saída (modo content):**

```python theme={null}
{
    "matches": [
        {
            "file": str,
            "line_number": int | None,
            "line": str,
            "before_context": list[str] | None,
            "after_context": list[str] | None,
        }
    ],
    "total_matches": int,
}
```

**Saída (modo files\_with\_matches):**

```python theme={null}
{
    "files": list[str],  # Arquivos contendo correspondências
    "count": int,  # Número de arquivos com correspondências
}
```

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Nome da ferramenta:** `NotebookEdit`

**Entrada:**

```python theme={null}
{
    "notebook_path": str,  # Caminho absoluto para o notebook Jupyter
    "cell_id": str | None,  # O ID da célula a editar
    "new_source": str,  # A nova fonte para a célula
    "cell_type": "code" | "markdown" | None,  # O tipo da célula
    "edit_mode": "replace" | "insert" | "delete" | None,  # Tipo de operação de edição
}
```

**Saída:**

```python theme={null}
{
    "message": str,  # Mensagem de sucesso
    "edit_type": "replaced" | "inserted" | "deleted",  # Tipo de edição realizada
    "cell_id": str | None,  # ID da célula que foi afetada
    "total_cells": int,  # Total de células no notebook após edição
}
```

<h3 id="webfetch">
  WebFetch
</h3>

**Nome da ferramenta:** `WebFetch`

**Entrada:**

```python theme={null}
{
    "url": str,  # A URL para buscar conteúdo
    "prompt": str,  # O prompt a executar no conteúdo buscado
}
```

**Saída:**

```python theme={null}
{
    "bytes": int,  # Tamanho do conteúdo buscado em bytes
    "code": int,  # Código de resposta HTTP
    "codeText": str,  # Texto do código de resposta HTTP
    "result": str,  # Resultado processado da aplicação do prompt ao conteúdo
    "durationMs": int,  # Tempo para buscar e processar o conteúdo, em milissegundos
    "url": str,  # URL que foi buscada
}
```

<h3 id="websearch">
  WebSearch
</h3>

**Nome da ferramenta:** `WebSearch`

**Entrada:**

```python theme={null}
{
    "query": str,  # A consulta de pesquisa a usar
    "allowed_domains": list[str] | None,  # Incluir apenas resultados desses domínios
    "blocked_domains": list[str] | None,  # Nunca incluir resultados desses domínios
}
```

**Saída:**

```python theme={null}
{
    "query": str,  # A consulta de pesquisa
    "results": list[str | {"tool_use_id": str, "content": list[{"title": str, "url": str}]}],
    "durationSeconds": float,  # Duração da pesquisa em segundos
}
```

<h3 id="todowrite">
  TodoWrite
</h3>

**Nome da ferramenta:** `TodoWrite`

<Note>
  A partir do Claude Code v2.1.142, `TodoWrite` está desabilitado por padrão. Use `TaskCreate`, `TaskGet`, `TaskUpdate` e `TaskList` em seu lugar. Veja [Migrar para ferramentas Task](/docs/pt/agent-sdk/todo-tracking#migrate-to-task-tools) para atualizar seu código de monitoramento, ou defina `CLAUDE_CODE_ENABLE_TASKS=0` para reverter para `TodoWrite`.
</Note>

**Entrada:**

```python theme={null}
{
    "todos": [
        {
            "content": str,  # A descrição da tarefa
            "status": "pending" | "in_progress" | "completed",  # Status da tarefa
            "activeForm": str,  # Forma ativa da descrição
        }
    ]
}
```

**Saída:**

```python theme={null}
{
    "message": str,  # Mensagem de sucesso
    "stats": {"total": int, "pending": int, "in_progress": int, "completed": int},
}
```

<h3 id="taskcreate">
  TaskCreate
</h3>

**Nome da ferramenta:** `TaskCreate`

**Entrada:**

```python theme={null}
{
    "subject": str,  # Título breve da tarefa
    "description": str,  # Corpo detalhado da tarefa
    "activeForm": str | None,  # Rótulo em tempo presente mostrado enquanto em progresso
    "metadata": dict | None,  # Metadados arbitrários do chamador
}
```

**Saída:**

```python theme={null}
{
    "task": {"id": str, "subject": str},  # Tarefa criada com ID atribuído
}
```

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Nome da ferramenta:** `TaskUpdate`

**Entrada:**

```python theme={null}
{
    "taskId": str,  # ID da tarefa a corrigir
    "status": Literal["pending", "in_progress", "completed", "deleted"] | None,
    "subject": str | None,
    "description": str | None,
    "activeForm": str | None,
    "addBlocks": list[str] | None,  # IDs de tarefas que esta tarefa agora bloqueia
    "addBlockedBy": list[str] | None,  # IDs de tarefas que agora bloqueiam esta tarefa
    "owner": str | None,
    "metadata": dict | None,
}
```

**Saída:**

```python theme={null}
{
    "success": bool,
    "taskId": str,
    "updatedFields": list[str],  # Nomes dos campos que mudaram
    "error": str | None,
    "statusChange": {"from": str, "to": str} | None,
}
```

<h3 id="taskget">
  TaskGet
</h3>

**Nome da ferramenta:** `TaskGet`

**Entrada:**

```python theme={null}
{
    "taskId": str,  # ID da tarefa a ler
}
```

**Saída:**

```python theme={null}
{
    "task": {
        "id": str,
        "subject": str,
        "description": str,
        "status": Literal["pending", "in_progress", "completed"],
        "blocks": list[str],
        "blockedBy": list[str],
    } | None,  # None quando o ID não é encontrado
}
```

<h3 id="tasklist">
  TaskList
</h3>

**Nome da ferramenta:** `TaskList`

**Entrada:**

```python theme={null}
{}
```

**Saída:**

```python theme={null}
{
    "tasks": [
        {
            "id": str,
            "subject": str,
            "status": Literal["pending", "in_progress", "completed"],
            "owner": str | None,
            "blockedBy": list[str],
        }
    ],
}
```

<h3 id="bashoutput">
  BashOutput
</h3>

**Nome da ferramenta:** `BashOutput`

**Entrada:**

```python theme={null}
{
    "bash_id": str,  # O ID do shell de fundo
    "filter": str | None,  # Regex opcional para filtrar linhas de saída
}
```

**Saída:**

```python theme={null}
{
    "output": str,  # Nova saída desde a última verificação
    "status": "running" | "completed" | "failed",  # Status atual do shell
    "exitCode": int | None,  # Código de saída quando concluído
}
```

<h3 id="killbash">
  KillBash
</h3>

**Nome da ferramenta:** `KillBash`

**Entrada:**

```python theme={null}
{
    "shell_id": str  # O ID do shell de fundo a interromper
}
```

**Saída:**

```python theme={null}
{
    "message": str,  # Mensagem de sucesso
    "shell_id": str,  # ID do shell interrompido
}
```

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nome da ferramenta:** `ExitPlanMode`

**Entrada:**

```python theme={null}
{
    "plan": str  # O plano a executar pelo usuário para aprovação
}
```

**Saída:**

```python theme={null}
{
    "message": str,  # Mensagem de confirmação
    "approved": bool | None,  # Se o usuário aprovou o plano
}
```

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nome da ferramenta:** `ListMcpResourcesTool`

**Entrada:**

```python theme={null}
{
    "server": str | None  # Nome de servidor opcional para filtrar recursos por
}
```

**Saída:**

```python theme={null}
{
    "resources": [
        {
            "uri": str,
            "name": str,
            "description": str | None,
            "mimeType": str | None,
            "server": str,
        }
    ],
    "total": int,
}
```

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Nome da ferramenta:** `ReadMcpResourceTool`

**Entrada:**

```python theme={null}
{
    "server": str,  # O nome do servidor MCP
    "uri": str,  # A URI do recurso a ler
}
```

**Saída:**

```python theme={null}
{
    "contents": [
        {"uri": str, "mimeType": str | None, "text": str | None, "blob": str | None}
    ],
    "server": str,
}
```

<h2 id="advanced-features-with-claudesdkclient">
  Recursos Avançados com ClaudeSDKClient
</h2>

<h3 id="building-a-continuous-conversation-interface">
  Construindo uma Interface de Conversa Contínua
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    TextBlock,
)
import asyncio


class ConversationSession:
    """Maintains a single conversation session with Claude."""

    def __init__(self, options: ClaudeAgentOptions | None = None):
        self.client = ClaudeSDKClient(options)
        self.turn_count = 0

    async def start(self):
        await self.client.connect()
        print("Starting conversation session. Claude will remember context.")
        print(
            "Commands: 'exit' to quit, 'interrupt' to stop current task, 'new' for new session"
        )

        while True:
            user_input = input(f"\n[Turn {self.turn_count + 1}] You: ")

            if user_input.lower() == "exit":
                break
            elif user_input.lower() == "interrupt":
                await self.client.interrupt()
                print("Task interrupted!")
                continue
            elif user_input.lower() == "new":
                # Disconnect and reconnect for a fresh session
                await self.client.disconnect()
                await self.client.connect()
                self.turn_count = 0
                print("Started new conversation session (previous context cleared)")
                continue

            # Send message - the session retains all previous messages
            await self.client.query(user_input)
            self.turn_count += 1

            # Process response
            print(f"[Turn {self.turn_count}] Claude: ", end="")
            async for message in self.client.receive_response():
                if isinstance(message, AssistantMessage):
                    for block in message.content:
                        if isinstance(block, TextBlock):
                            print(block.text, end="")
            print()  # New line after response

        await self.client.disconnect()
        print(f"Conversation ended after {self.turn_count} turns.")


async def main():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"], permission_mode="acceptEdits"
    )
    session = ConversationSession(options)
    await session.start()


# Example conversation:
# Turn 1 - You: "Create a file called hello.py"
# Turn 1 - Claude: "I'll create a hello.py file for you..."
# Turn 2 - You: "What's in that file?"
# Turn 2 - Claude: "The hello.py file I just created contains..." (remembers!)
# Turn 3 - You: "Add a main function to it"
# Turn 3 - Claude: "I'll add a main function to hello.py..." (knows which file!)

asyncio.run(main())
```

<h3 id="using-hooks-for-behavior-modification">
  Usando Hooks para Modificação de Comportamento
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    HookMatcher,
    HookContext,
)
import asyncio
from typing import Any


async def pre_tool_logger(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log all tool usage before execution."""
    tool_name = input_data.get("tool_name", "unknown")
    print(f"[PRE-TOOL] About to use: {tool_name}")

    # You can modify or block the tool execution here
    if tool_name == "Bash" and "rm -rf" in str(input_data.get("tool_input", {})):
        return {
            "hookSpecificOutput": {
                "hookEventName": "PreToolUse",
                "permissionDecision": "deny",
                "permissionDecisionReason": "Dangerous command blocked",
            }
        }
    return {}


async def post_tool_logger(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Log results after tool execution."""
    tool_name = input_data.get("tool_name", "unknown")
    print(f"[POST-TOOL] Completed: {tool_name}")
    return {}


async def user_prompt_modifier(
    input_data: dict[str, Any], tool_use_id: str | None, context: HookContext
) -> dict[str, Any]:
    """Add context to user prompts."""
    original_prompt = input_data.get("prompt", "")

    # Add a timestamp as additional context for Claude to see
    from datetime import datetime

    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")

    return {
        "hookSpecificOutput": {
            "hookEventName": "UserPromptSubmit",
            "additionalContext": f"[Submitted at {timestamp}] Original prompt: {original_prompt}",
        }
    }


async def main():
    options = ClaudeAgentOptions(
        hooks={
            "PreToolUse": [
                HookMatcher(hooks=[pre_tool_logger]),
                HookMatcher(matcher="Bash", hooks=[pre_tool_logger]),
            ],
            "PostToolUse": [HookMatcher(hooks=[post_tool_logger])],
            "UserPromptSubmit": [HookMatcher(hooks=[user_prompt_modifier])],
        },
        allowed_tools=["Read", "Write", "Bash"],
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("List files in current directory")

        async for message in client.receive_response():
            # Hooks will automatically log tool usage
            pass


asyncio.run(main())
```

<h3 id="real-time-progress-monitoring">
  Monitoramento de Progresso em Tempo Real
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    AssistantMessage,
    ToolUseBlock,
    ToolResultBlock,
    TextBlock,
)
import asyncio


async def monitor_progress():
    options = ClaudeAgentOptions(
        allowed_tools=["Write", "Bash"], permission_mode="acceptEdits"
    )

    async with ClaudeSDKClient(options=options) as client:
        await client.query("Create 5 Python files with different sorting algorithms")

        # Monitor progress in real-time
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, ToolUseBlock):
                        if block.name == "Write":
                            file_path = block.input.get("file_path", "")
                            print(f"Creating: {file_path}")
                    elif isinstance(block, ToolResultBlock):
                        print("Completed tool execution")
                    elif isinstance(block, TextBlock):
                        print(f"Claude says: {block.text[:100]}...")

        print("Task completed!")


asyncio.run(monitor_progress())
```

<h2 id="example-usage">
  Uso de Exemplo
</h2>

<h3 id="basic-file-operations-using-query">
  Operações básicas de arquivo (usando query)
</h3>

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, AssistantMessage, ToolUseBlock
import asyncio


async def create_project():
    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Bash"],
        permission_mode="acceptEdits",
        cwd="/home/user/project",
    )

    async for message in query(
        prompt="Create a Python project structure with setup.py", options=options
    ):
        if isinstance(message, AssistantMessage):
            for block in message.content:
                if isinstance(block, ToolUseBlock):
                    print(f"Using tool: {block.name}")


asyncio.run(create_project())
```

<h3 id="error-handling">
  Tratamento de erros
</h3>

```python theme={null}
from claude_agent_sdk import query, CLINotFoundError, ProcessError, CLIJSONDecodeError

try:
    async for message in query(prompt="Hello"):
        print(message)
except CLINotFoundError:
    print(
        "Claude Code CLI not found. Try reinstalling: pip install --force-reinstall claude-agent-sdk"
    )
except ProcessError as e:
    print(f"Process failed with exit code: {e.exit_code}")
except CLIJSONDecodeError as e:
    print(f"Failed to parse response: {e}")
```

<h3 id="streaming-mode-with-client">
  Modo de streaming com cliente
</h3>

```python theme={null}
from claude_agent_sdk import ClaudeSDKClient
import asyncio


async def interactive_session():
    async with ClaudeSDKClient() as client:
        # Send initial message
        await client.query("What's the weather like?")

        # Process responses
        async for msg in client.receive_response():
            print(msg)

        # Send follow-up
        await client.query("Tell me more about that")

        # Process follow-up response
        async for msg in client.receive_response():
            print(msg)


asyncio.run(interactive_session())
```

<h3 id="using-custom-tools-with-claudesdkclient">
  Usando ferramentas personalizadas com ClaudeSDKClient
</h3>

```python theme={null}
from claude_agent_sdk import (
    ClaudeSDKClient,
    ClaudeAgentOptions,
    tool,
    create_sdk_mcp_server,
    AssistantMessage,
    TextBlock,
)
import asyncio
from typing import Any


# Define custom tools with @tool decorator
@tool("calculate", "Perform mathematical calculations", {"expression": str})
async def calculate(args: dict[str, Any]) -> dict[str, Any]:
    try:
        result = eval(args["expression"], {"__builtins__": {}})
        return {"content": [{"type": "text", "text": f"Result: {result}"}]}
    except Exception as e:
        return {
            "content": [{"type": "text", "text": f"Error: {str(e)}"}],
            "is_error": True,
        }


@tool("get_time", "Get current time", {})
async def get_time(args: dict[str, Any]) -> dict[str, Any]:
    from datetime import datetime

    current_time = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    return {"content": [{"type": "text", "text": f"Current time: {current_time}"}]}


async def main():
    # Create SDK MCP server with custom tools
    my_server = create_sdk_mcp_server(
        name="utilities", version="1.0.0", tools=[calculate, get_time]
    )

    # Configure options with the server
    options = ClaudeAgentOptions(
        mcp_servers={"utils": my_server},
        allowed_tools=["mcp__utils__calculate", "mcp__utils__get_time"],
    )

    # Use ClaudeSDKClient for interactive tool usage
    async with ClaudeSDKClient(options=options) as client:
        await client.query("What's 123 * 456?")

        # Process calculation response
        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Calculation: {block.text}")

        # Follow up with time query
        await client.query("What time is it now?")

        async for message in client.receive_response():
            if isinstance(message, AssistantMessage):
                for block in message.content:
                    if isinstance(block, TextBlock):
                        print(f"Time: {block.text}")


asyncio.run(main())
```

<h2 id="sandbox-configuration">
  Configuração de Sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Configuração para comportamento de sandbox. Use isso para ativar sandboxing de comando e configurar restrições de rede programaticamente.

```python theme={null}
class SandboxSettings(TypedDict, total=False):
    enabled: bool
    autoAllowBashIfSandboxed: bool
    excludedCommands: list[str]
    allowUnsandboxedCommands: bool
    network: SandboxNetworkConfig
    ignoreViolations: SandboxIgnoreViolations
    enableWeakerNestedSandbox: bool
```

| Propriedade                 | Tipo                                                  | Padrão  | Descrição                                                                                                                                                                                                                                            |
| :-------------------------- | :---------------------------------------------------- | :------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `bool`                                                | `False` | Ativa modo sandbox para execução de comando                                                                                                                                                                                                          |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`  | Auto-aprova comandos bash quando sandbox está ativado                                                                                                                                                                                                |
| `excludedCommands`          | `list[str]`                                           | `[]`    | Comandos que sempre contornam restrições de sandbox (por exemplo, `["docker"]`). Esses executam sem sandbox automaticamente sem envolvimento do modelo                                                                                               |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`  | Permite que o modelo solicite executar comandos fora do sandbox. Quando `True`, o modelo pode definir `dangerouslyDisableSandbox` na entrada da ferramenta, que volta para o [sistema de permissões](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`  | Configuração de sandbox específica de rede                                                                                                                                                                                                           |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`  | Configure quais violações de sandbox ignorar                                                                                                                                                                                                         |
| `enableWeakerNestedSandbox` | `bool`                                                | `False` | Ativa um sandbox aninhado mais fraco para compatibilidade                                                                                                                                                                                            |

<Note>
  O sandbox depende do suporte de plataforma e, no Linux, de ferramentas como `bubblewrap` e `socat`. Por padrão, quando `enabled` é `True` mas o sandbox não consegue iniciar, comandos executam sem sandbox com um aviso em stderr. Este padrão difere do SDK TypeScript, onde `failIfUnavailable` tem padrão `true`.

  Defina `"failIfUnavailable": True` nas suas configurações de sandbox para parar em vez disso. A chave ainda não está declarada em `SandboxSettings`, mas o SDK a encaminha para Claude Code, que a honra. `query()` então relata uma `ResultMessage` com `subtype="error_during_execution"` e a razão em `errors`. Observe esse subtipo em vez de esperar que `query()` lance antes de ceder mensagens.
</Note>

<h4 id="example-usage-2">
  Exemplo de uso
</h4>

```python theme={null}
from claude_agent_sdk import query, ClaudeAgentOptions, SandboxSettings

sandbox_settings: SandboxSettings = {
    "enabled": True,
    "autoAllowBashIfSandboxed": True,
    "network": {"allowLocalBinding": True},
}

async for message in query(
    prompt="Build and test my project",
    options=ClaudeAgentOptions(sandbox=sandbox_settings),
):
    print(message)
```

<Warning>
  **Segurança de socket Unix**: A opção `allowUnixSockets` pode conceder acesso a serviços de sistema poderosos. Por exemplo, permitir `/var/run/docker.sock` efetivamente concede acesso completo ao sistema host através da API Docker, contornando isolamento de sandbox. Apenas permita sockets Unix que são estritamente necessários e entenda as implicações de segurança de cada um.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configuração específica de rede para modo sandbox. Essas configurações se aplicam a comandos Bash em sandbox quando `enabled` é `True` na [`SandboxSettings`](#sandboxsettings) pai. Elas não restringem a ferramenta WebFetch, que usa [regras de permissão](/docs/pt/permissions#webfetch) em vez disso.

```python theme={null}
class SandboxNetworkConfig(TypedDict, total=False):
    allowedDomains: list[str]
    deniedDomains: list[str]
    allowManagedDomainsOnly: bool
    allowUnixSockets: list[str]
    allowAllUnixSockets: bool
    allowLocalBinding: bool
    allowMachLookup: list[str]
    httpProxyPort: int
    socksProxyPort: int
```

| Propriedade               | Tipo        | Padrão  | Descrição                                                                                                                                                                                         |
| :------------------------ | :---------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `allowedDomains`          | `list[str]` | `[]`    | Nomes de domínio que processos em sandbox podem acessar                                                                                                                                           |
| `deniedDomains`           | `list[str]` | `[]`    | Nomes de domínio que processos em sandbox não podem acessar. Tem precedência sobre `allowedDomains`                                                                                               |
| `allowManagedDomainsOnly` | `bool`      | `False` | Apenas configurações gerenciadas: quando definido em configurações gerenciadas, ignore `allowedDomains` de fontes de configurações não gerenciadas. Não tem efeito quando definido via opções SDK |
| `allowUnixSockets`        | `list[str]` | `[]`    | Caminhos de socket Unix que processos podem acessar (por exemplo, socket Docker)                                                                                                                  |
| `allowAllUnixSockets`     | `bool`      | `False` | Permite acesso a todos os sockets Unix                                                                                                                                                            |
| `allowLocalBinding`       | `bool`      | `False` | Permite que processos se vinculem a portas locais (por exemplo, para servidores dev)                                                                                                              |
| `allowMachLookup`         | `list[str]` | `[]`    | Apenas macOS: nomes de serviço XPC/Mach para permitir. Suporta um curinga à direita                                                                                                               |
| `httpProxyPort`           | `int`       | `None`  | Porta de proxy HTTP para solicitações de rede                                                                                                                                                     |
| `socksProxyPort`          | `int`       | `None`  | Porta de proxy SOCKS para solicitações de rede                                                                                                                                                    |

<Note>
  O proxy de sandbox integrado aplica a lista de permissões de rede com base no nome de host solicitado e não encerra ou inspeciona tráfego TLS, portanto técnicas como [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) podem potencialmente contorná-lo. Veja [Limitações de segurança de Sandboxing](/docs/pt/sandboxing#security-limitations) para detalhes e [Implantação segura](/docs/pt/agent-sdk/secure-deployment#traffic-forwarding) para configurar um proxy que encerra TLS.
</Note>

<h3 id="sandboxignoreviolations">
  `SandboxIgnoreViolations`
</h3>

Configuração para ignorar violações de sandbox específicas.

```python theme={null}
class SandboxIgnoreViolations(TypedDict, total=False):
    file: list[str]
    network: list[str]
```

| Propriedade | Tipo        | Padrão | Descrição                                            |
| :---------- | :---------- | :----- | :--------------------------------------------------- |
| `file`      | `list[str]` | `[]`   | Padrões de caminho de arquivo para ignorar violações |
| `network`   | `list[str]` | `[]`   | Padrões de rede para ignorar violações               |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback de Permissões para Comandos Sem Sandbox
</h3>

Quando `allowUnsandboxedCommands` está ativado, o modelo pode solicitar executar comandos fora do sandbox definindo `dangerouslyDisableSandbox: True` na entrada da ferramenta. Essas solicitações voltam para o sistema de permissões existente, significando que seu manipulador `can_use_tool` será invocado, permitindo que você implemente lógica de autorização personalizada.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Uma lista estática de comandos que sempre contornam o sandbox automaticamente (por exemplo, `["docker"]`). O modelo não tem controle sobre isso.
  * `allowUnsandboxedCommands`: Permite que o modelo decida em tempo de execução se deve solicitar execução sem sandbox definindo `dangerouslyDisableSandbox: True` na entrada da ferramenta.
</Note>

```python theme={null}
from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    HookMatcher,
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


async def can_use_tool(
    tool: str, input: dict, context: ToolPermissionContext
) -> PermissionResultAllow | PermissionResultDeny:
    # Check if the model is requesting to bypass the sandbox
    if tool == "Bash" and input.get("dangerouslyDisableSandbox"):
        # The model is requesting to run this command outside the sandbox
        print(f"Unsandboxed command requested: {input.get('command')}")

        if is_command_authorized(input.get("command")):
            return PermissionResultAllow()
        return PermissionResultDeny(
            message="Command not authorized for unsandboxed execution"
        )
    return PermissionResultAllow()


# Required: dummy hook keeps the stream open for can_use_tool
async def dummy_hook(input_data, tool_use_id, context):
    return {"continue_": True}


async def prompt_stream():
    yield {
        "type": "user",
        "message": {"role": "user", "content": "Deploy my application"},
    }


async def main():
    async for message in query(
        prompt=prompt_stream(),
        options=ClaudeAgentOptions(
            sandbox={
                "enabled": True,
                "allowUnsandboxedCommands": True,  # Model can request unsandboxed execution
            },
            permission_mode="default",
            can_use_tool=can_use_tool,
            hooks={"PreToolUse": [HookMatcher(matcher=None, hooks=[dummy_hook])]},
        ),
    ):
        print(message)
```

Este padrão permite que você:

* **Audite solicitações de modelo**: Registre quando o modelo solicita execução sem sandbox
* **Implemente listas de permissão**: Apenas permita comandos específicos executarem sem sandbox
* **Adicione fluxos de trabalho de aprovação**: Exija autorização explícita para operações privilegiadas

<Warning>
  Comandos executando com `dangerouslyDisableSandbox: True` têm acesso completo ao sistema. Certifique-se de que seu manipulador `can_use_tool` valida essas solicitações cuidadosamente.

  Se `permission_mode` está definido para `bypassPermissions` e `allow_unsandboxed_commands` está ativado, o modelo pode autonomamente executar comandos fora do sandbox sem qualquer prompt de aprovação. Esta combinação efetivamente permite que o modelo escape do isolamento de sandbox silenciosamente.
</Warning>

<h2 id="see-also">
  Veja também
</h2>

* [SDK overview](/docs/pt/agent-sdk/overview) - Conceitos gerais do SDK
* [TypeScript SDK reference](/docs/pt/agent-sdk/typescript) - Documentação do SDK TypeScript
* [CLI reference](/docs/pt/cli-reference) - Interface de linha de comando
* [Common workflows](/docs/pt/common-workflows) - Guias passo a passo
