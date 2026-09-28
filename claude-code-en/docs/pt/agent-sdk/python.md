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

Para uv, Windows PowerShell e configuração de chave de API, consulte [Configuração no guia de início rápido do Agent SDK](/docs/pt/agent-sdk/quickstart#setup).

<h2 id="choosing-between-query-and-claudesdkclient">
  Escolhendo entre `query()` e `ClaudeSDKClient`
</h2>

O SDK Python fornece duas maneiras de interagir com Claude Code:

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

Use `ClaudeSDKClient` para aplicações interativas, como interfaces de chat, ou quando a próxima ação depende da resposta de Claude.

<h2 id="functions">
  Funções
</h2>

<Note>Blocos de assinatura e fragmentos `async for` / `async with` nus nesta página são ilustrativos. Para executá-los, envolva o corpo em `async def main(): ...` e chame `asyncio.run(main())`.</Note>

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

| Parâmetro      | Tipo                                            | Descrição                                                                                                          |
| :------------- | :---------------------------------------------- | :----------------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Identificador único para a ferramenta                                                                              |
| `description`  | `str`                                           | Descrição legível por humanos do que a ferramenta faz                                                              |
| `input_schema` | `type \| dict[str, Any]`                        | Schema definindo os parâmetros de entrada da ferramenta. Veja [Opções de schema de entrada](#input-schema-options) |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Anotações MCP opcionais fornecendo dicas de comportamento aos clientes                                             |

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

Dicas de comportamento para uma ferramenta, passadas como o argumento `annotations` de [`tool()`](#tool). `ToolAnnotations` estende o `mcp.types.ToolAnnotations` do SDK MCP com um campo `maxResultSizeChars`, e você pode escrever cada dica em camelCase ou snake\_case: `ToolAnnotations(readOnlyHint=True)` e `ToolAnnotations(read_only_hint=True)` são equivalentes. Você também pode passar um `mcp.types.ToolAnnotations` simples onde o SDK aceita anotações.

Os nomes snake\_case e o campo `maxResultSizeChars` tipado requerem Python Agent SDK 0.2.140 ou posterior. As versões 0.1.31 a 0.2.139 re-exportam `mcp.types.ToolAnnotations` inalterado. Nas versões 0.1.55 a 0.2.139 você ainda pode passar `maxResultSizeChars` como um argumento de palavra-chave: a classe MCP aceita campos extras, e o SDK encaminha o valor para Claude Code.

Todos os campos são opcionais. Os clientes não devem confiar nas dicas para decisões de segurança.

| Campo                | Tipo           | Padrão  | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :------------------- | :------------- | :------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`              | `str \| None`  | `None`  | Título legível por humanos para a ferramenta                                                                                                                                                                                                                                                                                                                                                                                           |
| `readOnlyHint`       | `bool \| None` | `False` | Se `True`, a ferramenta não modifica seu ambiente                                                                                                                                                                                                                                                                                                                                                                                      |
| `destructiveHint`    | `bool \| None` | `True`  | Se `True`, a ferramenta pode realizar atualizações destrutivas (apenas significativo quando `readOnlyHint` é `False`)                                                                                                                                                                                                                                                                                                                  |
| `idempotentHint`     | `bool \| None` | `False` | Se `True`, chamadas repetidas com os mesmos argumentos não têm efeito adicional (apenas significativo quando `readOnlyHint` é `False`)                                                                                                                                                                                                                                                                                                 |
| `openWorldHint`      | `bool \| None` | `True`  | Se `True`, a ferramenta interage com entidades externas (por exemplo, busca na web). Se `False`, o domínio da ferramenta é fechado (por exemplo, uma ferramenta de memória)                                                                                                                                                                                                                                                            |
| `maxResultSizeChars` | `int \| None`  | `None`  | Número de caracteres até o qual Claude Code mantém o resultado de texto desta ferramenta inline na conversa em vez de salvá-lo em um arquivo, até 500.000. Resultados que contêm imagens não são afetados. Uma configuração de Claude Code em vez de uma dica MCP: o SDK a envia no `_meta` da ferramenta como `anthropic/maxResultSizeChars`. Veja [Raise the limit for a specific tool](/docs/pt/mcp#raise-the-limit-for-a-specific-tool) |

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
from claude_agent_sdk import tool, create_sdk_mcp_server, ClaudeAgentOptions


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

| Propriedade          | Tipo                           | Descrição                                                                                                                                                                                                                                                                                    |
| :------------------- | :----------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `Literal["user", "assistant"]` | Papel da mensagem                                                                                                                                                                                                                                                                            |
| `uuid`               | `str`                          | Identificador único de mensagem                                                                                                                                                                                                                                                              |
| `session_id`         | `str`                          | Identificador de sessão                                                                                                                                                                                                                                                                      |
| `message`            | `Any`                          | Conteúdo bruto da mensagem                                                                                                                                                                                                                                                                   |
| `parent_tool_use_id` | `str \| None`                  | Para mensagens de subagente, o id do bloco de uso de ferramenta `Agent` que o gerou. `None` para mensagens de sessão principal e sessões mais antigas                                                                                                                                        |
| `parent_agent_id`    | `str \| None`                  | Para mensagens de um [subagente aninhado](/docs/pt/sub-agents#let-subagents-spawn-their-own-subagents), o id do agente do subagente pai. `None` para mensagens de sessão principal, mensagens de subagente de nível superior e sessões mais antigas. Requer Python Agent SDK 0.2.140 ou posterior |

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

# Tag the most recent session
sessions = list_sessions(directory="/path/to/project", limit=1)
if sessions:
    tag_session(sessions[0].session_id, "needs-review")

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

**Mantém uma sessão de conversa em múltiplas trocas.** Este é o equivalente Python de como a função `query()` do SDK TypeScript funciona internamente - cria um objeto cliente que pode continuar conversas. Veja a [comparação com `query()`](#choosing-between-query-and-claudesdkclient).

```python theme={null}
class ClaudeSDKClient:
    def __init__(self, options: ClaudeAgentOptions | None = None, transport: Transport | None = None)
    async def connect(self, prompt: str | AsyncIterable[dict] | None = None) -> None
    async def query(self, prompt: str | AsyncIterable[dict], session_id: str = "default") -> None
    async def receive_messages(self) -> AsyncIterator[Message]
    async def receive_response(self) -> AsyncIterator[Message]
    async def interrupt(self) -> None
    async def set_permission_mode(self, mode: PermissionMode) -> None
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
| `set_model(model)`                        | Altera o modelo para a sessão atual. Passe `None` para redefinir para o [modelo padrão do Claude Code](/docs/pt/model-config)                                                    |
| `rewind_files(user_message_id)`           | Restaura arquivos para seu estado na mensagem de usuário especificada. Requer `enable_file_checkpointing=True`. Veja [File checkpointing](/docs/pt/agent-sdk/file-checkpointing) |
| `get_mcp_status()`                        | Obtém o status de todos os servidores MCP configurados. Retorna [`McpStatusResponse`](#mcpstatusresponse)                                                                   |
| `reconnect_mcp_server(server_name)`       | Tenta reconectar a um servidor MCP que falhou ou foi desconectado                                                                                                           |
| `toggle_mcp_server(server_name, enabled)` | Ativa ou desativa um servidor MCP no meio da sessão. Desativar remove suas ferramentas                                                                                      |
| `stop_task(task_id)`                      | Para uma tarefa de fundo em execução. Uma [`TaskNotificationMessage`](#tasknotificationmessage) com status `"stopped"` segue no fluxo de mensagens                          |
| `get_server_info()`                       | Obtém as informações de inicialização do servidor, incluindo comandos disponíveis e estilos de saída                                                                        |
| `disconnect()`                            | Desconecta de Claude                                                                                                                                                        |

<h4 id="context-manager-support">
  Suporte a Gerenciador de Contexto
</h4>

O cliente pode ser usado como um gerenciador de contexto assíncrono para gerenciamento automático de conexão:

```python theme={null}
import asyncio
from claude_agent_sdk import ClaudeSDKClient


async def main():
    async with ClaudeSDKClient() as client:
        await client.query("Hello Claude")
        async for message in client.receive_response():
            print(message)


asyncio.run(main())
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
                print(f"Interrupted task: terminal_reason={message.terminal_reason!r}")
                # terminal_reason is "aborted_streaming" or "aborted_tools"
                # for interrupted turns

        # Send a new command
        await client.query("Just say hello instead")

        # Now receive the new response
        async for message in client.receive_response():
            if isinstance(message, ResultMessage) and message.subtype == "success":
                print(f"New result: {message.result}")


asyncio.run(interruptible_task())
```

<Note>
  **Comportamento do buffer após interrupção:** `interrupt()` envia um sinal de parada mas não limpa o buffer de mensagens. Mensagens já produzidas pela tarefa interrompida, incluindo sua `ResultMessage`, permanecem no fluxo. Você deve drená-las com `receive_response()` antes de ler a resposta a uma nova consulta. Se você enviar uma nova consulta imediatamente após `interrupt()` e chamar `receive_response()` apenas uma vez, você receberá as mensagens da tarefa interrompida, não a resposta da nova consulta.
</Note>

<h4 id="example-advanced-permission-control">
  Exemplo - Controle avançado de permissão
</h4>

```python theme={null}
import asyncio
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
  **`@dataclass` vs `TypedDict`:** Este SDK usa dois tipos de classes. Classes decoradas com `@dataclass` (como `ResultMessage`, `AgentDefinition`, `TextBlock`) são instâncias de objeto em tempo de execução e suportam acesso por atributo: `msg.result`. Classes definidas com `TypedDict` (como `ThinkingConfigEnabled`, `McpStdioServerConfig`, `SyncHookJSONOutput`) são **dicts simples em tempo de execução** e requerem acesso por chave: `config["budget_tokens"]`, não `config.budget_tokens`. A sintaxe de chamada `ClassName(field=value)` funciona para ambas, mas apenas dataclasses produzem objetos com atributos.
</Note>

<h3 id="sdkmcptool">
  `SdkMcpTool`
</h3>

Definição para uma ferramenta MCP do SDK criada com o decorador `@tool`.

```python theme={null}
@dataclass
class SdkMcpTool(Generic[T]):
    name: str
    description: str
    input_schema: type[T] | dict[str, Any]
    handler: Callable[[T], Awaitable[dict[str, Any]]]
    annotations: ToolAnnotations | None = None
```

| Propriedade    | Tipo                                            | Descrição                                                                                                                |
| :------------- | :---------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------- |
| `name`         | `str`                                           | Identificador único para a ferramenta                                                                                    |
| `description`  | `str`                                           | Descrição legível por humanos                                                                                            |
| `input_schema` | `type[T] \| dict[str, Any]`                     | Schema para validação de entrada                                                                                         |
| `handler`      | `Callable[[T], Awaitable[dict[str, Any]]]`      | Função assíncrona que manipula a execução da ferramenta                                                                  |
| `annotations`  | [`ToolAnnotations`](#toolannotations)` \| None` | Anotações opcionais da ferramenta (por exemplo `readOnlyHint`, `destructiveHint`, `openWorldHint`, `maxResultSizeChars`) |

<h3 id="transport">
  `Transport`
</h3>

Classe base abstrata para implementações de transporte personalizadas. Use isto para se comunicar com o processo Claude através de um canal personalizado (por exemplo, uma conexão remota em vez de um subprocesso local).

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

| Método            | Descrição                                                                             |
| :---------------- | :------------------------------------------------------------------------------------ |
| `connect()`       | Conectar o transporte e preparar para comunicação                                     |
| `write(data)`     | Escrever dados brutos (JSON + nova linha) para o transporte                           |
| `read_messages()` | Iterador assíncrono que produz mensagens JSON analisadas                              |
| `close()`         | Fechar a conexão e limpar recursos                                                    |
| `is_ready()`      | Retorna `True` se o transporte pode enviar e receber                                  |
| `end_input()`     | Fechar o fluxo de entrada (por exemplo, fechar stdin para transportes de subprocesso) |

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
    system_prompt: str | SystemPromptPreset | SystemPromptCustom | SystemPromptFile | None = None
    mcp_servers: dict[str, McpServerConfig] | str | Path = field(default_factory=dict)
    strict_mcp_config: bool = False
    permission_mode: PermissionMode | None = None
    continue_conversation: bool = False
    resume: str | None = None
    session_id: str | None = None
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
    forward_subagent_text: bool = False
    fork_session: bool = False
    resume_session_at: str | None = None
    resume_drops_turn: str | None = None
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
    load_timeout_ms: int = 60_000
    task_budget: TaskBudget | None = None
```

| Propriedade                   | Tipo                                                                                  | Padrão                                | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| :---------------------------- | :------------------------------------------------------------------------------------ | :------------------------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tools`                       | `list[str] \| ToolsPreset \| None`                                                    | `None`                                | Configuração de ferramentas. Use `{"type": "preset", "preset": "claude_code"}` para as ferramentas padrão do Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `allowed_tools`               | `list[str]`                                                                           | `[]`                                  | Ferramentas para aprovar automaticamente sem solicitar. Isto não restringe Claude apenas a estas ferramentas. Se você nomear uma das [ferramentas de rastreamento de tarefas](/docs/pt/agent-sdk/todo-tracking#model-availability) aqui, Claude Code também opta a sessão. Outras ferramentas não listadas caem em `permission_mode` e `can_use_tool`. Use `disallowed_tools` para bloquear ferramentas. Veja [Permissões](/docs/pt/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                              |
| `system_prompt`               | `str \| SystemPromptPreset \| SystemPromptCustom \| SystemPromptFile \| None`         | `None`                                | Configuração de prompt do sistema. Passe uma string para um prompt personalizado, `{"type": "preset", "preset": "claude_code"}` para o prompt do sistema do Claude Code com `"append"` opcional, `{"type": "custom", "prompt": "..."}` para um prompt personalizado que também pode definir `"snapshot"`, ou `{"type": "file", "path": "..."}` para carregar um prompt grande do disco. Veja [`SystemPromptPreset`](#systempromptpreset), [`SystemPromptCustom`](#systempromptcustom), e [`SystemPromptFile`](#systempromptfile)                                                                                                                                                                                                                   |
| `mcp_servers`                 | `dict[str, McpServerConfig] \| str \| Path`                                           | `{}`                                  | Configurações de servidor MCP ou caminho para arquivo de configuração                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `strict_mcp_config`           | `bool`                                                                                | `False`                               | Quando `True`, use apenas os servidores passados em `mcp_servers` e ignore o projeto `.mcp.json`, configurações do usuário, servidores MCP fornecidos por plugins, e [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai). Mapeia para a flag CLI `--strict-mcp-config`                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `permission_mode`             | `PermissionMode \| None`                                                              | `None`                                | Modo de permissão para uso de ferramentas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `continue_conversation`       | `bool`                                                                                | `False`                               | Continuar a conversa mais recente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `resume`                      | `str \| None`                                                                         | `None`                                | ID de sessão para retomar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `session_id`                  | `str \| None`                                                                         | `None`                                | Use um ID de sessão específico em vez de um gerado automaticamente. Deve ser um UUID válido. Não pode ser combinado com `continue_conversation` ou `resume` a menos que `fork_session` também esteja definido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `max_turns`                   | `int \| None`                                                                         | `None`                                | Máximo de turnos agênticos (rodadas de uso de ferramentas)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `max_budget_usd`              | `float \| None`                                                                       | `None`                                | Parar a consulta quando a estimativa de custo do lado do cliente atingir este valor em USD. Comparado com a mesma estimativa que `total_cost_usd`. Para ressalvas de precisão e comportamento de redefinição, veja [Rastrear custo e uso](/docs/pt/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `disallowed_tools`            | `list[str]`                                                                           | `[]`                                  | Ferramentas para negar. Um nome simples como `"Bash"` remove a ferramenta do contexto do Claude. Uma regra com escopo como `"Bash(rm *)"` deixa a ferramenta disponível e nega chamadas correspondentes em todos os modos de permissão, incluindo `bypassPermissions`, para o comando [conforme escrito](/docs/pt/permissions#bash-rule-limits). Veja [Permissões](/docs/pt/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                      |
| `enable_file_checkpointing`   | `bool`                                                                                | `False`                               | Ativar rastreamento de alterações de arquivo para retrocesso. Veja [Checkpointing de arquivo](/docs/pt/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `model`                       | `str \| None`                                                                         | `None`                                | Alias de modelo Claude ou nome de modelo completo. Veja [valores aceitos e IDs específicos do provedor](/docs/pt/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `fallback_model`              | `str \| None`                                                                         | `None`                                | Modelo de fallback para usar se o modelo primário falhar. Aceita uma lista separada por vírgulas. Para orientação, veja [Escolher um modelo](/docs/pt/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `betas`                       | `list[SdkBeta]`                                                                       | `[]`                                  | Recursos beta para ativar. Veja [`SdkBeta`](#sdkbeta) para opções disponíveis                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `output_format`               | `dict[str, Any] \| None`                                                              | `None`                                | Formato de saída para respostas estruturadas (por exemplo, `{"type": "json_schema", "schema": {...}}`). Veja [Saídas estruturadas](/docs/pt/agent-sdk/structured-outputs) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `permission_prompt_tool_name` | `str \| None`                                                                         | `None`                                | Nome da ferramenta MCP para prompts de permissão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `cwd`                         | `str \| Path \| None`                                                                 | `None`                                | Diretório de trabalho atual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `cli_path`                    | `str \| Path \| None`                                                                 | `None`                                | Caminho personalizado para o executável CLI do Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `settings`                    | `str \| None`                                                                         | `None`                                | Caminho para um arquivo de configurações ou uma string JSON inline                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `add_dirs`                    | `list[str \| Path]`                                                                   | `[]`                                  | Diretórios adicionais que Claude pode acessar. O SDK passa cada entrada para Claude Code como `--add-dir`, então com a fonte de configuração `project` Claude Code também [carrega as skills, comandos e subagentes do diretório](/docs/pt/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `env`                         | `dict[str, str]`                                                                      | `{}`                                  | Variáveis de ambiente mescladas no topo do ambiente de processo herdado. Veja [Variáveis de ambiente](/docs/pt/env-vars) para variáveis que a CLI subjacente lê, e [Lidar com respostas de API lentas ou travadas](#handle-slow-or-stalled-api-responses) para variáveis relacionadas a timeout. Defina `CLAUDE_AGENT_SDK_CLIENT_APP` para identificar seu aplicativo no header User-Agent                                                                                                                                                                                                                                                                                                                                                              |
| `extra_args`                  | `dict[str, str \| None]`                                                              | `{}`                                  | Argumentos CLI adicionais para passar diretamente para a CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `max_buffer_size`             | `int \| None`                                                                         | `None`                                | Máximo de bytes ao fazer buffer da stdout da CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `debug_stderr`                | `Any`                                                                                 | `sys.stderr`                          | *Descontinuado* - O SDK ignora este valor. Use o callback `stderr` para saída stderr da CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `stderr`                      | `Callable[[str], None] \| None`                                                       | `None`                                | Função de callback para saída stderr da CLI                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `can_use_tool`                | [`CanUseTool`](#canusetool) ` \| None`                                                | `None`                                | Callback de permissão de ferramenta, invocado apenas quando o [fluxo de permissão](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) cai em um prompt. Não invocado para chamadas pré-aprovadas por `allowed_tools`, regras de permissão, ou `permission_mode`. Uma regra de permissão não pré-aprova as [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves). Veja [`CanUseTool`](#canusetool) para detalhes                                                                                                                                                                                                                                                                                 |
| `hooks`                       | `dict[HookEvent, list[HookMatcher]] \| None`                                          | `None`                                | Configurações de hook para interceptar eventos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `user`                        | `str \| None`                                                                         | `None`                                | Em plataformas POSIX, a conta de usuário do SO em que o subprocesso Claude Code é executado. Claude Code mantém o ambiente do processo pai, incluindo `HOME`, e é executado em `cwd`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `include_partial_messages`    | `bool`                                                                                | `False`                               | Incluir eventos de streaming de mensagens parciais. Quando ativado, mensagens [`StreamEvent`](#streamevent) são produzidas                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `include_hook_events`         | `bool`                                                                                | `False`                               | Incluir eventos de ciclo de vida de hook no fluxo de mensagens como objetos `HookEventMessage`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `forward_subagent_text`       | `bool`                                                                                | `False`                               | Encaminhar blocos de texto e pensamento de subagentes no fluxo de mensagens. Sem esta opção, Claude Code emite blocos `tool_use` e `tool_result` de subagentes mas não texto ou pensamento. Requer Python Agent SDK 0.2.140 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `fork_session`                | `bool`                                                                                | `False`                               | Ao retomar com `resume`, bifurcar para um novo ID de sessão em vez de continuar a sessão original                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `resume_session_at`           | `str \| None`                                                                         | `None`                                | Ao retomar, carregar a conversa apenas até e incluindo a mensagem com este UUID. Use com `resume`, e geralmente `fork_session`, para ramificar de um ponto anterior. Requer Python Agent SDK 0.2.137 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `resume_drops_turn`           | `str \| None`                                                                         | `None`                                | UUID do prompt do usuário cuja rodada uma truncagem `resume_session_at` descarta. Quando definido, a CLI recusa o retorno se o intervalo descartado contiver entradas não atribuíveis a essa rodada. Requer Python Agent SDK 0.2.137 ou posterior e Claude Code v2.1.223 ou posterior; a CLI agrupada com essas versões do SDK satisfaz o requisito do Claude Code                                                                                                                                                                                                                                                                                                                                                                                 |
| `agents`                      | `dict[str, AgentDefinition] \| None`                                                  | `None`                                | Subagentes definidos programaticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `plugins`                     | `list[SdkPluginConfig]`                                                               | `[]`                                  | Carregar plugins personalizados de caminhos locais. Veja [Plugins](/docs/pt/agent-sdk/plugins) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sandbox`                     | [`SandboxSettings`](#sandboxsettings) ` \| None`                                      | `None`                                | Configurar comportamento de sandbox programaticamente. Veja [Configurações de sandbox](#sandboxsettings) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `setting_sources`             | `list[SettingSource] \| None`                                                         | `None` (padrões CLI: todas as fontes) | Controlar quais configurações do sistema de arquivos carregar. Passe `[]` para desabilitar configurações de usuário, projeto e local. Com `skills` definido e este campo indefinido, apenas fontes de usuário e projeto carregam. Defina `setting_sources` explicitamente para manter configurações locais. Política gerenciada por endpoint carrega independentemente; configurações gerenciadas por servidor são buscadas quando a sessão se autentica com uma credencial de organização em uma [configuração elegível](/docs/pt/server-managed-settings#platform-availability). Para entradas lidas independentemente desta opção, veja [O que settingSources não controla](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control) |
| `skills`                      | `list[str] \| Literal["all"] \| None`                                                 | `None`                                | Skills disponíveis para a sessão. Passe `"all"` para ativar cada skill descoberta, ou uma lista de nomes de skills. Passe apenas nomes exatos. O SDK rejeita nomes malformados e em forma de wildcard com um `ValueError` antes de iniciar o processo Claude Code; esta verificação requer Python Agent SDK 0.2.129 ou posterior. Quando definido, o SDK adiciona a ferramenta Skill a `allowed_tools` automaticamente. Se você também passar `tools`, inclua `"Skill"` nessa lista. Veja [Skills](/docs/pt/agent-sdk/skills)                                                                                                                                                                                                                           |
| `max_thinking_tokens`         | `int \| None`                                                                         | `None`                                | *Descontinuado* - Máximo de tokens para blocos de pensamento. Use `thinking` em vez disso                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `thinking`                    | [`ThinkingConfig`](#thinkingconfig) ` \| None`                                        | `None`                                | Controla comportamento de pensamento estendido. Tem precedência sobre `max_thinking_tokens`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `effort`                      | [`EffortLevel`](#effortlevel) ` \| None`                                              | `None`                                | Nível de esforço para profundidade de pensamento. Veja [ajustar o nível de esforço](/docs/pt/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `session_store`               | [`SessionStore`](/docs/pt/agent-sdk/session-storage#the-sessionstore-interface) ` \| None` | `None`                                | Espelhar transcrições de sessão para um backend externo para que outro host possa retomá-las. Veja [Persistir sessões para armazenamento externo](/docs/pt/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `session_store_flush`         | `Literal["batched", "eager"]`                                                         | `"batched"`                           | Quando fazer flush de entradas de transcrição espelhadas para `session_store`. `"batched"` faz flush uma vez por rodada ou quando o buffer enche; `"eager"` dispara um flush em background após cada frame. Ignorado quando `session_store` é `None`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `load_timeout_ms`             | `int`                                                                                 | `60000`                               | Timeout por chamada para `session_store.load()` e `list_subkeys()` durante materialização de retomada, em milissegundos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `task_budget`                 | `TaskBudget \| None`                                                                  | `None`                                | Orçamento de token do lado da API. Enviado como `output_config.task_budget` com o header beta `task-budgets-2026-03-13`. Passe `{"total": <int>}`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

<h4 id="handle-slow-or-stalled-api-responses">
  Lidar com respostas de API lentas ou travadas
</h4>

O subprocesso CLI lê várias variáveis de ambiente que controlam timeouts de API e detecção de travamento. Passe-as através de `ClaudeAgentOptions.env`:

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions

options = ClaudeAgentOptions(
    env={
        "API_TIMEOUT_MS": "120000",
        "CLAUDE_CODE_MAX_RETRIES": "2",
        "CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS": "120000",
    },
)
```

* `API_TIMEOUT_MS`: timeout por requisição no cliente Anthropic, em milissegundos. Padrão `600000`. Aplica-se ao loop principal e todos os subagentes.
* `CLAUDE_CODE_MAX_RETRIES`: máximo de tentativas de API. Padrão `10`, limitado a `15`. Cada tentativa obtém sua própria janela `API_TIMEOUT_MS`, então o tempo de parede no pior caso é aproximadamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` mais backoff. Para execuções sem supervisão que precisam esperar por interrupções mais longas, defina [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/pt/errors#tune-retry-behavior): ele tenta novamente erros de capacidade transitória indefinidamente e, no Claude Code v2.1.199 ou posterior, aumenta o padrão para outros erros transitórios para `300` e remove o limite nesta variável.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: watchdog de travamento para subagentes. Enquanto o watchdog de stream está ativo, o padrão é `CLAUDE_STREAM_IDLE_TIMEOUT_MS` mais 5 minutos, o que resulta em `600000` a menos que você aumente essa variável. Com o watchdog de stream desativado, o padrão é `600000`. Antes de v2.1.257, o padrão era sempre `600000`.

  O temporizador é redefinido em cada evento de stream. Em um travamento, Claude Code aborta o subagente e relata o travamento ao pai. Para um subagente em background, também marca a tarefa como falhada e anexa qualquer resultado parcial.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` com `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: watchdog de stream que aborta a requisição quando os headers chegaram mas o corpo da resposta para de fazer stream. O watchdog está ativado por padrão para todos os provedores; defina `CLAUDE_ENABLE_STREAM_WATCHDOG=0` para desativá-lo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` padrão é `300000` e é limitado a esse mínimo. Após o aborto, [Tentativas automáticas](/docs/pt/errors#automatic-retries) cobre o que Claude Code faz, baseado em quão longe a resposta havia progredido.

  Enquanto o watchdog aguarda uma resposta que um gateway atrás de `ANTHROPIC_BASE_URL` mantém aberta com pings keep-alive, um host que define `include_partial_messages` continua recebendo mensagens `ping` [`StreamEvent`](#streamevent). Leia esses frames como vivacidade em vez de fazer timeout da sessão no silêncio. Antes de v2.1.257, os frames paravam 5 minutos após o último evento de stream real.

<h3 id="outputformat">
  `OutputFormat`
</h3>

Configuração para validação de saída estruturada. Passe isto como um `dict` para o campo `output_format` em `ClaudeAgentOptions`:

```python theme={null}
# Forma de dict esperada para output_format
{
    "type": "json_schema",
    "schema": {...},  # Sua definição JSON Schema
}
```

| Campo    | Obrigatório | Descrição                                           |
| :------- | :---------- | :-------------------------------------------------- |
| `type`   | Sim         | Deve ser `"json_schema"` para validação JSON Schema |
| `schema` | Sim         | Definição JSON Schema para validação de saída       |

<h3 id="systempromptpreset">
  `SystemPromptPreset`
</h3>

Configuração para usar o prompt do sistema predefinido do Claude Code com adições opcionais.

```python theme={null}
class SystemPromptPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
    append: NotRequired[str]
    exclude_dynamic_sections: NotRequired[bool]
    snapshot: NotRequired[bool]
```

| Campo                      | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                           |
| :------------------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `type`                     | Sim         | Deve ser `"preset"` para usar um prompt do sistema predefinido                                                                                                                                                                                                                                                                                                      |
| `preset`                   | Sim         | Deve ser `"claude_code"` para usar o prompt do sistema do Claude Code                                                                                                                                                                                                                                                                                               |
| `append`                   | Não         | Instruções adicionais para anexar ao prompt do sistema predefinido                                                                                                                                                                                                                                                                                                  |
| `exclude_dynamic_sections` | Não         | Mover contexto por sessão como diretório de trabalho, a flag git-repo, e caminhos de memória automática do prompt do sistema para a primeira mensagem do usuário. Melhora a reutilização de cache de prompt entre usuários e máquinas. Veja [Modificar prompts do sistema](/docs/pt/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines) |
| `snapshot`                 | Não         | Defina como `False` para reconstruir o prompt do sistema em cada requisição em vez de [reutilizar o prompt que a sessão registrou em sua primeira requisição](/docs/pt/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Requer `claude-agent-sdk` v0.2.153 ou posterior                                                                     |

<h3 id="systempromptcustom">
  `SystemPromptCustom`
</h3>

Um prompt do sistema personalizado em forma de objeto, equivalente a passar uma string como `system_prompt`, que também pode definir `snapshot`. Requer `claude-agent-sdk` v0.2.153 ou posterior.

```python theme={null}
class SystemPromptCustom(TypedDict):
    type: Literal["custom"]
    prompt: str
    snapshot: NotRequired[bool]
```

| Campo      | Obrigatório | Descrição                                                                                                                                                                   |
| :--------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`     | Sim         | Deve ser `"custom"`                                                                                                                                                         |
| `prompt`   | Sim         | O texto do prompt do sistema. Passado para a CLI como um argumento de linha de comando, então os [limites de comprimento de linha de comando](#systempromptfile) se aplicam |
| `snapshot` | Não         | Mesmo que [`SystemPromptPreset.snapshot`](#systempromptpreset), aplicado a `prompt`                                                                                         |

<h3 id="systempromptfile">
  `SystemPromptFile`
</h3>

Configuração para carregar um prompt do sistema personalizado de um arquivo em vez de passá-lo como uma string. O SDK mapeia isto para a flag CLI [`--system-prompt-file`](/docs/pt/cli-reference#system-prompt-flags). Use a forma de arquivo quando o prompt é grande: o SDK passa um `system_prompt` string no argv do subprocesso CLI, que está sujeito a limites de comprimento de linha de comando do SO antes do SDK enviar qualquer requisição de API. No Linux um único argumento mais longo que aproximadamente 128 KB falha no spawn do processo com `Argument list too long`. No Windows toda a linha de comando é limitada a aproximadamente 32 KB, então a forma de string falha em um limite mais baixo.

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

| Valor       | Descrição                                                                                 | Localização                   |
| :---------- | :---------------------------------------------------------------------------------------- | :---------------------------- |
| `"user"`    | Configurações globais do usuário                                                          | `~/.claude/settings.json`     |
| `"project"` | Configurações de projeto compartilhadas (controladas por versão)                          | `.claude/settings.json`       |
| `"local"`   | Configurações de projeto local, gitignored quando Claude Code salva uma configuração nela | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamento padrão
</h4>

Quando `setting_sources` é omitido ou `None` e `skills` não está definido, `query()` carrega as mesmas configurações do sistema de arquivos que a CLI Claude Code: usuário, projeto e local. Com `skills` definido, a linha [`setting_sources`](#claudeagentoptions) descreve o padrão atual. Política gerenciada por endpoint é carregada em todos os casos; configurações gerenciadas por servidor são buscadas quando a sessão se autentica com uma credencial de organização em uma [configuração elegível](/docs/pt/server-managed-settings#platform-availability). Para mais informações, veja [O que settingSources não controla](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control).

<h4 id="why-use-setting_sources">
  Por que usar setting\_sources
</h4>

**Desabilitar configurações do sistema de arquivos:**

```python theme={null}
# Não carregar configurações de usuário, projeto ou local do disco
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    async for message in query(
        prompt="Analyze this code",
        options=ClaudeAgentOptions(
            setting_sources=[]
        ),
    ):
        print(message)


asyncio.run(main())
```

<Note>
  No Python SDK 0.1.59 e anterior, uma lista vazia era tratada da mesma forma que omitir a opção, então `setting_sources=[]` não desabilitava configurações do sistema de arquivos. Atualize para uma versão mais recente se você precisar que uma lista vazia tenha efeito. O SDK TypeScript não é afetado.
</Note>

**Carregar apenas fontes de configuração específicas:**

```python theme={null}
# Carregar apenas configurações de projeto, ignorar usuário e local
import asyncio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main():
    async for message in query(
        prompt="Run CI checks",
        options=ClaudeAgentOptions(
            setting_sources=["project"]  # Apenas .claude/settings.json
        ),
    ):
        print(message)


asyncio.run(main())
```

**Aplicações apenas SDK:**

```python theme={null}
# Definir tudo programaticamente.
# Passe [] para optar por não usar fontes de configuração do sistema de arquivos.
import asyncio
from claude_agent_sdk import AgentDefinition, ClaudeAgentOptions, query


async def main():
    async for message in query(
        prompt="Review this PR",
        options=ClaudeAgentOptions(
            setting_sources=[],
            agents={
                "code-reviewer": AgentDefinition(
                    description="Reviews code changes",
                    prompt="You are a code reviewer. Report issues in the diff.",
                ),
            },
            allowed_tools=["Read", "Grep", "Glob"],
        ),
    ):
        print(message)


asyncio.run(main())
```

Para carregar instruções de projeto CLAUDE.md, inclua `"project"` em `setting_sources`. Veja [Modificar prompts do sistema](/docs/pt/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions) para como o carregamento de CLAUDE.md interage com as opções de prompt do sistema.

<h4 id="settings-precedence">
  Precedência de configurações
</h4>

Quando múltiplas fontes são carregadas, as configurações são mescladas com esta precedência (maior para menor):

1. Configurações locais (`.claude/settings.local.json`)
2. Configurações de projeto (`.claude/settings.json`)
3. Configurações do usuário (`~/.claude/settings.json`)

Opções programáticas como `agents`, `allowed_tools`, e `settings` substituem configurações do sistema de arquivos de usuário, projeto e local. Configurações de política gerenciada têm precedência sobre opções programáticas.

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

| Campo             | Obrigatório | Descrição                                                                                                                                                                                                                                                                 |
| :---------------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `description`     | Sim         | Descrição em linguagem natural de quando usar este agente                                                                                                                                                                                                                 |
| `prompt`          | Sim         | O prompt do sistema do agente                                                                                                                                                                                                                                             |
| `tools`           | Não         | Array de nomes de ferramentas permitidas. Se omitido, herda cada [ferramenta disponível para subagentes](/docs/pt/sub-agents#available-tools)                                                                                                                                  |
| `disallowedTools` | Não         | Array de nomes de ferramentas para remover do conjunto de ferramentas do agente. Padrões de nível de servidor MCP também são aceitos: `mcp__server` ou `mcp__server__*` remove cada ferramenta desse servidor, e `mcp__*` remove cada ferramenta MCP de qualquer servidor |
| `model`           | Não         | Substituição de modelo para este agente. Aceita um alias como `"sonnet"`, `"opus"`, `"haiku"`, ou `"inherit"`, ou um ID de modelo completo. Quando você o omite, Claude Code escolhe o modelo na [ordem de modelo de subagente](/docs/pt/sub-agents#choose-a-model)            |
| `skills`          | Não         | Lista de nomes de skills para pré-carregar no contexto do agente na inicialização. Skills não listadas permanecem invocáveis através da ferramenta Skill                                                                                                                  |
| `memory`          | Não         | Fonte de memória para este agente: `"user"`, `"project"`, ou `"local"`                                                                                                                                                                                                    |
| `mcpServers`      | Não         | Servidores MCP disponíveis para este agente. Cada entrada é um nome de servidor ou um dict `{name: config}` inline                                                                                                                                                        |
| `initialPrompt`   | Não         | Auto-enviado como o primeiro turno do usuário quando este agente é executado como o agente de thread principal                                                                                                                                                            |
| `maxTurns`        | Não         | Número máximo de turnos agênticos antes do agente parar                                                                                                                                                                                                                   |
| `background`      | Não         | Executar este agente como uma tarefa em background não-bloqueante quando invocado                                                                                                                                                                                         |
| `effort`          | Não         | Nível de esforço de raciocínio para este agente. Aceita um nível nomeado ou um inteiro. Veja [`EffortLevel`](#effortlevel)                                                                                                                                                |
| `permissionMode`  | Não         | Modo de permissão para execução de ferramentas dentro deste agente. As [regras de herança de subagente](/docs/pt/agent-sdk/permissions#available-modes) decidem quando se aplica. Veja [`PermissionMode`](#permissionmode)                                                     |

<Note>
  Os nomes de campo `AgentDefinition` usam camelCase, como `disallowedTools`, `permissionMode`, e `maxTurns`. Esses nomes mapeiam diretamente para o formato de wire compartilhado com o SDK TypeScript. Isto difere de `ClaudeAgentOptions`, que usa snake\_case Python para campos de nível superior equivalentes como `disallowed_tools` e `permission_mode`. Como `AgentDefinition` é um dataclass, passar uma palavra-chave snake\_case levanta um `TypeError` no tempo de construção.
</Note>

<h3 id="permissionmode">
  `PermissionMode`
</h3>

Modos de permissão para controlar execução de ferramentas.

```python theme={null}
PermissionMode = Literal[
    "default",  # Comportamento de permissão padrão
    "acceptEdits",  # Auto-aceitar edições de arquivo
    "plan",  # Modo de planejamento - explorar sem editar
    "dontAsk",  # Negar qualquer coisa não pré-aprovada em vez de solicitar
    "bypassPermissions",  # Contornar verificações de permissão; regras de ask explícitas ainda solicitam (use com cuidado)
    "auto",  # Classificador de modelo aprova ou nega prompts de permissão
]
```

<h3 id="effortlevel">
  `EffortLevel`
</h3>

Níveis de esforço para guiar profundidade de pensamento.

```python theme={null}
EffortLevel = Literal[
    "low",  # Pensamento mínimo, respostas mais rápidas
    "medium",  # Pensamento moderado
    "high",  # Raciocínio profundo
    "xhigh",  # Raciocínio estendido; volta para "high" em modelos que não suportam
    "max",  # Esforço máximo
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

O callback é a substituição SDK para o prompt de permissão interativo: é invocado apenas quando o [fluxo de avaliação de permissão](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) se resolve em um prompt. Chamadas de ferramenta já aprovadas por uma entrada `allowed_tools`, uma regra de permissão de configurações, ou o modo de permissão, como `acceptEdits` ou `bypassPermissions`, nunca o invocam. Para controlar cada chamada de ferramenta, use um [hook `PreToolUse`](/docs/pt/agent-sdk/hooks) em vez disso.

Uma regra de permissão não pré-aprova as [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves); veja [Como permissões são avaliadas](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) para qual delas chega ao callback e o que acontece em modo `dontAsk` e `auto`.

<h3 id="toolpermissioncontext">
  `ToolPermissionContext`
</h3>

Informações de contexto passadas para callbacks de permissão de ferramenta.

```python theme={null}
@dataclass
class ToolPermissionContext:
    signal: Any | None = None  # Futuro: suporte a sinal de aborto
    suggestions: list[PermissionUpdate] = field(default_factory=list)
    tool_use_id: str | None = None
    agent_id: str | None = None
    blocked_path: str | None = None
    decision_reason: str | None = None
    title: str | None = None
    display_name: str | None = None
    description: str | None = None
```

| Campo             | Tipo                     | Descrição                                                                                                                                                                                                                             |
| :---------------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `signal`          | `Any \| None`            | Reservado para suporte futuro a sinal de aborto                                                                                                                                                                                       |
| `suggestions`     | `list[PermissionUpdate]` | Sugestões de atualização de permissão da CLI. Prompts Bash incluem uma sugestão com o destino `localSettings`, então retorná-la em `updated_permissions` escreve a regra para `.claude/settings.local.json` e persiste entre sessões. |
| `tool_use_id`     | `str \| None`            | Identificador da chamada de ferramenta específica para a qual este prompt é. Sempre preenchido quando entregue a `can_use_tool`                                                                                                       |
| `agent_id`        | `str \| None`            | ID do sub-agente quando a chamada origina de um subagente; `None` para o agente principal                                                                                                                                             |
| `blocked_path`    | `str \| None`            | Caminho de arquivo que disparou a solicitação de permissão, quando aplicável. Por exemplo, quando um comando Bash tenta acessar um caminho fora de diretórios permitidos                                                              |
| `decision_reason` | `str \| None`            | Razão pela qual esta solicitação de permissão foi disparada. Encaminhada do `permissionDecisionReason` de um hook PreToolUse quando o hook retornou `"ask"`                                                                           |
| `title`           | `str \| None`            | Sentença de prompt de permissão completa, como `Claude wants to read foo.txt`. Use como o texto de prompt principal quando presente                                                                                                   |
| `display_name`    | `str \| None`            | Frase de substantivo curta para a ação da ferramenta, como `Read file`, adequada para rótulos de botão                                                                                                                                |
| `description`     | `str \| None`            | Subtítulo legível por humanos para a UI de permissão                                                                                                                                                                                  |

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

| Campo                 | Tipo                             | Padrão    | Descrição                                       |
| :-------------------- | :------------------------------- | :-------- | :---------------------------------------------- |
| `behavior`            | `Literal["allow"]`               | `"allow"` | Deve ser "allow"                                |
| `updated_input`       | `dict[str, Any] \| None`         | `None`    | Entrada modificada para usar em vez da original |
| `updated_permissions` | `list[PermissionUpdate] \| None` | `None`    | Atualizações de permissão para aplicar          |

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

Uma regra para adicionar, substituir ou remover em uma atualização de permissão.

```python theme={null}
@dataclass
class PermissionRuleValue:
    tool_name: str
    rule_content: str | None = None
```

<h3 id="toolspreset">
  `ToolsPreset`
</h3>

Configuração de ferramentas predefinidas para usar o conjunto de ferramentas padrão do Claude Code.

```python theme={null}
class ToolsPreset(TypedDict):
    type: Literal["preset"]
    preset: Literal["claude_code"]
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Controla comportamento de pensamento estendido. Uma união de três configurações:

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

| Variante   | Campos                             | Descrição                                              |
| :--------- | :--------------------------------- | :----------------------------------------------------- |
| `adaptive` | `type`, `display`                  | Claude decide adaptativamente quando pensar            |
| `enabled`  | `type`, `budget_tokens`, `display` | Ativar pensamento com um orçamento de token específico |
| `disabled` | `type`                             | Desabilitar pensamento                                 |

O campo `display` opcional controla se o texto de pensamento é retornado `"summarized"` ou `"omitted"`. No Claude Opus 4.7 e posterior, o padrão da API é `"omitted"`, então defina `"summarized"` para receber conteúdo de pensamento em saídas [`ThinkingBlock`](#thinkingblock). Claude Code não envia `display` para Amazon Bedrock ou Google Cloud's Agent Platform, então nesses provedores Opus 4.7 e posterior retornam saídas `ThinkingBlock` vazias mesmo quando você define `display` para `"summarized"`.

Como estas são classes `TypedDict`, elas são dicts simples em tempo de execução. Construa-as como literais de dict ou chame a classe como um construtor; ambos produzem um `dict`. Acesse campos com `config["budget_tokens"]`, não `config.budget_tokens`:

```python theme={null}
from claude_agent_sdk import ClaudeAgentOptions, ThinkingConfigEnabled

# Opção 1: literal de dict (recomendado, sem importação necessária)
options = ClaudeAgentOptions(thinking={"type": "enabled", "budget_tokens": 20000})

# Opção 2: estilo construtor (retorna um dict simples)
config = ThinkingConfigEnabled(type="enabled", budget_tokens=20000)
print(config["budget_tokens"])  # 20000
# config.budget_tokens levantaria AttributeError
```

<h3 id="taskbudget">
  `TaskBudget`
</h3>

Orçamento de tarefa do lado da API em tokens, usado com o campo `task_budget` em `ClaudeAgentOptions`.

```python theme={null}
class TaskBudget(TypedDict):
    total: int
```

| Campo   | Tipo  | Descrição                              |
| :------ | :---- | :------------------------------------- |
| `total` | `int` | Orçamento de token total para a tarefa |

Como isto é um `TypedDict`, passe-o como um dict simples, como `ClaudeAgentOptions(task_budget={"total": 50000})`.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Tipo literal para recursos beta do SDK.

```python theme={null}
SdkBeta = Literal["context-1m-2025-08-07"]
```

Use com o campo `betas` em `ClaudeAgentOptions` para ativar recursos beta.

<Warning>
  O beta `context-1m-2025-08-07` foi descontinuado a partir de 30 de abril de 2026. Passar este header com Claude Sonnet 4.5 ou Sonnet 4 não tem efeito, e requisições que excedem a janela de contexto padrão de 200k-token retornam um erro. Para usar uma janela de contexto de 1M-token, migre para [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7, ou Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), que incluem contexto de 1M a preços padrão sem header beta necessário.
</Warning>

<h3 id="mcpsdkserverconfig">
  `McpSdkServerConfig`
</h3>

Configuração para servidores MCP do SDK criados com `create_sdk_mcp_server()`.

```python theme={null}
class McpSdkServerConfig(TypedDict):
    type: Literal["sdk"]
    name: str
    instance: Any  # Instância do servidor MCP
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
    type: NotRequired[Literal["stdio"]]  # Opcional para compatibilidade com versões anteriores
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

A configuração de um servidor MCP conforme relatado por [`get_mcp_status()`](#methods). Esta é a união de todas as variantes de transporte [`McpServerConfig`](#mcpserverconfig) mais uma variante de saída única `claudeai-proxy` para servidores proxied através de claude.ai.

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
    | ConversationResetMessage
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
    origin: MessageOrigin | None = None
```

| Campo                | Tipo                        | Descrição                                                                                                                                                                                        |
| :------------------- | :-------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `content`            | `str \| list[ContentBlock]` | Conteúdo da mensagem como texto ou blocos de conteúdo                                                                                                                                            |
| `uuid`               | `str \| None`               | Identificador único de mensagem                                                                                                                                                                  |
| `parent_tool_use_id` | `str \| None`               | ID de uso de ferramenta se esta mensagem é uma resposta de resultado de ferramenta                                                                                                               |
| `tool_use_result`    | `dict[str, Any] \| None`    | Dados de resultado de ferramenta se aplicável                                                                                                                                                    |
| `origin`             | `MessageOrigin \| None`     | Proveniência desta mensagem, preenchida em turnos injetados, como notificações de tarefas e mensagens de pares. `None` quando a CLI não a atribuiu. Requer Python Agent SDK 0.2.137 ou posterior |

O SDK passa `tool_use_result` através da CLI sem modificação. Para uma ferramenta em um servidor MCP externo cujo resultado contém blocos `resource_link`, o dict tem uma chave `resourceLinks` contendo uma lista de dicts com as chaves do tipo TypeScript [`SDKMcpResourceLink`](/docs/pt/agent-sdk/typescript#sdkmcpresourcelink). Claude recebe cada link como uma linha de texto no resultado da ferramenta. Para renderizar os arquivos que o servidor retornou, leia `resourceLinks` em vez de analisar esse texto. A chave `resourceLinks` requer Python Agent SDK 0.2.150 ou posterior e Claude Code v2.1.257 ou posterior; a CLI agrupada com essa versão do SDK satisfaz o requisito do Claude Code.

A CLI omite a chave quando o resultado não tem links e em resultados de subagentes. A CLI mantém no máximo 50 links por resultado e para de adicionar links quando a lista atinge 64 KiB de JSON serializado. Uma ferramenta que você define em processo com [`tool()`](#tool) nunca produz a chave, porque o SDK achata seus blocos `resource_link` em texto antes da CLI ver o resultado.

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
    stop_reason: str | None = None
    session_id: str | None = None
    uuid: str | None = None
```

| Campo                | Tipo                                                         | Descrição                                                                             |
| :------------------- | :----------------------------------------------------------- | :------------------------------------------------------------------------------------ |
| `content`            | `list[ContentBlock]`                                         | Lista de blocos de conteúdo na resposta                                               |
| `model`              | `str`                                                        | Modelo que gerou a resposta                                                           |
| `parent_tool_use_id` | `str \| None`                                                | ID de uso de ferramenta se esta é uma resposta aninhada                               |
| `error`              | [`AssistantMessageError`](#assistantmessageerror) ` \| None` | Tipo de erro se a resposta encontrou um erro                                          |
| `usage`              | `dict[str, Any] \| None`                                     | Uso de token por mensagem (mesmas chaves que [`ResultMessage.usage`](#resultmessage)) |
| `message_id`         | `str \| None`                                                | ID de mensagem da API. Múltiplas mensagens de um turno compartilham o mesmo ID        |
| `stop_reason`        | `str \| None`                                                | Motivo de parada da API (por exemplo, `end_turn`, `tool_use`)                         |
| `session_id`         | `str \| None`                                                | ID da sessão à qual esta mensagem pertence                                            |
| `uuid`               | `str \| None`                                                | Identificador único de mensagem dentro da transcrição da sessão                       |

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
    "unknown",
]
```

O processo CLI subjacente pode emitir tipos de erro que este Literal não lista, como `max_output_tokens`. O SDK passa o valor através sem modificação, então trate strings fora desta lista da forma que você trata `unknown`. O tipo TypeScript [`SDKAssistantMessageError`](/docs/pt/agent-sdk/typescript#sdkassistantmessage) lista o conjunto completo de valores que a CLI pode emitir.

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
    model_usage: dict[str, ModelUsage] | None = None
    permission_denials: list[Any] | None = None
    deferred_tool_use: DeferredToolUse | None = None
    errors: list[str] | None = None
    api_error_status: int | None = None
    uuid: str | None = None
    terminal_reason: str | None = None
    origin: MessageOrigin | None = None
```

O campo `subtype` determina quais outros campos são preenchidos. É um de `"success"`, `"error_during_execution"`, `"error_max_turns"`, `"error_max_budget_usd"` ou `"error_max_structured_output_retries"`. A dataclass Python achata todas as variantes em uma forma, portanto campos que não se aplicam ao subtipo retornado são `None`.

Vários campos carregam detalhes de diagnóstico sobre como a conversa terminou:

* `is_error`: `True` quando a conversa terminou em um estado de erro. Sempre `True` nos subtipos `error_*`. Em `subtype="success"` é `True` quando a solicitação final do modelo falhou, significando que o loop do agente foi concluído mas a última chamada da API retornou um erro.
* `api_error_status`: o código de status HTTP do erro de API de encerramento. `None` quando o turno terminou sem um. Preenchido apenas em `subtype="success"`.
* `result`: texto da mensagem final do assistente em `subtype="success"`, ou `None` nos subtipos `error_*`. Quando `subtype="success"` e `is_error=True`, isso contém a string de erro da API se uma estiver disponível mas pode estar vazio, então verifique `api_error_status` e o conteúdo anterior de `AssistantMessage` para detalhes.
* `errors`: strings de erro no nível do loop, como a mensagem de máximo de turnos. Preenchido apenas nos subtipos `error_*`.
* `terminal_reason`: por que o loop de consulta terminou, como `"completed"`, `"max_turns"`, `"api_error"`, `"aborted_streaming"` ou `"aborted_tools"`. Um valor de `"aborted_streaming"` ou `"aborted_tools"` significa que o turno foi abortado antes de ser concluído. As causas comuns são [`interrupt()`](#claudesdkclient) e um callback de permissão retornando [`PermissionResultDeny`](#permissionresultdeny) com `interrupt=True`. `None` em versões da CLI que antecedem o campo, em resultados de comandos locais como `/voice` ou `/usage`, que contornam o loop de consulta, ou em resultados de erro sintetizados emitidos quando a sessão falha fatalmente. Espelha o [`SDKResultMessage.terminal_reason`](/docs/pt/agent-sdk/typescript#sdkresultmessage) do SDK TypeScript, que lista o conjunto completo de valores.
* `origin`: origem da mensagem do usuário que acionou este turno. Em [modo de entrada de streaming](/docs/pt/agent-sdk/streaming-vs-single-mode), verifique isso para distinguir o resultado do seu próprio prompt, onde `origin` é `None` ou `{"kind": "human"}`, do resultado de um turno injetado, como uma notificação de tarefa de fundo. Requer Python Agent SDK 0.2.137 ou posterior.

O dict `usage` cobre apenas o loop do agente principal e exclui subagentes e outras chamadas de modelo aninhadas ou auxiliares. Em [modo de entrada de streaming](/docs/pt/agent-sdk/streaming-vs-single-mode), os valores são por turno. Prefira `model_usage` para contabilidade de token e custo. O dict `usage` contém as seguintes chaves quando presentes:

| Chave                         | Tipo  | Descrição                                                                                                                                                                                                                          |
| ----------------------------- | ----- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `input_tokens`                | `int` | Tokens de entrada consumidos pelo loop do agente de nível superior. [Tokens de subagente não estão incluídos](/docs/pt/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); use `model_usage` para contabilidade de árvore completa. |
| `output_tokens`               | `int` | Tokens de saída gerados pelo loop do agente de nível superior. Tokens de subagente não estão incluídos.                                                                                                                            |
| `cache_creation_input_tokens` | `int` | Tokens usados para criar novas entradas de cache.                                                                                                                                                                                  |
| `cache_read_input_tokens`     | `int` | Tokens lidos de entradas de cache existentes.                                                                                                                                                                                      |

O dict `model_usage` mapeia nomes de modelo para uso por modelo. Ele cobre cada chamada de modelo feita através do pipeline de consulta: o loop principal, subagentes e chamadas internas como compactação e agentes Workflow. Chamadas auxiliares fora desse pipeline, como o classificador de permissão e solicitações de contagem de tokens, são excluídas de `model_usage`. Trate `model_usage` como uma estimativa, não como um extrato de faturamento.

Em [modo de entrada de streaming](/docs/pt/agent-sdk/streaming-vs-single-mode), `model_usage` e `total_cost_usd` são cumulativos entre turnos, então leia o resultado mais recente em vez de somar entre resultados. Uma chamada que retoma uma sessão também conta os [totais restaurados das chamadas anteriores da sessão](/docs/pt/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Veja [Rastrear custos em modo de entrada de streaming](/docs/pt/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) para redefinições e [Recuperar totais após uma falha de sessão](/docs/pt/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) para resultados zerados.

Cada valor em `model_usage` é um TypedDict `ModelUsage`, importado via `from claude_agent_sdk.types import ModelUsage`. Suas chaves usam camelCase porque o SDK passa o valor através sem modificação do processo CLI subjacente, correspondendo ao tipo TypeScript [`ModelUsage`](/docs/pt/agent-sdk/typescript#modelusage):

| Chave                      | Tipo    | Descrição                                                                                                                                                                                                                                                                                            |
| -------------------------- | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `inputTokens`              | `int`   | Tokens de entrada para este modelo.                                                                                                                                                                                                                                                                  |
| `outputTokens`             | `int`   | Tokens de saída para este modelo.                                                                                                                                                                                                                                                                    |
| `cacheReadInputTokens`     | `int`   | Tokens de leitura de cache para este modelo.                                                                                                                                                                                                                                                         |
| `cacheCreationInputTokens` | `int`   | Tokens de criação de cache para este modelo.                                                                                                                                                                                                                                                         |
| `webSearchRequests`        | `int`   | Solicitações de busca na web feitas por este modelo.                                                                                                                                                                                                                                                 |
| `thinkingTokens`           | `int`   | Tokens de pensamento gerados por este modelo, já contados em `outputTokens`. Ausente até que um turno seja executado em uma versão do Claude Code que o registre, e não declarado no TypedDict, então leia com `.get()`. Requer Python Agent SDK 0.2.150 ou posterior, cuja CLI agrupada o registra. |
| `costUSD`                  | `float` | Custo estimado em USD para este modelo, computado no lado do cliente. Veja [Rastrear custo e uso](/docs/pt/agent-sdk/cost-tracking) para ressalvas de faturamento.                                                                                                                                        |
| `contextWindow`            | `int`   | Tamanho da janela de contexto para este modelo.                                                                                                                                                                                                                                                      |
| `maxOutputTokens`          | `int`   | Limite máximo de token de saída para este modelo.                                                                                                                                                                                                                                                    |
| `canonicalModel`           | `str`   | ID de modelo canônico usado para a busca de preço. Pode diferir da string de modelo bruto pela qual a entrada é codificada, como um ID específico do provedor ou alias. Nem sempre presente.                                                                                                         |
| `provider`                 | `str`   | Provedor de API que serviu este modelo, como `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle` ou `gateway`. Nem sempre presente.                                                                                                                                               |

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

| Campo                     | Tipo                      | Descrição                                                                                                                                                                      |
| :------------------------ | :------------------------ | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`                  | `RateLimitStatus`         | Status atual, um de `"allowed"`, `"allowed_warning"` ou `"rejected"`. `"allowed_warning"` significa aproximando-se do limite; `"rejected"` significa que o limite foi atingido |
| `resets_at`               | `int \| None`             | Timestamp Unix quando a janela de limite de taxa é redefinida                                                                                                                  |
| `rate_limit_type`         | `RateLimitType \| None`   | Qual janela de limite de taxa se aplica                                                                                                                                        |
| `utilization`             | `float \| None`           | Fração do limite de taxa consumido (0.0 a 1.0)                                                                                                                                 |
| `overage_status`          | `RateLimitStatus \| None` | Status do uso de excedente pré-pago, se aplicável                                                                                                                              |
| `overage_resets_at`       | `int \| None`             | Timestamp Unix quando a janela de excedente é redefinida                                                                                                                       |
| `overage_disabled_reason` | `str \| None`             | Por que o excedente está indisponível, se o status é `"rejected"`                                                                                                              |
| `raw`                     | `dict[str, Any]`          | Dict bruto completo do CLI, incluindo campos não modelados acima                                                                                                               |

<h3 id="conversationresetmessage">
  `ConversationResetMessage`
</h3>

Emitido quando a conversa é substituída sem encerrar a conexão, como após `/clear`. Veja [Rastrear custos em modo de entrada de streaming](/docs/pt/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) para como uma redefinição afeta os totais em execução em objetos `ResultMessage` posteriores. Requer Python Agent SDK 0.2.137 ou posterior.

```python theme={null}
@dataclass
class ConversationResetMessage:
    new_conversation_id: str
    uuid: str
    session_id: str
```

| Campo                 | Tipo  | Descrição                                                                                                                 |
| :-------------------- | :---- | :------------------------------------------------------------------------------------------------------------------------ |
| `new_conversation_id` | `str` | Identificador opaco para a conversa fresca. Não é o `session_id` de mensagens subsequentes; leia isso da próxima mensagem |
| `uuid`                | `str` | Identificador único de mensagem                                                                                           |
| `session_id`          | `str` | ID da sessão que foi redefinida. Mensagens após a redefinição carregam um novo `session_id`                               |

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

Quando a CLI [move uma chamada de ferramenta MCP longa para o fundo](/docs/pt/mcp#automatic-backgrounding-of-long-tool-calls), o resultado da ferramenta para essa chamada contém apenas um espaço reservado e o resultado real da chamada chega nesta mensagem. Em uma notificação `"completed"` para tal chamada, a CLI adiciona uma chave `resource_links` listando os arquivos que a ferramenta retornou por referência, com as mesmas entradas e limites que a chave `resourceLinks` em [`UserMessage.tool_use_result`](#usermessage). A chave `resource_links` requer Python Agent SDK 0.2.150 ou posterior e Claude Code v2.1.257 ou posterior; a CLI agrupada com essa versão do SDK satisfaz o requisito do Claude Code.

A dataclass não tem campo para `resource_links`. Leia-o do dict `data` que a mensagem herda de [`SystemMessage`](#systemmessage): `message.data.get("resource_links")`. Corresponda a notificação à chamada com `tool_use_id`. A CLI omite a chave quando o resultado não tinha links e em notificações para tarefas que não são chamadas de ferramenta MCP.

<h2 id="content-block-types">
  Tipos de Bloco de Conteúdo
</h2>

<h3 id="contentblock">
  `ContentBlock`
</h3>

Tipo de união de todos os blocos de conteúdo.

```python theme={null}
ContentBlock = (
    TextBlock
    | ThinkingBlock
    | ToolUseBlock
    | ToolResultBlock
    | ServerToolUseBlock
    | ServerToolResultBlock
)
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

Os tipos abaixo definem o que seu código captura. Para entradas com chave nas mensagens de erro que esses tipos levantam, com a causa e correção para cada um, consulte [Troubleshooting](/docs/pt/agent-sdk/troubleshooting).

<h3 id="claudesdkerror">
  `ClaudeSDKError`
</h3>

Classe de exceção base para todos os erros do SDK.

```python theme={null}
class ClaudeSDKError(Exception):
    """Base error for Claude SDK."""
```

Quando uma `query()` de uma única tentativa termina com um resultado de erro, por exemplo um erro de limite de turnos, o SDK levanta um [`ResultError`](#resulterror) após ceder a mensagem de resultado final. As versões do Python Agent SDK anteriores a 0.2.140 levantavam uma `Exception` simples que não era uma subclasse de `ClaudeSDKError`.

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

<h3 id="resulterror">
  `ResultError`
</h3>

Levantado após a [`ResultMessage`](#resultmessage) final quando o processo Claude Code sai porque a execução terminou com um resultado de erro, como um erro de limite de turnos ou um erro de API. `ResultError` é uma subclasse de `ProcessError`, portanto um manipulador `except ProcessError` existente também o captura. Seus atributos carregam os campos dessa mensagem de resultado, para que você possa ramificar o motivo da falha da execução sem analisar o texto da mensagem. Requer Python Agent SDK 0.2.140 ou posterior.

```python theme={null}
class ResultError(ProcessError):
    subtype: str | None  # "error_max_turns", "error_during_execution", ...; "success" when the run ended on a failed request
    errors: list[str]  # an empty list when the result message reported none
    result: str | None
    api_error_status: int | None
    terminal_reason: str | None  # "max_turns", "api_error", ...; check this before subtype
    session_id: str | None
    data: dict[str, Any]  # the raw result message payload
```

Para distinguir falhas, verifique `terminal_reason` antes de `subtype`. Quando a solicitação final falha, como em um erro de API, Claude Code relata `subtype` `"success"` com a causa em `terminal_reason`, por exemplo `"api_error"`; quando um limite que você definiu encerra a execução, como `max_turns` ou `max_budget_usd`, ele relata um subtipo `error_*`.

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
  O SDK TypeScript suporta eventos de hook adicionais não disponíveis ainda em Python. Veja a [tabela de disponibilidade de hooks](/docs/pt/agent-sdk/hooks#available-hooks) para suporte por SDK.
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

Retorna um [`HookJSONOutput`](#hookjsonoutput).

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
        None  # Timeout in seconds. When omitted, the per-event default applies:
        # 600 for most events, 30 for UserPromptSubmit
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

| Campo             | Tipo                            | Descrição                                                                                                                                                                                                                                                               |
| :---------------- | :------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `hook_event_name` | `Literal["PostToolUseFailure"]` | Sempre "PostToolUseFailure"                                                                                                                                                                                                                                             |
| `tool_name`       | `str`                           | Nome da ferramenta que falhou                                                                                                                                                                                                                                           |
| `tool_input`      | `dict[str, Any]`                | Parâmetros de entrada que foram usados                                                                                                                                                                                                                                  |
| `tool_use_id`     | `str`                           | Identificador único para este uso de ferramenta                                                                                                                                                                                                                         |
| `error`           | `str`                           | Mensagem de erro da execução falhada                                                                                                                                                                                                                                    |
| `is_interrupt`    | `bool` (opcional)               | Verdadeiro quando a falha chegou ao Claude Code como uma interrupção em vez de um erro que a ferramenta reportou. Cancelar uma ferramenta em execução com `interrupt()` não dispara este hook; o resultado da ferramenta carrega a mensagem de interrupção em vez disso |
| `agent_id`        | `str` (opcional)                | Identificador de subagente, presente quando o hook dispara dentro de um subagente                                                                                                                                                                                       |
| `agent_type`      | `str` (opcional)                | Tipo de subagente, presente quando o hook dispara dentro de um subagente                                                                                                                                                                                                |

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

Uma união discriminada de tipos de saída específicos do evento `TypedDict`. O campo `hookEventName` determina quais campos são válidos. Para detalhes completos sobre campos disponíveis por evento de hook, veja [Controlar execução com hooks](/docs/pt/agent-sdk/hooks#outputs).

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
import asyncio
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
            ),  # Applies to all tools (per-event default timeout)
        ],
        "PostToolUse": [HookMatcher(hooks=[log_tool_use])],
    }
)

async def main():
    async for message in query(prompt="Analyze this codebase", options=options):
        print(message)


asyncio.run(main())
```

<h2 id="tool-input/output-types">
  Tipos de Entrada/Saída de Ferramenta
</h2>

Documentação de schemas de entrada/saída para todas as ferramentas Claude Code integradas. Embora o SDK Python não exporte esses como tipos, eles representam a estrutura de entradas e saídas de ferramenta em mensagens.

<h3 id="agent">
  Agent
</h3>

**Nome da ferramenta:** `Agent`. O nome anterior `Task` ainda é aceito como alias, e a lista `tools` na [`SystemMessage`](#systemmessage) de inicialização relata essa ferramenta como `Task` para compatibilidade com versões anteriores.

**Entrada:**

```python theme={null}
{
    "description": str,  # Uma descrição breve (3-5 palavras) da tarefa
    "prompt": str,  # A tarefa para o agente executar
    "subagent_type": str | None,  # O tipo de agente especializado a usar
    "model": "sonnet" | "opus" | "haiku" | "fable" | None,  # Substituição de modelo para este agente
    "run_in_background": bool | None,  # Agentes executam em segundo plano por padrão; defina como False para executar sincronamente
    "name": str | None,  # Nome para o agente gerado
    "team_name": str | None,  # Descontinuado; ignorado
    "mode": "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan" | None,  # Descontinuado; ignorado. As regras de herança de subagente decidem o modo de permissão de um subagente
    "isolation": "worktree" | "remote" | None,  # Modo de isolamento para as alterações do agente
}
```

Inicia um novo agente para lidar com tarefas complexas e multi-etapas autonomamente.

**Saída (status: `"completed"`):**

```python theme={null}
{
    "status": "completed",
    "agentId": str,  # ID do agente que foi executado
    "agentType": str | None,  # O tipo de subagente que tratou a tarefa
    "content": [  # Blocos de conteúdo de resultado
        {
            "type": "text",
            "text": str,
            "citations": list | None,
        }
    ],
    "resolvedModel": str | None,  # Modelo em que o subagente iniciou
    "modelsUsed": list[str] | None,  # Modelos usados em ordem, com repetições consecutivas colapsadas
    "totalToolUseCount": int,  # Número de chamadas de ferramenta que o agente fez
    "totalDurationMs": int,  # Duração da execução em milissegundos
    "totalTokens": int,  # Contagem de tokens da solicitação final da API, não de toda a execução
    "usage": {  # Estatísticas de uso de tokens
        "input_tokens": int,
        "output_tokens": int,
        "cache_creation_input_tokens": int | None,
        "cache_read_input_tokens": int | None,
        "server_tool_use": {"web_search_requests": int, "web_fetch_requests": int} | None,
        "service_tier": str | None,
        "cache_creation": {"ephemeral_1h_input_tokens": int, "ephemeral_5m_input_tokens": int} | None,
        "inference_geo": str | None,
        "speed": str | None,
        "iterations": Any | None,
        "output_tokens_details": {"thinking_tokens": int | None} | None,
    },
    "toolStats": {  # Atividade de ferramenta agregada para a execução
        "readCount": int,
        "searchCount": int,
        "bashCount": int,
        "editFileCount": int,
        "linesAdded": int,
        "linesRemoved": int,
        "otherToolCount": int,
        "frameCount": int | None,
    } | None,
    "prompt": str,  # O prompt que o agente executou
    "worktreePath": str | None,  # Presente quando Claude Code manteve a worktree do subagente
    "worktreeBranch": str | None,  # Presente quando Claude Code criou essa worktree com git
}
```

**Saída (status: `"async_launched"`):**

```python theme={null}
{
    "status": "async_launched",
    "isAsync": bool | None,  # True em lançamentos em segundo plano
    "agentId": str,  # ID do agente lançado
    "description": str,  # A descrição da tarefa
    "resolvedModel": str | None,  # Modelo em uso na transição de backgrounding
    "modelsUsed": list[str] | None,  # Modelos usados antes do backgrounding, em ordem, com repetições consecutivas colapsadas
    "prompt": str,  # O prompt que o agente executa
    "outputFile": str,  # Caminho do arquivo onde a saída do agente é escrita
    "canReadOutputFile": bool | None,  # Se o arquivo de saída pode ser lido diretamente
}
```

**Saída (status: `"remote_launched"`):**

```python theme={null}
{
    "status": "remote_launched",
    "taskId": str,  # ID da tarefa despachada
    "sessionUrl": str,  # Link para a sessão em nuvem
    "description": str,  # A descrição da tarefa
    "prompt": str,  # O prompt que o agente executa
    "outputFile": str,  # Caminho do arquivo onde a saída do agente é escrita
}
```

Retorna o resultado do subagente. A saída é discriminada no campo `status`: `"completed"` para tarefas concluídas, `"async_launched"` para tarefas em segundo plano, e `"remote_launched"` para tarefas que Claude Code despachou para uma sessão em nuvem, onde `sessionUrl` vincula a essa sessão e `taskId` a identifica. Se Claude Code [manteve a worktree isolada do subagente](/docs/pt/worktrees#isolate-subagents-with-worktrees), `worktreePath` na variante `completed` é onde encontrá-la, e `worktreeBranch` é seu branch quando Claude Code criou a worktree com git.

Na variante `completed`, `resolvedModel` nomeia o modelo em que o subagente iniciou, que pode diferir do `model` de entrada solicitado quando [`availableModels`](/docs/pt/model-config#restrict-model-selection) ou outra substituição se aplica. Este campo requer Claude Code v2.1.174 ou posterior. Na variante `async_launched`, `resolvedModel` nomeia o modelo em uso quando o agente se moveu para o segundo plano, então uma troca que aconteceu antes do backgrounding é refletida lá. O campo `modelsUsed` em ambas as variantes lista os modelos usados em ordem, com repetições consecutivas colapsadas; é definido apenas quando o modelo foi trocado durante a execução. `modelsUsed` e o comportamento de `resolvedModel` no tempo de backgrounding requerem Claude Code v2.1.212 ou posterior.

Claude Code preenche `usage` e `totalTokens` da solicitação final da API do subagente, não de toda a execução. Quando presente, `thinking_tokens` sob `output_tokens_details` em `usage` é o número de tokens de saída dessa solicitação que eram tokens de pensamento. A chave `output_tokens_details` requer Python SDK v0.2.136 ou posterior, que agrupa Claude Code v2.1.228.

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
                    "preview": str | None,  # Conteúdo de visualização renderizado quando a opção está em foco
                }
            ],
            "multiSelect": bool,  # Defina como true para permitir múltiplas seleções
        }
    ],
    "answers": dict[str, str] | None,
    # Respostas do usuário preenchidas pelo sistema de permissões. Respostas
    # de múltipla seleção são uma string separada por vírgula dos rótulos selecionados; uma
    # lista de rótulos é aceita na entrada e coagida para essa forma
    "annotations": dict[str, dict] | None,
    # Anotações por pergunta do usuário, indexadas por texto da pergunta.
    # Cada valor pode conter "preview" (conteúdo de visualização da opção selecionada)
    # e "notes" (notas de texto livre na seleção)
    "metadata": dict | None,  # Metadados de análise, como {"source": "remember"}; não exibido ao usuário
}
```

**Saída:**

```python theme={null}
{
    "questions": [  # As perguntas que foram feitas
        {
            "question": str,
            "header": str,
            "options": [{"label": str, "description": str, "preview": str | None}],
            "multiSelect": bool,
        }
    ],
    "answers": dict[str, str],  # Mapeia texto da pergunta para string de resposta
    # Respostas de múltipla seleção são separadas por vírgula
    "response": str | None,
    # Resposta de forma livre digitada em vez de responder às perguntas; quando definido,
    # Claude recebe "O usuário respondeu: ..." no lugar da lista de respostas
    "annotations": dict[str, dict] | None,  # "preview" e "notes" por pergunta das seleções do usuário
    "afkTimeoutMs": int | None,  # Definido quando o diálogo se resolveu automaticamente após este muitos milissegundos de inatividade do usuário; ausente quando o usuário respondeu
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
    "stdout": str,  # Saída do comando; stdout e stderr chegam mesclados neste fluxo intercalado
    "stderr": str,  # Avisos que a própria ferramenta adiciona, não o stderr do comando
    "interrupted": bool,  # Se o comando foi interrompido
    "isImage": bool | None,  # Se stdout contém dados de imagem
    "backgroundTaskId": str | None,  # ID da tarefa em segundo plano se o comando está executando em segundo plano
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
    "timeout_ms": int | None,  # Prazo em milissegundos (padrão 300000, máx 3600000; o prazo efetivo é no máximo 1800000)
}
```

**Saída:**

```python theme={null}
{
    "taskId": str,  # ID da tarefa de monitor de fundo
    "timeoutMs": int,  # O prazo efetivo da observação em milissegundos
    "persistent": bool | None,  # False: cada observação tem um prazo
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
  The following tools are available by default only on Claude 3.x models, Opus 4 through 4.7, Sonnet 4 through 4.6, and Haiku 4.5. On every other model, including model IDs Claude Code doesn't recognize, they aren't available unless you opt in:

  * `TodoWrite`
  * `TaskCreate`
  * `TaskGet`
  * `TaskUpdate`
  * `TaskList`

  Wherever the tools are available, Claude Code provides the four Task tools, or `TodoWrite` instead when you set `CLAUDE_CODE_ENABLE_TASKS=0`.

  This default set applies in Claude Code v2.1.268 and later, which the TypeScript Agent SDK bundles from v0.3.268.

  Veja [Disponibilidade de modelo](/docs/pt/agent-sdk/todo-tracking#model-availability) para optar por participar.
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

<h3 id="taskoutput">
  TaskOutput
</h3>

Removido em Claude Code v2.1.277. Anteriormente recuperava saída de uma tarefa de fundo em execução ou concluída, com `BashOutput` aceito como alias; Claude lê o arquivo de saída de uma tarefa de fundo com `Read` em seu lugar.

Uma entrada `disallowed_tools` ou uma regra de negação que ainda nomeia qualquer um dos nomes é ignorada sem um aviso.

<h3 id="taskstop">
  TaskStop
</h3>

**Nome da ferramenta:** `TaskStop`. Os nomes anteriores `KillShell` e `KillBash` ainda são aceitos como aliases.

**Entrada:**

```python theme={null}
{
    "task_id": str | None,  # O ID da tarefa de fundo a parar
    "shell_id": str | None,  # Descontinuado: use task_id em seu lugar
}
```

**Saída:**

```python theme={null}
{
    "message": str,  # Mensagem de status sobre a operação
    "task_id": str,  # O ID da tarefa que foi parada
    "task_type": str,  # O tipo da tarefa que foi parada
    "command": str | None,  # O comando ou descrição da tarefa parada
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

<h2 id="build-a-continuous-conversation-interface">
  Construir uma interface de conversa contínua
</h2>

O exemplo a seguir mantém um `ClaudeSDKClient` conectado entre turnos, para que Claude se lembre das mensagens anteriores. Digite `new` para desconectar e reconectar para uma nova sessão, ou `exit` para encerrar a conversa.

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

<h2 id="error-handling">
  Tratamento de erros
</h2>

O exemplo a seguir envolve uma chamada `query()` em manipuladores para quatro dos [tipos de erro](#error-types) que o SDK levanta.

Este exemplo captura [`ResultError`](#resulterror), que requer Python Agent SDK 0.2.140 ou posterior.

```python theme={null}
import asyncio

from claude_agent_sdk import (
    query,
    CLINotFoundError,
    ProcessError,
    ResultError,
    CLIJSONDecodeError,
)


async def main():
    try:
        async for message in query(prompt="Hello"):
            print(message)
    except CLINotFoundError:
        print(
            "Claude Code CLI not found. Try reinstalling: pip install --force-reinstall claude-agent-sdk"
        )
    # Catch ResultError before ProcessError, which it subclasses. Its message
    # carries the error text. A failed final request, such as an API error,
    # arrives with subtype "success", so branch on terminal_reason first.
    except ResultError as e:
        if e.terminal_reason == "api_error":
            print(f"API request failed: {e}")
        else:
            print(f"Query ended with an error result ({e.terminal_reason or e.subtype}): {e}")
    except ProcessError as e:
        print(f"Process failed with exit code: {e.exit_code}")
    except CLIJSONDecodeError as e:
        print(f"Failed to parse response: {e}")


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

| Propriedade                 | Tipo                                                  | Padrão  | Descrição                                                                                                                                                                                                                                                  |
| :-------------------------- | :---------------------------------------------------- | :------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `bool`                                                | `False` | Ativa modo sandbox para execução de comando                                                                                                                                                                                                                |
| `autoAllowBashIfSandboxed`  | `bool`                                                | `True`  | Auto-aprova comandos bash quando sandbox está ativado                                                                                                                                                                                                      |
| `excludedCommands`          | `list[str]`                                           | `[]`    | Comandos que contornam restrições de sandbox, como `["docker *"]`. Esses executam sem sandbox automaticamente sem envolvimento do modelo; [`sandbox.excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands) cobre quando uma entrada se aplica |
| `allowUnsandboxedCommands`  | `bool`                                                | `True`  | Permite que o modelo solicite executar comandos fora do sandbox. Quando `True`, o modelo pode definir `dangerouslyDisableSandbox` na entrada da ferramenta, que volta para o [sistema de permissões](#permissions-fallback-for-unsandboxed-commands)       |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `None`  | Configuração de sandbox específica de rede                                                                                                                                                                                                                 |
| `ignoreViolations`          | [`SandboxIgnoreViolations`](#sandboxignoreviolations) | `None`  | Configure quais violações de sandbox ignorar                                                                                                                                                                                                               |
| `enableWeakerNestedSandbox` | `bool`                                                | `False` | Ativa um sandbox aninhado mais fraco para compatibilidade                                                                                                                                                                                                  |

<Note>
  O sandbox depende do suporte de plataforma e, no Linux, de ferramentas como `bubblewrap` e `socat`. Por padrão, quando `enabled` é `True` mas o sandbox não consegue iniciar, comandos executam sem sandbox com um aviso em stderr. Este padrão difere do SDK TypeScript, onde `failIfUnavailable` tem padrão `true`.

  Defina `"failIfUnavailable": True` nas suas configurações de sandbox para parar em vez disso. A chave ainda não está declarada em `SandboxSettings`, mas o SDK a encaminha para Claude Code, que a honra. `query()` então relata uma `ResultMessage` com `subtype="error_during_execution"` e a razão em `errors`. Como esta é uma chamada `query()` de um único disparo, o SDK lança após ceder esse resultado de erro, então envolva o loop em um bloco try para continuar além dele. Veja [Lidar com o resultado](/docs/pt/agent-sdk/agent-loop#handle-the-result) para o contrato de erro.
</Note>

<h4 id="example-usage">
  Exemplo de uso
</h4>

```python theme={null}
import asyncio

from claude_agent_sdk import query, ClaudeAgentOptions

sandbox_settings = {
    "enabled": True,
    "autoAllowBashIfSandboxed": True,
    "failIfUnavailable": True,
    "network": {"allowLocalBinding": True},
}


async def main():
    try:
        async for message in query(
            prompt="Build and test my project",
            options=ClaudeAgentOptions(sandbox=sandbox_settings),
        ):
            print(message)
    except Exception as error:
        # A single-shot query() raises after yielding an error result,
        # such as when failIfUnavailable is set and the sandbox can't start.
        print(f"Session ended with an error: {error}")


asyncio.run(main())
```

<Warning>
  **Segurança de socket Unix**: A opção `allowUnixSockets` pode conceder acesso a serviços de sistema que alcançam fora do sandbox. Por exemplo, permitir `/var/run/docker.sock` efetivamente concede acesso completo ao sistema host através da API Docker, contornando isolamento de sandbox. Apenas permita sockets Unix que são estritamente necessários e entenda as implicações de segurança de cada um.
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

| Propriedade               | Tipo        | Padrão  | Descrição                                                                                                                                                                                                                                      |
| :------------------------ | :---------- | :------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `list[str]` | `[]`    | Nomes de domínio que processos em sandbox podem acessar                                                                                                                                                                                        |
| `deniedDomains`           | `list[str]` | `[]`    | Nomes de domínio que processos em sandbox não podem acessar. Tem precedência sobre `allowedDomains`                                                                                                                                            |
| `allowManagedDomainsOnly` | `bool`      | `False` | Apenas configurações gerenciadas: quando definido em configurações gerenciadas, ignore `allowedDomains` e `WebFetch(domain:...)` regras de permissão de fontes de configurações não gerenciadas. Não tem efeito quando definido via opções SDK |
| `allowUnixSockets`        | `list[str]` | `[]`    | Apenas macOS: caminhos de socket Unix que processos podem acessar, como o socket Docker. Ignorado no Linux                                                                                                                                     |
| `allowAllUnixSockets`     | `bool`      | `False` | Permite acesso a todos os sockets Unix                                                                                                                                                                                                         |
| `allowLocalBinding`       | `bool`      | `False` | Permite que processos se vinculem a portas locais (por exemplo, para servidores dev)                                                                                                                                                           |
| `allowMachLookup`         | `list[str]` | `[]`    | Apenas macOS: nomes de serviço XPC/Mach para permitir. Suporta um curinga à direita                                                                                                                                                            |
| `httpProxyPort`           | `int`       | `None`  | Porta de proxy HTTP para solicitações de rede                                                                                                                                                                                                  |
| `socksProxyPort`          | `int`       | `None`  | Porta de proxy SOCKS para solicitações de rede                                                                                                                                                                                                 |

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

Suas entradas `excludedCommands` em vez disso contornam o sandbox com nenhum envolvimento do modelo; [`sandbox.excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands) cobre quando uma entrada se aplica.

O exemplo a seguir registra cada solicitação sem sandbox e a nega a menos que sua própria lógica de autorização a permita:

```python theme={null}
import asyncio
from claude_agent_sdk import (
    query,
    ClaudeAgentOptions,
    HookMatcher,
    PermissionResultAllow,
    PermissionResultDeny,
    ToolPermissionContext,
)


def is_command_authorized(command: str | None) -> bool:
    # Replace with your own authorization logic
    return False



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


asyncio.run(main())
```

<Warning>
  Comandos executando com `dangerouslyDisableSandbox: True` têm acesso completo ao sistema. Certifique-se de que seu manipulador `can_use_tool` valida essas solicitações cuidadosamente.

  Se `permission_mode` está definido para `bypassPermissions` e `allow_unsandboxed_commands` está ativado, o modelo pode autonomamente executar comandos fora do sandbox sem prompts de aprovação, além das [ações que nenhum modo auto-aprova](/docs/pt/permission-modes#actions-no-mode-auto-approves). Esta combinação efetivamente permite que o modelo escape do isolamento de sandbox silenciosamente.
</Warning>

<h2 id="see-also">
  Veja também
</h2>

* [SDK overview](/docs/pt/agent-sdk/overview) - Conceitos gerais do SDK
* [TypeScript SDK reference](/docs/pt/agent-sdk/typescript) - Documentação do SDK TypeScript
* [Custom tools](/docs/pt/agent-sdk/custom-tools) - Defina ferramentas MCP em processo para Claude chamar
* [CLI reference](/docs/pt/cli-reference) - Interface de linha de comando
* [Common workflows](/docs/pt/common-workflows) - Guias passo a passo
