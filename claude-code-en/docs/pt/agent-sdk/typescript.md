> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Referência do Agent SDK - TypeScript

> Referência completa da API para o Agent SDK TypeScript, incluindo todas as funções, tipos e interfaces.

<script src="/docs/components/typescript-sdk-type-links.js" defer />

<h2 id="installation">
  Instalação
</h2>

```bash theme={null}
npm install @anthropic-ai/claude-agent-sdk
```

<Note>
  O SDK agrupa um binário nativo do Claude Code para sua plataforma como uma dependência opcional, como `@anthropic-ai/claude-agent-sdk-darwin-arm64`. Você não precisa instalar o Claude Code separadamente. Se seu gerenciador de pacotes pular dependências opcionais, o SDK lança `Native CLI binary for <platform> not found`; defina [`pathToClaudeCodeExecutable`](#options) para um binário `claude` instalado separadamente.
</Note>

<h3 id="compile-to-a-single-executable">
  Compilar para um executável único
</h3>

Quando você compila sua aplicação em um executável de arquivo único com `bun build --compile`, o SDK não consegue resolver o binário CLI agrupado em tempo de execução. `require.resolve` não funciona dentro do sistema de arquivos virtual `$bunfs` do executável compilado, então o SDK lança `Native CLI binary for <platform> not found`.

Para contornar isso, incorpore o binário da plataforma como um ativo de arquivo, extraia-o para um caminho real na inicialização com `extractFromBunfs()` e passe esse caminho para [`pathToClaudeCodeExecutable`](#options).

O auxiliar `extractFromBunfs()` requer `@anthropic-ai/claude-agent-sdk` v0.3.144 ou posterior. O exemplo abaixo compila para macOS no Apple Silicon:

```typescript theme={null}
import binPath from "@anthropic-ai/claude-agent-sdk-darwin-arm64/claude" with { type: "file" };
import { extractFromBunfs } from "@anthropic-ai/claude-agent-sdk/extract";
import { query } from "@anthropic-ai/claude-agent-sdk";

const cliPath = extractFromBunfs(binPath);

for await (const message of query({
  prompt: "Hello",
  options: { pathToClaudeCodeExecutable: cliPath },
})) {
  console.log(message);
}
```

`extractFromBunfs()` copia o binário incorporado do sistema de arquivos virtual do executável compilado para um diretório temporário por usuário e retorna o caminho real. Fora de um executável compilado, ele retorna o caminho de entrada inalterado, então o mesmo código é executado em desenvolvimento sem modificação.

Cada executável compilado incorpora o binário de uma única plataforma. Corresponda o pacote da plataforma na importação ao seu `--target`:

* Para compilação cruzada, instale o pacote de plataforma não correspondente, por exemplo `npm install @anthropic-ai/claude-agent-sdk-linux-x64 --force`.
* No Windows, o subcaminho do binário é `claude.exe`, por exemplo `@anthropic-ai/claude-agent-sdk-win32-x64/claude.exe`.

<h2 id="functions">
  Funções
</h2>

<h3 id="query">
  `query()`
</h3>

A função principal para interagir com o Claude Code. Cria um gerador assíncrono que transmite mensagens conforme chegam.

```typescript theme={null}
function query({
  prompt,
  options
}: {
  prompt: string | AsyncIterable<SDKUserMessage>;
  options?: Options;
}): Query;
```

<h4 id="parameters">
  Parâmetros
</h4>

| Parâmetro | Tipo                                                             | Descrição                                                                           |
| :-------- | :--------------------------------------------------------------- | :---------------------------------------------------------------------------------- |
| `prompt`  | `string \| AsyncIterable<`[`SDKUserMessage`](#sdkusermessage)`>` | O prompt de entrada como uma string ou iterável assíncrono para modo de transmissão |
| `options` | [`Options`](#options)                                            | Objeto de configuração opcional (veja o tipo Options abaixo)                        |

<h4 id="returns">
  Retorna
</h4>

Retorna um objeto [`Query`](#query-object) que estende `AsyncGenerator<`[`SDKMessage`](#sdkmessage)`, void>` com métodos adicionais.

<h3 id="startup">
  `startup()`
</h3>

Pré-aquece o subprocesso CLI gerando-o e completando o handshake de inicialização antes de um prompt estar disponível. O handle [`WarmQuery`](#warmquery) retornado aceita um prompt depois e o escreve em um processo já pronto, então a primeira chamada `query()` é resolvida sem pagar o custo de geração e inicialização do subprocesso inline.

```typescript theme={null}
function startup(params?: {
  options?: Options;
  initializeTimeoutMs?: number;
}): Promise<WarmQuery>;
```

<h4 id="parameters-2">
  Parâmetros
</h4>

| Parâmetro             | Tipo                  | Descrição                                                                                                                                                                                 |
| :-------------------- | :-------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options`             | [`Options`](#options) | Objeto de configuração opcional. Igual ao parâmetro `options` para `query()`                                                                                                              |
| `initializeTimeoutMs` | `number`              | Tempo máximo em milissegundos para aguardar a inicialização do subprocesso. Padrão é `60000`. Se a inicialização não for concluída no tempo, a promise é rejeitada com um erro de timeout |

<h4 id="returns-2">
  Retorna
</h4>

Retorna uma `Promise<`[`WarmQuery`](#warmquery)`>` que é resolvida assim que o subprocesso é gerado e completa seu handshake de inicialização.

<h4 id="example">
  Exemplo
</h4>

Chame `startup()` cedo, por exemplo no boot da aplicação, depois chame `.query()` no handle retornado assim que um prompt estiver pronto. Isso move a geração do subprocesso e inicialização para fora do caminho crítico.

```typescript theme={null}
import { startup } from "@anthropic-ai/claude-agent-sdk";

// Pague o custo de inicialização antecipadamente
const warm = await startup({ options: { maxTurns: 3 } });

// Depois, quando um prompt estiver pronto, isso é imediato
for await (const message of warm.query("What files are here?")) {
  console.log(message);
}
```

<h3 id="tool">
  `tool()`
</h3>

Cria uma definição de ferramenta MCP type-safe para uso com servidores MCP do SDK.

```typescript theme={null}
function tool<Schema extends AnyZodRawShape>(
  name: string,
  description: string,
  inputSchema: Schema,
  handler: (args: InferShape<Schema>, extra: unknown) => Promise<CallToolResult>,
  extras?: { annotations?: ToolAnnotations }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Parâmetros
</h4>

| Parâmetro     | Tipo                                                              | Descrição                                                                           |
| :------------ | :---------------------------------------------------------------- | :---------------------------------------------------------------------------------- |
| `name`        | `string`                                                          | O nome da ferramenta                                                                |
| `description` | `string`                                                          | Uma descrição do que a ferramenta faz                                               |
| `inputSchema` | `Schema extends AnyZodRawShape`                                   | Schema Zod definindo os parâmetros de entrada da ferramenta (suporta Zod 3 e Zod 4) |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>` | Função assíncrona que executa a lógica da ferramenta                                |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)` }`       | Anotações MCP opcionais da ferramenta fornecendo dicas comportamentais aos clientes |

<h4 id="toolannotations">
  `ToolAnnotations`
</h4>

Re-exportado de `@modelcontextprotocol/sdk/types.js`. Todos os campos são dicas opcionais; os clientes não devem confiar neles para decisões de segurança.

| Campo             | Tipo      | Padrão      | Descrição                                                                                                                                                                   |
| :---------------- | :-------- | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `title`           | `string`  | `undefined` | Título legível para a ferramenta                                                                                                                                            |
| `readOnlyHint`    | `boolean` | `false`     | Se `true`, a ferramenta não modifica seu ambiente                                                                                                                           |
| `destructiveHint` | `boolean` | `true`      | Se `true`, a ferramenta pode realizar atualizações destrutivas (apenas significativo quando `readOnlyHint` é `false`)                                                       |
| `idempotentHint`  | `boolean` | `false`     | Se `true`, chamadas repetidas com os mesmos argumentos não têm efeito adicional (apenas significativo quando `readOnlyHint` é `false`)                                      |
| `openWorldHint`   | `boolean` | `true`      | Se `true`, a ferramenta interage com entidades externas (por exemplo, busca na web). Se `false`, o domínio da ferramenta é fechado (por exemplo, uma ferramenta de memória) |

```typescript theme={null}
import { tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

const searchTool = tool(
  "search",
  "Search the web",
  { query: z.string() },
  async ({ query }) => {
    return { content: [{ type: "text", text: `Results for: ${query}` }] };
  },
  { annotations: { readOnlyHint: true, openWorldHint: true } }
);
```

<h3 id="createsdkmcpserver">
  `createSdkMcpServer()`
</h3>

Cria uma instância de servidor MCP que é executada no mesmo processo que sua aplicação.

```typescript theme={null}
function createSdkMcpServer(options: {
  name: string;
  version?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Parâmetros
</h4>

| Parâmetro         | Tipo                          | Descrição                                                        |
| :---------------- | :---------------------------- | :--------------------------------------------------------------- |
| `options.name`    | `string`                      | O nome do servidor MCP                                           |
| `options.version` | `string`                      | String de versão opcional                                        |
| `options.tools`   | `Array<SdkMcpToolDefinition>` | Array de definições de ferramentas criadas com [`tool()`](#tool) |

<h3 id="listsessions">
  `listSessions()`
</h3>

Descobre e lista sessões passadas com metadados leves. Filtre por diretório de projeto ou liste sessões em todos os projetos.

```typescript theme={null}
function listSessions(options?: ListSessionsOptions): Promise<SDKSessionInfo[]>;
```

<h4 id="parameters-5">
  Parâmetros
</h4>

| Parâmetro                  | Tipo      | Padrão      | Descrição                                                                                       |
| :------------------------- | :-------- | :---------- | :---------------------------------------------------------------------------------------------- |
| `options.dir`              | `string`  | `undefined` | Diretório para listar sessões. Quando omitido, retorna sessões em todos os projetos             |
| `options.limit`            | `number`  | `undefined` | Número máximo de sessões a retornar                                                             |
| `options.includeWorktrees` | `boolean` | `true`      | Quando `dir` está dentro de um repositório git, inclua sessões de todos os caminhos de worktree |

<h4 id="return-type-sdksessioninfo">
  Tipo de retorno: `SDKSessionInfo`
</h4>

| Propriedade    | Tipo                  | Descrição                                                                                  |
| :------------- | :-------------------- | :----------------------------------------------------------------------------------------- |
| `sessionId`    | `string`              | Identificador único de sessão (UUID)                                                       |
| `summary`      | `string`              | Título de exibição: título personalizado, resumo gerado automaticamente ou primeiro prompt |
| `lastModified` | `number`              | Tempo da última modificação em milissegundos desde a época                                 |
| `fileSize`     | `number \| undefined` | Tamanho do arquivo de sessão em bytes. Apenas preenchido para armazenamento JSONL local    |
| `customTitle`  | `string \| undefined` | Título de sessão definido pelo usuário (via `/rename`)                                     |
| `firstPrompt`  | `string \| undefined` | Primeiro prompt de usuário significativo na sessão                                         |
| `gitBranch`    | `string \| undefined` | Branch git no final da sessão                                                              |
| `cwd`          | `string \| undefined` | Diretório de trabalho para a sessão                                                        |
| `tag`          | `string \| undefined` | Tag de sessão definida pelo usuário (veja [`tagSession()`](#tagsession))                   |
| `createdAt`    | `number \| undefined` | Tempo de criação em milissegundos desde a época, do timestamp da primeira entrada          |

<h4 id="example-2">
  Exemplo
</h4>

Imprima as 10 sessões mais recentes para um projeto. Os resultados são classificados por `lastModified` descendente, então o primeiro item é o mais novo. Omita `dir` para pesquisar em todos os projetos.

```typescript theme={null}
import { listSessions } from "@anthropic-ai/claude-agent-sdk";

const sessions = await listSessions({ dir: "/path/to/project", limit: 10 });

for (const session of sessions) {
  console.log(`${session.summary} (${session.sessionId})`);
}
```

<h3 id="getsessionmessages">
  `getSessionMessages()`
</h3>

Lê mensagens de usuário e assistente de uma transcrição de sessão passada.

```typescript theme={null}
function getSessionMessages(
  sessionId: string,
  options?: GetSessionMessagesOptions
): Promise<SessionMessage[]>;
```

<h4 id="parameters-6">
  Parâmetros
</h4>

| Parâmetro        | Tipo     | Padrão      | Descrição                                                                                |
| :--------------- | :------- | :---------- | :--------------------------------------------------------------------------------------- |
| `sessionId`      | `string` | obrigatório | UUID da sessão a ler (veja `listSessions()`)                                             |
| `options.dir`    | `string` | `undefined` | Diretório do projeto para encontrar a sessão. Quando omitido, pesquisa todos os projetos |
| `options.limit`  | `number` | `undefined` | Número máximo de mensagens a retornar                                                    |
| `options.offset` | `number` | `undefined` | Número de mensagens a pular do início                                                    |

<h4 id="return-type-sessionmessage">
  Tipo de retorno: `SessionMessage`
</h4>

| Propriedade          | Tipo                    | Descrição                                                                                                                                                                                                                                                                     |
| :------------------- | :---------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Papel da mensagem                                                                                                                                                                                                                                                             |
| `uuid`               | `string`                | Identificador único de mensagem                                                                                                                                                                                                                                               |
| `session_id`         | `string`                | Sessão a que esta mensagem pertence                                                                                                                                                                                                                                           |
| `message`            | `unknown`               | Payload de mensagem bruta da transcrição                                                                                                                                                                                                                                      |
| `parent_tool_use_id` | `string \| null`        | Para mensagens de subagente, o `tool_use_id` da chamada de ferramenta `Agent` geradora. `null` para mensagens de sessão principal e sessões mais antigas                                                                                                                      |
| `parent_agent_id`    | `string \| null`        | Para mensagens de um [subagente aninhado](/docs/pt/sub-agents#spawn-nested-subagents), o `agentId` do subagente que o gerou. `null` para mensagens de sessão principal, mensagens de subagentes de nível superior e sessões mais antigas. Requer Claude Code v2.1.202 ou posterior |

<h4 id="example-3">
  Exemplo
</h4>

```typescript theme={null}
import { listSessions, getSessionMessages } from "@anthropic-ai/claude-agent-sdk";

const [latest] = await listSessions({ dir: "/path/to/project", limit: 1 });

if (latest) {
  const messages = await getSessionMessages(latest.sessionId, {
    dir: "/path/to/project",
    limit: 20
  });

  for (const msg of messages) {
    console.log(`[${msg.type}] ${msg.uuid}`);
  }
}
```

<h3 id="getsessioninfo">
  `getSessionInfo()`
</h3>

Lê metadados para uma única sessão por ID sem verificar o diretório do projeto completo.

```typescript theme={null}
function getSessionInfo(
  sessionId: string,
  options?: GetSessionInfoOptions
): Promise<SDKSessionInfo | undefined>;
```

<h4 id="parameters-7">
  Parâmetros
</h4>

| Parâmetro     | Tipo     | Padrão      | Descrição                                                                                |
| :------------ | :------- | :---------- | :--------------------------------------------------------------------------------------- |
| `sessionId`   | `string` | obrigatório | UUID da sessão a procurar                                                                |
| `options.dir` | `string` | `undefined` | Caminho do diretório do projeto. Quando omitido, pesquisa todos os diretórios de projeto |

Retorna [`SDKSessionInfo`](#return-type-sdksessioninfo), ou `undefined` se a sessão não for encontrada.

<h3 id="renamesession">
  `renameSession()`
</h3>

Renomeia uma sessão anexando uma entrada de título personalizado. Chamadas repetidas são seguras; o título mais recente vence.

```typescript theme={null}
function renameSession(
  sessionId: string,
  title: string,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-8">
  Parâmetros
</h4>

| Parâmetro     | Tipo     | Padrão      | Descrição                                                                                |
| :------------ | :------- | :---------- | :--------------------------------------------------------------------------------------- |
| `sessionId`   | `string` | obrigatório | UUID da sessão a renomear                                                                |
| `title`       | `string` | obrigatório | Novo título. Deve ser não-vazio após aparar espaços em branco                            |
| `options.dir` | `string` | `undefined` | Caminho do diretório do projeto. Quando omitido, pesquisa todos os diretórios de projeto |

<h3 id="tagsession">
  `tagSession()`
</h3>

Marca uma sessão. Passe `null` para limpar a tag. Chamadas repetidas são seguras; a tag mais recente vence.

```typescript theme={null}
function tagSession(
  sessionId: string,
  tag: string | null,
  options?: SessionMutationOptions
): Promise<void>;
```

<h4 id="parameters-9">
  Parâmetros
</h4>

| Parâmetro     | Tipo             | Padrão      | Descrição                                                                                |
| :------------ | :--------------- | :---------- | :--------------------------------------------------------------------------------------- |
| `sessionId`   | `string`         | obrigatório | UUID da sessão a marcar                                                                  |
| `tag`         | `string \| null` | obrigatório | String de tag, ou `null` para limpar                                                     |
| `options.dir` | `string`         | `undefined` | Caminho do diretório do projeto. Quando omitido, pesquisa todos os diretórios de projeto |

<h3 id="resolvesettings">
  `resolveSettings()`
</h3>

Resolve as configurações efetivas do Claude Code para um determinado diretório usando o mesmo mecanismo de mesclagem que o CLI, sem gerar o Claude CLI. Use-o para inspecionar qual configuração uma chamada `query()` veria antes de invocar uma.

<Note>
  Esta função é alfa e sua API pode mudar antes da estabilização. Ela lê fontes MDM, incluindo plist do macOS e HKLM/HKCU do Windows, para paridade com inicialização do CLI, mas não executa o subprocesso `policyHelper` configurado pelo administrador. O campo `permissions.defaultMode` é retornado como está de todos os níveis, incluindo configurações de projeto. O filtro de confiança que o CLI aplica antes de honrar modos de permissão crescentes não é aplicado.
</Note>

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Parâmetros
</h4>

`resolveSettings()` aceita um único objeto de opções. Todos os campos são opcionais.

| Parâmetro                       | Tipo                                  | Padrão          | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :------------------------------ | :------------------------------------ | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `options.cwd`                   | `string`                              | `process.cwd()` | Diretório para resolver configurações de projeto e local relativas a                                                                                                                                                                                                                                                                                                                                                                  |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Todas as fontes | Quais fontes do sistema de arquivos carregar. Passe `[]` para pular configurações de usuário, projeto e local. Configurações de política gerenciada carregam em todos os casos. Configurações gerenciadas pelo servidor são obtidas de `serverManagedSettings` quando o host as passa, ou lidas do cache em disco do CLI caso contrário; o snapshot não as busca da rede                                                              |
| `options.managedSettings`       | `Settings`                            | `undefined`     | Configurações de política restritiva fornecidas pelo host de incorporação. Descartadas por padrão quando uma camada gerenciada implantada pelo administrador está presente; mescladas sob essa camada quando [`parentSettingsBehavior`](/docs/pt/settings#available-settings) é `"merge"`. Chaves não restritivas como `model` são silenciosamente descartadas para que essa opção possa apertar a política gerenciada, mas não afrouxá-la |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | Payload de configurações gerenciadas pelo servidor de `/api/claude_code/settings`. Chaves não restritivas passam sem filtro                                                                                                                                                                                                                                                                                                           |

<h4 id="return-type-resolvedsettings">
  Tipo de retorno: `ResolvedSettings`
</h4>

`resolveSettings()` retorna um objeto descrevendo as configurações mescladas e a fonte que contribuiu para cada chave.

| Propriedade  | Tipo                                                | Descrição                                                                                |
| :----------- | :-------------------------------------------------- | :--------------------------------------------------------------------------------------- |
| `effective`  | `Settings`                                          | Configurações mescladas após aplicar todas as fontes habilitadas em ordem de precedência |
| `provenance` | `Partial<Record<keyof Settings, ProvenanceEntry>>`  | Para cada chave de nível superior em `effective`, qual fonte forneceu o valor            |
| `sources`    | `Array<{ source, settings, path?, policyOrigin? }>` | Configurações brutas por fonte, ordenadas de precedência mais baixa para mais alta       |

<h4 id="example-4">
  Exemplo
</h4>

O exemplo abaixo resolve configurações para um diretório de projeto e imprime a fonte que controla o período de limpeza.

```typescript theme={null}
import { resolveSettings } from "@anthropic-ai/claude-agent-sdk";

const { effective, provenance } = await resolveSettings({
  cwd: "/path/to/project",
  settingSources: ["user", "project", "local"],
});

console.log(`Cleanup period: ${effective.cleanupPeriodDays} days`);
console.log(`Set by: ${provenance.cleanupPeriodDays?.source}`);
```

<h2 id="types">
  Tipos
</h2>

<h3 id="options">
  `Options`
</h3>

Objeto de configuração para a função `query()`.

| Propriedade                       | Tipo                                                                                                     | Padrão                                         | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------- | :--------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                        | `new AbortController()`                        | Controlador para cancelar operações                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `additionalDirectories`           | `string[]`                                                                                               | `[]`                                           | Diretórios adicionais que Claude pode acessar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `agent`                           | `string`                                                                                                 | `undefined`                                    | Nome do agente para a thread principal. O agente deve ser definido na opção `agents` ou em configurações                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                  | `undefined`                                    | Defina subagentes programaticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `agentProgressSummaries`          | `boolean`                                                                                                | `false`                                        | Quando `true`, gera resumos de progresso de uma linha para subagentes e os encaminha em eventos [`task_progress`](#sdktaskprogressmessage) através do campo `summary`. Aplica-se a subagentes em primeiro plano e em segundo plano                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                | `false`                                        | Ativar bypass de permissões. Obrigatório ao usar `permissionMode: 'bypassPermissions'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `allowedTools`                    | `string[]`                                                                                               | `[]`                                           | Ferramentas para auto-aprovar sem solicitar. Isso não restringe Claude apenas a essas ferramentas; ferramentas não listadas caem em `permissionMode` e `canUseTool`. Use `disallowedTools` para bloquear ferramentas. Veja [Permissões](/docs/pt/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                             |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                | `[]`                                           | Ativar recursos beta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                              | `undefined`                                    | Função de permissão personalizada, invocada apenas quando o [fluxo de permissão](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) cai em um prompt. Não invocada para chamadas auto-aprovadas por `allowedTools`, regras de permissão, ou `permissionMode`. `AskUserQuestion`, ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools), e ferramentas MCP marcadas [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool) a alcançam mesmo se você as permitiu; em modo `dontAsk` essas são negadas em vez disso. Veja [`CanUseTool`](#canusetool) para detalhes |
| `continue`                        | `boolean`                                                                                                | `false`                                        | Continuar a conversa mais recente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `cwd`                             | `string`                                                                                                 | `process.cwd()`                                | Diretório de trabalho atual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `debug`                           | `boolean`                                                                                                | `false`                                        | Ativar modo de depuração para o processo Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `debugFile`                       | `string`                                                                                                 | `undefined`                                    | Escrever logs de depuração em um caminho de arquivo específico. Ativa implicitamente o modo de depuração                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `disallowedTools`                 | `string[]`                                                                                               | `[]`                                           | Ferramentas para negar. Um nome simples como `"Bash"` remove a ferramenta do contexto do Claude. Uma regra com escopo como `"Bash(rm *)"` deixa a ferramenta disponível e nega chamadas correspondentes em todos os modos de permissão, incluindo `bypassPermissions`. Veja [Permissões](/docs/pt/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                            |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                        | Padrão do modelo                               | Controla quanto esforço Claude coloca em sua resposta. Funciona com pensamento adaptativo para guiar a profundidade do pensamento. Veja [ajustar o nível de esforço](/docs/pt/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `enableFileCheckpointing`         | `boolean`                                                                                                | `false`                                        | Ativar rastreamento de mudanças de arquivo para retrocesso. Veja [File checkpointing](/docs/pt/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `env`                             | `Record<string, string \| undefined>`                                                                    | `process.env`                                  | Variáveis de ambiente. Quando definido, isso substitui o ambiente do subprocesso em vez de mesclar com `process.env`, então passe `{ ...process.env, YOUR_VAR: 'value' }` para manter variáveis herdadas como `PATH`. Veja [Lidar com respostas de API lentas ou travadas](#handle-slow-or-stalled-api-responses) para um exemplo deste padrão, e [Variáveis de ambiente](/docs/pt/env-vars) para variáveis que a CLI subjacente lê. Defina `CLAUDE_AGENT_SDK_CLIENT_APP` para identificar sua aplicação no cabeçalho User-Agent                                                                                                                         |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                              | Auto-detectado                                 | Runtime JavaScript a usar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `executableArgs`                  | `string[]`                                                                                               | `[]`                                           | Argumentos a passar para o executável                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `extraArgs`                       | `Record<string, string \| null>`                                                                         | `{}`                                           | Argumentos adicionais                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `fallbackModel`                   | `string`                                                                                                 | `undefined`                                    | Modelo a usar se o primário falhar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `forkSession`                     | `boolean`                                                                                                | `false`                                        | Ao retomar com `resume`, bifurcar para um novo ID de sessão em vez de continuar a sessão original                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `forwardSubagentText`             | `boolean`                                                                                                | `false`                                        | Encaminhar blocos de texto e pensamento de subagentes como mensagens de assistente e usuário com `parent_tool_use_id` definido, para que os consumidores possam renderizar uma transcrição aninhada. Por padrão, apenas blocos `tool_use` e `tool_result` de subagentes são emitidos                                                                                                                                                                                                                                                                                                                                                                |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`        | `{}`                                           | Callbacks de hook para eventos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `includeHookEvents`               | `boolean`                                                                                                | `false`                                        | Incluir eventos de ciclo de vida de hook para cada evento de hook no fluxo de mensagens como [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage) e [`SDKHookResponseMessage`](#sdkhookresponsemessage). Eventos de ciclo de vida para hooks `SessionStart` e `Setup` são sempre incluídos e não precisam desta opção                                                                                                                                                                                                                                                                             |
| `includePartialMessages`          | `boolean`                                                                                                | `false`                                        | Incluir eventos de mensagem parcial                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `loadTimeoutMs`                   | `number`                                                                                                 | `60000`                                        | *Alfa.* Timeout em milissegundos para cada chamada `sessionStore.load()` e `sessionStore.listSubkeys()` durante materialização de retomada. Se o adaptador não se resolver dentro desta janela, a consulta falha em vez de travar. Ignorado quando `sessionStore` não está definido                                                                                                                                                                                                                                                                                                                                                                 |
| `managedSettings`                 | `Settings`                                                                                               | `undefined`                                    | Configurações de nível de política fornecidas pelo processo pai que está gerando. Descartadas quando uma camada de configurações gerenciadas controlada por TI já existe na máquina, a menos que esse administrador opte por `parentSettingsBehavior: 'merge'`. Filtradas apenas para chaves restritivas independentemente                                                                                                                                                                                                                                                                                                                          |
| `maxBudgetUsd`                    | `number`                                                                                                 | `undefined`                                    | Parar a consulta quando a estimativa de custo do lado do cliente atingir este valor em USD. Comparado com a mesma estimativa que `total_cost_usd`; veja [Rastrear custo e uso](/docs/pt/agent-sdk/cost-tracking) para ressalvas de precisão                                                                                                                                                                                                                                                                                                                                                                                                              |
| `maxThinkingTokens`               | `number`                                                                                                 | `undefined`                                    | *Descontinuado:* Use `thinking` em vez disso. Tokens máximos para processo de pensamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `maxTurns`                        | `number`                                                                                                 | `undefined`                                    | Turnos agênticos máximos (round trips de uso de ferramenta)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                  | `{}`                                           | Configurações de servidor MCP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `model`                           | `string`                                                                                                 | Padrão da CLI                                  | Alias de modelo Claude ou nome de modelo completo. Veja [valores aceitos e IDs específicos do provedor](/docs/pt/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`          | `undefined`                                    | Callback para lidar com solicitações de elicitação MCP. Chamado quando um servidor MCP solicita entrada do usuário e nenhum hook a trata primeiro. Quando não fornecido, solicitações de elicitação não tratadas são recusadas automaticamente                                                                                                                                                                                                                                                                                                                                                                                                      |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                            | `undefined`                                    | Defina o formato de saída para resultados de agente. Veja [Structured outputs](/docs/pt/agent-sdk/structured-outputs) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `outputStyle`                     | `string`                                                                                                 | `undefined`                                    | Não é um campo `Options`. Defina `outputStyle` no objeto [`settings`](/docs/pt/settings) inline ou em um arquivo de configurações. Veja [Ativar um estilo de saída](/docs/pt/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                 | Auto-resolvido do binário nativo agrupado      | Caminho para executável Claude Code. Apenas necessário se dependências opcionais foram puladas durante a instalação ou sua plataforma não está no conjunto suportado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                      | `'default'`                                    | Modo de permissão para a sessão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `permissionPromptToolName`        | `string`                                                                                                 | `undefined`                                    | Nome da ferramenta MCP para prompts de permissão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `persistSession`                  | `boolean`                                                                                                | `true`                                         | Quando `false`, desativa persistência de sessão em disco. Sessões não podem ser retomadas depois                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `planModeInstructions`            | `string`                                                                                                 | `undefined`                                    | Instruções de fluxo de trabalho personalizado para Plan Mode. Quando `permissionMode` é `'plan'`, esta string substitui o corpo de fluxo de trabalho de Plan Mode padrão. A CLI ainda o envolve com o preâmbulo de imposição somente leitura e o rodapé do protocolo ExitPlanMode                                                                                                                                                                                                                                                                                                                                                                   |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                | `[]`                                           | Carregar plugins personalizados de caminhos locais. Veja [Plugins](/docs/pt/agent-sdk/plugins) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `promptSuggestions`               | `boolean`                                                                                                | `false`                                        | Ativar sugestões de prompt. Emite uma mensagem `prompt_suggestion` após cada turno com um prompt de usuário previsto                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `resume`                          | `string`                                                                                                 | `undefined`                                    | ID de sessão a retomar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `resumeSessionAt`                 | `string`                                                                                                 | `undefined`                                    | Retomar sessão em um UUID de mensagem específico                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                    | `undefined`                                    | Configurar comportamento de sandbox programaticamente. Veja [Sandbox settings](#sandboxsettings) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `sessionId`                       | `string`                                                                                                 | Auto-gerado                                    | Use um UUID específico para a sessão em vez de auto-gerar um                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `sessionStore`                    | [`SessionStore`](/docs/pt/agent-sdk/session-storage#the-sessionstore-interface)                               | `undefined`                                    | Espelhar transcrições de sessão para um backend externo para que qualquer host possa retomá-las. Veja [Persist sessions to external storage](/docs/pt/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                   | `'batched'`                                    | *Alfa.* Modo de flush para `sessionStore`. Ignorado quando `sessionStore` não está definido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `settings`                        | `string \| Settings`                                                                                     | `undefined`                                    | Objeto de [configurações](/docs/pt/settings) inline ou caminho para um arquivo de configurações. Popula a camada de configurações de flag na [ordem de precedência](/docs/pt/settings#settings-precedence). Altere em tempo de execução com [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                       |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                    | Padrões da CLI (todas as fontes)               | Controle quais configurações do sistema de arquivos carregar. Passe `[]` para desativar configurações de usuário, projeto e local. Configurações de política gerenciada carregam independentemente; configurações gerenciadas pelo servidor são buscadas quando a sessão se autentica com uma credencial organizacional em uma [configuração elegível](/docs/pt/server-managed-settings#platform-availability). Veja [Use Claude Code features](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                 |
| `skills`                          | `string[] \| 'all'`                                                                                      | `undefined`                                    | Skills disponíveis para a sessão. Passe `'all'` para ativar cada skill descoberta, ou uma lista de nomes de skills. Quando definido, o SDK ativa a ferramenta Skill automaticamente em `allowedTools`. Se você também passar `tools`, inclua `'Skill'` nessa lista. Veja [Skills](/docs/pt/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                             |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                              | `undefined`                                    | Função personalizada para gerar o processo Claude Code. Use para executar Claude Code em VMs, contêineres ou ambientes remotos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `stderr`                          | `(data: string) => void`                                                                                 | `undefined`                                    | Callback para saída stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `strictMcpConfig`                 | `boolean`                                                                                                | `false`                                        | Use apenas os servidores passados em `mcpServers` e ignore o projeto `.mcp.json`, configurações do usuário, servidores MCP fornecidos por plugin e [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `systemPrompt`                    | `string \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean }` | `undefined` (prompt mínimo)                    | Configuração de prompt do sistema. Passe uma string para prompt personalizado, ou `{ type: 'preset', preset: 'claude_code' }` para usar o prompt do sistema do Claude Code. Ao usar a forma de objeto preset, adicione `append` para estendê-lo com instruções adicionais, e defina `excludeDynamicSections: true` para mover contexto por sessão para a primeira mensagem do usuário para [melhor reutilização de cache de prompt entre máquinas](/docs/pt/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines)                                                                                                         |
| `taskBudget`                      | `{ total: number }`                                                                                      | `undefined`                                    | *Alfa.* Orçamento de tarefa do lado da API em tokens. Quando definido, o modelo é informado sobre seu orçamento de token restante para que possa controlar o uso de ferramentas e encerrar antes do limite                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                      | `{ type: 'adaptive' }` para modelos suportados | Controla o comportamento de pensamento/raciocínio do Claude. Veja [`ThinkingConfig`](#thinkingconfig) para opções                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `title`                           | `string`                                                                                                 | `undefined`                                    | Título de exibição para a sessão. Ao retomar via `resume` ou `continue`, o título persistido da sessão retomada tem precedência; use [`renameSession()`](#renamesession) para renomear uma sessão existente                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `toolAliases`                     | `Record<string, string>`                                                                                 | `undefined`                                    | Mapear nomes de ferramentas integradas para nomes de ferramentas MCP para que Claude chame sua implementação MCP em vez da integrada. Por exemplo, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                              | `undefined`                                    | Configuração para comportamento de ferramenta integrada. Veja [`ToolConfig`](#toolconfig) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                  | `undefined`                                    | Configuração de ferramenta. Passe um array de nomes de ferramentas ou use o preset para obter as ferramentas padrão do Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |

<h4 id="handle-slow-or-stalled-api-responses">
  Lidar com respostas de API lentas ou travadas
</h4>

O subprocesso da CLI lê várias variáveis de ambiente que controlam timeouts de API e detecção de travamento. Passe-as através da opção `env`:

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    env: {
      ...process.env,
      API_TIMEOUT_MS: "120000",
      CLAUDE_CODE_MAX_RETRIES: "2",
      CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS: "120000",
    },
  },
});
```

* `API_TIMEOUT_MS`: timeout por solicitação no cliente Anthropic, em milissegundos. Padrão `600000`. Aplica-se ao loop principal e a todos os subagentes.
* `CLAUDE_CODE_MAX_RETRIES`: máximo de tentativas de API. Padrão `10`, limitado a `15`. Cada tentativa obtém sua própria janela `API_TIMEOUT_MS`, então o tempo de parede no pior caso é aproximadamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` mais backoff. Para execuções sem supervisão que precisam aguardar através de interrupções mais longas, defina `CLAUDE_CODE_RETRY_WATCHDOG=1`: ele tenta erros de capacidade indefinidamente, e a partir do Claude Code v2.1.199 aumenta o padrão para outros erros transitórios para `300` e remove o limite nesta variável.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: watchdog de travamento para subagentes lançados com `run_in_background`. Padrão `600000`. Redefine em cada evento de stream; em caso de travamento, aborta o subagente, marca a tarefa como falhada e expõe o erro ao pai com qualquer resultado parcial. Não se aplica a subagentes síncronos.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` com `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: aborta a solicitação quando os cabeçalhos chegaram mas o corpo da resposta para de fazer stream. O watchdog está ativado por padrão para todos os provedores; defina `CLAUDE_ENABLE_STREAM_WATCHDOG=0` para desativá-lo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` padrão é `300000` e é fixado nesse mínimo. A solicitação abortada passa pelo caminho de tentativa normal.

<h3 id="query-object">
  Objeto `Query`
</h3>

Interface retornada pela função `query()`.

```typescript theme={null}
interface Query extends AsyncGenerator<SDKMessage, void> {
  interrupt(): Promise<SDKControlInterruptResponse | undefined>;
  rewindFiles(
    userMessageId: string,
    options?: { dryRun?: boolean }
  ): Promise<RewindFilesResult>;
  setPermissionMode(mode: PermissionMode): Promise<void>;
  setModel(model?: string): Promise<void>;
  setMaxThinkingTokens(maxThinkingTokens: number | null): Promise<void>;
  applyFlagSettings(settings: { [K in keyof Settings]?: Settings[K] | null }): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Métodos
</h4>

| Método                                 | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Interrompe a consulta. Apenas disponível em modo de entrada de transmissão. Quando a CLI anuncia a capacidade `interrupt_receipt_v1` em [`SDKSystemMessage.capabilities`](#sdksystemmessage), resolve com um [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) listando as mensagens enfileiradas que sobrevivem à interrupção. Resolve `undefined` em CLIs anteriores a v2.1.205                                                                                                                |
| `rewindFiles(userMessageId, options?)` | Restaura arquivos para seu estado na mensagem de usuário especificada. Passe `{ dryRun: true }` para visualizar mudanças. Requer `enableFileCheckpointing: true`. Veja [File checkpointing](/docs/pt/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                |
| `setPermissionMode()`                  | Altera o modo de permissão (apenas disponível em modo de entrada de transmissão)                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `setModel()`                           | Altera o modelo (apenas disponível em modo de entrada de transmissão)                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `setMaxThinkingTokens()`               | *Descontinuado:* Use a opção `thinking` em vez disso. Altera os tokens de pensamento máximos. Passar `null` redefine o pensamento para o padrão da sessão: uma substituição no meio da sessão é limpa, e o pensamento permanece desativado para sessões que o têm desativado                                                                                                                                                                                                                                 |
| `applyFlagSettings(settings)`          | Mescla configurações na camada de configurações de flag da sessão em tempo de execução (apenas disponível em modo de entrada de transmissão). Veja [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                               |
| `initializationResult()`               | Retorna o resultado de inicialização completo incluindo comandos suportados, modelos, informações de conta e configuração de estilo de saída                                                                                                                                                                                                                                                                                                                                                                 |
| `reinitialize()`                       | Re-envia a solicitação de controle `initialize` para a CLI em execução e retorna um resultado novo em vez do resultado de primeira conexão em cache. Use-o após uma lacuna de transporte, como reconectar a uma sessão após uma desconexão, para que solicitações de permissão pendentes alcancem seu callback `canUseTool` novamente. Torne o callback idempotente por ID de solicitação, porque uma solicitação cuja resposta foi perdida é despachada novamente. Requer Claude Code v2.1.195 ou posterior |
| `supportedCommands()`                  | Retorna comandos slash disponíveis                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `supportedModels()`                    | Retorna modelos disponíveis com informações de exibição                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `supportedAgents()`                    | Retorna subagentes disponíveis como [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `mcpServerStatus()`                    | Retorna status de servidores MCP conectados                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `accountInfo()`                        | Retorna informações de conta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `reconnectMcpServer(serverName)`       | Reconectar um servidor MCP por nome                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `toggleMcpServer(serverName, enabled)` | Ativar ou desativar um servidor MCP por nome                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `setMcpServers(servers)`               | Substituir dinamicamente o conjunto de servidores MCP para esta sessão. Retorna informações sobre quais servidores foram adicionados, removidos e quaisquer erros                                                                                                                                                                                                                                                                                                                                            |
| `streamInput(stream)`                  | Transmitir mensagens de entrada para a consulta para conversas multi-turno                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `stopTask(taskId)`                     | Parar uma tarefa de fundo em execução por ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `close()`                              | Fechar a consulta e encerrar o processo subjacente. Força o término da consulta e limpa todos os recursos                                                                                                                                                                                                                                                                                                                                                                                                    |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Altera qualquer [configuração](/docs/pt/settings) em uma sessão em execução sem reiniciar a consulta. Use-a quando uma configuração que não tem um setter dedicado precisa mudar no meio da sessão, como apertar `permissions` depois que o agente lê entrada não confiável. `setModel()` e `setPermissionMode()` são setters dedicados para essas duas chaves; `applyFlagSettings()` é a forma geral que aceita qualquer subconjunto das chaves de configurações, e passar `model` aqui se comporta igual a `setModel()`.

Apenas algumas chaves têm efeito no meio da sessão:

* **Aplicadas no próximo turno**: `model`, `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Mudar `agent` também aplica a substituição de modelo, hooks e prompt do sistema desse agente no próximo turno.
* **Sem efeito no meio da sessão**: as opções de prompt do sistema. Estes são resolvidos uma vez na inicialização, então a sessão em execução mantém o valor original mesmo que a chamada tenha sucesso. Para alterá-los, inicie uma nova sessão.

`effortLevel` aceita um nome de [nível de esforço](/docs/pt/model-config#adjust-effort-level). Também aceita `"ultracode"`, que executa a sessão em esforço `xhigh` e ativa [ultracode](/docs/pt/workflows#let-claude-decide-with-ultracode). O tipo `Settings` declara `effortLevel` sem esse valor, então passe o equivalente `{ ultracode: true }` em TypeScript. O valor `ultracode` requer Claude Code v2.1.203 ou posterior e é aceito apenas por `applyFlagSettings()`, não pela chave `effortLevel` em um arquivo de configurações.

Os valores são escritos na camada de configurações de flag, a mesma camada que a opção `settings` inline de `query()` popula na inicialização. Configurações de flag ficam perto do topo da [ordem de precedência de configurações](/docs/pt/settings#settings-precedence): elas substituem configurações de usuário, projeto e local, e apenas configurações de política gerenciada podem substituí-las. Esta é a mesma camada que a [seção de precedência na página](#settings-precedence) chama de opções programáticas.

Chamadas sucessivas fazem shallow-merge de chaves de nível superior. Uma segunda chamada com `{ permissions: {...} }` substitui o objeto `permissions` inteiro da chamada anterior em vez de fazer deep-merge nele. Para limpar uma chave da camada de flag e voltar a fontes de precedência mais baixa, passe `null` para essa chave. Passar `undefined` não tem efeito porque a serialização JSON a descarta.

Apenas disponível em modo de entrada de transmissão, a mesma restrição que `setModel()` e `setPermissionMode()`.

O exemplo abaixo muda o modelo ativo no meio da sessão, depois limpa a substituição para que o modelo volte ao que as configurações de usuário ou projeto especificam.

```typescript theme={null}
const q = query({ prompt: messageStream });

// Substituir o modelo para o resto da sessão
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Depois: limpar a substituição e voltar a configurações de precedência mais baixa
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` é apenas TypeScript. O SDK Python não expõe um método equivalente.
</Note>

<h3 id="warmquery">
  `WarmQuery`
</h3>

Handle retornado por [`startup()`](#startup). O subprocesso já está gerado e inicializado, então chamar `query()` neste handle escreve o prompt diretamente em um processo pronto sem latência de inicialização.

```typescript theme={null}
interface WarmQuery extends AsyncDisposable {
  query(prompt: string | AsyncIterable<SDKUserMessage>): Query;
  close(): void;
}
```

<h4 id="methods-2">
  Métodos
</h4>

| Método          | Descrição                                                                                                                                 |
| :-------------- | :---------------------------------------------------------------------------------------------------------------------------------------- |
| `query(prompt)` | Enviar um prompt para o subprocesso pré-aquecido e retornar uma [`Query`](#query-object). Pode ser chamado apenas uma vez por `WarmQuery` |
| `close()`       | Fechar o subprocesso sem enviar um prompt. Use isso para descartar uma consulta quente que não é mais necessária                          |

`WarmQuery` implementa `AsyncDisposable`, então pode ser usado com `await using` para limpeza automática.

<h3 id="sdkcontrolinitializeresponse">
  `SDKControlInitializeResponse`
</h3>

Tipo de retorno de `initializationResult()`. Contém dados de inicialização de sessão.

```typescript theme={null}
type SDKControlInitializeResponse = {
  commands: SlashCommand[];
  agents: AgentInfo[];
  output_style: string;
  available_output_styles: string[];
  models: ModelInfo[];
  account: AccountInfo;
  fast_mode_state?: "off" | "cooldown" | "on";
};
```

Quando um cliente envia `initialize` para uma sessão que já está em execução, o wrapper de resposta de controle também carrega um array `pending_permission_requests` opcional. O campo está no wrapper de resposta em si, não na carga `SDKControlInitializeResponse` acima. Cada entrada é uma mensagem `control_request` completa com a mesma forma `{ type: "control_request", request_id, request }` que a sessão transmite para solicitações de permissão durante a execução.

Estas são solicitações que foram emitidas antes do cliente se conectar e ainda estão aguardando uma resposta. O SDK lê o array para você e despacha cada entrada para seu callback [`canUseTool`](#canusetool), o mesmo reenvio que [`reinitialize()`](#query-object) dispara após uma lacuna de transporte. Trate IDs de solicitação repetidos idempotentemente, porque uma entrada pode repetir uma solicitação que o callback já recebeu antes da conexão cair.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

O recebimento de interrupção: o valor que [`interrupt()`](#query-object) resolve em uma CLI que anuncia a capacidade `interrupt_receipt_v1` em [`SDKSystemMessage.capabilities`](#sdksystemmessage). Requer Claude Code v2.1.205 ou posterior. CLIs anteriores respondem à interrupção com uma carga de sucesso vazia, então `interrupt()` resolve para `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
};
```

`still_queued` lista os UUIDs das mensagens de usuário que sobrevivem à interrupção: mensagens ainda na fila, mais qualquer lote já removido da fila para o próximo turno mas ainda não alcançável pela anulação. Cada uma é executada como seu próprio turno após a interrupção a menos que você a cancele primeiro. Use o recebimento para decidir se deve reenviar algo; reenviar uma mensagem que já está listada produz um turno duplicado.

Interprete a lista com estas ressalvas:

* Apenas mensagens que foram enfileiradas com um UUID aparecem. Um array vazio não significa que nada mais será executado.
* Apenas mensagens da thread principal estão listadas. Mensagens endereçadas a um subagente estão fora do escopo.
* A lista pode incluir UUIDs que seu cliente nunca enviou, como acionadores de [tarefa agendada](/docs/pt/scheduled-tasks). Ignore UUIDs que você não reconhece em vez de tratá-los como um erro.

O recebimento é um snapshot tirado no momento em que a interrupção é processada, e em uma interrupção limpa chega antes do [`SDKResultMessage`](#sdkresultmessage) do turno interrompido. Leia o recebimento em vez de inspecionar a fila após esse resultado: o loop inicia o próximo turno enfileirado imediatamente, então a fila que você inspeciona após o resultado já mudou.

<h3 id="agentdefinition">
  `AgentDefinition`
</h3>

Configuração para um subagente definido programaticamente.

```typescript theme={null}
type AgentDefinition = {
  description: string;
  tools?: string[];
  disallowedTools?: string[];
  prompt: string;
  model?: string;
  mcpServers?: AgentMcpServerSpec[];
  skills?: string[];
  initialPrompt?: string;
  maxTurns?: number;
  background?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Campo                                 | Obrigatório | Descrição                                                                                                                                                                                                                                                               |
| :------------------------------------ | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | Sim         | Descrição em linguagem natural de quando usar este agente                                                                                                                                                                                                               |
| `tools`                               | Não         | Array de nomes de ferramentas permitidas. Se omitido, herda todas as ferramentas do pai. Para pré-carregar Skills no contexto do agente, use o campo `skills` em vez de listar `'Skill'` aqui                                                                           |
| `disallowedTools`                     | Não         | Array de nomes de ferramentas para explicitamente desallocar para este agente. Padrões de nível de servidor MCP também são aceitos: `mcp__server` ou `mcp__server__*` remove cada ferramenta desse servidor, e `mcp__*` remove cada ferramenta MCP de qualquer servidor |
| `prompt`                              | Sim         | O prompt do sistema do agente                                                                                                                                                                                                                                           |
| `model`                               | Não         | Substituição de modelo para este agente. Aceita um alias como `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, ou um ID de modelo completo. Se omitido ou `'inherit'`, usa o modelo principal                                                                   |
| `mcpServers`                          | Não         | Especificações de servidor MCP para este agente                                                                                                                                                                                                                         |
| `skills`                              | Não         | Array de nomes de skills para pré-carregar no contexto do agente                                                                                                                                                                                                        |
| `initialPrompt`                       | Não         | Auto-enviado como o primeiro turno de usuário quando este agente é executado como o agente da thread principal                                                                                                                                                          |
| `maxTurns`                            | Não         | Número máximo de turnos agênticos (round-trips de API) antes de parar                                                                                                                                                                                                   |
| `background`                          | Não         | Executar este agente como uma tarefa de fundo não-bloqueante quando invocado                                                                                                                                                                                            |
| `memory`                              | Não         | Fonte de memória para este agente: `'user'`, `'project'`, ou `'local'`                                                                                                                                                                                                  |
| `effort`                              | Não         | Nível de esforço de raciocínio para este agente. Aceita um nível nomeado ou um inteiro                                                                                                                                                                                  |
| `permissionMode`                      | Não         | Modo de permissão para execução de ferramenta dentro deste agente. Veja [`PermissionMode`](#permissionmode)                                                                                                                                                             |
| `criticalSystemReminder_EXPERIMENTAL` | Não         | Experimental: Lembrete crítico adicionado ao prompt do sistema                                                                                                                                                                                                          |

<h3 id="agentmcpserverspec">
  `AgentMcpServerSpec`
</h3>

Especifica servidores MCP disponíveis para um subagente. Pode ser um nome de servidor (string referenciando um servidor da configuração `mcpServers` do pai) ou um registro de configuração de servidor inline mapeando nomes de servidor para configs.

```typescript theme={null}
type AgentMcpServerSpec = string | Record<string, McpServerConfigForProcessTransport>;
```

Onde `McpServerConfigForProcessTransport` é `McpStdioServerConfig | McpSSEServerConfig | McpHttpServerConfig | McpSdkServerConfig`.

<h3 id="settingsource">
  `SettingSource`
</h3>

Controla quais fontes de configuração baseadas em sistema de arquivos o SDK carrega configurações.

```typescript theme={null}
type SettingSource = "user" | "project" | "local";
```

| Valor       | Descrição                                                        | Localização                   |
| :---------- | :--------------------------------------------------------------- | :---------------------------- |
| `'user'`    | Configurações globais do usuário                                 | `~/.claude/settings.json`     |
| `'project'` | Configurações de projeto compartilhadas (controladas por versão) | `.claude/settings.json`       |
| `'local'`   | Configurações de projeto local (não controladas por versão)      | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamento padrão
</h4>

Quando `settingSources` é omitido ou `undefined`, `query()` carrega as mesmas configurações do sistema de arquivos que a CLI do Claude Code: usuário, projeto e local. Configurações de política gerenciada são carregadas em todos os casos; configurações gerenciadas pelo servidor são buscadas quando a sessão se autentica com uma credencial organizacional em uma [configuração elegível](/docs/pt/server-managed-settings#platform-availability). Veja [What settingSources does not control](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control) para entradas que são lidas independentemente desta opção, e como desativá-las.

<h4 id="why-use-settingsources">
  Por que usar settingSources
</h4>

**Desativar configurações do sistema de arquivos:**

```typescript theme={null}
// Não carregar configurações de usuário, projeto ou local do disco
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Carregar todas as configurações do sistema de arquivos explicitamente:**

```typescript theme={null}
const result = query({
  prompt: "Analyze this code",
  options: {
    settingSources: ["user", "project", "local"] // Carregar todas as configurações
  }
});
```

**Carregar apenas fontes de configuração específicas:**

```typescript theme={null}
// Carregar apenas configurações de projeto, ignorar usuário e local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Apenas .claude/settings.json
  }
});
```

**Ambientes de teste e CI:**

```typescript theme={null}
// Garantir comportamento consistente em CI excluindo configurações locais
const result = query({
  prompt: "Run tests",
  options: {
    settingSources: ["project"], // Apenas configurações compartilhadas da equipe
    permissionMode: "bypassPermissions"
  }
});
```

**Aplicações apenas SDK:**

```typescript theme={null}
// Defina tudo programaticamente.
// Passe [] para optar por não usar fontes de configuração do sistema de arquivos.
const result = query({
  prompt: "Review this PR",
  options: {
    settingSources: [],
    agents: {
      /* ... */
    },
    mcpServers: {
      /* ... */
    },
    allowedTools: ["Read", "Grep", "Glob"]
  }
});
```

**Carregando instruções de projeto CLAUDE.md:**

```typescript theme={null}
// Carregar configurações de projeto para incluir arquivos CLAUDE.md
const result = query({
  prompt: "Add a new feature following project conventions",
  options: {
    systemPrompt: {
      type: "preset",
      preset: "claude_code" // Usar o prompt do sistema do Claude Code
    },
    settingSources: ["project"], // Carrega CLAUDE.md do diretório do projeto
    allowedTools: ["Read", "Write", "Edit"]
  }
});
```

<h4 id="settings-precedence">
  Precedência de configurações
</h4>

Quando múltiplas fontes são carregadas, as configurações são mescladas com esta precedência (maior para menor):

1. Configurações locais (`.claude/settings.local.json`)
2. Configurações de projeto (`.claude/settings.json`)
3. Configurações do usuário (`~/.claude/settings.json`)

Opções programáticas como `agents`, `allowedTools` e `settings` substituem configurações do sistema de arquivos de usuário, projeto e local. Configurações de política gerenciada têm precedência sobre opções programáticas.

<h3 id="permissionmode">
  `PermissionMode`
</h3>

```typescript theme={null}
type PermissionMode =
  | "default" // Comportamento de permissão padrão
  | "acceptEdits" // Auto-aceitar edições de arquivo
  | "bypassPermissions" // Bypass de verificações de permissão; regras de solicitação explícita ainda solicitam
  | "plan" // Plan Mode - explorar sem editar
  | "dontAsk" // Não solicitar permissões, negar se não pré-aprovado
  | "auto"; // Usar um classificador de modelo para aprovar ou negar cada chamada de ferramenta
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Tipo de função de permissão personalizada para controlar o uso de ferramentas.

A função é a substituição do SDK para o prompt de permissão interativo: é invocada apenas quando o [fluxo de avaliação de permissão](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) se resolve em um prompt. Chamadas de ferramenta já aprovadas por uma entrada `allowedTools`, uma regra de permissão de configurações, ou o modo de permissão, como `acceptEdits` ou `bypassPermissions`, nunca a invocam. Para controlar cada chamada de ferramenta, use um [hook `PreToolUse`](/docs/pt/agent-sdk/hooks) em vez disso.

`AskUserQuestion`, ferramentas MCP marcadas [`requiresUserInteraction`](/docs/pt/mcp#require-approval-for-a-specific-tool), e ferramentas de conector [sua organização definida como `ask`](/docs/pt/mcp#organization-controls-on-connector-tools) a alcançam mesmo quando uma regra de permissão corresponde. Em modo `dontAsk` essas chamadas são negadas em vez disso, sem invocá-la.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    decisionReason?: string;
    toolUseID: string;
    agentID?: string;
    requestId: string;
  }
) => Promise<PermissionResult | null>;
```

| Opção            | Tipo                                        | Descrição                                                                                                                                                                                                                                                                                                                                      |
| :--------------- | :------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `signal`         | `AbortSignal`                               | Sinalizado se a operação deve ser abortada                                                                                                                                                                                                                                                                                                     |
| `suggestions`    | [`PermissionUpdate`](#permissionupdate)`[]` | Atualizações de permissão sugeridas para que o usuário não seja solicitado novamente para esta ferramenta. Prompts de Bash incluem uma sugestão com o destino `localSettings` [destination](#permissionupdatedestination), então retorná-la em `updatedPermissions` escreve a regra em `.claude/settings.local.json` e persiste entre sessões. |
| `blockedPath`    | `string`                                    | O caminho do arquivo que acionou a solicitação de permissão, se aplicável                                                                                                                                                                                                                                                                      |
| `decisionReason` | `string`                                    | Explica por que esta solicitação de permissão foi acionada                                                                                                                                                                                                                                                                                     |
| `toolUseID`      | `string`                                    | Identificador único para esta chamada de ferramenta específica dentro da mensagem do assistente                                                                                                                                                                                                                                                |
| `agentID`        | `string`                                    | Se executando dentro de um sub-agente, o ID do sub-agente                                                                                                                                                                                                                                                                                      |
| `requestId`      | `string`                                    | O `request_id` do envelope `control_request`. Uma `control_response` que sua aplicação envia fora do SDK, como um POST HTTP assinado, deve ecoar este valor para que o processo Claude Code possa corresponder a resposta à solicitação                                                                                                        |

O callback normalmente resolve a solicitação retornando um [`PermissionResult`](#permissionresult), que o SDK escreve de volta sobre seu transporte como a `control_response`. Retorne `null` apenas quando sua aplicação já enviou a `control_response` para esta solicitação sobre seu próprio canal, ecoando `requestId`; o SDK então pula escrever a resposta em seu transporte. Retornar `null` em qualquer outro caso deixa a chamada de ferramenta bloqueada indefinidamente, porque nenhuma `control_response` é jamais enviada e prompts de permissão não expiram.

A opção `requestId` e o valor de retorno `null` requerem Claude Code v2.1.199 ou posterior.

<h3 id="permissionresult">
  `PermissionResult`
</h3>

Resultado de uma verificação de permissão.

```typescript theme={null}
type PermissionResult =
  | {
      behavior: "allow";
      updatedInput?: Record<string, unknown>;
      updatedPermissions?: PermissionUpdate[];
      toolUseID?: string;
    }
  | {
      behavior: "deny";
      message: string;
      interrupt?: boolean;
      toolUseID?: string;
    };
```

<h3 id="toolconfig">
  `ToolConfig`
</h3>

Configuração para comportamento de ferramenta integrada.

```typescript theme={null}
type ToolConfig = {
  askUserQuestion?: {
    previewFormat?: "markdown" | "html";
  };
};
```

| Campo                           | Tipo                   | Descrição                                                                                                                                                                               |
| :------------------------------ | :--------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `askUserQuestion.previewFormat` | `'markdown' \| 'html'` | Opta pelo campo `preview` em opções [`AskUserQuestion`](/docs/pt/agent-sdk/user-input#question-format) e define seu formato de conteúdo. Quando não definido, Claude não emite visualizações |

<h3 id="mcpserverconfig">
  `McpServerConfig`
</h3>

Configuração para servidores MCP.

```typescript theme={null}
type McpServerConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfigWithInstance;
```

<h4 id="mcpstdioserverconfig">
  `McpStdioServerConfig`
</h4>

```typescript theme={null}
type McpStdioServerConfig = {
  type?: "stdio";
  command: string;
  args?: string[];
  env?: Record<string, string>;
};
```

<h4 id="mcpsseserverconfig">
  `McpSSEServerConfig`
</h4>

```typescript theme={null}
type McpSSEServerConfig = {
  type: "sse";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcphttpserverconfig">
  `McpHttpServerConfig`
</h4>

```typescript theme={null}
type McpHttpServerConfig = {
  type: "http";
  url: string;
  headers?: Record<string, string>;
};
```

<h4 id="mcpsdkserverconfigwithinstance">
  `McpSdkServerConfigWithInstance`
</h4>

```typescript theme={null}
type McpSdkServerConfigWithInstance = {
  type: "sdk";
  name: string;
  instance: McpServer;
};
```

<h4 id="mcpclaudeaiproxyserverconfig">
  `McpClaudeAIProxyServerConfig`
</h4>

```typescript theme={null}
type McpClaudeAIProxyServerConfig = {
  type: "claudeai-proxy";
  url: string;
  id: string;
};
```

<h3 id="sdkpluginconfig">
  `SdkPluginConfig`
</h3>

Configuração para carregar plugins no SDK.

```typescript theme={null}
type SdkPluginConfig = {
  type: "local";
  path: string;
  skipMcpDiscovery?: boolean;
};
```

| Campo              | Tipo      | Descrição                                                                                                                                                                                           |
| :----------------- | :-------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`             | `'local'` | Deve ser `'local'` (apenas plugins locais atualmente suportados)                                                                                                                                    |
| `path`             | `string`  | Caminho absoluto ou relativo para o diretório do plugin                                                                                                                                             |
| `skipMcpDiscovery` | `boolean` | Quando `true`, o SDK carrega skills, hooks, agentes e comandos deste plugin mas não lê seu `.mcp.json` ou manifest `mcpServers`. Defina isso quando sua aplicação possui as conexões MCP do plugin. |

**Exemplo:**

```typescript theme={null}
plugins: [
  { type: "local", path: "./my-plugin" },
  { type: "local", path: "/absolute/path/to/plugin" }
];
```

Para informações completas sobre criação e uso de plugins, veja [Plugins](/docs/pt/agent-sdk/plugins).

<h2 id="message-types">
  Tipos de Mensagem
</h2>

<h3 id="sdkmessage">
  `SDKMessage`
</h3>

Tipo de união de todas as mensagens possíveis retornadas pela consulta.

```typescript theme={null}
type SDKMessage =
  | SDKAssistantMessage
  | SDKUserMessage
  | SDKUserMessageReplay
  | SDKResultMessage
  | SDKSystemMessage
  | SDKPartialAssistantMessage
  | SDKCompactBoundaryMessage
  | SDKStatusMessage
  | SDKLocalCommandOutputMessage
  | SDKHookStartedMessage
  | SDKHookProgressMessage
  | SDKHookResponseMessage
  | SDKPluginInstallMessage
  | SDKToolProgressMessage
  | SDKAuthStatusMessage
  | SDKTaskNotificationMessage
  | SDKTaskStartedMessage
  | SDKTaskProgressMessage
  | SDKTaskUpdatedMessage
  | SDKBackgroundTasksChangedMessage
  | SDKThinkingTokensMessage
  | SDKSessionStateChangedMessage
  | SDKWorkerShuttingDownMessage
  | SDKCommandsChangedMessage
  | SDKNotificationMessage
  | SDKFilesPersistedEvent
  | SDKToolUseSummaryMessage
  | SDKMemoryRecallMessage
  | SDKRateLimitEvent
  | SDKElicitationCompleteMessage
  | SDKPermissionDeniedMessage
  | SDKPromptSuggestionMessage
  | SDKAPIRetryMessage
  | SDKMirrorErrorMessage
  | SDKInformationalMessage
  | SDKConversationResetMessage;
```

<h3 id="sdkassistantmessage">
  `SDKAssistantMessage`
</h3>

Mensagem de resposta do assistente.

```typescript theme={null}
type SDKAssistantMessage = {
  type: "assistant";
  uuid: UUID;
  session_id: string;
  message: BetaMessage; // Do SDK Anthropic
  parent_tool_use_id: string | null;
  error?: SDKAssistantMessageError;
};
```

O campo `message` é uma [`BetaMessage`](https://platform.claude.com/docs/pt/api/messages/create) do SDK Anthropic. Inclui campos como `id`, `content`, `model`, `stop_reason` e `usage`.

`SDKAssistantMessageError` é um de: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, ou `'unknown'`. `'model_not_found'` significa que o modelo selecionado não existe ou não está disponível para sua conta ou implantação. `'overloaded'` significa que a API retornou um 529 porque o servidor está em capacidade máxima, em contraste com `'rate_limit'`, que é um 429 contra sua cota.

<h3 id="sdkusermessage">
  `SDKUserMessage`
</h3>

Mensagem de entrada do usuário.

```typescript theme={null}
type SDKUserMessage = {
  type: "user";
  uuid?: UUID;
  session_id?: string;
  message: MessageParam; // Do SDK Anthropic
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
};
```

Defina `shouldQuery` como `false` para anexar a mensagem à transcrição sem acionar um turno do assistente. A mensagem é mantida e mesclada na próxima mensagem do usuário que aciona um turno. Use isso para injetar contexto, como a saída de um comando que você executou fora de banda, sem gastar uma chamada de modelo nela.

No campo de uma mensagem que carrega um bloco `tool_result`, `tool_use_result` é o objeto de saída estruturada da ferramenta em vez do texto enviado ao modelo. Sua forma depende da ferramenta nomeada pelo bloco `tool_use` correspondente, portanto o campo é digitado como `unknown`; as formas integradas estão listadas em [Tipos de Saída de Ferramenta](#tool-output-types).

Para a ferramenta `Agent`, `tool_use_result` é [`AgentOutput`](#agent-2). Em um resultado `completed`, `content` contém o relatório do subagente sem o ID do agente e o trailer de uso que Claude Code anexa ao texto `tool_result`, portanto renderize a partir de `tool_use_result` em vez de analisar esse texto.

<h3 id="sdkusermessagereplay">
  `SDKUserMessageReplay`
</h3>

Mensagem de usuário repetida com UUID obrigatório.

```typescript theme={null}
type SDKUserMessageReplay = {
  type: "user";
  uuid: UUID;
  session_id: string;
  message: MessageParam;
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  isReplay: true;
};
```

Um turno de usuário injetado de fora da sessão, aquele cuja [`origin`](#sdkmessageorigin) é `peer` ou `channel`, chega ao fluxo como uma repetição, independentemente de ter sido entregue durante um turno ativo ou iniciado um novo turno enquanto a sessão estava ociosa. Antes da v2.1.207, um turno injetado entregue enquanto a sessão estava ociosa não produzia nenhuma mensagem no fluxo e apenas aparecia quando você relê a transcrição.

<h3 id="sdkresultmessage">
  `SDKResultMessage`
</h3>

Mensagem de resultado final.

```typescript theme={null}
type SDKResultMessage =
  | {
      type: "result";
      subtype: "success";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      api_error_status?: number | null;
      num_turns: number;
      result: string;
      stop_reason: string | null;
      ttft_ms?: number;
      ttft_stream_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    }
  | {
      type: "result";
      subtype:
        | "error_max_turns"
        | "error_during_execution"
        | "error_max_budget_usd"
        | "error_max_structured_output_retries";
      uuid: UUID;
      session_id: string;
      duration_ms: number;
      duration_api_ms: number;
      is_error: boolean;
      num_turns: number;
      stop_reason: string | null;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      errors: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      origin?: SDKMessageOrigin;
    };
```

Vários campos no resultado carregam detalhes de diagnóstico além de `subtype`:

* `api_error_status`: o código de status HTTP do erro de API que encerrou a conversa. Ausente ou `null` quando o turno terminou sem um erro de API.
* `ttft_ms`: tempo até o primeiro token em milissegundos, medido quando a primeira mensagem completa do assistente chega. Presente apenas no braço de sucesso.
* `ttft_stream_ms`: tempo em milissegundos até o primeiro evento de fluxo `message_start`, quando o fluxo de resposta abre. Menor que `ttft_ms`; a lacuna entre os dois é o tempo gasto transmitindo a primeira mensagem. Presente apenas no braço de sucesso.
* `terminal_reason`: por que o loop terminou. Um de `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"`, ou `"turn_setup_failed"`.
* `fast_mode_state`: um de `"on"`, `"off"`, ou `"cooldown"`.

O campo `origin` encaminha a [`SDKMessageOrigin`](#sdkmessageorigin) da mensagem do usuário que acionou este resultado. Quando uma tarefa em segundo plano é concluída e o SDK injeta um turno de acompanhamento sintético, a `SDKResultMessage` resultante carrega `origin: { kind: "task-notification" }`. Verifique este campo para distinguir resultados que respondem ao seu prompt de resultados emitidos para acompanhamentos de tarefas em segundo plano, para que você possa rotear ou suprimir os últimos. O campo está ausente para resultados emitidos antes de qualquer turno do usuário, como erros de inicialização.

Quando um hook `PreToolUse` retorna `permissionDecision: "defer"`, o resultado tem `stop_reason: "tool_deferred"` e `deferred_tool_use` carrega o `id`, `name` e `input` da ferramenta pendente. Leia este campo para exibir a solicitação em sua própria interface do usuário, depois retome com o mesmo `session_id` para continuar. Consulte [Adiar uma chamada de ferramenta para mais tarde](/docs/pt/hooks#defer-a-tool-call-for-later) para a volta completa.

<h3 id="sdksystemmessage">
  `SDKSystemMessage`
</h3>

Mensagem de inicialização do sistema.

```typescript theme={null}
type SDKSystemMessage = {
  type: "system";
  subtype: "init";
  uuid: UUID;
  session_id: string;
  agents?: string[];
  apiKeySource: ApiKeySource;
  betas?: string[];
  claude_code_version: string;
  cwd: string;
  tools: string[];
  mcp_servers: {
    name: string;
    status: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  capabilities?: string[];
};
```

O array `capabilities` nomeia os comportamentos de protocolo que esta CLI implementa, para que você possa fazer detecção de recursos em vez de comparar strings `claude_code_version`. É um conjunto aberto: ignore valores que você não reconhecer e verifique a capacidade específica cujo comportamento você depende. O campo requer Claude Code v2.1.205 ou posterior e está ausente em CLIs anteriores.

| Capacidade             | Significado                                                                                                                                                                           |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1` | [`interrupt()`](#query-object) resolve com uma resposta [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) nomeando as mensagens enfileiradas que sobrevivem à interrupção |

<h3 id="sdkpartialassistantmessage">
  `SDKPartialAssistantMessage`
</h3>

Mensagem parcial de transmissão (apenas quando `includePartialMessages` é true). O campo `parent_tool_use_id` é sempre `null`: eventos de fluxo são emitidos apenas para a sessão principal. Para atribuição de subagente, use mensagens completas, que carregam `parent_tool_use_id`, ou ative [`forwardSubagentText`](#options) para receber texto e pensamento de subagente como mensagens completas.

```typescript theme={null}
type SDKPartialAssistantMessage = {
  type: "stream_event";
  event: BetaRawMessageStreamEvent; // Do SDK Anthropic
  parent_tool_use_id: string | null;
  uuid: UUID;
  session_id: string;
  ttft_ms?: number; // Tempo até o primeiro token em ms, presente apenas em eventos message_start
};
```

<h3 id="sdkcompactboundarymessage">
  `SDKCompactBoundaryMessage`
</h3>

Mensagem indicando um limite de compactação de conversa.

```typescript theme={null}
type SDKCompactBoundaryMessage = {
  type: "system";
  subtype: "compact_boundary";
  uuid: UUID;
  session_id: string;
  compact_metadata: {
    trigger: "manual" | "auto";
    pre_tokens: number;
  };
};
```

<h3 id="sdkinformationalmessage">
  `SDKInformationalMessage`
</h3>

Banner de texto genérico emitido pelo loop. Carrega linhas de status sem erro, feedback de hook como a razão de bloqueio de um hook `UserPromptSubmit`, e saída de comando. Renderize `content` como texto simples no `level` fornecido.

```typescript theme={null}
type SDKInformationalMessage = {
  type: "system";
  subtype: "informational";
  content: string;
  level: "info" | "notice" | "suggestion" | "warning";
  tool_use_id?: string;
  prevent_continuation?: boolean;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkworkershuttingdownmessage">
  `SDKWorkerShuttingDownMessage`
</h3>

Emitido no encerramento gracioso do worker para que clientes remotos possam mostrar por que o worker desapareceu em vez de esperar pelo timeout de heartbeat. O `reason` é uma string curta em snake\_case definida pela CLI do host, como `"host_exit"` ou `"remote_control_disabled"`. Aja sobre isso apenas ao transmitir ao vivo. Uma sessão retomada reproduz instâncias passadas desta mensagem, então ignore-as nesse caso.

```typescript theme={null}
type SDKWorkerShuttingDownMessage = {
  type: "system";
  subtype: "worker_shutting_down";
  reason: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkplugininstallmessage">
  `SDKPluginInstallMessage`
</h3>

Evento de progresso de instalação de plugin. Emitido quando [`CLAUDE_CODE_SYNC_PLUGIN_INSTALL`](/docs/pt/env-vars) está definido, para que sua aplicação Agent SDK possa rastrear a instalação de plugin do marketplace antes do primeiro turno. Os status `started` e `completed` delimitam a instalação geral. Os status `installed` e `failed` relatam marketplaces individuais e incluem `name`.

```typescript theme={null}
type SDKPluginInstallMessage = {
  type: "system";
  subtype: "plugin_install";
  status: "started" | "installed" | "failed" | "completed";
  name?: string;
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpermissiondeniedmessage">
  `SDKPermissionDeniedMessage`
</h3>

Evento de fluxo emitido quando o sistema de permissão nega automaticamente uma chamada de ferramenta sem um prompt interativo. Use-o para renderizar a negação em sua interface do usuário conforme ela acontece, em vez de apenas observar o resultado da ferramenta `is_error` que se segue. O caminho de solicitação interativa chega à sua aplicação separadamente através do callback [`canUseTool`](#canusetool). As negações emitidas por um hook `PreToolUse` não são relatadas através deste evento.

Este evento requer Claude Code v2.1.136 ou posterior.

```typescript theme={null}
type SDKPermissionDeniedMessage = {
  type: "system";
  subtype: "permission_denied";
  tool_name: string;
  tool_use_id: string;
  agent_id?: string;
  decision_reason_type?: string;
  decision_reason?: string;
  message: string;
  uuid: UUID;
  session_id: string;
};
```

| Campo                  | Tipo     | Descrição                                                                                                                                     |
| ---------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| `tool_name`            | `string` | Nome da ferramenta que foi negada                                                                                                             |
| `tool_use_id`          | `string` | ID do bloco `tool_use` que esta negação responde                                                                                              |
| `agent_id`             | `string` | ID do subagente quando a chamada negada originou-se dentro de um subagente. Espelha o campo em `can_use_tool` para roteamento do lado do host |
| `decision_reason_type` | `string` | Discriminador para o componente que decidiu, como `"rule"`, `"mode"`, `"classifier"`, ou `"asyncAgent"`                                       |
| `decision_reason`      | `string` | Razão legível por humanos do componente que decidiu, quando disponível                                                                        |
| `message`              | `string` | Mensagem de rejeição retornada ao modelo no `tool_result`                                                                                     |

<h3 id="sdkpermissiondenial">
  `SDKPermissionDenial`
</h3>

Informações sobre um uso de ferramenta negado.

```typescript theme={null}
type SDKPermissionDenial = {
  tool_name: string;
  tool_use_id: string;
  tool_input: Record<string, unknown>;
};
```

<h3 id="sdkmessageorigin">
  `SDKMessageOrigin`
</h3>

Proveniência de uma mensagem com função de usuário. Isso aparece como `origin` em [`SDKUserMessage`](#sdkusermessage) e é encaminhado para a [`SDKResultMessage`](#sdkresultmessage) correspondente para que você possa dizer o que acionou um determinado turno.

```typescript theme={null}
type SDKMessageOrigin =
  | { kind: "human" }
  | { kind: "channel"; server: string }
  | {
      kind: "peer";
      from: string;
      name?: string;
      senderTaskId?: string;
      body?: string;
    }
  | { kind: "task-notification" }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" };
```

| `kind`              | Significado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Entrada direta do usuário final. Em mensagens de usuário, uma `origin` ausente também significa entrada humana.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `channel`           | Mensagem chegando em um [canal](/docs/pt/channels). `server` é o nome do servidor MCP de origem.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `peer`              | Mensagem de outro agente. Para um [colega de equipe](/docs/pt/agent-teams) em processo enviando para `main` via `SendMessage`, `from` é o nome do colega de equipe e `senderTaskId` é seu ID de tarefa. Para um par entre sessões, como outro processo Claude Code local, `from` é o endereço do remetente e `senderTaskId` está ausente. `name` e `body` requerem Claude Code v2.1.205 ou posterior. `name` é o nome de exibição do remetente, normalizado pelo Claude Code: remove pontos de código de controle, formato, substituto e separador de linha ou parágrafo Unicode, depois corta o resultado e o limita a 64 pontos de código com reticências. `body` é o corpo da mensagem decodificado com o envelope de par removido, byte-exato com o que o modelo vê. Para uma mensagem de colega de equipe `body` está sempre presente; para um par entre sessões está presente apenas quando o turno é exatamente um envelope de par formado pelo Claude Code. Renderize `name` e `body` em vez de reanalisar o texto da mensagem. |
| `task-notification` | Turno sintético injetado após a conclusão de uma tarefa em segundo plano. Consulte [`SDKTaskNotificationMessage`](#sdktasknotificationmessage).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `coordinator`       | Mensagem de um coordenador de equipe em uma [equipe de agente](/docs/pt/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `auto-continuation` | Turno sintético injetado quando a sessão continua sem entrada de usuário nova, como um resultado de comando que aciona um prompt de acompanhamento.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |

<h2 id="hook-types">
  Tipos de Hook
</h2>

Para um guia abrangente sobre o uso de hooks com exemplos e padrões comuns, veja o [guia de Hooks](/docs/pt/agent-sdk/hooks).

<h3 id="hookevent">
  `HookEvent`
</h3>

Eventos de hook disponíveis.

```typescript theme={null}
type HookEvent =
  | "PreToolUse"
  | "PostToolUse"
  | "PostToolUseFailure"
  | "PostToolBatch"
  | "Notification"
  | "UserPromptSubmit"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PermissionRequest"
  | "Setup"
  | "TeammateIdle"
  | "TaskCompleted"
  | "ConfigChange"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "MessageDisplay";
```

<h3 id="hookcallback">
  `HookCallback`
</h3>

Tipo de função de callback de hook.

```typescript theme={null}
type HookCallback = (
  input: HookInput, // União de todos os tipos de entrada de hook
  toolUseID: string | undefined,
  options: { signal: AbortSignal }
) => Promise<HookJSONOutput>;
```

<h3 id="hookcallbackmatcher">
  `HookCallbackMatcher`
</h3>

Configuração de hook com matcher opcional.

```typescript theme={null}
interface HookCallbackMatcher {
  matcher?: string;
  hooks: HookCallback[];
  timeout?: number; // Timeout em segundos para todos os hooks neste matcher
}
```

<h3 id="hookinput">
  `HookInput`
</h3>

Tipo de união de todos os tipos de entrada de hook.

```typescript theme={null}
type HookInput =
  | PreToolUseHookInput
  | PostToolUseHookInput
  | PostToolUseFailureHookInput
  | PostToolBatchHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCompletedHookInput
  | ConfigChangeHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | MessageDisplayHookInput;
```

<h3 id="basehookinput">
  `BaseHookInput`
</h3>

Interface base que todos os tipos de entrada de hook estendem.

```typescript theme={null}
type BaseHookInput = {
  session_id: string;
  transcript_path: string;
  cwd: string;
  prompt_id?: string;
  permission_mode?: string;
  effort?: { level: string };
  agent_id?: string;
  agent_type?: string;
};
```

O campo `prompt_id` é um UUID que identifica o prompt do usuário sendo processado atualmente. Ele corresponde ao [atributo `prompt.id` em eventos OpenTelemetry](/docs/pt/monitoring-usage#event-correlation-attributes) e está ausente até a primeira entrada do usuário. Requer Claude Code v2.1.196 ou posterior.

<h4 id="pretoolusehookinput">
  `PreToolUseHookInput`
</h4>

```typescript theme={null}
type PreToolUseHookInput = BaseHookInput & {
  hook_event_name: "PreToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
};
```

<h4 id="posttoolusehookinput">
  `PostToolUseHookInput`
</h4>

```typescript theme={null}
type PostToolUseHookInput = BaseHookInput & {
  hook_event_name: "PostToolUse";
  tool_name: string;
  tool_input: unknown;
  tool_response: unknown;
  tool_use_id: string;
  duration_ms?: number;
};
```

<h4 id="posttoolusefailurehookinput">
  `PostToolUseFailureHookInput`
</h4>

```typescript theme={null}
type PostToolUseFailureHookInput = BaseHookInput & {
  hook_event_name: "PostToolUseFailure";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  error: string;
  is_interrupt?: boolean;
  duration_ms?: number;
};
```

<h4 id="posttoolbatchhookinput">
  `PostToolBatchHookInput`
</h4>

Dispara uma vez após cada chamada de ferramenta em um lote ter sido resolvida, antes da próxima solicitação do modelo. `tool_response` carrega o conteúdo serializado de `tool_result` que o modelo vê; a forma difere do objeto estruturado `Output` de `PostToolUseHookInput`.

```typescript theme={null}
type PostToolBatchHookInput = BaseHookInput & {
  hook_event_name: "PostToolBatch";
  tool_calls: PostToolBatchToolCall[];
};

type PostToolBatchToolCall = {
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  tool_response?: unknown;
};
```

<h4 id="notificationhookinput">
  `NotificationHookInput`
</h4>

```typescript theme={null}
type NotificationHookInput = BaseHookInput & {
  hook_event_name: "Notification";
  message: string;
  title?: string;
  notification_type: string;
};
```

<h4 id="userpromptsubmithookinput">
  `UserPromptSubmitHookInput`
</h4>

```typescript theme={null}
type UserPromptSubmitHookInput = BaseHookInput & {
  hook_event_name: "UserPromptSubmit";
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact";
  agent_type?: string;
  model?: string;
};
```

<h4 id="sessionendhookinput">
  `SessionEndHookInput`
</h4>

```typescript theme={null}
type SessionEndHookInput = BaseHookInput & {
  hook_event_name: "SessionEnd";
  reason: ExitReason; // String do array EXIT_REASONS
};
```

<h4 id="stophookinput">
  `StopHookInput`
</h4>

```typescript theme={null}
type StopHookInput = BaseHookInput & {
  hook_event_name: "Stop";
  stop_hook_active: boolean;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};
```

<h4 id="subagentstarthookinput">
  `SubagentStartHookInput`
</h4>

```typescript theme={null}
type SubagentStartHookInput = BaseHookInput & {
  hook_event_name: "SubagentStart";
  agent_id: string;
  agent_type: string;
};
```

<h4 id="subagentstophookinput">
  `SubagentStopHookInput`
</h4>

```typescript theme={null}
type SubagentStopHookInput = BaseHookInput & {
  hook_event_name: "SubagentStop";
  stop_hook_active: boolean;
  agent_id: string;
  agent_transcript_path: string;
  agent_type: string;
  last_assistant_message?: string;
  background_tasks?: BackgroundTaskSummary[];
  session_crons?: SessionCronSummary[];
};

type BackgroundTaskSummary = {
  id: string;
  type: string;
  status: string;
  description: string;
  command?: string;
  agent_type?: string;
  server?: string;
  tool?: string;
  name?: string;
};

type SessionCronSummary = {
  id: string;
  schedule: string;
  recurring: boolean;
  prompt: string;
};
```

<h4 id="precompacthookinput">
  `PreCompactHookInput`
</h4>

```typescript theme={null}
type PreCompactHookInput = BaseHookInput & {
  hook_event_name: "PreCompact";
  trigger: "manual" | "auto";
  custom_instructions: string | null;
};
```

<h4 id="permissionrequesthookinput">
  `PermissionRequestHookInput`
</h4>

```typescript theme={null}
type PermissionRequestHookInput = BaseHookInput & {
  hook_event_name: "PermissionRequest";
  tool_name: string;
  tool_input: unknown;
  permission_suggestions?: PermissionUpdate[];
};
```

<h4 id="setuphookinput">
  `SetupHookInput`
</h4>

```typescript theme={null}
type SetupHookInput = BaseHookInput & {
  hook_event_name: "Setup";
  trigger: "init" | "maintenance";
};
```

<h4 id="teammateidlehookinput">
  `TeammateIdleHookInput`
</h4>

```typescript theme={null}
type TeammateIdleHookInput = BaseHookInput & {
  hook_event_name: "TeammateIdle";
  teammate_name: string;
  /** @deprecated desde v2.1.178. Carrega o nome da equipe derivado da sessão; será removido. */
  team_name: string;
};
```

<h4 id="taskcompletedhookinput">
  `TaskCompletedHookInput`
</h4>

```typescript theme={null}
type TaskCompletedHookInput = BaseHookInput & {
  hook_event_name: "TaskCompleted";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated desde v2.1.178. Carrega o nome da equipe derivado da sessão; será removido. */
  team_name?: string;
};
```

<h4 id="configchangehookinput">
  `ConfigChangeHookInput`
</h4>

```typescript theme={null}
type ConfigChangeHookInput = BaseHookInput & {
  hook_event_name: "ConfigChange";
  source:
    | "user_settings"
    | "project_settings"
    | "local_settings"
    | "policy_settings"
    | "skills";
  file_path?: string;
};
```

<h4 id="worktreecreatehookinput">
  `WorktreeCreateHookInput`
</h4>

```typescript theme={null}
type WorktreeCreateHookInput = BaseHookInput & {
  hook_event_name: "WorktreeCreate";
  name: string;
};
```

<h4 id="worktreeremovehookinput">
  `WorktreeRemoveHookInput`
</h4>

```typescript theme={null}
type WorktreeRemoveHookInput = BaseHookInput & {
  hook_event_name: "WorktreeRemove";
  worktree_path: string;
};
```

<h4 id="messagedisplayhookinput">
  `MessageDisplayHookInput`
</h4>

```typescript theme={null}
type MessageDisplayHookInput = BaseHookInput & {
  hook_event_name: "MessageDisplay";
  turn_id: string;
  message_id: string;
  index: number;
  final: boolean;
  delta: string;
};
```

<h3 id="hookjsonoutput">
  `HookJSONOutput`
</h3>

Valor de retorno de hook.

```typescript theme={null}
type HookJSONOutput = AsyncHookJSONOutput | SyncHookJSONOutput;
```

<h4 id="asynchookjsonoutput">
  `AsyncHookJSONOutput`
</h4>

```typescript theme={null}
type AsyncHookJSONOutput = {
  async: true;
  asyncTimeout?: number;
};
```

<h4 id="synchookjsonoutput">
  `SyncHookJSONOutput`
</h4>

```typescript theme={null}
type SyncHookJSONOutput = {
  continue?: boolean;
  suppressOutput?: boolean;
  stopReason?: string;
  decision?: "approve" | "block";
  systemMessage?: string;
  reason?: string;
  hookSpecificOutput?:
    | {
        hookEventName: "PreToolUse";
        permissionDecision?: "allow" | "deny" | "ask" | "defer";
        permissionDecisionReason?: string;
        updatedInput?: Record<string, unknown>;
        additionalContext?: string;
      }
    | {
        hookEventName: "UserPromptSubmit";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated Use `updatedToolOutput`, which works for all tools. */
        updatedMCPToolOutput?: unknown;
      }
    | {
        hookEventName: "PostToolUseFailure";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolBatch";
        additionalContext?: string;
      }
    | {
        hookEventName: "Notification";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionRequest";
        decision:
          | {
              behavior: "allow";
              updatedInput?: Record<string, unknown>;
              updatedPermissions?: PermissionUpdate[];
            }
          | {
              behavior: "deny";
              message?: string;
              interrupt?: boolean;
            };
      };
};
```

<h2 id="tool-input-types">
  Tipos de Entrada de Ferramenta
</h2>

Documentação de esquemas de entrada para todas as ferramentas integradas do Claude Code. Esses tipos são exportados de `@anthropic-ai/claude-agent-sdk` e podem ser usados para interações de ferramenta type-safe.

<h3 id="toolinputschemas">
  `ToolInputSchemas`
</h3>

União de todos os tipos de entrada de ferramenta, exportada de `@anthropic-ai/claude-agent-sdk`.

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | AskUserQuestionInput
  | BashInput
  | TaskOutputInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ReadMcpResourceInput
  | SubscribeMcpResourceInput
  | SubscribePollingInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | UnsubscribeMcpResourceInput
  | UnsubscribePollingInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Nome da ferramenta:** `Agent` (anteriormente `Task`, que ainda é aceito como alias)

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan";
  isolation?: "worktree";
};
```

Lança um novo agente para lidar com tarefas complexas e multi-etapas autonomamente.

<h3 id="askuserquestion">
  AskUserQuestion
</h3>

**Nome da ferramenta:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionInput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
};
```

Faz perguntas de esclarecimento ao usuário durante a execução. Veja [Lidar com aprovações e entrada do usuário](/docs/pt/agent-sdk/user-input#handle-clarifying-questions) para detalhes de uso.

<h3 id="bash">
  Bash
</h3>

**Nome da ferramenta:** `Bash`

```typescript theme={null}
type BashInput = {
  command: string;
  timeout?: number; // milliseconds, max 600000; higher values are clamped to the max
  description?: string;
  run_in_background?: boolean;
  dangerouslyDisableSandbox?: boolean;
};
```

Executa comandos bash em uma sessão de shell persistente com timeout opcional e execução em background.

<h3 id="monitor">
  Monitor
</h3>

**Nome da ferramenta:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
  description: string;
  timeout_ms?: number;
  persistent?: boolean;
};
```

Executa uma fonte de background e entrega cada evento para Claude para que possa reagir sem polling: `command` executa um script e emite um evento por linha stdout, e `ws` abre um WebSocket e emite um evento por frame de texto. Forneça exatamente um de `command` ou `ws`. A fonte `ws` requer Claude Code v2.1.195 ou posterior.

Defina `persistent: true` para watches de comprimento de sessão, como tails de log. Quando Monitor executa um comando, ele segue as mesmas regras de permissão que Bash; um watch de WebSocket solicita aprovação separadamente. Veja a [referência da ferramenta Monitor](/docs/pt/tools-reference#monitor-tool) para comportamento e disponibilidade de provedor.

<h3 id="taskoutput">
  TaskOutput
</h3>

**Nome da ferramenta:** `TaskOutput`

```typescript theme={null}
type TaskOutputInput = {
  task_id: string;
  block: boolean;
  timeout: number;
};
```

Recupera saída de uma tarefa de background em execução ou concluída.

<h3 id="edit">
  Edit
</h3>

**Nome da ferramenta:** `Edit`

```typescript theme={null}
type FileEditInput = {
  file_path: string;
  old_string: string;
  new_string: string;
  replace_all?: boolean;
};
```

Realiza substituições exatas de string em arquivos.

<h3 id="read">
  Read
</h3>

**Nome da ferramenta:** `Read`

```typescript theme={null}
type FileReadInput = {
  file_path: string;
  offset?: number;
  limit?: number;
  pages?: string;
};
```

Lê arquivos do sistema de arquivos local, incluindo texto, imagens, PDFs e notebooks Jupyter. Use `pages` para intervalos de página PDF (por exemplo, `"1-5"`).

<h3 id="write">
  Write
</h3>

**Nome da ferramenta:** `Write`

```typescript theme={null}
type FileWriteInput = {
  file_path: string;
  content: string;
};
```

Escreve um arquivo no sistema de arquivos local, sobrescrevendo se existir.

<h3 id="glob">
  Glob
</h3>

**Nome da ferramenta:** `Glob`

```typescript theme={null}
type GlobInput = {
  pattern: string;
  path?: string;
};
```

Correspondência rápida de padrão de arquivo que funciona com qualquer tamanho de codebase.

<h3 id="grep">
  Grep
</h3>

**Nome da ferramenta:** `Grep`

```typescript theme={null}
type GrepInput = {
  pattern: string;
  path?: string;
  glob?: string;
  type?: string;
  output_mode?: "content" | "files_with_matches" | "count";
  "-i"?: boolean;
  "-n"?: boolean;
  "-B"?: number;
  "-A"?: number;
  "-C"?: number;
  context?: number;
  head_limit?: number;
  offset?: number;
  multiline?: boolean;
};
```

Ferramenta de busca poderosa construída em ripgrep com suporte a regex.

<h3 id="taskstop">
  TaskStop
</h3>

**Nome da ferramenta:** `TaskStop`

```typescript theme={null}
type TaskStopInput = {
  task_id?: string;
  shell_id?: string; // Descontinuado: use task_id
};
```

Para uma tarefa de background em execução ou shell por ID. A partir de v2.1.198, `task_id` também aceita um colega de equipe de agentes ou um agente de background nomeado por ID de agente ou nome.

<h3 id="notebookedit">
  NotebookEdit
</h3>

**Nome da ferramenta:** `NotebookEdit`

```typescript theme={null}
type NotebookEditInput = {
  notebook_path: string;
  cell_id?: string;
  new_source: string;
  cell_type?: "code" | "markdown";
  edit_mode?: "replace" | "insert" | "delete";
};
```

Edita células em arquivos de notebook Jupyter.

<h3 id="webfetch">
  WebFetch
</h3>

**Nome da ferramenta:** `WebFetch`

```typescript theme={null}
type WebFetchInput = {
  url: string;
  prompt: string;
};
```

Busca conteúdo de uma URL e o processa com um modelo de IA.

<h3 id="websearch">
  WebSearch
</h3>

**Nome da ferramenta:** `WebSearch`

```typescript theme={null}
type WebSearchInput = {
  query: string;
  allowed_domains?: string[];
  blocked_domains?: string[];
};
```

Pesquisa a web e retorna resultados formatados.

<h3 id="workflow">
  Workflow
</h3>

**Nome da ferramenta:** `Workflow`

```typescript theme={null}
type WorkflowInput = {
  script?: string;
  name?: string;
  scriptPath?: string;
  args?: unknown;
  resumeFromRunId?: string;
};
```

Executa um [workflow dinâmico](/docs/pt/workflows): um script que orquestra muitos subagentes em background e retorna um resultado consolidado. A ferramenta `Workflow` está disponível no Agent SDK v0.3.149 e posterior. Pelo menos um de `script`, `name` ou `scriptPath` é obrigatório.

| Campo             | Tipo      | Descrição                                                                                                                                                                                                                                                                                              |
| ----------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `script`          | `string`  | Script de workflow inline. Deve começar com `export const meta = { name, description }` como um literal, seguido pelo corpo do script usando `agent()`, `parallel()`, `pipeline()` e `phase()`. Um array `phases` opcional em `meta` agrupa agentes sob estágios nomeados na visualização de progresso |
| `name`            | `string`  | Nome de um workflow integrado ou um salvo em `.claude/workflows/`. Resolvido para um script                                                                                                                                                                                                            |
| `scriptPath`      | `string`  | Caminho para um arquivo de script de workflow no disco. Tem precedência sobre `script` e `name`. Cada invocação persiste seu script e retorna o caminho no resultado, para que você possa editar esse arquivo e reinvocar com o mesmo `scriptPath` para iterar                                         |
| `args`            | `unknown` | Valor de entrada exposto ao script como o `args` global, para workflows nomeados parametrizados, como uma pergunta de pesquisa ou uma lista de caminhos de arquivo. Passe arrays e objetos como valores JSON reais, não como uma string codificada em JSON                                             |
| `resumeFromRunId` | `string`  | ID de execução de uma invocação anterior de `Workflow` para retomar. Chamadas `agent()` concluídas com entradas inalteradas retornam resultados em cache; apenas chamadas alteradas ou novas são executadas ao vivo. Apenas a mesma sessão                                                             |

<h3 id="todowrite">
  TodoWrite
</h3>

**Nome da ferramenta:** `TodoWrite`

```typescript theme={null}
type TodoWriteInput = {
  todos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Cria e gerencia uma lista de tarefas estruturada para rastrear progresso.

<Note>
  A partir do TypeScript Agent SDK 0.3.142, `TodoWrite` está desabilitado por padrão. Use `TaskCreate`, `TaskGet`, `TaskUpdate` e `TaskList` em vez disso. Veja [Migrar para ferramentas Task](/docs/pt/agent-sdk/todo-tracking#migrate-to-task-tools) para atualizar seu código de monitoramento, ou defina `CLAUDE_CODE_ENABLE_TASKS=0` para reverter para `TodoWrite`.
</Note>

<h3 id="taskcreate">
  TaskCreate
</h3>

**Nome da ferramenta:** `TaskCreate`

```typescript theme={null}
type TaskCreateInput = {
  subject: string;
  description: string;
  activeForm?: string;
  metadata?: Record<string, unknown>;
};
```

Cria uma única tarefa e retorna seu ID atribuído.

<h3 id="taskupdate">
  TaskUpdate
</h3>

**Nome da ferramenta:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateInput = {
  taskId: string;
  status?: "pending" | "in_progress" | "completed" | "deleted";
  subject?: string;
  description?: string;
  activeForm?: string;
  addBlocks?: string[];
  addBlockedBy?: string[];
  owner?: string;
  metadata?: Record<string, unknown>;
};
```

Corrige uma tarefa por ID. Defina `status` para `"deleted"` para removê-la.

<h3 id="taskget">
  TaskGet
</h3>

**Nome da ferramenta:** `TaskGet`

```typescript theme={null}
type TaskGetInput = {
  taskId: string;
};
```

Retorna detalhes completos para uma tarefa, ou `null` quando o ID não é encontrado.

<h3 id="tasklist">
  TaskList
</h3>

**Nome da ferramenta:** `TaskList`

```typescript theme={null}
type TaskListInput = {};
```

Retorna um snapshot de todas as tarefas na lista atual.

<h3 id="exitplanmode">
  ExitPlanMode
</h3>

**Nome da ferramenta:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeInput = {
  /** Descontinuado: não é mais usado. */
  allowedPrompts?: Array<{
    tool: "Bash";
    prompt: string;
  }>;
};
```

Sai do modo de planejamento. O campo `allowedPrompts` está descontinuado e ignorado; Claude Code ainda o aceita para que chamadores existentes e transcrições sejam validados. Antes de v2.1.205, ele solicitava permissões Bash baseadas em prompt para implementar o plano.

<h3 id="listmcpresources">
  ListMcpResources
</h3>

**Nome da ferramenta:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesInput = {
  server?: string;
};
```

Lista recursos MCP disponíveis de servidores conectados.

<h3 id="readmcpresource">
  ReadMcpResource
</h3>

**Nome da ferramenta:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceInput = {
  server: string;
  uri: string;
};
```

Lê um recurso MCP específico de um servidor.

<h3 id="enterworktree">
  EnterWorktree
</h3>

**Nome da ferramenta:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeInput = {
  name?: string;
  path?: string;
};
```

Cria e entra em um worktree git temporário para trabalho isolado. Passe `path` para mudar para um worktree existente em vez de criar um novo. Na primeira entrada, o alvo deve ser um worktree registrado do repositório atual ou, em um workspace multi-repo, de um repositório aninhado dentro dele; de dentro de uma sessão de worktree, deve estar sob `.claude/worktrees/` do repositório da sessão. `name` e `path` são mutuamente exclusivos.

<h2 id="tool-output-types">
  Tipos de Saída de Ferramenta
</h2>

Documentação de esquemas de saída para todas as ferramentas integradas do Claude Code. Esses tipos são exportados de `@anthropic-ai/claude-agent-sdk` e representam os dados de resposta reais retornados por cada ferramenta.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

União de todos os tipos de saída de ferramenta.

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | AskUserQuestionOutput
  | BashOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | MonitorOutput
  | NotebookEditOutput
  | ReadMcpResourceOutput
  | TaskCreateOutput
  | TaskGetOutput
  | TaskListOutput
  | TaskStopOutput
  | TaskUpdateOutput
  | TodoWriteOutput
  | WebFetchOutput
  | WebSearchOutput
  | WorkflowOutput;
```

<h3 id="agent-2">
  Agent
</h3>

**Nome da ferramenta:** `Agent` (anteriormente `Task`, que ainda é aceito como alias)

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      totalToolUseCount: number;
      totalDurationMs: number;
      totalTokens: number;
      usage: {
        input_tokens: number;
        output_tokens: number;
        cache_creation_input_tokens: number | null;
        cache_read_input_tokens: number | null;
        server_tool_use: {
          web_search_requests: number;
          web_fetch_requests: number;
        } | null;
        service_tier: string | null;
        cache_creation: {
          ephemeral_1h_input_tokens: number;
          ephemeral_5m_input_tokens: number;
        } | null;
        inference_geo?: string | null;
        speed?: string | null;
        iterations?: unknown;
      };
      toolStats?: {
        readCount: number;
        searchCount: number;
        bashCount: number;
        editFileCount: number;
        linesAdded: number;
        linesRemoved: number;
        otherToolCount: number;
        frameCount?: number;
      };
      prompt: string;
      worktreePath?: string;
      worktreeBranch?: string;
    }
  | {
      status: "async_launched";
      isAsync?: true;
      agentId: string;
      description: string;
      resolvedModel?: string;
      prompt: string;
      outputFile: string;
      canReadOutputFile?: boolean;
    }
  | {
      status: "remote_launched";
      taskId: string;
      sessionUrl: string;
      description: string;
      prompt: string;
      outputFile: string;
    };
```

Retorna o resultado do subagente. Discriminado no campo `status`: `"completed"` para tarefas concluídas, `"async_launched"` para tarefas em background e `"remote_launched"` para tarefas que o Claude Code despachou para uma sessão em nuvem remota, onde `sessionUrl` vincula a essa sessão e `taskId` a identifica.

O campo `resolvedModel` nas variantes `completed` e `async_launched` nomeia o modelo em que o subagente realmente foi executado, que pode diferir do input `model` solicitado quando [`availableModels`](/docs/pt/model-config#restrict-model-selection) ou outra substituição se aplica. Este campo requer Claude Code v2.1.174 ou posterior.

Na variante `completed`, `worktreePath` é definido quando o subagente foi executado em um worktree git isolado, e `worktreeBranch` nomeia o branch desse worktree quando o Claude Code o criou. `usage.service_tier` carrega a string de nível de serviço que a API relatou para as solicitações do subagente.

Antes da v2.1.207, o tipo publicado era mais restrito. Ele omitia `worktreePath`, `worktreeBranch`, `citations`, `toolStats.frameCount` e os campos de uso `inference_geo`, `speed` e `iterations`, e digitava `service_tier` como `"standard" | "priority" | "batch"`. Os campos que o tipo marca como opcionais podem estar ausentes nos resultados registrados por versões anteriores.

<h3 id="askuserquestion-2">
  AskUserQuestion
</h3>

**Nome da ferramenta:** `AskUserQuestion`

```typescript theme={null}
type AskUserQuestionOutput = {
  questions: Array<{
    question: string;
    header: string;
    options: Array<{ label: string; description: string; preview?: string }>;
    multiSelect: boolean;
  }>;
  answers: Record<string, string>;
  response?: string;
};
```

Retorna as perguntas feitas e as respostas do usuário. `response` é definido quando o usuário digitou uma resposta de forma livre em vez de responder às perguntas estruturadas; quando presente, Claude recebe "O usuário respondeu: …" em vez da lista de respostas por pergunta.

<h3 id="bash-2">
  Bash
</h3>

**Nome da ferramenta:** `Bash`

```typescript theme={null}
type BashOutput = {
  stdout: string;
  stderr: string;
  rawOutputPath?: string;
  interrupted: boolean;
  isImage?: boolean;
  backgroundTaskId?: string;
  backgroundedByUser?: boolean;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
};
```

Retorna saída de comando com stdout/stderr divididos. Comandos em background incluem um `backgroundTaskId`.

<h3 id="monitor-2">
  Monitor
</h3>

**Nome da ferramenta:** `Monitor`

```typescript theme={null}
type MonitorOutput = {
  taskId: string;
  timeoutMs: number;
  persistent?: boolean;
};
```

Retorna o ID da tarefa em background para o monitor em execução. Use este ID com `TaskStop` para cancelar a observação antecipadamente.

<h3 id="edit-2">
  Edit
</h3>

**Nome da ferramenta:** `Edit`

```typescript theme={null}
type FileEditOutput = {
  filePath: string;
  oldString: string;
  newString: string;
  originalFile: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  userModified: boolean;
  replaceAll: boolean;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
  };
};
```

Retorna o diff estruturado da operação de edição.

<h3 id="read-2">
  Read
</h3>

**Nome da ferramenta:** `Read`

```typescript theme={null}
type FileReadOutput =
  | {
      type: "text";
      file: {
        filePath: string;
        content: string;
        numLines: number;
        startLine: number;
        totalLines: number;
      };
    }
  | {
      type: "image";
      file: {
        base64: string;
        type: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        originalSize: number;
        dimensions?: {
          originalWidth?: number;
          originalHeight?: number;
          displayWidth?: number;
          displayHeight?: number;
        };
      };
    }
  | {
      type: "notebook";
      file: {
        filePath: string;
        cells: unknown[];
      };
    }
  | {
      type: "pdf";
      file: {
        filePath: string;
        base64: string;
        originalSize: number;
      };
    }
  | {
      type: "parts";
      file: {
        filePath: string;
        originalSize: number;
        count: number;
        outputDir: string;
      };
    };
```

Retorna conteúdo do arquivo em um formato apropriado ao tipo de arquivo. Discriminado no campo `type`.

<h3 id="write-2">
  Write
</h3>

**Nome da ferramenta:** `Write`

```typescript theme={null}
type FileWriteOutput = {
  type: "create" | "update";
  filePath: string;
  content: string;
  structuredPatch: Array<{
    oldStart: number;
    oldLines: number;
    newStart: number;
    newLines: number;
    lines: string[];
  }>;
  originalFile: string | null;
  gitDiff?: {
    filename: string;
    status: "modified" | "added";
    additions: number;
    deletions: number;
    changes: number;
    patch: string;
  };
};
```

Retorna o resultado da escrita com informações de diff estruturado.

<h3 id="glob-2">
  Glob
</h3>

**Nome da ferramenta:** `Glob`

```typescript theme={null}
type GlobOutput = {
  durationMs: number;
  numFiles: number;
  filenames: string[];
  truncated: boolean;
};
```

Retorna caminhos de arquivo correspondentes ao padrão glob, classificados por tempo de modificação.

<h3 id="grep-2">
  Grep
</h3>

**Nome da ferramenta:** `Grep`

```typescript theme={null}
type GrepOutput = {
  mode?: "content" | "files_with_matches" | "count";
  numFiles: number;
  filenames: string[];
  content?: string;
  numLines?: number;
  numMatches?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Retorna resultados de busca. A forma varia por `mode`: lista de arquivo, conteúdo com correspondências ou contagens de correspondência.

<h3 id="taskstop-2">
  TaskStop
</h3>

**Nome da ferramenta:** `TaskStop`

```typescript theme={null}
type TaskStopOutput = {
  message: string;
  task_id: string;
  task_type: string;
  command?: string;
};
```

Retorna confirmação após parar a tarefa em background.

<h3 id="notebookedit-2">
  NotebookEdit
</h3>

**Nome da ferramenta:** `NotebookEdit`

```typescript theme={null}
type NotebookEditOutput = {
  new_source: string;
  cell_id?: string;
  cell_type: "code" | "markdown";
  language: string;
  edit_mode: string;
  error?: string;
  notebook_path: string;
  original_file: string;
  updated_file: string;
};
```

Retorna o resultado da edição do notebook com conteúdo de arquivo original e atualizado.

<h3 id="webfetch-2">
  WebFetch
</h3>

**Nome da ferramenta:** `WebFetch`

```typescript theme={null}
type WebFetchOutput = {
  bytes: number;
  code: number;
  codeText: string;
  result: string;
  durationMs: number;
  url: string;
};
```

Retorna o conteúdo buscado com status HTTP e metadados.

<h3 id="websearch-2">
  WebSearch
</h3>

**Nome da ferramenta:** `WebSearch`

```typescript theme={null}
type WebSearchOutput = {
  query: string;
  results: Array<
    | {
        tool_use_id: string;
        content: Array<{ title: string; url: string }>;
      }
    | string
  >;
  durationSeconds: number;
};
```

Retorna resultados de busca da web.

<h3 id="workflow-2">
  Workflow
</h3>

**Nome da ferramenta:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched";
  taskId: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  error?: string;
};
```

Retorna imediatamente após a ferramenta aceitar a invocação. O resultado final chega mais tarde como uma conclusão de tarefa. Verifique `error` antes de tratar a execução como iniciada: um script que falha sua verificação de sintaxe retorna `status: "async_launched"` com `error` definido e nunca é executado.

| Campo           | Tipo               | Descrição                                                                                                                                                 |
| --------------- | ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched"` | A ferramenta aceitou a invocação. Este é o único valor que o campo assume                                                                                 |
| `taskId`        | `string`           | Identificador de tarefa em background para a execução                                                                                                     |
| `runId`         | `string`           | Identificador de execução de workflow para passar como `resumeFromRunId` em uma invocação posterior                                                       |
| `summary`       | `string`           | Descrição de uma linha do que o workflow faz                                                                                                              |
| `transcriptDir` | `string`           | Diretório onde transcrições de subagente são escritas durante a execução                                                                                  |
| `scriptPath`    | `string`           | Caminho para o script de workflow persistido para esta execução. Edite-o e passe de volta como `scriptPath` para executar novamente sem reenviar o script |
| `error`         | `string`           | Definido quando o script falha sua verificação de sintaxe. Quando presente, a execução não foi iniciada apesar do status `async_launched`                 |

<h3 id="todowrite-2">
  TodoWrite
</h3>

**Nome da ferramenta:** `TodoWrite`

```typescript theme={null}
type TodoWriteOutput = {
  oldTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
  newTodos: Array<{
    content: string;
    status: "pending" | "in_progress" | "completed";
    activeForm: string;
  }>;
};
```

Retorna as listas de tarefas anteriores e atualizadas.

<Note>
  A partir do TypeScript Agent SDK 0.3.142, `TodoWrite` está desabilitado por padrão. Use `TaskCreate`, `TaskGet`, `TaskUpdate` e `TaskList` em seu lugar. Veja [Migrar para ferramentas de Task](/docs/pt/agent-sdk/todo-tracking#migrate-to-task-tools) para atualizar seu código de monitoramento, ou defina `CLAUDE_CODE_ENABLE_TASKS=0` para reverter para `TodoWrite`.
</Note>

<h3 id="taskcreate-2">
  TaskCreate
</h3>

**Nome da ferramenta:** `TaskCreate`

```typescript theme={null}
type TaskCreateOutput = {
  task: {
    id: string;
    subject: string;
  };
};
```

Retorna a tarefa criada com seu ID atribuído.

<h3 id="taskupdate-2">
  TaskUpdate
</h3>

**Nome da ferramenta:** `TaskUpdate`

```typescript theme={null}
type TaskUpdateOutput = {
  success: boolean;
  taskId: string;
  updatedFields: string[];
  error?: string;
  statusChange?: {
    from: string;
    to: string;
  };
};
```

Retorna o resultado da atualização, incluindo quais campos foram alterados.

<h3 id="taskget-2">
  TaskGet
</h3>

**Nome da ferramenta:** `TaskGet`

```typescript theme={null}
type TaskGetOutput = {
  task: {
    id: string;
    subject: string;
    description: string;
    status: "pending" | "in_progress" | "completed";
    blocks: string[];
    blockedBy: string[];
  } | null;
};
```

Retorna o registro completo da tarefa, ou `null` quando o ID não é encontrado.

<h3 id="tasklist-2">
  TaskList
</h3>

**Nome da ferramenta:** `TaskList`

```typescript theme={null}
type TaskListOutput = {
  tasks: Array<{
    id: string;
    subject: string;
    status: "pending" | "in_progress" | "completed";
    owner?: string;
    blockedBy: string[];
  }>;
};
```

Retorna um snapshot de todas as tarefas na lista atual.

<h3 id="exitplanmode-2">
  ExitPlanMode
</h3>

**Nome da ferramenta:** `ExitPlanMode`

```typescript theme={null}
type ExitPlanModeOutput = {
  plan: string | null;
  isAgent: boolean;
  filePath?: string;
  hasTaskTool?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

Retorna o estado do plano após sair do modo de planejamento.

<h3 id="listmcpresources-2">
  ListMcpResources
</h3>

**Nome da ferramenta:** `ListMcpResourcesTool`

```typescript theme={null}
type ListMcpResourcesOutput = Array<{
  uri: string;
  name: string;
  mimeType?: string;
  description?: string;
  server: string;
}>;
```

Retorna um array de recursos MCP disponíveis.

<h3 id="readmcpresource-2">
  ReadMcpResource
</h3>

**Nome da ferramenta:** `ReadMcpResourceTool`

```typescript theme={null}
type ReadMcpResourceOutput = {
  contents: Array<{
    uri: string;
    mimeType?: string;
    text?: string;
  }>;
};
```

Retorna o conteúdo do recurso MCP solicitado.

<h3 id="enterworktree-2">
  EnterWorktree
</h3>

**Nome da ferramenta:** `EnterWorktree`

```typescript theme={null}
type EnterWorktreeOutput = {
  worktreePath: string;
  worktreeBranch?: string;
  message: string;
};
```

Retorna informações sobre o worktree git.

<h2 id="permission-types">
  Tipos de Permissão
</h2>

<h3 id="permissionupdate">
  `PermissionUpdate`
</h3>

Operações para atualizar permissões.

```typescript theme={null}
type PermissionUpdate =
  | {
      type: "addRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "replaceRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeRules";
      rules: PermissionRuleValue[];
      behavior: PermissionBehavior;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "setMode";
      mode: PermissionMode;
      destination: PermissionUpdateDestination;
    }
  | {
      type: "addDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    }
  | {
      type: "removeDirectories";
      directories: string[];
      destination: PermissionUpdateDestination;
    };
```

<h3 id="permissionbehavior">
  `PermissionBehavior`
</h3>

```typescript theme={null}
type PermissionBehavior = "allow" | "deny" | "ask";
```

<h3 id="permissionupdatedestination">
  `PermissionUpdateDestination`
</h3>

```typescript theme={null}
type PermissionUpdateDestination =
  | "userSettings" // Configurações globais do usuário
  | "projectSettings" // Configurações de projeto por diretório
  | "localSettings" // Configurações locais do projeto
  | "session" // Apenas sessão atual
  | "cliArg"; // Argumento CLI
```

<h3 id="permissionrulevalue">
  `PermissionRuleValue`
</h3>

```typescript theme={null}
type PermissionRuleValue = {
  toolName: string;
  ruleContent?: string;
};
```

<h2 id="other-types">
  Outros Tipos
</h2>

<h3 id="apikeysource">
  `ApiKeySource`
</h3>

```typescript theme={null}
type ApiKeySource = "user" | "project" | "org" | "temporary" | "oauth";
```

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Recursos beta disponíveis que podem ser ativados via opção `betas`. Veja [Beta headers](https://platform.claude.com/docs/pt/api/beta-headers) para mais informações.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  O beta `context-1m-2025-08-07` foi descontinuado a partir de 30 de abril de 2026. Passar este valor com Claude Sonnet 4.5 ou Sonnet 4 não tem efeito, e requisições que excedem a janela de contexto padrão de 200k-token retornam um erro. Para usar uma janela de contexto de 1M-token, migre para [Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 ou Claude Opus 4.8](https://platform.claude.com/docs/pt/about-claude/models/overview), que incluem contexto de 1M a preço padrão sem header beta necessário.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Informações sobre um comando slash disponível.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
};
```

<h3 id="modelinfo">
  `ModelInfo`
</h3>

Informações sobre um modelo disponível.

```typescript theme={null}
type ModelInfo = {
  value: string;
  resolvedModel?: string;
  displayName: string;
  description: string;
  supportsEffort?: boolean;
  supportedEffortLevels?: ("low" | "medium" | "high" | "xhigh" | "max")[];
  supportsAdaptiveThinking?: boolean;
  supportsFastMode?: boolean;
  supportsAutoMode?: boolean;
};
```

| Campo                      | Tipo                                                               | Descrição                                                                                                                                                                                                                                                                                                              |
| :------------------------- | :----------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `value`                    | `string`                                                           | Identificador de modelo para passar em chamadas de API                                                                                                                                                                                                                                                                 |
| `resolvedModel`            | `string \| undefined`                                              | ID de modelo canônico que o `value` desta entrada resolve. Uma entrada de alias como `sonnet` resolve para um ID de modelo explícito como `claude-sonnet-5`, para que um host possa corresponder um ID de modelo explícito armazenado contra a entrada de alias que o cobre. Requer Claude Code v2.1.197 ou posterior. |
| `displayName`              | `string`                                                           | Nome de exibição legível para humanos                                                                                                                                                                                                                                                                                  |
| `description`              | `string`                                                           | Descrição das capacidades do modelo                                                                                                                                                                                                                                                                                    |
| `supportsEffort`           | `boolean \| undefined`                                             | Se este modelo suporta níveis de esforço                                                                                                                                                                                                                                                                               |
| `supportedEffortLevels`    | `("low" \| "medium" \| "high" \| "xhigh" \| "max")[] \| undefined` | Níveis de esforço que este modelo aceita                                                                                                                                                                                                                                                                               |
| `supportsAdaptiveThinking` | `boolean \| undefined`                                             | Se este modelo suporta pensamento adaptativo, onde Claude decide quando e quanto pensar                                                                                                                                                                                                                                |
| `supportsFastMode`         | `boolean \| undefined`                                             | Se este modelo suporta modo rápido                                                                                                                                                                                                                                                                                     |
| `supportsAutoMode`         | `boolean \| undefined`                                             | Se este modelo suporta modo automático                                                                                                                                                                                                                                                                                 |

<h3 id="agentinfo">
  `AgentInfo`
</h3>

Informações sobre um subagente disponível que pode ser invocado via ferramenta Agent.

```typescript theme={null}
type AgentInfo = {
  name: string;
  description: string;
  model?: string;
};
```

| Campo         | Tipo                  | Descrição                                                                       |
| :------------ | :-------------------- | :------------------------------------------------------------------------------ |
| `name`        | `string`              | Identificador de tipo de agente (por exemplo, `"Explore"`, `"general-purpose"`) |
| `description` | `string`              | Descrição de quando usar este agente                                            |
| `model`       | `string \| undefined` | Alias de modelo que este agente usa. Se omitido, herda o modelo do pai          |

<h3 id="mcpserverstatus">
  `McpServerStatus`
</h3>

Status de um servidor MCP conectado.

```typescript theme={null}
type McpServerStatus = {
  name: string;
  status: "connected" | "failed" | "needs-auth" | "pending" | "disabled";
  serverInfo?: {
    name: string;
    version: string;
  };
  error?: string;
  config?: McpServerStatusConfig;
  scope?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
  }[];
};
```

<h3 id="mcpserverstatusconfig">
  `McpServerStatusConfig`
</h3>

A configuração de um servidor MCP conforme relatado por `mcpServerStatus()`. Esta é a união de todos os tipos de transporte de servidor MCP.

```typescript theme={null}
type McpServerStatusConfig =
  | McpStdioServerConfig
  | McpSSEServerConfig
  | McpHttpServerConfig
  | McpSdkServerConfig
  | McpClaudeAIProxyServerConfig;
```

Veja [`McpServerConfig`](#mcpserverconfig) para detalhes sobre cada tipo de transporte.

<h3 id="accountinfo">
  `AccountInfo`
</h3>

Informações de conta para o usuário autenticado.

```typescript theme={null}
type AccountInfo = {
  email?: string;
  organization?: string;
  subscriptionType?: string;
  tokenSource?: string;
  apiKeySource?: string;
};
```

<h3 id="modelusage">
  `ModelUsage`
</h3>

Estatísticas de uso por modelo retornadas em mensagens de resultado. O valor `costUSD` é uma estimativa do lado do cliente. Veja [Rastrear custo e uso](/docs/pt/agent-sdk/cost-tracking) para ressalvas de faturamento.

```typescript theme={null}
type ModelUsage = {
  inputTokens: number;
  outputTokens: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
};
```

<h3 id="configscope">
  `ConfigScope`
</h3>

```typescript theme={null}
type ConfigScope = "local" | "user" | "project";
```

<h3 id="nonnullableusage">
  `NonNullableUsage`
</h3>

Uma versão de [`Usage`](#usage) com todos os campos anuláveis tornados não-anuláveis.

```typescript theme={null}
type NonNullableUsage = {
  [K in keyof Usage]: NonNullable<Usage[K]>;
};
```

<h3 id="usage">
  `Usage`
</h3>

Estatísticas de uso de token. Este é o tipo `BetaUsage` de `@anthropic-ai/sdk`.

```typescript theme={null}
type Usage = {
  input_tokens: number;
  output_tokens: number;
  cache_creation_input_tokens: number | null;
  cache_read_input_tokens: number | null;
  cache_creation: {
    ephemeral_5m_input_tokens: number;
    ephemeral_1h_input_tokens: number;
  } | null;
  server_tool_use: BetaServerToolUsage | null;
  service_tier: "standard" | "priority" | "batch" | null;
  speed: "standard" | "fast" | null;
  inference_geo: string | null;
  iterations: BetaIterationsUsage | null;
};
```

`BetaServerToolUsage` e `BetaIterationsUsage` são definidos em `@anthropic-ai/sdk`.

<h3 id="calltoolresult">
  `CallToolResult`
</h3>

Tipo de resultado de ferramenta MCP (de `@modelcontextprotocol/sdk/types.js`). `structuredContent` é um objeto JSON que pode ser retornado junto com `content`, incluindo blocos de imagem. Veja [Retornar dados estruturados](/docs/pt/agent-sdk/custom-tools#return-structured-data).

```typescript theme={null}
type CallToolResult = {
  content: Array<{
    type: "text" | "image" | "audio" | "resource" | "resource_link";
    // Campos adicionais variam por tipo
  }>;
  structuredContent?: Record<string, unknown>;
  isError?: boolean;
};
```

<h3 id="thinkingconfig">
  `ThinkingConfig`
</h3>

Controla o comportamento de pensamento/raciocínio do Claude. Tem precedência sobre o `maxThinkingTokens` descontinuado.

```typescript theme={null}
type ThinkingDisplay = "summarized" | "omitted";

type ThinkingConfig =
  | { type: "adaptive"; display?: ThinkingDisplay } // O modelo determina quando e quanto raciocinar (Opus 4.6+)
  | { type: "enabled"; budgetTokens?: number; display?: ThinkingDisplay } // Orçamento de token de pensamento fixo
  | { type: "disabled" }; // Sem pensamento estendido
```

O campo `display` opcional controla se o texto de pensamento é retornado `"summarized"` ou `"omitted"`. No Claude Opus 4.7 e posterior, o padrão da API é `"omitted"`, então defina `"summarized"` para receber conteúdo de pensamento em blocos `thinking`.

<h3 id="spawnedprocess">
  `SpawnedProcess`
</h3>

Interface para geração de processo personalizado (usada com opção `spawnClaudeCodeProcess`). `ChildProcess` já satisfaz esta interface.

```typescript theme={null}
interface SpawnedProcess {
  stdin: Writable;
  stdout: Readable;
  readonly killed: boolean;
  readonly exitCode: number | null;
  kill(signal: NodeJS.Signals): boolean;
  on(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  on(event: "error", listener: (error: Error) => void): void;
  once(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  once(event: "error", listener: (error: Error) => void): void;
  off(
    event: "exit",
    listener: (code: number | null, signal: NodeJS.Signals | null) => void
  ): void;
  off(event: "error", listener: (error: Error) => void): void;
}
```

<h3 id="spawnoptions">
  `SpawnOptions`
</h3>

Opções passadas para a função de geração personalizada.

```typescript theme={null}
interface SpawnOptions {
  command: string;
  args: string[];
  cwd?: string;
  env: Record<string, string | undefined>;
  signal: AbortSignal;
}
```

<Note>
  O campo `signal` informa sua função de geração quando desativar o processo. Passe-o como a opção `signal` para `spawn()` do Node, ou passe-o para seu manipulador de desmontagem de VM ou contêiner.

  Este sinal não dispara no instante em que [`Options.abortController`](#options) aborta. O SDK primeiro fecha o stdin do processo e aguarda cerca de dois segundos para que a CLI possa desligar corretamente, depois aborta este sinal. Para reagir no momento em que o chamador aborta, em vez disso, ouça seu próprio `Options.abortController.signal`, que sua função de geração pode referenciar de seu escopo envolvente.
</Note>

<h3 id="mcpsetserversresult">
  `McpSetServersResult`
</h3>

Resultado de uma operação `setMcpServers()`.

```typescript theme={null}
type McpSetServersResult = {
  added: string[];
  removed: string[];
  errors: Record<string, string>;
};
```

<h3 id="rewindfilesresult">
  `RewindFilesResult`
</h3>

Resultado de uma operação `rewindFiles()`.

```typescript theme={null}
type RewindFilesResult = {
  canRewind: boolean;
  error?: string;
  filesChanged?: string[];
  insertions?: number;
  deletions?: number;
};
```

<h3 id="sdkstatusmessage">
  `SDKStatusMessage`
</h3>

Mensagem de atualização de status (por exemplo, compactando).

```typescript theme={null}
type SDKStatusMessage = {
  type: "system";
  subtype: "status";
  status: "compacting" | null;
  permissionMode?: PermissionMode;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktasknotificationmessage">
  `SDKTaskNotificationMessage`
</h3>

Notificação quando uma tarefa de background é concluída, falha ou é parada. Tarefas de background incluem comandos Bash `run_in_background`, watches [Monitor](#monitor) e subagentes de background.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolusesummarymessage">
  `SDKToolUseSummaryMessage`
</h3>

Resumo do uso de ferramenta em uma conversa.

```typescript theme={null}
type SDKToolUseSummaryMessage = {
  type: "tool_use_summary";
  summary: string;
  preceding_tool_use_ids: string[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookstartedmessage">
  `SDKHookStartedMessage`
</h3>

Emitido quando um hook começa a executar.

Claude Code entrega esta mensagem, [`SDKHookProgressMessage`](#sdkhookprogressmessage), e [`SDKHookResponseMessage`](#sdkhookresponsemessage) para o fluxo de mensagens imediatamente, incluindo enquanto um hook `SessionStart` ou `Setup` ainda está em execução durante a inicialização da sessão. Claude Code v2.1.169 através de v2.1.203 entregou estas mensagens em um lote após um hook `SessionStart` ou `Setup` ser concluído; v2.1.204 restaurou a entrega ao vivo.

```typescript theme={null}
type SDKHookStartedMessage = {
  type: "system";
  subtype: "hook_started";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookprogressmessage">
  `SDKHookProgressMessage`
</h3>

Emitido enquanto um hook está em execução, com saída stdout/stderr.

```typescript theme={null}
type SDKHookProgressMessage = {
  type: "system";
  subtype: "hook_progress";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  stdout: string;
  stderr: string;
  output: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkhookresponsemessage">
  `SDKHookResponseMessage`
</h3>

Emitido quando um hook termina de executar.

```typescript theme={null}
type SDKHookResponseMessage = {
  type: "system";
  subtype: "hook_response";
  hook_id: string;
  hook_name: string;
  hook_event: string;
  output: string;
  stdout: string;
  stderr: string;
  exit_code?: number;
  outcome: "success" | "error" | "cancelled";
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktoolprogressmessage">
  `SDKToolProgressMessage`
</h3>

Emitido periodicamente enquanto uma ferramenta está sendo executada para indicar progresso.

```typescript theme={null}
type SDKToolProgressMessage = {
  type: "tool_progress";
  tool_use_id: string;
  tool_name: string;
  parent_tool_use_id: string | null;
  elapsed_time_seconds: number;
  task_id?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkauthstatusmessage">
  `SDKAuthStatusMessage`
</h3>

Emitido durante fluxos de autenticação.

```typescript theme={null}
type SDKAuthStatusMessage = {
  type: "auth_status";
  isAuthenticating: boolean;
  output: string[];
  error?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskstartedmessage">
  `SDKTaskStartedMessage`
</h3>

Emitido quando uma tarefa de background começa. O campo `task_type` é `"local_bash"` para comandos Bash de background e watches [Monitor](#monitor), `"local_agent"` para subagentes, ou `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskprogressmessage">
  `SDKTaskProgressMessage`
</h3>

Emitido periodicamente enquanto um subagente ou tarefa de background está em execução. O campo `summary` é preenchido apenas quando [`agentProgressSummaries`](#options) está ativado.

```typescript theme={null}
type SDKTaskProgressMessage = {
  type: "system";
  subtype: "task_progress";
  task_id: string;
  tool_use_id?: string;
  description: string;
  subagent_type?: string;
  usage: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  last_tool_name?: string;
  summary?: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdktaskupdatedmessage">
  `SDKTaskUpdatedMessage`
</h3>

Emitido quando o estado de uma tarefa de background muda, como quando ela faz a transição de `running` para `completed`. Mescle `patch` em seu mapa de tarefas local com chave `task_id`. O campo `end_time` é um timestamp de época Unix em milissegundos, comparável com `Date.now()`.

```typescript theme={null}
type SDKTaskUpdatedMessage = {
  type: "system";
  subtype: "task_updated";
  task_id: string;
  patch: {
    status?: "pending" | "running" | "completed" | "failed" | "killed";
    description?: string;
    end_time?: number;
    total_paused_ms?: number;
    error?: string;
    is_backgrounded?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkbackgroundtaskschangedmessage">
  `SDKBackgroundTasksChangedMessage`
</h3>

Emitido sempre que o conjunto de tarefas de background ativas muda: uma tarefa inicia, é concluída, é eliminada, ou um agente em primeiro plano é colocado em background. O array `tasks` é o conjunto completo ativo. Substitua qualquer conjunto em cache por cada payload em vez de emparelhar eventos `task_started` e `task_notification`, para que a próxima mudança de associação corrija qualquer evento que você tenha perdido.

A ordenação relativa a esses eventos por tarefa é não especificada, então não correlacione os dois fluxos.

Nada é emitido na inicialização. Redefina para um conjunto vazio sempre que o processo CLI da sessão inicia ou reinicia e deixe a próxima mudança de associação repopulá-lo.

Requer Claude Code v2.1.203 ou posterior.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Emitido enquanto Claude está produzindo um bloco de pensamento, incluindo um redatado, carregando uma estimativa em execução dos tokens de pensamento gerados até agora. `estimated_tokens` é o total em execução para o bloco de pensamento atual e `estimated_tokens_delta` é o incremento carregado por este frame. Use-o para exibição de progresso. A contagem final para o loop de agente de nível superior é o `usage.output_tokens` da mensagem de resultado, que [não inclui tokens de subagente](/docs/pt/agent-sdk/cost-tracking#get-the-total-cost-of-a-query); use [`modelUsage`](#modelusage) para contabilidade de árvore completa.

Requer Claude Code v2.1.153 ou posterior.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkfilespersistedevent">
  `SDKFilesPersistedEvent`
</h3>

Emitido quando checkpoints de arquivo são persistidos em disco.

```typescript theme={null}
type SDKFilesPersistedEvent = {
  type: "system";
  subtype: "files_persisted";
  files: { filename: string; file_id: string }[];
  failed: { filename: string; error: string }[];
  processed_at: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkratelimitevent">
  `SDKRateLimitEvent`
</h3>

Emitido quando a sessão encontra um limite de taxa.

```typescript theme={null}
type SDKRateLimitEvent = {
  type: "rate_limit_event";
  rate_limit_info: {
    status: "allowed" | "allowed_warning" | "rejected";
    resetsAt?: number;
    utilization?: number;
    errorCode?: "credits_required";
    canUserPurchaseCredits?: boolean;
    hasChargeableSavedPaymentMethod?: boolean;
  };
  uuid: UUID;
  session_id: string;
};
```

Quando `errorCode` é `"credits_required"`, a rejeição é de uma assinatura claude.ai cujo uso incluído está esgotado, e a sessão não pode continuar até que o usuário compre créditos de uso. `canUserPurchaseCredits` indica se o usuário autenticado pode comprar créditos para a conta, e `hasChargeableSavedPaymentMethod` indica se um método de pagamento salvo está registrado. Todos os três campos estão ausentes em eventos de limite de taxa que não são rejeições de créditos necessários. Requer Claude Code v2.1.181 ou posterior.

<h3 id="sdklocalcommandoutputmessage">
  `SDKLocalCommandOutputMessage`
</h3>

Saída de um comando slash local (por exemplo, `/voice` ou `/usage`). Exibido como texto estilo assistente na transcrição.

```typescript theme={null}
type SDKLocalCommandOutputMessage = {
  type: "system";
  subtype: "local_command_output";
  content: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkcommandschangedmessage">
  `SDKCommandsChangedMessage`
</h3>

Emitido quando o conjunto de comandos disponíveis muda durante a sessão, como quando skills são descobertos conforme o agente entra em um subdiretório. O array `commands` é a lista completa atualizada, então substitua qualquer lista de comandos em cache por este payload. Chamar `supportedCommands()` novamente não é equivalente: esse método retorna o snapshot capturado na inicialização e não reflete mudanças durante a sessão.

```typescript theme={null}
type SDKCommandsChangedMessage = {
  type: "system";
  subtype: "commands_changed";
  commands: SlashCommand[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkpromptsuggestionmessage">
  `SDKPromptSuggestionMessage`
</h3>

Emitido após cada turno quando `promptSuggestions` está ativado. Contém um prompt de usuário previsto.

```typescript theme={null}
type SDKPromptSuggestionMessage = {
  type: "prompt_suggestion";
  suggestion: string;
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkconversationresetmessage">
  `SDKConversationResetMessage`
</h3>

Emitido quando a conversa da sessão é substituída sem encerrar a sessão, como após `/clear`, na saída do modo de plano, ou quando uma nova conversa inicia. Monte uma transcrição vazia sob `new_conversation_id` e descarte qualquer título de sessão em cache.

```typescript theme={null}
type SDKConversationResetMessage = {
  type: "conversation_reset";
  new_conversation_id: UUID;
  uuid: UUID;
  session_id: string;
};
```

As tipagens publicadas do SDK declaram `SDKConversationResetMessage` no Claude Code v2.1.203 e posterior. Antes de v2.1.203, `SDKMessage` referenciava o tipo sem declará-lo, então o estreitamento em `type === "conversation_reset"` falhou ao verificar o tipo quando `skipLibCheck` estava desativado.

<h3 id="aborterror">
  `AbortError`
</h3>

Classe de erro personalizada para operações de abort.

```typescript theme={null}
class AbortError extends Error {}
```

<h2 id="sandbox-configuration">
  Configuração de Sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Configuração para comportamento de sandbox. Use isso para ativar sandboxing de comando e configurar restrições de rede programaticamente.

```typescript theme={null}
type SandboxSettings = {
  enabled?: boolean;
  failIfUnavailable?: boolean;
  autoAllowBashIfSandboxed?: boolean;
  excludedCommands?: string[];
  allowUnsandboxedCommands?: boolean;
  network?: SandboxNetworkConfig;
  filesystem?: SandboxFilesystemConfig;
  ignoreViolations?: Record<string, string[]>;
  enableWeakerNestedSandbox?: boolean;
  ripgrep?: { command: string; args?: string[] };
};
```

| Propriedade                 | Tipo                                                  | Padrão      | Descrição                                                                                                                                                                                                                                             |
| :-------------------------- | :---------------------------------------------------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`     | Ativar modo sandbox para execução de comando                                                                                                                                                                                                          |
| `failIfUnavailable`         | `boolean`                                             | `true`      | Parar na inicialização se `enabled` é `true` mas o sandbox não consegue iniciar. Defina `false` para voltar para execução sem sandbox com um aviso em stderr                                                                                          |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | Auto-aprovar comandos bash quando sandbox está ativado                                                                                                                                                                                                |
| `excludedCommands`          | `string[]`                                            | `[]`        | Comandos que sempre contornam restrições de sandbox (por exemplo, `['docker']`). Esses executam sem sandbox automaticamente sem envolvimento do modelo                                                                                                |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | Permitir que o modelo solicite executar comandos fora do sandbox. Quando `true`, o modelo pode definir `dangerouslyDisableSandbox` na entrada da ferramenta, que volta para o [sistema de permissões](#permissions-fallback-for-unsandboxed-commands) |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | Configuração de sandbox específica de rede                                                                                                                                                                                                            |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | Configuração de sandbox específica do sistema de arquivos para restrições de leitura/escrita                                                                                                                                                          |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | Mapa de categorias de violação para padrões a ignorar (por exemplo, `{ file: ['/tmp/*'], network: ['localhost'] }`)                                                                                                                                   |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | Ativar um sandbox aninhado mais fraco para compatibilidade                                                                                                                                                                                            |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | Configuração de binário ripgrep personalizado para ambientes sandbox                                                                                                                                                                                  |

<Note>
  O sandbox depende do suporte de plataforma e, no Linux, ferramentas como `bubblewrap` e `socat`. Quando `enabled` é `true` e o sandbox não consegue iniciar, `query()` relata uma mensagem `result` com `subtype: "error_during_execution"` e o motivo em `errors`. Para uma única chamada de mensagem `query()`, o SDK lança após gerar esse resultado de erro, então envolva o loop em um bloco try para continuar além dele. Veja [Lidar com o resultado](/docs/pt/agent-sdk/agent-loop#handle-the-result) para o contrato de erro.

  Para executar sem sandbox, defina `failIfUnavailable: false`.
</Note>

<h4 id="example-usage">
  Exemplo de uso
</h4>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

try {
  for await (const message of query({
    prompt: "Build and test my project",
    options: {
      sandbox: {
        enabled: true,
        autoAllowBashIfSandboxed: true,
        network: {
          allowLocalBinding: true
        }
      }
    }
  })) {
    if ("result" in message) console.log(message.result);
  }
} catch (error) {
  // Uma query() de uma única vez lança após gerar um resultado de erro,
  // como quando o sandbox não consegue iniciar (failIfUnavailable padrão é true).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Segurança de socket Unix:** A opção `allowUnixSockets` pode conceder acesso a serviços de sistema poderosos. Por exemplo, permitir `/var/run/docker.sock` efetivamente concede acesso completo ao sistema host através da API Docker, contornando isolamento de sandbox. Apenas permita sockets Unix que são estritamente necessários e entenda as implicações de segurança de cada um.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configuração específica de rede para modo sandbox. Essas configurações se aplicam a comandos Bash sandboxed quando `enabled` é `true` na [`SandboxSettings`](#sandboxsettings) pai. Elas não restringem a ferramenta WebFetch, que usa [regras de permissão](/docs/pt/permissions#webfetch) em vez disso.

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Propriedade               | Tipo       | Padrão      | Descrição                                                                                                                                                                                                                                                                                                            |
| :------------------------ | :--------- | :---------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | Nomes de domínio que processos sandboxed podem acessar                                                                                                                                                                                                                                                               |
| `deniedDomains`           | `string[]` | `[]`        | Nomes de domínio que processos sandboxed não podem acessar. Tem precedência sobre `allowedDomains`                                                                                                                                                                                                                   |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | Apenas configurações gerenciadas. Quando definido em [configurações gerenciadas](/docs/pt/permissions#managed-settings), apenas entradas `allowedDomains` de configurações gerenciadas são honradas e entradas de configurações de usuário, projeto ou local são ignoradas. Não tem efeito quando definido via opções SDK |
| `allowLocalBinding`       | `boolean`  | `false`     | Permitir que processos se vinculem a portas locais (por exemplo, para servidores dev)                                                                                                                                                                                                                                |
| `allowUnixSockets`        | `string[]` | `[]`        | Caminhos de socket Unix que processos podem acessar (por exemplo, socket Docker)                                                                                                                                                                                                                                     |
| `allowAllUnixSockets`     | `boolean`  | `false`     | Permitir acesso a todos os sockets Unix                                                                                                                                                                                                                                                                              |
| `httpProxyPort`           | `number`   | `undefined` | Porta de proxy HTTP para requisições de rede                                                                                                                                                                                                                                                                         |
| `socksProxyPort`          | `number`   | `undefined` | Porta de proxy SOCKS para requisições de rede                                                                                                                                                                                                                                                                        |

<Note>
  O proxy de sandbox integrado impõe `allowedDomains` com base no nome de host solicitado e não encerra ou inspeciona tráfego TLS, portanto técnicas como [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) podem potencialmente contorná-lo. Veja [Limitações de segurança de sandboxing](/docs/pt/sandboxing#security-limitations) para detalhes e [Implantação segura](/docs/pt/agent-sdk/secure-deployment#traffic-forwarding) para configurar um proxy que encerra TLS.
</Note>

<h3 id="sandboxfilesystemconfig">
  `SandboxFilesystemConfig`
</h3>

Configuração específica do sistema de arquivos para modo sandbox.

```typescript theme={null}
type SandboxFilesystemConfig = {
  allowWrite?: string[];
  denyWrite?: string[];
  denyRead?: string[];
};
```

| Propriedade  | Tipo       | Padrão | Descrição                                                     |
| :----------- | :--------- | :----- | :------------------------------------------------------------ |
| `allowWrite` | `string[]` | `[]`   | Padrões de caminho de arquivo para permitir acesso de escrita |
| `denyWrite`  | `string[]` | `[]`   | Padrões de caminho de arquivo para negar acesso de escrita    |
| `denyRead`   | `string[]` | `[]`   | Padrões de caminho de arquivo para negar acesso de leitura    |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback de Permissões para Comandos Sem Sandbox
</h3>

Quando `allowUnsandboxedCommands` está ativado, o modelo pode solicitar executar comandos fora do sandbox definindo `dangerouslyDisableSandbox: true` na entrada da ferramenta. Essas solicitações voltam para o sistema de permissões existente, significando que seu handler `canUseTool` é invocado, permitindo que você implemente lógica de autorização personalizada. No exemplo abaixo, `isCommandAuthorized` representa uma verificação de autorização que você define.

<Note>
  **`excludedCommands` vs `allowUnsandboxedCommands`:**

  * `excludedCommands`: Uma lista estática de comandos que sempre contornam o sandbox automaticamente (por exemplo, `['docker']`). O modelo não tem controle sobre isso.
  * `allowUnsandboxedCommands`: Permite que o modelo decida em tempo de execução se solicita execução sem sandbox definindo `dangerouslyDisableSandbox: true` na entrada da ferramenta.
</Note>

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // Modelo pode solicitar execução sem sandbox
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Verificar se o modelo está solicitando bypass do sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // O modelo está solicitando executar este comando fora do sandbox
        console.log(`Unsandboxed command requested: ${input.command}`);

        if (isCommandAuthorized(input.command)) {
          return { behavior: "allow" as const, updatedInput: input };
        }
        return {
          behavior: "deny" as const,
          message: "Command not authorized for unsandboxed execution"
        };
      }
      return { behavior: "allow" as const, updatedInput: input };
    }
  }
})) {
  if ("result" in message) console.log(message.result);
}
```

Este padrão permite que você:

* **Auditar solicitações do modelo:** Registre quando o modelo solicita execução sem sandbox
* **Implementar listas de permissão:** Apenas permitir comandos específicos para executar sem sandbox
* **Adicionar fluxos de aprovação:** Exigir autorização explícita para operações privilegiadas

<Warning>
  Comandos executando com `dangerouslyDisableSandbox: true` têm acesso completo ao sistema. Garanta que seu handler `canUseTool` valide essas solicitações cuidadosamente.

  Se `permissionMode` está definido como `bypassPermissions` e `allowUnsandboxedCommands` está ativado, o modelo pode autonomamente executar comandos fora do sandbox sem quaisquer prompts de aprovação (uma [`ask` rule](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) explícita ainda força uma). Esta combinação efetivamente permite que o modelo escape do isolamento de sandbox silenciosamente.
</Warning>

<h2 id="see-also">
  Veja também
</h2>

* [Visão geral do SDK](/docs/pt/agent-sdk/overview) - Conceitos gerais do SDK
* [Referência do SDK Python](/docs/pt/agent-sdk/python) - Documentação do SDK Python
* [Referência da CLI](/docs/pt/cli-reference) - Interface de linha de comando
* [Fluxos de trabalho comuns](/docs/pt/common-workflows) - Guias passo a passo
