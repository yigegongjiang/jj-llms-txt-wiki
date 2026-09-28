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
  O SDK agrupa um binário nativo do Claude Code para sua plataforma como uma dependência opcional, como `@anthropic-ai/claude-agent-sdk-darwin-arm64`. A maioria das instalações não precisa de uma instalação separada do Claude Code. A versão do SDK acompanha a versão do Claude Code agrupada. SDK v0.3.191 agrupa Claude Code v2.1.191, portanto um recurso nesta página que requer uma versão do Claude Code precisa da versão do SDK com o mesmo número de patch ou posterior. Se seu gerenciador de pacotes pular dependências opcionais, o SDK lança `Native CLI binary for <platform>-<arch> not found`; defina [`pathToClaudeCodeExecutable`](#options) para um binário `claude` instalado separadamente.

  Se seu gerenciador de pacotes não aplicar o campo `libc` do npm, como o Yarn 1.x não faz, você obtém os pacotes de plataforma glibc e musl no Linux, aproximadamente dobrando o tamanho da instalação. No Agent SDK v0.2.141 ou posterior, o SDK ainda inicia a variante correta. Para recuperar o espaço em uma imagem de contêiner, delete o pacote de plataforma que não corresponde ao libc onde seu aplicativo é executado; para um tempo de execução glibc em x64, isso é `rm -rf node_modules/@anthropic-ai/claude-agent-sdk-linux-x64-musl`. Em uma máquina de desenvolvimento, a exclusão é temporária, pois o Yarn reinstala o pacote na próxima alteração de dependência.
</Note>

<h3 id="compile-to-a-single-executable">
  Compilar para um executável único
</h3>

Quando você compila sua aplicação em um executável de arquivo único com `bun build --compile`, o SDK não consegue resolver o binário CLI agrupado em tempo de execução. `require.resolve` não funciona dentro do sistema de arquivos virtual `$bunfs` do executável compilado, então o SDK lança `Native CLI binary for <platform>-<arch> not found`.

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
  extras?: { annotations?: ToolAnnotations; searchHint?: string; alwaysLoad?: boolean }
): SdkMcpToolDefinition<Schema>;
```

<h4 id="parameters-3">
  Parâmetros
</h4>

| Parâmetro     | Tipo                                                                                                   | Descrição                                                                                                                                                                                                                                                                                                                                 |
| :------------ | :----------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`                                                                                               | O nome da ferramenta                                                                                                                                                                                                                                                                                                                      |
| `description` | `string`                                                                                               | Uma descrição do que a ferramenta faz                                                                                                                                                                                                                                                                                                     |
| `inputSchema` | `Schema extends AnyZodRawShape`                                                                        | Schema Zod definindo os parâmetros de entrada da ferramenta (suporta Zod 3 e Zod 4)                                                                                                                                                                                                                                                       |
| `handler`     | `(args, extra) => Promise<`[`CallToolResult`](#calltoolresult)`>`                                      | Função assíncrona que executa a lógica da ferramenta                                                                                                                                                                                                                                                                                      |
| `extras`      | `{ annotations?: `[`ToolAnnotations`](#toolannotations)`; searchHint?: string; alwaysLoad?: boolean }` | Extras opcionais. `annotations` fornece dicas comportamentais MCP aos clientes. `searchHint` é uma frase de capacidade de uma linha mostrada na lista de ferramentas adiadas quando [tool search](/docs/pt/agent-sdk/tool-search) está ativo. `alwaysLoad: true` mantém o schema completo desta ferramenta no prompt inicial em vez de adiá-lo |

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
  instructions?: string;
  tools?: Array<SdkMcpToolDefinition<any>>;
  alwaysLoad?: boolean;
  timeout?: number;
}): McpSdkServerConfigWithInstance;
```

<h4 id="parameters-4">
  Parâmetros
</h4>

| Parâmetro              | Tipo                          | Descrição                                                                                                                                                                                                                                                                                     |
| :--------------------- | :---------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.name`         | `string`                      | O nome do servidor MCP                                                                                                                                                                                                                                                                        |
| `options.version`      | `string`                      | String de versão opcional                                                                                                                                                                                                                                                                     |
| `options.instructions` | `string`                      | Instruções opcionais do servidor, retornadas de `initialize` e apresentadas ao modelo como um bloco de instruções MCP                                                                                                                                                                         |
| `options.tools`        | `Array<SdkMcpToolDefinition>` | Array de definições de ferramentas criadas com [`tool()`](#tool)                                                                                                                                                                                                                              |
| `options.alwaysLoad`   | `boolean`                     | Quando `true`, cada ferramenta deste servidor permanece no prompt inicial e nunca é adiada atrás de [tool search](/docs/pt/agent-sdk/tool-search). Combina com `alwaysLoad` por ferramenta em [`tool()`](#tool)                                                                                    |
| `options.timeout`      | `number`                      | Timeout em milissegundos para as chamadas de ferramenta deste servidor. Claude Code o aplica a este servidor no lugar de [`MCP_TOOL_TIMEOUT`](/docs/pt/env-vars). Passe um número inteiro de pelo menos 1000. Claude Code ignora outros valores. Requer TypeScript Agent SDK v0.3.248 ou posterior |

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

| Propriedade          | Tipo                    | Descrição                                                                                                                                                                                                                                                                                      |
| :------------------- | :---------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `type`               | `"user" \| "assistant"` | Papel da mensagem                                                                                                                                                                                                                                                                              |
| `uuid`               | `string`                | Identificador único de mensagem                                                                                                                                                                                                                                                                |
| `session_id`         | `string`                | Sessão a que esta mensagem pertence                                                                                                                                                                                                                                                            |
| `message`            | `unknown`               | Payload de mensagem bruta da transcrição                                                                                                                                                                                                                                                       |
| `parent_tool_use_id` | `string \| null`        | Para mensagens de subagente, o `tool_use_id` da chamada de ferramenta `Agent` ou `Skill` geradora. `null` para mensagens de sessão principal e sessões mais antigas                                                                                                                            |
| `parent_agent_id`    | `string \| null`        | Para mensagens de um [subagente aninhado](/docs/pt/sub-agents#let-subagents-spawn-their-own-subagents), o `agentId` do subagente que o gerou. `null` para mensagens de sessão principal, mensagens de subagentes de nível superior e sessões mais antigas. Requer Claude Code v2.1.202 ou posterior |

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
  Esta função é alfa e sua API pode mudar antes da estabilização.
</Note>

O snapshot difere do que uma sessão `query()` ao vivo aplica:

* **`policyHelper`**: `resolveSettings()` lê fontes MDM, incluindo plist do macOS e Windows HKLM/HKCU, mas não executa o subprocesso `policyHelper` configurado pelo administrador.
* **Configurações gerenciadas pelo servidor**: `resolveSettings()` não busca [configurações gerenciadas pelo servidor](/docs/pt/server-managed-settings#fetch-and-caching-behavior). Passe-as como `options.serverManagedSettings` para incluí-las.
* **`defaultMode`**: o snapshot retorna `permissions.defaultMode` como está de cada camada, então pode incluir os valores `'auto'` e `'bypassPermissions'` de configurações de projeto e local, que [uma sessão ao vivo ignora](/docs/pt/permission-modes#which-mode-a-session-starts-in).

```typescript theme={null}
function resolveSettings(
  options?: ResolveSettingsOptions
): Promise<ResolvedSettings>;
```

<h4 id="parameters-10">
  Parâmetros
</h4>

`resolveSettings()` aceita um único objeto de opções. Todos os campos são opcionais.

| Parâmetro                       | Tipo                                  | Padrão          | Descrição                                                                                                                                                                                                                                                                                                                                          |
| :------------------------------ | :------------------------------------ | :-------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `options.cwd`                   | `string`                              | `process.cwd()` | Diretório para resolver configurações de projeto e local relativas a                                                                                                                                                                                                                                                                               |
| `options.settingSources`        | [`SettingSource`](#settingsource)`[]` | Todas as fontes | Quais fontes do sistema de arquivos carregar. Passe `[]` para pular configurações de usuário, projeto e local. [Política gerenciada por endpoint](/docs/pt/managed-settings#delivery-mechanisms) carrega em todos os casos. `resolveSettings()` inclui configurações gerenciadas pelo servidor apenas quando você passa `options.serverManagedSettings` |
| `options.managedSettings`       | `Settings`                            | `undefined`     | Configurações de nível de política fornecidas pelo host de incorporação. Segue as mesmas regras que [`managedSettings` em `Options`](#options), exceto que `resolveSettings()` não executa um [`policyHelper`](/docs/pt/settings-reference#policyhelper) configurado, então o snapshot pode incluir configurações que uma sessão ao vivo descarta       |
| `options.serverManagedSettings` | `Settings`                            | `undefined`     | Payload de configurações gerenciadas pelo servidor de `/api/claude_code/settings`. Chaves não restritivas passam sem filtro                                                                                                                                                                                                                        |

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

O exemplo abaixo resolve configurações para um diretório de projeto e imprime a fonte que controla o período de limpeza. Em uma máquina onde nenhum arquivo de configurações define `cleanupPeriodDays`, ambas as linhas impressas mostram `undefined` para o valor, que é a saída esperada em vez de um erro.

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

| Propriedade                       | Tipo                                                                                                                                                                                                           | Padrão                                         | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :-------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `abortController`                 | `AbortController`                                                                                                                                                                                              | `new AbortController()`                        | Controlador para cancelar operações                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `additionalDirectories`           | `string[]`                                                                                                                                                                                                     | `[]`                                           | Diretórios adicionais que Claude pode acessar. O SDK passa cada entrada para Claude Code como `--add-dir`, então com a configuração `project` o Claude Code também [carrega as skills, comandos e subagentes do diretório](/docs/pt/permissions#additional-directories-grant-file-access-not-configuration)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `agent`                           | `string`                                                                                                                                                                                                       | `undefined`                                    | Nome do agente para a thread principal. O agente deve ser definido na opção `agents` ou em configurações                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `agents`                          | `Record<string, [`AgentDefinition`](#agentdefinition)>`                                                                                                                                                        | `undefined`                                    | Defina subagentes programaticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `agentProgressSummaries`          | `boolean`                                                                                                                                                                                                      | `false`                                        | Quando `true`, gera resumos de progresso de uma linha para subagentes e os encaminha em eventos [`task_progress`](#sdktaskprogressmessage) através do campo `summary`. Aplica-se a subagentes em primeiro plano e em segundo plano                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `allowDangerouslySkipPermissions` | `boolean`                                                                                                                                                                                                      | `false`                                        | Ativar bypass de permissões. Obrigatório ao usar `permissionMode: 'bypassPermissions'`, na inicialização ou depois através de `setPermissionMode()`. Veja [plan mode](/docs/pt/agent-sdk/permissions#plan-mode-plan) para como interage com `permissionMode: 'plan'`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `allowedTools`                    | `string[]`                                                                                                                                                                                                     | `[]`                                           | Ferramentas para auto-aprovar sem solicitar. Isso não restringe Claude apenas a essas ferramentas. Se você nomear uma das [ferramentas de rastreamento de tarefas](/docs/pt/agent-sdk/todo-tracking#model-availability) aqui, Claude Code também opta a sessão. Outras ferramentas não listadas caem em `permissionMode` e `canUseTool`. Use `disallowedTools` para bloquear ferramentas. Veja [Permissões](/docs/pt/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `betas`                           | [`SdkBeta`](#sdkbeta)`[]`                                                                                                                                                                                      | `[]`                                           | Ativar recursos beta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `canUseTool`                      | [`CanUseTool`](#canusetool)                                                                                                                                                                                    | `undefined`                                    | Função de permissão personalizada, invocada apenas quando o [fluxo de permissão](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) cai em um prompt. Não invocada para chamadas auto-aprovadas por `allowedTools`, regras de permissão, ou `permissionMode`. Uma regra de permissão não pré-aprova as [ações que nenhum modo auto-aprova](/docs/pt/permission-modes#actions-no-mode-auto-approves). Veja [`CanUseTool`](#canusetool) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `continue`                        | `boolean`                                                                                                                                                                                                      | `false`                                        | Continuar a conversa mais recente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `cwd`                             | `string`                                                                                                                                                                                                       | `process.cwd()`                                | Diretório de trabalho atual                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `debug`                           | `boolean`                                                                                                                                                                                                      | `false`                                        | Ativar modo de depuração para o processo Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `debugFile`                       | `string`                                                                                                                                                                                                       | `undefined`                                    | Escrever logs de depuração em um caminho de arquivo específico. Ativa implicitamente o modo de depuração                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `disallowedTools`                 | `string[]`                                                                                                                                                                                                     | `[]`                                           | Ferramentas para negar. Um nome simples como `"Bash"` remove a ferramenta do contexto do Claude. Uma regra com escopo como `"Bash(rm *)"` deixa a ferramenta disponível e nega chamadas correspondentes em todos os modos de permissão, incluindo `bypassPermissions`, para o comando [conforme escrito](/docs/pt/permissions#bash-rule-limits). Veja [Permissões](/docs/pt/agent-sdk/permissions#allow-and-deny-rules)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `effort`                          | `'low' \| 'medium' \| 'high' \| 'xhigh' \| 'max'`                                                                                                                                                              | `undefined`                                    | Controla quanto esforço Claude coloca em sua resposta. Funciona com pensamento adaptativo para guiar a profundidade do pensamento. Veja [ajustar o nível de esforço](/docs/pt/model-config#adjust-effort-level)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `enableFileCheckpointing`         | `boolean`                                                                                                                                                                                                      | `false`                                        | Ativar rastreamento de mudanças de arquivo para retrocesso. Veja [File checkpointing](/docs/pt/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `env`                             | `Record<string, string \| undefined>`                                                                                                                                                                          | `process.env`                                  | Variáveis de ambiente. Quando definido, isso substitui o ambiente do subprocesso em vez de mesclar com `process.env`, então passe `{ ...process.env, YOUR_VAR: 'value' }` para manter variáveis herdadas como `PATH`. Veja [Lidar com respostas de API lentas ou travadas](#handle-slow-or-stalled-api-responses) para um exemplo deste padrão, e [Variáveis de ambiente](/docs/pt/env-vars) para variáveis que a CLI subjacente lê. Defina `CLAUDE_AGENT_SDK_CLIENT_APP` para identificar sua aplicação no cabeçalho User-Agent                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `executable`                      | `'bun' \| 'deno' \| 'node'`                                                                                                                                                                                    | Auto-detectado                                 | Runtime JavaScript a usar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `executableArgs`                  | `string[]`                                                                                                                                                                                                     | `[]`                                           | Argumentos a passar para o executável                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `extraArgs`                       | `Record<string, string \| null>`                                                                                                                                                                               | `{}`                                           | Argumentos adicionais                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `fallbackModel`                   | `string`                                                                                                                                                                                                       | `undefined`                                    | Modelo a usar se o primário falhar. Aceita uma lista separada por vírgula. Para a ordem e o limite, veja [Cadeias de modelo de fallback](/docs/pt/model-config#fallback-model-chains). Para orientação, veja [Escolher um modelo](/docs/pt/agent-sdk/configuration#choose-a-model)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `forkSession`                     | `boolean`                                                                                                                                                                                                      | `false`                                        | Ao retomar com `resume`, bifurcar para um novo ID de sessão em vez de continuar a sessão original                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `forwardSubagentText`             | `boolean`                                                                                                                                                                                                      | `false`                                        | Encaminhar blocos de texto e pensamento de subagentes como mensagens de assistente e usuário com `parent_tool_use_id` definido, para que os consumidores possam renderizar uma transcrição aninhada. Sem esta opção, Claude Code emite blocos `tool_use` e `tool_result` de subagentes mas não texto ou pensamento. Mensagens de subagentes em cada profundidade de aninhamento são encaminhadas no Claude Code v2.1.219 e posterior; antes de v2.1.219, apenas mensagens de subagentes de profundidade-1 apareciam. Mensagens de subagentes que uma skill bifurcada gera, e de skills bifurcadas aninhadas, requerem v2.1.275 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `hooks`                           | `Partial<Record<`[`HookEvent`](#hookevent)`, `[`HookCallbackMatcher`](#hookcallbackmatcher)`[]>>`                                                                                                              | `{}`                                           | Callbacks de hook para eventos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `includeHookEvents`               | `boolean`                                                                                                                                                                                                      | `false`                                        | Incluir eventos de ciclo de vida de hook no fluxo de mensagens como [`SDKHookStartedMessage`](#sdkhookstartedmessage), [`SDKHookProgressMessage`](#sdkhookprogressmessage), e [`SDKHookResponseMessage`](#sdkhookresponsemessage). Eventos de ciclo de vida para hooks `SessionStart` e `Setup` são sempre incluídos e não precisam desta opção. Alguns eventos de hook, como `Notification`, `SessionEnd`, `PreCompact`, e `PostCompact`, nunca produzem um `SDKHookStartedMessage`, mesmo com esta opção. Para esses eventos, Claude Code ainda emite um `SDKHookProgressMessage` enquanto um hook de comando que é executado por mais de um segundo produz saída, e emite um `SDKHookResponseMessage` apenas quando um hook [que é executado em segundo plano](/docs/pt/hooks#run-hooks-in-the-background) termina                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `includePartialMessages`          | `boolean`                                                                                                                                                                                                      | `false`                                        | Incluir eventos de mensagem parcial                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `loadTimeoutMs`                   | `number`                                                                                                                                                                                                       | `60000`                                        | *Alfa.* Timeout em milissegundos para cada chamada `sessionStore.load()` e `sessionStore.listSubkeys()` durante materialização de retomada. Se o adaptador não se resolver dentro desta janela, a consulta falha em vez de travar. Ignorado quando `sessionStore` não está definido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `managedSettings`                 | `Settings`                                                                                                                                                                                                     | `undefined`                                    | Configurações de nível de política que seu processo host fornece para a sessão gerada. Em máquinas com configurações gerenciadas implantadas por administrador, Claude Code ignora estas a menos que a fonte gerenciada de maior prioridade do administrador defina `parentSettingsBehavior: 'merge'`, e nunca as mescla enquanto um [`policyHelper`](/docs/pt/settings-reference#policyhelper) fornece configurações gerenciadas. Valores mesclados passam por um filtro apenas restritivo; [Restringir configurações pai](/docs/pt/claude-apps-gateway#restrict-parent-settings) cobre o que o filtro admite e os bloqueios `allowManaged*Only`. Um host que define [`CLAUDE_CODE_PROVIDER_MANAGED_BY_HOST`](/docs/pt/env-vars) tem três chaves lidas diretamente desta carga: sua [configuração de modelo](/docs/pt/model-config#restrict-model-selection) no Claude Code v2.1.222 ou posterior, [`modelPricing`](/docs/pt/settings-reference#modelpricing) quando nenhuma fonte gerenciada a define no v2.1.246 ou posterior, e sua entrada `ENABLE_TOOL_SEARCH` env no v2.1.247 ou posterior                                                                                                                                                                                                    |
| `maxBudgetUsd`                    | `number`                                                                                                                                                                                                       | `undefined`                                    | Parar a consulta quando a estimativa de custo do lado do cliente atingir este valor em USD. Comparado com a mesma estimativa que `total_cost_usd`. Para ressalvas de precisão e comportamento de reset, veja [Rastrear custo e uso](/docs/pt/agent-sdk/cost-tracking)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `maxThinkingTokens`               | `number`                                                                                                                                                                                                       | `undefined`                                    | *Descontinuado:* Use `thinking` em vez disso. Tokens máximos para processo de pensamento                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `maxTurns`                        | `number`                                                                                                                                                                                                       | `undefined`                                    | Turnos agênticos máximos (round trips de uso de ferramenta)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `mcpServers`                      | `Record<string, [`McpServerConfig`](#mcpserverconfig)>`                                                                                                                                                        | `{}`                                           | Configurações de servidor MCP                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `model`                           | `string`                                                                                                                                                                                                       | Padrão da CLI                                  | Alias de modelo Claude ou nome de modelo completo. Veja [valores aceitos e IDs específicos do provedor](/docs/pt/model-config#available-models)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `onElicitation`                   | `(request: ElicitationRequest, options: { signal: AbortSignal }) => Promise<ElicitationResult>`                                                                                                                | `undefined`                                    | Callback para lidar com solicitações de elicitação MCP. Chamado quando um servidor MCP solicita entrada do usuário e nenhum hook a trata primeiro. Quando não fornecido, solicitações de elicitação não tratadas são recusadas automaticamente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `outputFormat`                    | `{ type: 'json_schema', schema: JSONSchema }`                                                                                                                                                                  | `undefined`                                    | Defina o formato de saída para resultados de agente. Veja [Structured outputs](/docs/pt/agent-sdk/structured-outputs) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `outputStyle`                     | `string`                                                                                                                                                                                                       | `undefined`                                    | Não é um campo `Options`. Defina `outputStyle` no objeto [`settings`](/docs/pt/settings) inline ou em um arquivo de configurações. Veja [Ativar um estilo de saída](/docs/pt/agent-sdk/modifying-system-prompts#activate-an-output-style)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `pathToClaudeCodeExecutable`      | `string`                                                                                                                                                                                                       | Auto-resolvido do binário nativo agrupado      | Caminho para executável Claude Code. Apenas necessário se dependências opcionais foram puladas durante a instalação ou sua plataforma não está no conjunto suportado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `permissionMode`                  | [`PermissionMode`](#permissionmode)                                                                                                                                                                            | `'default'`                                    | Modo de permissão para a sessão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `permissionPromptToolName`        | `string`                                                                                                                                                                                                       | `undefined`                                    | Nome da ferramenta MCP para prompts de permissão                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `permissionPrompts`               | `'host' \| 'none'`                                                                                                                                                                                             | `'host'`                                       | Quem responde aos prompts de permissão: `'host'` os encaminha para seu callback [`canUseTool`](#canusetool) ou a ferramenta `permissionPromptToolName`, e `'none'` [nega as chamadas que teriam solicitado](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated). Requer Claude Code v2.1.259 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `persistSession`                  | `boolean`                                                                                                                                                                                                      | `true`                                         | Quando `false`, desativa persistência de sessão em disco. Sessões não podem ser retomadas depois                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `planModeInstructions`            | `string`                                                                                                                                                                                                       | `undefined`                                    | Instruções de fluxo de trabalho personalizado para Plan Mode. Quando `permissionMode` é `'plan'`, esta string substitui o corpo de fluxo de trabalho de Plan Mode padrão. A CLI ainda o envolve com o preâmbulo de imposição somente leitura e o rodapé do protocolo ExitPlanMode                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `plugins`                         | [`SdkPluginConfig`](#sdkpluginconfig)`[]`                                                                                                                                                                      | `[]`                                           | Carregar plugins personalizados de caminhos locais. Veja [Plugins](/docs/pt/agent-sdk/plugins) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `projectConfigRoot`               | `string`                                                                                                                                                                                                       | `undefined`                                    | Caminho absoluto do checkout confiável que `cwd` é uma worktree de. Claude Code lê configurações de projeto, `.mcp.json`, e os comandos, agentes, skills, workflows, rotinas e estilos de saída do projeto `.claude/` deste diretório em vez de `cwd`, e define `CLAUDE_PROJECT_DIR` para ele. Hooks, scripts auxiliares como `apiKeyHelper`, e servidores MCP stdio começam com este diretório como seu diretório de trabalho. Arquivos `CLAUDE.md` e `.claude/rules/` ainda carregam de `cwd`. Requer Claude Code v2.1.275 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| `promptSuggestions`               | `boolean`                                                                                                                                                                                                      | `false`                                        | Ativar sugestões de prompt. Após um turno, Claude Code emite uma mensagem `prompt_suggestion` carregando um prompt de usuário previsto. Claude Code não gera sugestão para alguns turnos, como quando sua conta está próxima ou no limite de uso. Veja [Quando Claude Code pula sugestões](/docs/pt/interactive-mode#when-claude-code-skips-suggestions)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `resume`                          | `string`                                                                                                                                                                                                       | `undefined`                                    | ID de sessão a retomar                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `resumeDropsTurn`                 | `string`                                                                                                                                                                                                       | `undefined`                                    | Com `resumeSessionAt`: o UUID do prompt do turno que a retomada truncada pretende descartar. Claude Code recusa a retomada quando o intervalo descartado contém algo não atribuível a esse turno, como mensagens enfileiradas absorvidas ou notificações de tarefas, e nomeia o sinalizador `--resume-drops-turn` na mensagem de rejeição. Apenas o Agent SDK e retomadas em modo de impressão leem o par. Requer Claude Code v2.1.223 ou posterior                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `resumeSessionAt`                 | `string`                                                                                                                                                                                                       | `undefined`                                    | Retomar sessão em um UUID de mensagem específico                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `sandbox`                         | [`SandboxSettings`](#sandboxsettings)                                                                                                                                                                          | `undefined`                                    | Configurar comportamento de sandbox programaticamente. Veja [Sandbox settings](#sandboxsettings) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `sessionId`                       | `string`                                                                                                                                                                                                       | Auto-gerado                                    | Use um UUID específico para a sessão em vez de auto-gerar um                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `sessionStore`                    | [`SessionStore`](/docs/pt/agent-sdk/session-storage#the-sessionstore-interface)                                                                                                                                     | `undefined`                                    | Espelhar transcrições de sessão para um backend externo para que outro host possa retomá-las. Veja [Persist sessions to external storage](/docs/pt/agent-sdk/session-storage)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `sessionStoreFlush`               | `'batched' \| 'eager'`                                                                                                                                                                                         | `'batched'`                                    | *Alfa.* Modo de flush para `sessionStore`. Ignorado quando `sessionStore` não está definido                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `settings`                        | `string \| Settings`                                                                                                                                                                                           | `undefined`                                    | Objeto de [configurações](/docs/pt/settings) inline, caminho para um arquivo de configurações, ou uma string JSON inline. Popula a camada de configurações de flag na [ordem de precedência](/docs/pt/settings#settings-precedence). Altere em tempo de execução com [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `settingSources`                  | [`SettingSource`](#settingsource)`[]`                                                                                                                                                                          | Padrões da CLI (todas as fontes)               | Controle quais configurações do sistema de arquivos carregar. Passe `[]` para desativar configurações de usuário, projeto e local. [Política gerenciada por endpoint](/docs/pt/managed-settings#delivery-mechanisms) carrega independentemente; configurações gerenciadas pelo servidor são buscadas quando a sessão se autentica com uma credencial organizacional em uma [configuração elegível](/docs/pt/server-managed-settings#platform-availability). Veja [Use Claude Code features](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `skills`                          | `string[] \| 'all'`                                                                                                                                                                                            | `undefined`                                    | Skills disponíveis para a sessão. Passe `'all'` para ativar cada skill descoberta, ou uma lista de nomes de skills. Passe apenas nomes exatos. No Agent SDK v0.3.221 ou posterior, o SDK rejeita nomes malformados e em forma de wildcard com um erro antes de iniciar o processo Claude Code. Quando definido, o SDK adiciona a ferramenta Skill a `allowedTools` automaticamente. Se você também passar `tools`, inclua `'Skill'` nessa lista. Veja [Skills](/docs/pt/agent-sdk/skills)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `spawnClaudeCodeProcess`          | `(options: SpawnOptions) => SpawnedProcess`                                                                                                                                                                    | `undefined`                                    | Função personalizada para gerar o processo Claude Code. Use para executar Claude Code em VMs, contêineres ou ambientes remotos                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `stderr`                          | `(data: string) => void`                                                                                                                                                                                       | `undefined`                                    | Callback para saída stderr                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `strictMcpConfig`                 | `boolean`                                                                                                                                                                                                      | `false`                                        | Use apenas os servidores passados em `mcpServers` e ignore o projeto `.mcp.json`, configurações do usuário, servidores MCP fornecidos por plugin, e [conectores claude.ai](/docs/pt/mcp#use-mcp-servers-from-claude-ai)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
| `systemPrompt`                    | `string \| string[] \| { type: 'custom'; prompt: string \| string[]; snapshot?: boolean } \| { type: 'preset'; preset: 'claude_code'; append?: string; excludeDynamicSections?: boolean; snapshot?: boolean }` | `undefined` (prompt mínimo)                    | Configuração de prompt do sistema. Passe uma string para prompt personalizado, ou `{ type: 'preset', preset: 'claude_code' }` para usar o prompt do sistema do Claude Code. Passe um array de strings com a constante exportada `SYSTEM_PROMPT_DYNAMIC_BOUNDARY` entre as partes estática e por solicitação para [cachear a parte estática de um prompt personalizado](/docs/pt/agent-sdk/modifying-system-prompts#cache-the-static-part-of-a-custom-prompt). Ao usar a forma de objeto preset, adicione `append` para estendê-lo com instruções adicionais, e defina `excludeDynamicSections: true` para mover contexto por sessão para a primeira mensagem do usuário para [melhor reutilização de cache de prompt entre máquinas](/docs/pt/agent-sdk/modifying-system-prompts#improve-prompt-caching-across-users-and-machines). Defina `snapshot: false` para reconstruir o prompt em cada solicitação em vez de [reutilizar o prompt que a sessão registrou em sua primeira solicitação](/docs/pt/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session). Para definir `snapshot` em um prompt personalizado, passe a forma `{ type: 'custom', prompt }`. A forma `{ type: 'custom' }` e o campo `snapshot` requerem TypeScript Agent SDK v0.3.257 ou posterior |
| `taskBudget`                      | `{ total: number }`                                                                                                                                                                                            | `undefined`                                    | *Alfa.* Orçamento de tarefa do lado da API em tokens. Quando definido, o modelo é informado sobre seu orçamento de token restante para que possa controlar o uso de ferramentas e encerrar antes do limite                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `thinking`                        | [`ThinkingConfig`](#thinkingconfig)                                                                                                                                                                            | `{ type: 'adaptive' }` para modelos suportados | Controla o comportamento de pensamento/raciocínio do Claude. Veja [`ThinkingConfig`](#thinkingconfig) para opções                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `title`                           | `string`                                                                                                                                                                                                       | `undefined`                                    | Título de exibição para a sessão. Ao retomar via `resume` ou `continue`, o título persistido da sessão retomada tem precedência; use [`renameSession()`](#renamesession) para renomear uma sessão existente                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `toolAliases`                     | `Record<string, string>`                                                                                                                                                                                       | `undefined`                                    | Mapear nomes de ferramentas integradas para nomes de ferramentas MCP para que Claude chame sua implementação MCP em vez da integrada. Por exemplo, `{ Bash: 'mcp__workspace__bash' }`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `toolConfig`                      | [`ToolConfig`](#toolconfig)                                                                                                                                                                                    | `undefined`                                    | Configuração para comportamento de ferramenta integrada. Veja [`ToolConfig`](#toolconfig) para detalhes                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `tools`                           | `string[] \| { type: 'preset'; preset: 'claude_code' }`                                                                                                                                                        | `undefined`                                    | Configuração de ferramenta. Passe um array de nomes de ferramentas ou use o preset para obter as ferramentas padrão do Claude Code                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |

<h4 id="handle-slow-or-stalled-api-responses">
  Lidar com respostas de API lentas ou travadas
</h4>

O subprocesso da CLI lê várias variáveis de ambiente que controlam timeouts de API e detecção de travamento. Passe-as através da opção `env`:

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

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
* `CLAUDE_CODE_MAX_RETRIES`: máximo de tentativas de API. Padrão `10`, limitado a `15`. Cada tentativa obtém sua própria janela `API_TIMEOUT_MS`, então o tempo de parede no pior caso é aproximadamente `API_TIMEOUT_MS × (CLAUDE_CODE_MAX_RETRIES + 1)` mais backoff. Para execuções sem supervisão que precisam aguardar através de interrupções mais longas, defina [`CLAUDE_CODE_RETRY_WATCHDOG=1`](/docs/pt/errors#tune-retry-behavior): ele tenta erros de capacidade transitória indefinidamente e, no Claude Code v2.1.199 ou posterior, aumenta o padrão para outros erros transitórios para `300` e remove o limite nesta variável.
* `CLAUDE_ASYNC_AGENT_STALL_TIMEOUT_MS`: watchdog de travamento para subagentes. Enquanto o watchdog de stream está ativado, o padrão é `CLAUDE_STREAM_IDLE_TIMEOUT_MS` mais 5 minutos, que chega a `600000` a menos que você aumente essa variável. Com o watchdog de stream desativado, o padrão é `600000`. Antes de v2.1.257, o padrão era sempre `600000`.

  O temporizador redefine em cada evento de stream. Em caso de travamento, Claude Code aborta o subagente e relata o travamento ao pai. Para um subagente em segundo plano, também marca a tarefa como falhada e anexa qualquer resultado parcial.
* `CLAUDE_ENABLE_STREAM_WATCHDOG` com `CLAUDE_STREAM_IDLE_TIMEOUT_MS`: watchdog de stream que aborta a solicitação quando os cabeçalhos chegaram mas o corpo da resposta para de fazer stream. O watchdog está ativado por padrão para todos os provedores; defina `CLAUDE_ENABLE_STREAM_WATCHDOG=0` para desativá-lo. `CLAUDE_STREAM_IDLE_TIMEOUT_MS` padrão é `300000` e é fixado nesse mínimo. Após a anulação, [Tentativas automáticas](/docs/pt/errors#automatic-retries) cobre o que Claude Code faz, com base em quanto a resposta havia progredido.

  Enquanto o watchdog aguarda uma resposta que um gateway atrás de `ANTHROPIC_BASE_URL` mantém aberta com pings keep-alive, um host que define `includePartialMessages` continua recebendo eventos de `ping` [stream](#sdkpartialassistantmessage), então leia esses frames como vivacidade em vez de expirar a sessão no silêncio. Antes de v2.1.257, os frames paravam 5 minutos após o último evento de stream real.

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
  applyFlagSettings(settings: {
    [K in keyof Settings]?: K extends 'effortLevel'
      ? 'low' | 'medium' | 'high' | 'xhigh' | 'max' | null
      : Settings[K] | null;
  }): Promise<void>;
  updateSettings(
    source: 'localSettings' | 'userSettings',
    settings: Record<string, unknown>,
  ): Promise<void>;
  initializationResult(): Promise<SDKControlInitializeResponse>;
  reinitialize(): Promise<SDKControlInitializeResponse>;
  supportedCommands(): Promise<SlashCommand[]>;
  supportedModels(): Promise<ModelInfo[]>;
  supportedAgents(): Promise<AgentInfo[]>;
  mcpServerStatus(): Promise<McpServerStatus[]>;
  getContextUsage(opts?: {
    detail?: 'summary' | 'full';
  }): Promise<SDKControlGetContextUsageResponse>;
  readFile(
    path: string,
    options?: { maxBytes?: number; encoding?: 'utf-8' | 'base64' }
  ): Promise<SDKControlReadFileResponse | null>;
  reloadSkills(): Promise<SDKControlReloadSkillsResponse>;
  accountInfo(): Promise<AccountInfo>;
  reconnectMcpServer(serverName: string): Promise<void>;
  toggleMcpServer(serverName: string, enabled: boolean): Promise<void>;
  setMcpServers(servers: Record<string, McpServerConfig>): Promise<McpSetServersResult>;
  readMcpResource(serverName: string, uri: string): Promise<SDKControlMcpReadResourceResponse>;
  streamInput(stream: AsyncIterable<SDKUserMessage>): Promise<void>;
  stopTask(taskId: string): Promise<void>;
  close(): void;
}
```

<h4 id="methods">
  Métodos
</h4>

| Método                                 | Descrição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| :------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt()`                          | Interrompe a consulta. Apenas disponível em modo de entrada de transmissão. Quando a CLI anuncia a capacidade `interrupt_receipt_v1` em [`SDKSystemMessage.capabilities`](#sdksystemmessage), resolve com um [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) listando as mensagens que estavam pendentes quando a interrupção chegou. Resolve `undefined` em CLIs anteriores a v2.1.205                                                                                                                                  |
| `rewindFiles(userMessageId, options?)` | Restaura arquivos para seu estado na mensagem de usuário especificada. Passe `{ dryRun: true }` para visualizar mudanças. Requer `enableFileCheckpointing: true`. Veja [File checkpointing](/docs/pt/agent-sdk/file-checkpointing)                                                                                                                                                                                                                                                                                                          |
| `setPermissionMode()`                  | Altera o modo de permissão (apenas disponível em modo de entrada de transmissão)                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| `setModel()`                           | Altera o modelo (apenas disponível em modo de entrada de transmissão). Passar `undefined` ou a string `"default"` redefine para [o modelo padrão do Claude Code](/docs/pt/model-config)                                                                                                                                                                                                                                                                                                                                                     |
| `setMaxThinkingTokens()`               | *Descontinuado:* Use a opção `thinking` em vez disso. Altera os tokens de pensamento máximos. Passar `null` redefine o pensamento para o padrão da sessão: uma substituição no meio da sessão é limpa, e o pensamento permanece desativado para sessões que o têm desativado                                                                                                                                                                                                                                                           |
| `applyFlagSettings(settings)`          | Mescla configurações na camada de configurações de flag da sessão em tempo de execução (apenas disponível em modo de entrada de transmissão). Veja [`applyFlagSettings()`](#applyflagsettings)                                                                                                                                                                                                                                                                                                                                         |
| `updateSettings(source, settings)`     | Escreve uma chave permitida no arquivo de configurações local do projeto ou no arquivo de configurações do usuário, para que o valor persista para sessões posteriores. Veja [`updateSettings()`](#updatesettings). Requer TypeScript SDK v0.3.257 ou posterior, que agrupa Claude Code v2.1.257                                                                                                                                                                                                                                       |
| `initializationResult()`               | Retorna o resultado de inicialização completo incluindo comandos suportados, modelos, informações de conta e configuração de estilo de saída                                                                                                                                                                                                                                                                                                                                                                                           |
| `reinitialize()`                       | Re-envia a solicitação de controle `initialize` para a CLI em execução e retorna um resultado novo em vez do resultado de primeira conexão em cache. Use-o após uma lacuna de transporte, como reconectar a uma sessão após uma desconexão, para que solicitações de permissão pendentes alcancem seu callback `canUseTool` novamente. Torne o callback idempotente por ID de solicitação, porque uma solicitação cuja resposta foi perdida é despachada novamente. Requer Claude Code v2.1.195 ou posterior                           |
| `supportedCommands()`                  | Retorna comandos disponíveis. A partir do Agent SDK v0.3.216 a lista reflete mudanças de comando no meio da sessão; veja [`SDKCommandsChangedMessage`](#sdkcommandschangedmessage)                                                                                                                                                                                                                                                                                                                                                     |
| `supportedModels()`                    | Retorna modelos disponíveis com informações de exibição                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| `supportedAgents()`                    | Retorna subagentes disponíveis como [`AgentInfo`](#agentinfo)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `mcpServerStatus()`                    | Retorna status de servidores MCP conectados como [`McpServerStatus`](#mcpserverstatus)`[]`                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `getContextUsage(opts?)`               | Retorna um [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse) dividindo o uso da janela de contexto da sessão por categoria, skill e ferramenta. Com o `detail` padrão, é o mesmo dado que `/context` mostra em uma sessão interativa. A [opção `detail`](#sdkcontrolgetcontextusageresponse) requer Agent SDK v0.3.257 ou posterior                                                                                                                                                                            |
| `readFile(path, options?)`             | Lê um arquivo do sistema de arquivos da sessão. Claude Code resolve o caminho contra `cwd`; [O que `readFile()` pode ler](#what-readfile-can-read) lista os arquivos que ele serve. Passe `{ maxBytes }` para alterar o limite de leitura (padrão 1 MB, teto 10 MB) e `{ encoding: 'base64' }` para arquivos binários como imagens. Resolve com um [`SDKControlReadFileResponse`](#sdkcontrolreadfileresponse), ou `null` em negação de permissão, arquivo ausente, ou erro de transporte. Requer TypeScript SDK v0.2.121 ou posterior |
| `reloadSkills()`                       | Recarrega skills do disco, para que skills que você adiciona ou edita no meio da sessão fiquem disponíveis para a sessão em execução. Resolve com um [`SDKControlReloadSkillsResponse`](#sdkcontrolreloadskillsresponse) listando as skills disponíveis após o recarregamento. Requer Agent SDK v0.3.163 ou posterior                                                                                                                                                                                                                  |
| `accountInfo()`                        | Retorna informações de conta                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `reconnectMcpServer(serverName)`       | Reconectar um servidor MCP por nome. Se o nome também corresponder a uma entrada em um arquivo de configurações como `.mcp.json` ou `~/.claude.json`, Claude Code reconecta o servidor que você configurou através de [`mcpServers`](#options) ou `setMcpServers()`, não a entrada do arquivo de configurações. Essa ordem de resolução requer Claude Code v2.1.257 ou posterior                                                                                                                                                       |
| `toggleMcpServer(serverName, enabled)` | Ativar ou desativar um servidor MCP por nome, com a mesma resolução de nome que `reconnectMcpServer()`. Desativar desconecta o servidor                                                                                                                                                                                                                                                                                                                                                                                                |
| `setMcpServers(servers)`               | Substituir dinamicamente o conjunto de servidores MCP para esta sessão. Resolve com um [`McpSetServersResult`](#mcpsetserversresult) nomeando quais servidores foram adicionados e removidos, e quaisquer erros                                                                                                                                                                                                                                                                                                                        |
| `readMcpResource(serverName, uri)`     | *Alfa.* Lê um recurso MCP Apps `ui://` de um servidor MCP conectado para que sua aplicação possa renderizar um widget de ferramenta. Resolve com um [`SDKControlMcpReadResourceResponse`](#sdkcontrolmcpreadresourceresponse). Requer TypeScript Agent SDK v0.3.280 ou posterior                                                                                                                                                                                                                                                       |
| `streamInput(stream)`                  | Transmitir mensagens de entrada para a consulta para conversas multi-turno                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `stopTask(taskId)`                     | Parar uma tarefa de fundo em execução por ID                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `close()`                              | Fechar a consulta e encerrar o processo subjacente. Força o término da consulta e limpa todos os recursos                                                                                                                                                                                                                                                                                                                                                                                                                              |

<h4 id="applyflagsettings">
  `applyFlagSettings()`
</h4>

Altera [configurações](/docs/pt/settings) em uma sessão em execução sem reiniciar a consulta. Use-a quando uma configuração que não tem um setter dedicado precisa mudar no meio da sessão, como apertar `permissions` depois que o agente lê entrada não confiável. `setModel()` e `setPermissionMode()` são setters dedicados para essas duas chaves; `applyFlagSettings()` é a forma geral que aceita qualquer subconjunto das chaves de configurações, e passar `model` aqui se comporta igual a `setModel()`.

Apenas algumas chaves têm efeito no meio da sessão:

* **Aplicadas no próximo turno**: `effortLevel`, `ultracode`, `permissions`, `hooks`, `skillOverrides`, `fastMode`, `agent`. Mudar `agent` também aplica a substituição de modelo e hooks desse agente no próximo turno. Seu prompt do sistema se aplica no próximo turno, ou, em uma sessão que [reutiliza um prompt do sistema registrado](/docs/pt/agent-sdk/modifying-system-prompts#change-the-prompt-of-an-existing-session), uma vez que a sessão é compactada.
* **Aplicadas durante o turno atual**: `model`. Se você mudar `model` enquanto Claude está trabalhando em um turno, a resposta que Claude já está gerando termina no modelo antigo, e o resto do turno, começando com a próxima chamada que Claude Code faz para o modelo, usa o novo. Subagentes mantêm seu próprio modelo. Antes de v2.1.212, uma mudança no meio do turno aguardava o próximo turno.
* **Sem efeito no meio da sessão**: as opções de prompt do sistema. Estas são resolvidas uma vez na inicialização, então a sessão em execução mantém o valor original mesmo que a chamada tenha sucesso. Para alterá-los, inicie uma nova sessão.

`effortLevel` aceita um nome de [nível de esforço](/docs/pt/model-config#adjust-effort-level). Também aceita `"ultracode"`, que executa a sessão em esforço `xhigh` e ativa [ultracode](/docs/pt/workflows#let-claude-decide-with-ultracode). `applyFlagSettings()` declara `effortLevel` sem esse valor, então passe o equivalente `{ ultracode: true }` em TypeScript. O valor `ultracode` requer Claude Code v2.1.203 ou posterior e é aceito apenas por `applyFlagSettings()`, não pela chave `effortLevel` em um arquivo de configurações.

Os valores são escritos na camada de configurações de flag, mesclados sobre o que a opção `settings` inline de `query()` definiu na inicialização. Esta é a mesma camada que a [seção de precedência na página](#settings-precedence) chama de opções programáticas.

Chamadas sucessivas fazem shallow-merge de chaves de nível superior. Uma segunda chamada com `{ permissions: {...} }` substitui o objeto `permissions` inteiro da chamada anterior em vez de fazer deep-merge nele.

Para limpar uma chave que você definiu com `applyFlagSettings()`, passe `null` para essa chave. A maioria das chaves então volta a um valor que a opção `settings` de `query()` definiu na inicialização, depois a fontes de precedência mais baixa. Um `model` limpo redefine para [o modelo padrão do Claude Code](/docs/pt/model-config), mesmo quando um arquivo de configurações define `model`. Passar `undefined` não tem efeito porque a serialização JSON a descarta.

Três chaves além de `model` redefinem o estado da sessão em vez de voltar:

* `effortLevel: null` retorna a sessão ao nível de esforço padrão do modelo, não à opção `effort` de `query()` ou um `effortLevel` de um arquivo de configurações.
* `agent: null` executa a thread principal sem agente, começando com o próximo turno, em vez de restaurar a opção `agent` de `query()` ou um `agent` de um arquivo de configurações. Se o agente limpo tivesse aplicado seu próprio modelo, a sessão volta ao modelo que resolveu na inicialização.
* `ultracode: null` desativa ultracode, como `false` faz, em vez de restaurar um valor `ultracode` de um arquivo de configurações. A sessão mantém seu nível de esforço atual, então passe `effortLevel` na mesma chamada para alterá-lo.

Apenas disponível em modo de entrada de transmissão, a mesma restrição que `setModel()` e `setPermissionMode()`.

O exemplo abaixo muda o modelo ativo no meio da sessão, depois limpa a substituição para que o modelo volte ao [modelo padrão do Claude Code](/docs/pt/model-config).

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

const q = query({ prompt: messageStream });

// Substituir o modelo para o resto da sessão
await q.applyFlagSettings({ model: "claude-opus-4-6" });

// Depois: limpar a substituição; o modelo redefine para o modelo padrão do Claude Code
await q.applyFlagSettings({ model: null });
```

<Note>
  `applyFlagSettings()` é apenas TypeScript. O SDK Python não expõe um método equivalente.
</Note>

<h4 id="updatesettings">
  `updateSettings()`
</h4>

Escreve uma chave permitida em um arquivo de configurações em disco, para que o valor persista para sessões posteriores que carregam essa fonte. Cada fonte aceita uma chave, com um valor de string:

* **`"localSettings"`**: aceita `outputStyle` e mescla em um arquivo de configurações local do projeto, `.claude/settings.local.json`. O novo estilo entra em vigor na próxima solicitação da sessão.
* **`"userSettings"`**: aceita `effortLevel` e o salva como o [nível de esforço](/docs/pt/model-config#adjust-effort-level) padrão para o modelo atual da sessão, sob [`modelSettings`](/docs/pt/settings-reference#modelsettings) no arquivo de configurações do usuário. Passar `max` não escreve nada, porque `max` é apenas de sessão. A sessão em execução mantém seu nível de esforço atual de qualquer forma, então chame [`applyFlagSettings()`](#applyflagsettings) quando você também quiser alterar isso. Esta fonte requer TypeScript SDK v0.3.277 ou posterior, que agrupa Claude Code v2.1.277.

A chamada rejeita quando a solicitação carrega qualquer outra chave, quando a sessão é executada sobre um transporte remoto, e quando os [`settingSources`](#options) da sessão excluem a fonte que você nomeia. Deletar uma chave não é suportado.

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
  fast_mode_disabled_reason?: FastModeDisabledReason;
  hooks_applied?: boolean;
};
```

`hooks_applied` relata se Claude Code registrou os `hooks` que a solicitação `initialize` carregava. O SDK envia essa solicitação uma vez quando a sessão inicia e novamente em cada chamada [`reinitialize()`](#query-object). O campo requer Agent SDK v0.3.238 ou posterior.

Claude Code omite o campo quando a solicitação não carregava hooks. Quando a solicitação carregava hooks, o valor depende se a solicitação é a primeira inicialização da sessão e, para uma repetida, de como ela alcançou a sessão:

* `true`: Claude Code registrou os hooks. A primeira inicialização de uma sessão retorna esse valor. Uma inicialização repetida enviada sobre stdin da CLI também retorna `true`. Nesse caso os hooks na nova solicitação substituem os hooks registrados anteriormente.
* `false`: Claude Code ignorou os hooks. Uma inicialização repetida enviada para uma sessão remota retorna esse valor, então um segundo cliente que se junta a uma sessão não pode substituir os hooks que o primeiro cliente registrou.

Antes do Agent SDK v0.3.238, a resposta nunca carregava o campo, e Claude Code ignorava `hooks` em cada inicialização repetida.

A resposta sempre relata `fast_mode_state`, e quando algo bloqueia [fast mode](/docs/pt/fast-mode), `fast_mode_disabled_reason` carrega o código de razão junto com ele, para que você possa explicar o estado bloqueado em vez de re-derivar a disponibilidade. Ambos os comportamentos requerem Claude Code v2.1.219 ou posterior. Antes de v2.1.219, a resposta omitia `fast_mode_state` quando fast mode não estava disponível e nunca carregava uma razão. Para os códigos de razão e seus significados, veja [`fast_mode_disabled_reason`](#sdkresultmessage) na mensagem de resultado.

O wrapper de resposta de controle para um `initialize` bem-sucedido também carrega um array `pending_permission_requests`. O campo está no wrapper de resposta em si, não na carga `SDKControlInitializeResponse` acima. Cada entrada é uma mensagem `control_request` completa com a mesma forma `{ type: "control_request", request_id, request }` que a sessão transmite para solicitações de permissão durante a execução.

O array lista as solicitações de permissão que este processo Claude Code emitiu e ainda não resolveu. O SDK lê o array para você e despacha cada entrada para seu callback [`canUseTool`](#canusetool), o mesmo reenvio que [`reinitialize()`](#query-object) dispara após uma lacuna de transporte. Trate IDs de solicitação repetidos idempotentemente, porque uma entrada pode repetir uma solicitação que o callback já recebeu antes da conexão cair.

O array está sempre presente em uma resposta `initialize` bem-sucedida e está vazio quando este processo não tem nenhuma solicitação de permissão não resolvida. Requer Claude Code v2.1.268 ou posterior. Versões anteriores poderiam omitir o campo, então se você analisar o protocolo de fio você mesmo, trate um campo ausente como uma CLI mais antiga em vez de como prova de que nada está pendente.

<h3 id="sdkcontrolinterruptresponse">
  `SDKControlInterruptResponse`
</h3>

O recebimento de interrupção: o valor que [`interrupt()`](#query-object) resolve em uma CLI que anuncia a capacidade `interrupt_receipt_v1` em [`SDKSystemMessage.capabilities`](#sdksystemmessage). Requer Claude Code v2.1.205 ou posterior. CLIs anteriores respondem à interrupção com uma carga de sucesso vazia, então `interrupt()` resolve para `undefined`.

```typescript theme={null}
type SDKControlInterruptResponse = {
  still_queued: string[];
  cancelled?: string[];
};
```

`still_queued` lista os UUIDs das mensagens de usuário que estavam pendentes quando a interrupção chegou: mensagens ainda na fila, mais qualquer mensagem que Claude Code já havia tirado da fila para o próximo turno. Uma vez que o primeiro turno da sessão começou, Claude Code processa as mensagens listadas após a interrupção a menos que você as cancele primeiro, e pode mesclar várias em um turno. Se você interromper antes do primeiro turno começar, Claude Code aborta esse turno assim que ele inicia, e as mensagens listadas nesse turno não recebem resposta.

Use o recebimento para decidir se deve reenviar algo. Uma mensagem listada que você não cancela entra na conversa independentemente de receber uma resposta, então reenviá-la a entrega para Claude duas vezes.

Interprete a lista com estas ressalvas:

* Apenas mensagens que foram enfileiradas com um UUID aparecem. Um array vazio não significa que nada mais será executado.
* Apenas mensagens da thread principal estão listadas. Mensagens endereçadas a um subagente estão fora do escopo.
* A lista pode incluir UUIDs que seu cliente nunca enviou, como [acionadores de tarefa agendada](/docs/pt/scheduled-tasks). Ignore UUIDs que você não reconhece em vez de tratá-los como um erro.

Um cliente que dirige o protocolo de controle da CLI diretamente, em vez de através de `interrupt()`, pode definir `cancel_queued: true` na solicitação de controle `interrupt`. Claude Code v2.1.219 e posterior anuncia suporte com a capacidade `interrupt_cancel_queued_v1` em [`SDKSystemMessage.capabilities`](#sdksystemmessage); CLIs mais antigas ignoram o campo e deixam mensagens enfileiradas para executar como de costume. Tal interrupção também cancela cada mensagem que seria listada sob `still_queued`: o recebimento as lista sob `cancelled` em vez disso, `still_queued` está vazio, e nenhuma delas é executada.

A lista `cancelled` carrega as mesmas ressalvas que `still_queued`. O método `interrupt()` nunca envia `cancel_queued`, então recebimentos que ele resolve não carregam `cancelled`.

O recebimento é um snapshot tirado no momento em que a interrupção é processada, e em uma interrupção limpa chega antes do [`SDKResultMessage`](#sdkresultmessage) do turno interrompido. Leia o recebimento em vez de inspecionar a fila após esse resultado: o loop inicia o próximo turno enfileirado imediatamente, então a fila que você inspeciona após o resultado já mudou.

<h3 id="sdkcontrolgetcontextusageresponse">
  `SDKControlGetContextUsageResponse`
</h3>

Tipo de retorno de [`getContextUsage()`](#query-object). Com o `detail` padrão, este é o mesmo payload que Claude Code renderiza para o comando `/context` em uma sessão interativa, então junto com as contagens de token carrega campos de exibição como `color` e `gridRows` que Claude Code usa para desenhar a grade de uso `/context`.

O argumento `detail` opcional do método escolhe como Claude Code conta cada categoria. Com o padrão, `'full'`, Claude Code conta cada categoria com solicitações de API de contagem de token. Passe `{ detail: 'summary' }` para obter uma resposta do uso da última resposta e estimativas locais em vez disso. Nenhuma solicitação de contagem de token sai, e os números por categoria são aproximados. O argumento `detail` requer Agent SDK v0.3.257 ou posterior.

Quando você envia `/context` como um prompt em vez de chamar o método, Claude Code anexa uma carga [`SDKContextUsage`](#sdkcontextusage) ao campo `context_usage` da mensagem do assistente que entrega o resultado. Esse campo requer Agent SDK v0.3.232 ou posterior.

```typescript theme={null}
type SDKControlGetContextUsageResponse = {
  categories: {
    name: string;
    tokens: number;
    color: string;
    isDeferred?: boolean;
  }[];
  totalTokens: number;
  maxTokens: number;
  rawMaxTokens: number;
  percentage: number;
  gridRows: {
    color: string;
    isFilled: boolean;
    categoryName: string;
    tokens: number;
    percentage: number;
    squareFullness: number;
  }[][];
  model: string;
  memoryFiles: {
    path: string;
    type: string;
    tokens: number;
  }[];
  mcpTools: {
    name: string;
    serverName: string;
    tokens: number;
    isLoaded?: boolean;
  }[];
  deferredBuiltinTools?: {
    name: string;
    tokens: number;
    isLoaded: boolean;
  }[];
  systemTools?: {
    name: string;
    tokens: number;
  }[];
  systemPromptSections?: {
    name: string;
    tokens: number;
  }[];
  agents: {
    agentType: string;
    source: string;
    tokens: number;
  }[];
  slashCommands?: {
    totalCommands: number;
    includedCommands: number;
    tokens: number;
  };
  skills?: {
    totalSkills: number;
    includedSkills: number;
    tokens: number;
    skillFrontmatter: {
      name: string;
      source: string;
      tokens: number;
    }[];
  };
  autoCompactThreshold?: number;
  isAutoCompactEnabled: boolean;
  messageBreakdown?: {
    toolCallTokens: number;
    toolResultTokens: number;
    attachmentTokens: number;
    assistantMessageTokens: number;
    userMessageTokens: number;
    redirectedContextTokens: number;
    unattributedTokens: number;
    toolCallsByType: {
      name: string;
      callTokens: number;
      resultTokens: number;
    }[];
    attachmentsByType: {
      name: string;
      tokens: number;
    }[];
  };
  apiUsage: {
    input_tokens: number;
    output_tokens: number;
    cache_creation_input_tokens: number;
    cache_read_input_tokens: number;
  } | null;
};
```

Leia atribuição de token da coleção de campos:

* `categories` contém os totais por categoria.
* `mcpTools` e `agents` atribuem tokens a ferramentas MCP individuais e subagentes.
* `memoryFiles` lista cada arquivo de memória carregado com seu custo.
* `skills.skillFrontmatter` atribui os tokens da listagem de skills a cada skill incluída. As contagens por skill medem cada entrada de listagem de skill conforme Claude Code realmente a envia, que pode ser mais curta que o frontmatter completo da skill. Compare `skills.totalSkills` com `skills.includedSkills` para ver se cada skill descoberta fez parte da listagem.

`totalTokens` é o uso de contexto atual da sessão, e `maxTokens` é a janela contra a qual o uso é medido. Essa janela é a janela de contexto do modelo, ou a janela de auto-compactação mais baixa quando uma se aplica. `rawMaxTokens` carrega o mesmo valor que `maxTokens`, e `percentage` é `totalTokens` como uma porcentagem arredondada dessa janela.

Claude Code deixa os diagnósticos opcionais `deferredBuiltinTools`, `systemTools`, e `systemPromptSections` não definidos, então espere que estejam ausentes mesmo que o tipo os declare.

<h3 id="sdkcontrolreadfileresponse">
  `SDKControlReadFileResponse`
</h3>

Tipo de retorno de [`readFile()`](#query-object).

```typescript theme={null}
type SDKControlReadFileResponse = {
  contents: string;
  absPath: string;
  truncated?: boolean;
  encoding?: 'base64';
};
```

`contents` contém o texto do arquivo, ou dados base64 quando você solicitou `encoding: 'base64'`; o campo `encoding` da resposta é definido como `'base64'` nesse caso. `absPath` é o caminho absoluto resolvido. `truncated` é definido quando o arquivo era mais longo que o limite `maxBytes` e o conteúdo foi cortado nesse limite.

<h4 id="what-readfile-can-read">
  O que `readFile()` pode ler
</h4>

`readFile()` serve um conjunto mais estreito de arquivos do que a ferramenta Read:

* Um arquivo regular dentro de um dos diretórios de trabalho da sessão, como `cwd` e `additionalDirectories`
* Alguns dos próprios arquivos do Claude Code para a sessão, como resultados de ferramentas

As regras de negação e solicitação de Read ainda bloqueiam um caminho correspondente, e uma regra de permissão ampla de Read não abre o resto do sistema de arquivos para `readFile()`. Para qualquer outra coisa a chamada resolve com `null`.

<h3 id="sdkcontrolreloadskillsresponse">
  `SDKControlReloadSkillsResponse`
</h3>

Tipo de retorno de [`reloadSkills()`](#query-object).

```typescript theme={null}
type SDKControlReloadSkillsResponse = {
  skills: SlashCommand[];
};
```

`skills` lista as skills disponíveis após o recarregamento, na mesma forma [`SlashCommand`](#slashcommand) que `supportedCommands()` retorna.

<h3 id="sdkcontrolmcpreadresourceresponse">
  `SDKControlMcpReadResourceResponse`
</h3>

Tipo de retorno de [`readMcpResource()`](#query-object), carregando o resultado `resources/read` do servidor MCP. Requer TypeScript Agent SDK v0.3.280 ou posterior.

```typescript theme={null}
type SDKControlMcpReadResourceResponse = {
  contents: {
    uri: string;
    mimeType?: string;
    text?: string;
    blob?: string;
    _meta?: Record<string, unknown>;
  }[];
};
```

Passe `readMcpResource()` o nome do servidor conforme `mcpServerStatus()` o relata e um URI `ui://`, como o `ui.resourceUri` que uma ferramenta declara em sua [`_meta`](#mcpserverstatus). A chamada rejeita para qualquer outro esquema de URI, para um [servidor MCP SDK](#createsdkmcpserver) que sua aplicação hospeda a si mesma, e para um servidor que não está conectado. Está disponível quando a mensagem de inicialização [`capabilities`](#sdksystemmessage) incluem `mcp_read_resource_v1`.

Cada entrada `contents` é um item de conteúdo conforme o servidor o enviou. `blob` contém dados base64 para um item binário, e `_meta` é o próprio `_meta` do item, onde um servidor MCP Apps coloca o `ui.csp` e `ui.permissions` do recurso. O conteúdo é HTML de terceiros não confiável, então renderize-o em um sandbox.

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
  omitClaudeMd?: boolean;
  memory?: "user" | "project" | "local";
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | number;
  permissionMode?: PermissionMode;
  criticalSystemReminder_EXPERIMENTAL?: string;
};
```

| Campo                                 | Obrigatório | Descrição                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------------------------ | :---------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `description`                         | Sim         | Descrição em linguagem natural de quando usar este agente                                                                                                                                                                                                                                                                                                                         |
| `tools`                               | Não         | Array de nomes de ferramentas permitidas. Se omitido, herda cada [ferramenta disponível para subagentes](/docs/pt/sub-agents#available-tools). Para pré-carregar Skills no contexto do agente, use o campo `skills` em vez de listar `'Skill'` aqui                                                                                                                                    |
| `disallowedTools`                     | Não         | Array de nomes de ferramentas para explicitamente desallocar para este agente. Padrões de nível de servidor MCP também são aceitos: `mcp__server` ou `mcp__server__*` remove cada ferramenta desse servidor, e `mcp__*` remove cada ferramenta MCP de qualquer servidor                                                                                                           |
| `prompt`                              | Sim         | O prompt do sistema do agente                                                                                                                                                                                                                                                                                                                                                     |
| `model`                               | Não         | Substituição de modelo para este agente. Aceita um alias como `'fable'`, `'opus'`, `'sonnet'`, `'haiku'`, `'inherit'`, ou um ID de modelo completo. `'inherit'` usa o modelo principal. Quando você o omite, Claude Code escolhe o modelo na [ordem de modelo de subagente](/docs/pt/sub-agents#choose-a-model)                                                                        |
| `mcpServers`                          | Não         | Especificações de servidor MCP para este agente                                                                                                                                                                                                                                                                                                                                   |
| `skills`                              | Não         | Array de nomes de skills para pré-carregar no contexto do agente                                                                                                                                                                                                                                                                                                                  |
| `initialPrompt`                       | Não         | Auto-enviado como o primeiro turno de usuário quando este agente é executado como o agente da thread principal                                                                                                                                                                                                                                                                    |
| `maxTurns`                            | Não         | Número máximo de turnos agênticos (round-trips de API) antes de parar                                                                                                                                                                                                                                                                                                             |
| `background`                          | Não         | Executar este agente como uma tarefa de fundo não-bloqueante quando invocado                                                                                                                                                                                                                                                                                                      |
| `omitClaudeMd`                        | Não         | Executar este agente sem os arquivos CLAUDE.md de usuário, projeto e local quando ele é executado como um subagente; arquivos de política gerenciada ainda carregam. Use-o para agentes que pegam tudo o que precisam do prompt da ferramenta Agent. Ignorado quando este agente é executado como o agente da thread principal. Requer TypeScript Agent SDK v0.3.271 ou posterior |
| `memory`                              | Não         | Fonte de memória para este agente: `'user'`, `'project'`, ou `'local'`                                                                                                                                                                                                                                                                                                            |
| `effort`                              | Não         | Nível de esforço de raciocínio para este agente. Aceita um nível nomeado ou um inteiro                                                                                                                                                                                                                                                                                            |
| `permissionMode`                      | Não         | Modo de permissão para execução de ferramenta dentro deste agente. As [regras de herança de subagente](/docs/pt/agent-sdk/permissions#available-modes) decidem quando se aplica. Veja [`PermissionMode`](#permissionmode)                                                                                                                                                              |
| `criticalSystemReminder_EXPERIMENTAL` | Não         | Experimental: Lembrete crítico adicionado ao prompt do sistema                                                                                                                                                                                                                                                                                                                    |

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

| Valor       | Descrição                                                                                 | Localização                   |
| :---------- | :---------------------------------------------------------------------------------------- | :---------------------------- |
| `'user'`    | Configurações globais do usuário                                                          | `~/.claude/settings.json`     |
| `'project'` | Configurações de projeto compartilhadas (controladas por versão)                          | `.claude/settings.json`       |
| `'local'`   | Configurações de projeto local, gitignored quando Claude Code salva uma configuração nela | `.claude/settings.local.json` |

<h4 id="default-behavior">
  Comportamento padrão
</h4>

Quando `settingSources` é omitido ou `undefined`, `query()` carrega as mesmas configurações do sistema de arquivos que a CLI do Claude Code: usuário, projeto e local. Veja [O que settingSources não controla](/docs/pt/agent-sdk/claude-code-features#what-settingsources-does-not-control) para entradas que são lidas independentemente desta opção, e como desativá-las.

<h4 id="why-use-settingsources">
  Por que usar settingSources
</h4>

**Desativar configurações do sistema de arquivos:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Não carregar configurações de usuário, projeto ou local do disco
const result = query({
  prompt: "Analyze this code",
  options: { settingSources: [] }
});
```

**Carregar apenas fontes de configuração específicas:**

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

// Carregar apenas configurações de projeto, ignorar usuário e local
const result = query({
  prompt: "Run CI checks",
  options: {
    settingSources: ["project"] // Apenas .claude/settings.json
  }
});
```

Para carregar instruções de projeto CLAUDE.md, inclua `"project"` em `settingSources`. Veja [Modificar prompts do sistema](/docs/pt/agent-sdk/modifying-system-prompts#claude-md-files-for-project-level-instructions) para como o carregamento de CLAUDE.md interage com as opções de prompt do sistema.

<h4 id="settings-precedence">
  Precedência de configurações
</h4>

Quando múltiplas fontes são carregadas, as configurações são mescladas com esta precedência (maior para menor):

1. Configurações locais (`.claude/settings.local.json`)
2. Configurações de projeto (`.claude/settings.json`)
3. Configurações do usuário (`~/.claude/settings.json`)

Opções programáticas como `agents`, `allowedTools`, e `settings` substituem configurações do sistema de arquivos de usuário, projeto e local. Configurações de política gerenciada têm precedência sobre opções programáticas.

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
  | "auto"; // Classificador de modelo aprova ou nega prompts de permissão
```

<h3 id="canusetool">
  `CanUseTool`
</h3>

Tipo de função de permissão personalizada para controlar o uso de ferramentas.

A função é a substituição do SDK para o prompt de permissão interativo: é invocada apenas quando o [fluxo de avaliação de permissão](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) se resolve em um prompt. Chamadas de ferramenta já aprovadas por uma entrada `allowedTools`, uma regra de configurações de permissão, ou o modo de permissão, como `acceptEdits` ou `bypassPermissions`, nunca a invocam. Para controlar cada chamada de ferramenta, use um [hook `PreToolUse`](/docs/pt/agent-sdk/hooks) em vez disso.

Uma regra de permissão não pré-aprova as [ações que nenhum modo auto-aprova](/docs/pt/permission-modes#actions-no-mode-auto-approves); veja [Como permissões são avaliadas](/docs/pt/agent-sdk/permissions#how-permissions-are-evaluated) para qual delas alcança o callback e o que acontece em modo `dontAsk` e `auto`.

```typescript theme={null}
type CanUseTool = (
  toolName: string,
  input: Record<string, unknown>,
  options: {
    signal: AbortSignal;
    suggestions?: PermissionUpdate[];
    blockedPath?: string;
    mcpServer?: { name: string; source: string };
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
| `mcpServer`      | `{ name: string; source: string }`          | Para uma ferramenta `mcp__*`, o servidor MCP que a serve e de onde a definição desse servidor veio, com os campos de [`McpServerProvenance`](#mcpserverprovenance). Ausente para outras ferramentas. Requer Agent SDK v0.3.274 ou posterior                                                                                                    |
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
  timeout?: number;
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
  aborted?: true;
  timestamp?: string;
  context_usage?: SDKContextUsage;
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

O campo `message` é uma [`BetaMessage`](https://platform.claude.com/docs/pt/api/messages/create) do SDK Anthropic. Inclui campos como `id`, `content`, `model`, `stop_reason` e `usage`.

`SDKAssistantMessageError` é um de: `'authentication_failed'`, `'oauth_org_not_allowed'`, `'account_on_hold'`, `'billing_error'`, `'rate_limit'`, `'overloaded'`, `'invalid_request'`, `'model_not_found'`, `'server_error'`, `'max_output_tokens'`, `'cloud_credential_error'`, ou `'unknown'`. Quatro desses valores significam mais do que seus nomes dizem:

* `'model_not_found'`: o modelo selecionado não existe ou não está disponível para sua conta ou implantação
* `'overloaded'`: a API retornou um 529 porque o servidor está em capacidade máxima, em contraste com `'rate_limit'`, que é um 429 contra sua cota
* `'account_on_hold'`: [sua conta está em espera](/docs/pt/errors#your-account-is-on-hold)
* `'cloud_credential_error'`: Claude Code não conseguiu obter credenciais AWS ou Google Cloud utilizáveis na máquina em que é executado, portanto nenhuma solicitação chegou ao provedor de nuvem. A causa usual é um login na nuvem que expirou ou nunca foi concluído nessa máquina, embora um serviço de credenciais brevemente inacessível relate o mesmo valor. Veja [Não foi possível carregar credenciais AWS ou Google Cloud](/docs/pt/errors#could-not-load-aws-or-google-cloud-credentials). Requer TypeScript Agent SDK v0.3.267 ou posterior, que agrupa Claude Code v2.1.267

`aborted` é `true` quando uma interrupção ou cancelamento truncou a mensagem do assistente antes do fluxo ser concluído: a mensagem não tem `stop_reason` e o conteúdo pode terminar no meio de uma palavra. O campo está ausente em mensagens normalmente concluídas. Requer Agent SDK v0.3.214 ou posterior.

Claude Code define `user_message_uuid` e `user_message_uuids` na primeira mensagem do assistente do turno, sob as condições em [`user_message_uuid`](#user_message_uuid).

`timestamp` é a hora ISO 8601 quando o conteúdo da mensagem terminou de ser gerado no processo que o produziu. O valor vem do relógio dessa máquina, portanto use-o apenas para exibição e não ordene mensagens por ele. Um turno de API pode produzir várias mensagens do assistente que compartilham um `message.id`, cada uma com seu próprio `timestamp`. Quando o campo está ausente, retorne à hora em que você recebeu a mensagem.

`context_usage` é uma cópia estruturada do relatório `/context`, digitada como [`SDKContextUsage`](#sdkcontextusage), e requer Agent SDK v0.3.232 ou posterior. Quando você envia `/context` como um prompt, Claude Code entrega o relatório como uma mensagem do assistente cujo `message.content` contém a tabela markdown, e anexa `context_usage` à mesma mensagem. Claude Code não define o campo em nenhuma outra mensagem do assistente, e versões anteriores entregam a tabela `/context` sem ele, portanto leia o detalhamento do campo quando estiver presente e retorne ao texto markdown quando não estiver.

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
  pasted_content?: MessageParam["content"][];
  parent_tool_use_id: string | null;
  isSynthetic?: boolean;
  shouldQuery?: boolean;
  tool_use_result?: unknown;
  origin?: SDKMessageOrigin;
  inline_pastes?: string[];
};
```

Defina `pasted_content` para enviar conteúdo que o usuário colou em sua interface de prompt em vez de digitar, uma entrada por colagem, cada uma uma string ou um array de blocos de conteúdo. Claude Code anexa o texto de cada entrada após o texto digitado, em ordem, e pode envolver cada colagem em tags `<pasted_content>`. Blocos diferentes de texto são ignorados, portanto envie imagens e documentos em `message.content`. Requer Agent SDK v0.3.277 ou posterior.

Defina `shouldQuery` como `false` para anexar a mensagem à transcrição sem acionar um turno do assistente. A mensagem é mantida e mesclada na próxima mensagem do usuário que aciona um turno. Use isso para injetar contexto, como a saída de um comando que você executou fora de banda, sem gastar uma chamada de modelo nela.

Em uma mensagem que carrega um bloco `tool_result`, `tool_use_result` é o objeto de saída estruturada da ferramenta em vez do texto enviado ao modelo. Sua forma depende da ferramenta nomeada pelo bloco `tool_use` correspondente, portanto o campo é digitado como `unknown`; as formas integradas estão listadas em [Tipos de Saída de Ferramenta](#tool-output-types).

Para a ferramenta `Agent`, `tool_use_result` é [`AgentOutput`](#agent-2). Em um resultado `completed`, `content` contém o relatório do subagente sem o ID do agente e o trailer de uso que Claude Code anexa ao texto `tool_result`, portanto renderize a partir de `tool_use_result` em vez de analisar esse texto.

Para uma ferramenta MCP cujo resultado contém blocos `resource_link`, `tool_use_result` é um objeto com um array `resourceLinks` de entradas [`SDKMcpResourceLink`](#sdkmcpresourcelink). Claude recebe cada link como uma linha de texto no bloco `tool_result`, portanto leia `resourceLinks` para renderizar os arquivos que o servidor retornou em vez de analisar esse texto. Claude Code omite `resourceLinks` quando o resultado não tem links e em resultados de subagentes, mantém no máximo 50 links por resultado, e para de adicionar links quando o array atinge 64 KiB de JSON serializado. `resourceLinks` requer Agent SDK v0.3.257 ou posterior.

Defina `inline_pastes` para informar ao Claude Code quais partes de `message.content` o usuário colou em vez de digitar, uma string por colagem. O texto do prompt fica onde o usuário o colocou. Claude Code pode envolver cada colagem listada em tags `<pasted_content>` onde ela está, para que Claude possa distinguir material colado das próprias palavras do usuário. Apenas colagens no último bloco de texto do prompt são envolvidas. Requer TypeScript Agent SDK v0.3.280 ou posterior.

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
      user_message_uuid?: string;
      user_message_uuids?: string[];
      request_sent_wall_ms?: number;
      first_content_frame_ms?: number;
      first_stream_post_ms?: number;
      first_stream_post_ack_ms?: number;
      first_stream_post_wall_ms?: number;
      total_cost_usd: number;
      usage: NonNullableUsage;
      modelUsage: { [modelName: string]: ModelUsage };
      permission_denials: SDKPermissionDenial[];
      queued_turn_count?: number;
      structured_output?: unknown;
      deferred_tool_use?: { id: string; name: string; input: Record<string, unknown> };
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
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
      queued_turn_count?: number;
      errors: string[];
      startup_failure_reason?: SDKStartupFailureReason;
      user_message_uuid?: string;
      user_message_uuids?: string[];
      terminal_reason?: TerminalReason;
      fast_mode_state?: FastModeState;
      fast_mode_disabled_reason?: FastModeDisabledReason;
      origin?: SDKMessageOrigin;
    };
```

Vários campos no resultado carregam detalhes de diagnóstico além de `subtype`:

* `api_error_status`: o código de status HTTP do erro de API que encerrou a conversa. Ausente ou `null` quando o turno terminou sem um erro de API.
* `ttft_ms`: tempo até o primeiro token em milissegundos, medido quando a primeira mensagem completa do assistente chega. Presente apenas no braço de sucesso.
* `ttft_stream_ms`: tempo em milissegundos até o primeiro evento de fluxo `message_start`, quando o fluxo de resposta abre. Menor que `ttft_ms`; a lacuna entre os dois é o tempo gasto transmitindo a primeira mensagem. Presente apenas no braço de sucesso.
* `user_message_uuid`: o `uuid` da mensagem que você enviou que este turno respondeu. Veja [`user_message_uuid`](#user_message_uuid) para quais resultados o carregam.
* `user_message_uuids`: os `uuid`s de cada mensagem que você enviou que Claude Code respondeu neste turno. Veja [`user_message_uuids`](#user_message_uuids).
* `request_sent_wall_ms`: milissegundos de época em que Claude Code despachou a solicitação de API, para junções contra timestamps do lado do servidor. Presente apenas junto com [`user_message_uuid`](#user_message_uuid), em um resultado de sucesso com `is_error` false cujo turno enviou uma solicitação de API.
* `first_content_frame_ms`: tempo em milissegundos até o primeiro evento de fluxo `content_block_start` ou `content_block_delta`, contando blocos de pensamento como conteúdo. Presente apenas no braço de sucesso, quando `is_error` é false. Requer Agent SDK v0.3.260 ou posterior.
* `first_stream_post_ms`, `first_stream_post_ack_ms`, `first_stream_post_wall_ms`: cronometragens para fazer upload do primeiro evento de fluxo do turno. Claude Code os registra apenas em sessões que transmite para claude.ai, como [sessões em nuvem](/docs/pt/claude-code-on-the-web), e os resultados que `query()` produz não os carregam. Requer Agent SDK v0.3.260 ou posterior.
* `usage`: apenas loop do agente principal. Exclui chamadas de subagente e modelo auxiliar, e é por turno em sessões de entrada de fluxo. Prefira `modelUsage` para contabilidade de token/custo.
* `modelUsage`: totais por modelo para cada chamada de modelo feita através do pipeline de consulta durante esta chamada `query()`, incluindo o loop principal, subagentes e chamadas internas como compactação e agentes Workflow. Chamadas auxiliares fora desse pipeline, como o classificador de permissão e solicitações de contagem de tokens, são excluídas. Uma chamada que retoma uma sessão também conta os [totais por modelo restaurados das chamadas anteriores da sessão](/docs/pt/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). Em sessões de entrada de fluxo os totais são cumulativos entre turnos, portanto leia o resultado mais recente em vez de somar entre resultados. Veja [Rastrear custos no modo de entrada de fluxo](/docs/pt/agent-sdk/cost-tracking#track-costs-in-streaming-input-mode) para redefinições e [Recuperar totais após uma falha de sessão](/docs/pt/agent-sdk/cost-tracking#recover-totals-after-a-session-crash) para resultados zerados.
* `total_cost_usd`: custo estimado cumulativo em USD, cobrindo as mesmas chamadas que `modelUsage` e redefinindo nos mesmos pontos. Uma chamada que retoma uma sessão também conta os [totais restaurados das chamadas anteriores da sessão](/docs/pt/agent-sdk/cost-tracking#accumulate-costs-across-multiple-calls). É uma estimativa, não uma declaração de faturamento. Veja [Rastrear custo e uso](/docs/pt/agent-sdk/cost-tracking) para ressalvas de precisão.
* `queued_turn_count`: o número de mensagens que você enviou com `origin: { kind: "human" }` que ainda estão esperando quando Claude Code produziu o resultado. Veja [`queued_turn_count`](#queued_turn_count) para o que `0` e um campo ausente dizem a você.
* `startup_failure_reason`: por que Claude Code recusou iniciar, na mensagem de resultado `error_during_execution` que escreve antes de sair em uma falha de inicialização conhecida. Veja [`startup_failure_reason`](#startup_failure_reason) para os valores e quais falhas o carregam. Requer Agent SDK v0.3.274 ou posterior.
* `terminal_reason`: por que o loop terminou. Um de `"completed"`, `"max_turns"`, `"tool_deferred"`, `"aborted_streaming"`, `"aborted_tools"`, `"hook_stopped"`, `"stop_hook_prevented"`, `"background_requested"`, `"blocking_limit"`, `"rapid_refill_breaker"`, `"prompt_too_long"`, `"image_error"`, `"model_error"`, `"api_error"`, `"malformed_tool_use_exhausted"`, `"budget_exhausted"`, `"structured_output_retry_exhausted"`, `"tool_deferred_unavailable"`, ou `"turn_setup_failed"`.
* `fast_mode_state`: um de `"on"`, `"off"`, ou `"cooldown"`.
* `fast_mode_disabled_reason`: por que [modo rápido](/docs/pt/fast-mode) não está disponível agora. Ausente quando nada bloqueia o modo rápido, embora uma solicitação ainda possa ser executada em velocidade padrão. Durante o resfriamento após um limite de taxa de modo rápido, Claude Code relata `fast_mode_state: "cooldown"` sem código de razão e reativa o modo rápido quando o resfriamento expira. Requer Claude Code v2.1.219 ou posterior.

Use o código de razão para explicar por que o modo rápido está desativado em sua própria interface do usuário em vez de rederivá-lo. Cada código nomeia a verificação que bloqueou o modo rápido:

| Código de razão        | Significado                                                                                                                                           |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| `free`                 | A conta não tem a assinatura paga ou créditos de uso que o modo rápido requer                                                                         |
| `preference`           | A organização desativou o modo rápido                                                                                                                 |
| `extra_usage_disabled` | Créditos de uso estão desativados para a conta                                                                                                        |
| `network_error`        | A [verificação de disponibilidade](/docs/pt/fast-mode#use-fast-mode-behind-proxies-and-llm-gateways) não conseguiu alcançar `api.anthropic.com`            |
| `unknown`              | Claude Code não conseguiu determinar a disponibilidade                                                                                                |
| `not_first_party`      | A sessão usa um provedor diferente da API Anthropic                                                                                                   |
| `disabled_by_env`      | [`CLAUDE_CODE_DISABLE_FAST_MODE`](/docs/pt/env-vars) está definido                                                                                         |
| `model_not_allowed`    | O modelo Opus de modo rápido não está na lista de permissões [`availableModels`](/docs/pt/model-config#restrict-model-selection) da organização            |
| `sdk_opt_in_required`  | A sessão não optou pelo modo rápido: passe `fastMode: true` na opção [`settings`](#options) ou através de [`applyFlagSettings()`](#applyflagsettings) |
| `pending`              | A verificação de disponibilidade ainda não foi concluída                                                                                              |

O mesmo par de campos aparece em [`SDKSystemMessage`](#sdksystemmessage) e em [`SDKControlInitializeResponse`](#sdkcontrolinitializeresponse), para que você possa ler o estado do modo rápido antes do primeiro turno.

O campo `origin` encaminha a [`SDKMessageOrigin`](#sdkmessageorigin) da mensagem do usuário que acionou este resultado. Quando o SDK injeta um turno de acompanhamento sintético, como para uma tarefa em segundo plano concluída, a `SDKResultMessage` resultante carrega `origin: { kind: "task-notification" }`. Rotinas cujo gatilho disparou e mensagens verificadas pelo servidor de suas outras sessões chegam com este tipo também, cada uma com o `subkind` descrito em [Subtipos de notificação de tarefa](#task-notification-subkinds). Verifique `kind` para distinguir resultados que respondem ao seu prompt de acompanhamentos injetados antes de roteá-los ou suprimi-los. Se sua aplicação [declara execuções agendadas](#declare-a-scheduled-run), seus resultados carregam `kind: "task-notification"` também, portanto não suprima apenas em `kind`.

Quando várias conclusões de tarefas em segundo plano são enfileiradas juntas, Claude Code pode respondê-las em um turno em vez de um turno cada. Cada conclusão ainda produz seu próprio resultado com esta origem. Todos exceto o último das conclusões que Claude Code responde juntas produzem resultados vazios com `num_turns: 0`, em ordem, e o resultado do último carrega o turno que responde a todos eles.

O campo está ausente para resultados emitidos antes de qualquer turno do usuário, como erros de inicialização.

Quando um hook `PreToolUse` retorna `permissionDecision: "defer"`, o resultado tem `stop_reason: "tool_deferred"` e `deferred_tool_use` carrega o `id`, `name` e `input` da ferramenta pendente. Leia este campo para exibir a solicitação em sua própria interface do usuário, depois retome com o mesmo `session_id` para continuar. Veja [Adiar uma chamada de ferramenta para mais tarde](/docs/pt/hooks#defer-a-tool-call-for-later) para a volta completa.

<h4 id="user_message_uuid">
  `user_message_uuid`
</h4>

O `uuid` da [`SDKUserMessage`](#sdkusermessage) que o turno está respondendo, ecoado para que você possa corresponder a resposta de Claude Code à mensagem que você enviou. Claude Code ecoa um `uuid` apenas se você definir um na mensagem. O campo é opcional em `SDKUserMessage`, e um prompt de string passado para `query()` não carrega nenhum.

Qual de suas mensagens um turno responde depende de como o turno começou:

* **Uma mensagem regular que você enviou**, significando uma sem `isSynthetic: true`: o turno responde essa mensagem por toda sua execução. Quando você envia várias mensagens próximas, Claude Code pode mesclá-las em um turno, e o campo então carrega apenas o `uuid` da última mensagem. Para corresponder a resposta a qualquer uma das mensagens mescladas, use [`user_message_uuids`](#user_message_uuids).
* **Uma mensagem que você enviou com `isSynthetic: true`**: o turno responde essa mensagem no início. Se Claude Code pegar uma mensagem regular sua entre chamadas de ferramenta, o turno responde a mensagem capturada a partir de então. Ecoar o `uuid` de uma mensagem sintética requer Agent SDK v0.3.265 ou posterior; versões anteriores não ecoam nada em turnos sintéticos.
* **Um prompt que Claude Code gerou a si mesmo**, como o turno que continua o trabalho interrompido após uma sessão reiniciar: o turno não responde nenhuma mensagem sua no início e seus frames não carregam nenhum eco. Se Claude Code pegar uma mensagem regular sua entre chamadas de ferramenta, o turno responde essa mensagem a partir de então. O eco de captura requer Agent SDK v0.3.265 ou posterior; versões anteriores não ecoam nada nesses turnos.

Claude Code ecoa o `uuid` da mensagem respondida em três tipos de frame:

* **O resultado**: cada resultado de um turno que respondeu uma mensagem que você enviou. Cada tal resultado o carrega em Agent SDK v0.3.265 ou posterior. Antes da v0.3.265, o resultado de sucesso de um turno que uma mensagem regular iniciou o faltava quando o turno não enviou nenhuma solicitação de API ou terminou com uma chamada de ferramenta adiada. Antes da v0.3.246, resultados de erro também o faltavam, e antes da v0.3.216 cada resultado o faltava.
* **A primeira resposta do turno**: a primeira [mensagem do assistente](#sdkassistantmessage), ou com `includePartialMessages` o primeiro [evento de fluxo](#sdkpartialassistantmessage) cujo `event.type` não é `ping`, para que você possa vincular a resposta antes do resultado chegar. Quando um turno não transmite nada, Claude Code o define na primeira mensagem do assistente. O eco de primeira resposta requer Agent SDK v0.3.246 ou posterior. Quando a mensagem que o turno está respondendo muda no meio do turno, a primeira resposta após a mudança carrega o campo também, em Agent SDK v0.3.265 ou posterior; versões anteriores o definem em um frame de resposta por turno.
* **Cada frame [`thinking_tokens`](#sdkthinkingtokensmessage) do turno**: para que você possa atribuir progresso de pensamento à mensagem que você enviou sem esperar pela primeira resposta do turno. Requer Agent SDK v0.3.260 ou posterior.

Claude Code omite o campo nestes casos:

* Frames de resposta diferentes daqueles primeiros frames de resposta
* Frames de subagente
* Turnos que não respondem nenhuma mensagem com um `uuid`: o turno respondeu uma mensagem que você enviou sem um, ou Claude Code iniciou o turno a si mesmo e não capturou nenhuma mensagem regular que tenha um
* Resultados que não respondem nenhuma mensagem que você enviou, como o resultado zerado após uma falha de processo de worker

<h4 id="user_message_uuids">
  `user_message_uuids`
</h4>

Os `uuid`s de cada mensagem que você enviou que Claude Code respondeu neste turno. Quando você envia várias mensagens próximas, Claude Code pode mesclá-las em um turno, e `user_message_uuid` então nomeia apenas a última delas. Para corresponder a resposta a qualquer uma das mensagens mescladas, procure o `uuid` dessa mensagem em qualquer lugar nesta lista. Requer Agent SDK v0.3.259 ou posterior.

Claude Code define a lista junto com `user_message_uuid` em cada frame de resposta que carrega esse campo e no resultado. Para o conjunto completo de frames que carregam `user_message_uuid`, e a versão que cada um requer, veja [`user_message_uuid`](#user_message_uuid). A lista sempre contém `user_message_uuid` e mantém no máximo 64 entradas.

Quando Claude Code pega uma mensagem regular que você enviou enquanto um turno estava em execução, ele adiciona o `uuid` dessa mensagem à lista do resultado.

Quando uma primeira resposta ou resultado carrega `user_message_uuid` sem a lista, veio de uma versão anterior de Claude Code, portanto retorne ao campo único.

<h4 id="queued_turn_count">
  `queued_turn_count`
</h4>

O número de mensagens que você enviou com [`origin: { kind: "human" }`](#sdkmessageorigin) que ainda estão esperando na fila de comando quando Claude Code produziu o resultado. Requer Agent SDK v0.3.242 ou posterior.

O que `0` e um campo ausente dizem a você:

* **`0`**: Claude Code não conta mensagens que você enviou sem esse `origin`, e não conta notificações de tarefa, portanto um turno ainda pode seguir.
* **Ausente**: o resultado final que Claude Code emite após uma falha ou erro fatal de inicialização omite o campo, e [pode carregar totais zerados](/docs/pt/agent-sdk/cost-tracking#recover-totals-after-a-session-crash).

<h4 id="startup_failure_reason">
  `startup_failure_reason`
</h4>

Por que Claude Code recusou iniciar, para que sua aplicação possa oferecer a correção em vez de uma tentativa. Claude Code o define na mensagem de resultado `error_during_execution` que escreve antes de sair em uma falha de inicialização conhecida. Esse resultado carrega totais zerados, e seu array `errors` carrega o mesmo texto que stderr. O campo está ausente em todos os outros resultados. Requer Agent SDK v0.3.274 ou posterior.

Defina `CLAUDE_CODE_STARTUP_FAILURE_RESULTS` como `1` em [`env`](#options) para receber este resultado para cada valor `SDKStartupFailureReason`. Sem essa variável, Claude Code escreve o resultado apenas para essas falhas, e o resto termina com saída stderr, uma saída não-zero e nenhuma mensagem de resultado:

* Uma retomada que Claude Code para porque não consegue [retornar a sessão para sua worktree](/docs/pt/worktrees#the-session-resumes-outside-its-worktree), com `worktree_unverified` ou `worktree_resume_refused`. Essa seção diz qual erro carrega qual valor.
* Uma [`continue`](#options) recusada de uma conversa que uma sessão em segundo plano mantém, com `session_held_by_background`. Para uma [`resume`](#options) recusada de tal conversa, Claude Code escreve o resultado apenas quando a variável está definida.

```typescript theme={null}
type SDKStartupFailureReason =
  | "org_pin_api_key_conflict"
  | "org_verify_failed"
  | "org_pin_mismatch"
  | "managed_settings_invalid"
  | "remote_settings_required_unavailable"
  | "gateway_signin_required"
  | "gateway_access_denied"
  | "proxy_invalid"
  | "temp_dir_unusable"
  | "cwd_unavailable"
  | "shell_tool_missing"
  | "session_held_by_background"
  | "worktree_resume_refused"
  | "worktree_unverified"
  | "cli_version_too_old"
  | "bypass_root";
```

Cada valor nomeia uma recusa:

| Valor                                  | O que parou a sessão                                                                                                                                                                                                                             |
| :------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `org_pin_api_key_conflict`             | Configurações gerenciadas [requerem um login de gateway de primeira parte ou Cloud](/docs/pt/authentication#restrict-login-to-your-organization), e uma chave de API Anthropic, token de autenticação ou `apiKeyHelper` está configurado em vez disso |
| `org_verify_failed`                    | A organização do login não pôde ser verificada contra o pin, por exemplo devido a uma falha de rede ou um token revogado                                                                                                                         |
| `org_pin_mismatch`                     | O login pertence a uma organização que o pin não permite                                                                                                                                                                                         |
| `managed_settings_invalid`             | Configurações de política gerenciada não puderam ser lidas, ou o pin não nomeia nenhuma organização                                                                                                                                              |
| `remote_settings_required_unavailable` | Configurações gerenciadas que a organização requer não puderam ser carregadas                                                                                                                                                                    |
| `gateway_signin_required`              | O [gateway Cloud](/docs/pt/claude-apps-gateway) encerrou este login                                                                                                                                                                                   |
| `gateway_access_denied`                | A solicitação de configurações gerenciadas para o gateway Cloud voltou com um 403, que a [tabela de solução de problemas](/docs/pt/claude-apps-gateway-deploy#troubleshooting) do gateway cobre                                                       |
| `proxy_invalid`                        | Uma configuração de proxy não é uma URL completa                                                                                                                                                                                                 |
| `temp_dir_unusable`                    | O diretório temporário por usuário é inseguro ou não pôde ser criado                                                                                                                                                                             |
| `cwd_unavailable`                      | O diretório de trabalho foi deletado, movido ou não pode ser lido                                                                                                                                                                                |
| `shell_tool_missing`                   | No Windows, nenhuma ferramenta shell está disponível: Git Bash está faltando, e PowerShell está faltando ou desativado com `CLAUDE_CODE_USE_POWERSHELL_TOOL`                                                                                     |
| `session_held_by_background`           | A conversa para retomar ou continuar está sendo executada como uma [sessão em segundo plano](/docs/pt/agent-view)                                                                                                                                     |
| `worktree_resume_refused`              | A worktree da sessão falhou em suas verificações de segurança, ou a retomada foi lançada de dentro dela. `errors` diz se executar a mesma retomada novamente continua sem a worktree                                                             |
| `worktree_unverified`                  | A worktree da sessão não pôde ser verificada agora, e tentar novamente pode ter sucesso                                                                                                                                                          |
| `cli_version_too_old`                  | Esta versão de Claude Code está abaixo do mínimo que Anthropic requer                                                                                                                                                                            |
| `bypass_root`                          | Modo de permissões de bypass foi solicitado enquanto executava como root                                                                                                                                                                         |

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
    source?: string;
  }[];
  model: string;
  permissionMode: PermissionMode;
  slash_commands: string[];
  terminal_slash_commands?: string[];
  output_style: string;
  skills: string[];
  plugins: { name: string; path: string }[];
  fast_mode_state?: FastModeState;
  fast_mode_disabled_reason?: FastModeDisabledReason;
  effort?: "low" | "medium" | "high" | "xhigh" | "max" | null;
  capabilities?: string[];
};
```

`fast_mode_state` relata o estado [modo rápido](/docs/pt/fast-mode) da sessão. Quando algo bloqueia o modo rápido, `fast_mode_disabled_reason` nomeia a verificação que o bloqueou; o campo requer Claude Code v2.1.219 ou posterior. Para os códigos de razão e seus significados, veja [`fast_mode_disabled_reason`](#sdkresultmessage) na mensagem de resultado.

`terminal_slash_commands` nomeia as entradas em `slash_commands` cuja interface está vinculada ao terminal local, como `exit`. Você pode enviá-las como qualquer outra entrada em `slash_commands`; o campo existe para que um cliente remoto ou móvel possa ocultá-las de seus menus de comando. O campo está presente apenas quando não vazio, e requer Agent SDK v0.3.229 ou posterior.

*

`source` em cada entrada `mcp_servers`: de onde veio a definição do servidor, com os mesmos valores que [`McpServerStatus`](#mcpserverstatus)'s `source`. Requer Agent SDK v0.3.274 ou posterior.

*

`effort`: o [nível de esforço](/docs/pt/model-config#adjust-effort-level) que Claude Code envia na próxima solicitação da sessão, ou `null` quando não envia nenhum. Claude Code define o campo apenas na mensagem de inicialização que envia para clientes [Remote Control](/docs/pt/remote-control), e o omite da mensagem de inicialização que sua aplicação lê. Requer Agent SDK v0.3.234 ou posterior.

O array `capabilities` nomeia os comportamentos de protocolo que esta CLI implementa, para que você possa fazer detecção de recursos em vez de comparar strings `claude_code_version`. É um conjunto aberto: ignore valores que você não reconhecer, e verifique a capacidade específica cujo comportamento você depende. O campo requer Claude Code v2.1.205 ou posterior e está ausente em CLIs anteriores.

| Capacidade                   | Significado                                                                                                                                                                                                                                                                                    |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `interrupt_receipt_v1`       | [`interrupt()`](#query-object) resolve com uma resposta [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse) listando as mensagens que estavam pendentes quando a interrupção chegou                                                                                                  |
| `interrupt_cancel_queued_v1` | A solicitação de controle `interrupt` honra `cancel_queued: true`, cancelando as mensagens que a resposta listaria sob `still_queued` e listando-as sob `cancelled` em vez disso. Veja [`SDKControlInterruptResponse`](#sdkcontrolinterruptresponse). Requer Claude Code v2.1.219 ou posterior |

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
  user_message_uuid?: string;
  user_message_uuids?: string[];
};
```

Claude Code define `user_message_uuid` e `user_message_uuids` no primeiro evento de fluxo não-ping do turno, e novamente quando a mensagem que o turno está respondendo muda, sob as condições em [`user_message_uuid`](#user_message_uuid).

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

Banner de texto genérico emitido pelo loop. Carrega linhas de status sem erro, feedback de hook como a razão de bloqueio de um hook `UserPromptSubmit`, e saída de comando. Em Claude Code v2.1.227 ou posterior, a [`systemMessage`](/docs/pt/hooks#json-output) de um hook pode chegar como esta mensagem, com cada linha prefixada pelo nome do hook, como `PostToolUse:Bash says:`. Se a `systemMessage` de um hook chega como esta mensagem depende do evento. Cada [seção de evento](/docs/pt/hooks#hook-events) na página de hooks diz como a saída aparece. Renderize `content` como texto simples no `level` fornecido.

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

Evento de fluxo emitido quando o sistema de permissão nega uma chamada de ferramenta sem um prompt interativo. Use-o para renderizar a negação em sua interface do usuário conforme ela acontece, em vez de apenas observar o resultado da ferramenta `is_error` que se segue. Qual negação ele relata depende de como a execução lida com prompts de permissão:

* **Com um callback [`canUseTool`](#canusetool)** e o padrão [`permissionPrompts: 'host'`](#options): prompts de permissão vão para seu callback, e este evento relata as negações que Claude Code decide por conta própria sem chamá-lo.
*

**Com nenhum**: uma execução `-p` simples, ou `query()` que não define nem `canUseTool` nem `permissionPromptToolName`, nega qualquer chamada de ferramenta que teria solicitado, e este evento relata essas negações bem como as que Claude Code decide por conta própria. Antes da v2.1.223, Claude Code não emitia este evento em execuções sem um callback.

* **Com uma ferramenta de prompt MCP**, definida com `permissionPromptToolName` ou a flag [`--permission-prompt-tool`](/docs/pt/cli-reference#cli-flags), e o padrão `permissionPrompts: 'host'`: Claude Code não emite este evento, nem mesmo para as negações de regra que decide por conta própria.
*

**Com [`permissionPrompts: 'none'`](#options)**: Claude Code nega as chamadas que teriam solicitado, mesmo quando `canUseTool` ou uma ferramenta de prompt MCP também está definida, e este evento relata essas negações bem como as que Claude Code decide por conta própria. Requer Claude Code v2.1.259 ou posterior.

Em cada configuração, este evento pula qualquer negação decidida no caminho do hook `PreToolUse`, independentemente de o hook ter negado a chamada a si mesmo ou uma regra de negação ter substituído a decisão de permitir ou perguntar do hook. O evento também é melhor esforço: ocasionalmente Claude Code registra uma negação sem emitir este evento, portanto `permission_denials` na [mensagem de resultado](#sdkresultmessage) é o registro autoritário.

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

<h3 id="sdkcontextusage">
  `SDKContextUsage`
</h3>

Forma estruturada do relatório `/context`, carregada como `context_usage` na [`SDKAssistantMessage`](#sdkassistantmessage) que entrega um resultado `/context`. Agent SDK v0.3.232 e posterior exportam o tipo. Diferentemente de [`SDKControlGetContextUsageResponse`](#sdkcontrolgetcontextusageresponse), carrega apenas os dados necessários para renderizar o detalhamento de uso, sem campos de exibição como `color` e `gridRows`.

```typescript theme={null}
type SDKContextUsage = {
  model: string;
  total_tokens: number;
  raw_max_tokens: number;
  percentage: number;
  over_limit?: {
    tokens_over: number;
    kind: "hard_limit" | "compaction_window";
  };
  categories: SDKContextUsageCategory[];
  mcp_tools: {
    name: string;
    server_name: string;
    tokens: number;
  }[];
  memory_files: {
    path: string;
    type: string;
    tokens: number;
  }[];
  agents: {
    agent_type: string;
    source: string;
    tokens: number;
  }[];
  skills?: {
    name: string;
    source: string;
    plugin_name?: string;
    tokens: number;
  }[];
};
```

A tabela lista o que Claude Code coloca em cada campo. Os campos de `model` até `over_limit` descrevem a sessão como um todo, e os campos de coleção atribuem tokens a itens individuais.

| Campo            | Tipo                                                      | Descrição                                                                                                                                                                                                                                                                                                                    |
| ---------------- | --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `model`          | `string`                                                  | O modelo do loop principal para o qual Claude Code computou o uso, não o de um subagente                                                                                                                                                                                                                                     |
| `total_tokens`   | `number`                                                  | Estimativa de Claude Code dos tokens em uso. Não fixada à janela, portanto pode exceder `raw_max_tokens` quando a sessão está acima do limite                                                                                                                                                                                |
| `raw_max_tokens` | `number`                                                  | A janela de contexto do modelo, ou a [janela de auto-compactação](/docs/pt/model-config#context-window-and-auto-compaction) mais baixa quando uma se aplica, como uma que você definiu ou o limite de 200K que Claude Code aplica a alguns modelos com uma janela de 1M-token. Claude Code mede `total_tokens` contra esta janela |
| `percentage`     | `number`                                                  | `total_tokens` como uma porcentagem arredondada de `raw_max_tokens`, portanto pode exceder 100 quando a sessão está acima do limite                                                                                                                                                                                          |
| `over_limit`     | `object`                                                  | Presente apenas quando `total_tokens` excede `raw_max_tokens`. `tokens_over` é a quantidade acima, e `kind` diz como Claude Code resolveu a janela                                                                                                                                                                           |
| `categories`     | [`SDKContextUsageCategory`](#sdkcontextusagecategory)`[]` | Uma entrada por linha do detalhamento de uso por categoria                                                                                                                                                                                                                                                                   |
| `mcp_tools`      | `object[]`                                                | Tokens atribuídos a cada ferramenta MCP, com seu nome de fio, como `mcp__linear__create_issue`, e seu `server_name`                                                                                                                                                                                                          |
| `memory_files`   | `object[]`                                                | Tokens atribuídos a cada arquivo de memória carregado, com seu `path` e um rótulo de origem como `Project` ou `User` em `type`                                                                                                                                                                                               |
| `agents`         | `object[]`                                                | Tokens atribuídos a cada definição de subagente customizado, com um identificador de origem como `projectSettings`, `userSettings`, ou `plugin`. Subagentes integrados não estão listados                                                                                                                                    |
| `skills`         | `object[]`                                                | Tokens atribuídos a cada skill na listagem de skills, com um identificador de origem e, para skills de plugin, o nome do plugin em `plugin_name`. Ausente quando nenhuma skill contribui tokens                                                                                                                              |

`over_limit.kind` registra como Claude Code resolveu a janela, não se a API aceita a próxima solicitação:

* `hard_limit`: a janela é o que Claude Code acredita ser o próprio limite do modelo, além do qual a API recusa solicitações
* `compaction_window`: a janela é uma janela de política de compactação, que pode ou não coincidir com o limite do modelo

Claude Code evolui o tipo aditivamente, adicionando novos dados como campos opcionais em vez de remodelar os existentes. Leia os campos que você conhece e ignore qualquer um que você não reconheça.

<h3 id="sdkcontextusagecategory">
  `SDKContextUsageCategory`
</h3>

Uma linha do detalhamento de uso por categoria `/context`.

```typescript theme={null}
type SDKContextUsageCategory = {
  name: string;
  tokens: number;
  kind: "used" | "free" | "buffer" | "deferred";
};
```

A tabela lista o que Claude Code coloca em cada campo de uma linha.

| Campo    | Tipo     | Descrição                                                                                                           |
| -------- | -------- | ------------------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | O nome de exibição da linha como `/context` a imprime, como `Messages`. Classifique linhas por `kind`, não por nome |
| `tokens` | `number` | A contagem de tokens da linha. Linhas podem carregar zero tokens                                                    |
| `kind`   | `string` | O que a linha representa: `used`, `free`, `buffer`, ou `deferred`                                                   |

Cada valor `kind` diz o que os tokens da linha são:

* `used`: conteúdo que ocupa a janela de contexto
* `free`: a janela restante
* `buffer`: a reserva de compactação
* `deferred`: esquemas de ferramenta que Claude Code mantém fora da janela e exclui do cálculo de uso, listados para conscientização

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
      fromMode?: "bypass" | "prompting";
      name?: string;
      fromSession?: string;
      senderTaskId?: string;
      body?: string;
      verifiedPeerPid?: number;
    }
  | {
      kind: "task-notification";
      subkind?: "scheduled-trigger" | "peer-send-message";
      fireReason?: string;
    }
  | { kind: "coordinator" }
  | { kind: "auto-continuation" }
  | { kind: "unclassified" };
```

| `kind`              | Significado                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `human`             | Entrada direta do usuário final. Se sua aplicação encaminha o que o usuário digitou como uma mensagem de usuário, defina seu `origin` como `{ kind: "human" }` explicitamente: Claude Code trata uma mensagem de usuário sem `origin` como não atribuída, e verifica que requerem um prompt digitado por humano, como a [palavra-chave de workflow `ultracode`](/docs/pt/workflows#ask-for-a-workflow-in-your-prompt), não a aceitam. Antes da v2.1.210, Claude Code tratava um `origin` ausente em uma mensagem de usuário como entrada humana. |
| `channel`           | Mensagem chegando em um [canal](/docs/pt/channels). `server` é o nome do servidor MCP de origem.                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `peer`              | Mensagem de outro agente: um [colega de equipe](/docs/pt/agent-teams) em processo ou um [par entre sessões](/docs/pt/cross-session-messaging), outra de suas sessões Claude Code. Veja [Campos de origem de par](#peer-origin-fields) para a semântica por campo e o modelo de confiança.                                                                                                                                                                                                                                                             |
| `task-notification` | Turno sintético injetado para uma entrega que chega sem um prompt de usuário novo, como uma tarefa em segundo plano concluída; veja [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) para esse braço. O `subkind` opcional marca o que levantou a notificação. Veja [Subtipos de notificação de tarefa](#task-notification-subkinds).                                                                                                                                                                                            |
| `coordinator`       | Mensagem de um coordenador de equipe em uma [equipe de agente](/docs/pt/agent-teams).                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `auto-continuation` | Turno sintético injetado quando a sessão continua sem entrada de usuário nova, como um resultado de comando que aciona um prompt de acompanhamento.                                                                                                                                                                                                                                                                                                                                                                                         |
| `unclassified`      | Turno injetado cuja origem não pôde ser determinada. Requer Claude Code v2.1.223 ou posterior. Quando Claude Code recebe uma [`SDKUserMessage`](#sdkusermessage) com `isSynthetic: true` e não consegue classificá-la como qualquer outro `kind`, define este kind conforme a mensagem chega e enquadra o turno para o modelo como uma fonte não-usuário em vez de tratá-lo como entrada humana. Sua aplicação não deve definir este valor.                                                                                                 |

<h3 id="task-notification-subkinds">
  Subtipos de notificação de tarefa
</h3>

Quando Claude Code entrega uma notificação de tarefa em uma sessão, ele define `subkind` na `origin` da notificação apenas se servidores Anthropic verificaram de onde essa notificação veio. Ele também define `subkind` quando sua aplicação [declara a mensagem como uma execução agendada](#declare-a-scheduled-run) a si mesma, o que requer TypeScript Agent SDK v0.3.280 ou posterior. `subkind` requer Claude Code v2.1.213 ou posterior, e toma um de dois valores:

* `scheduled-trigger`: a notificação é um prompt armazenado de uma [rotina](/docs/pt/routines), entregue porque um dos gatilhos da rotina disparou: seu cronograma, seu [gatilho de API](/docs/pt/routines#add-an-api-trigger), seu [gatilho GitHub](/docs/pt/routines#add-a-github-trigger), ou **Executar agora**. Uma mensagem que sua aplicação [declara como uma execução agendada](#declare-a-scheduled-run) carrega este valor também. Claude Code enquadra estes para o modelo como a tarefa atribuída da sessão, com um aviso diferente do [aviso que outras notificações de tarefa carregam](#sdktasknotificationmessage).
*

`peer-send-message`: a notificação é uma mensagem que outra de suas sessões enviou com a ferramenta `send_message` do lado do servidor que [Claude Code na web](/docs/pt/claude-code-on-the-web) sessões usam para se mensagear, não a [ferramenta `SendMessage` entre sessões](/docs/pt/cross-session-messaging), e servidores Anthropic verificaram que ambas as sessões pertencem ao mesmo grupo privado de sessões. Requer Claude Code v2.1.224 ou posterior. Uma entrega `send_message` que os servidores não verificaram dessa forma não tem `subkind`.

Toda outra notificação de tarefa não tem `subkind`. Isso inclui [tarefas agendadas](/docs/pt/scheduled-tasks) que disparam em sua própria máquina, [atividade de PR](/docs/pt/claude-code-on-the-web#how-claude-responds-to-pr-activity) entregue em uma sessão, e eventos em segundo plano como uma tarefa concluída. Mensagens da [ferramenta `SendMessage` entre sessões](/docs/pt/cross-session-messaging) não são notificações de tarefa: independentemente de virem de uma sessão na mesma máquina ou através de servidores Anthropic de outra máquina, Claude Code lhes dá `kind: "peer"` e os [campos de origem de par](#peer-origin-fields).

`fireReason` diz por que uma notificação `scheduled-trigger` disparou, como um token em minúsculas curto como `scheduled`, `manual`, `retry`, `catch_up`, ou `api`. Servidores Anthropic o definem nas entregas de uma [rotina](/docs/pt/routines), e sua aplicação o define quando declara uma execução agendada. Está ausente quando nenhum dos dois enviou um. Requer TypeScript Agent SDK v0.3.280 ou posterior.

<h4 id="declare-a-scheduled-run">
  Declare a scheduled run
</h4>

Se sua aplicação executa prompts em seu próprio cronograma, declare cada execução para que Claude Code enquadre o turno para o modelo como uma tarefa agendada em vez de como entrada ao vivo do usuário. Inicie a sessão com `CLAUDE_CODE_HOST_SCHEDULED_RUN` definido como `1` em [`env`](#options), depois envie a [`SDKUserMessage`](#sdkusermessage) da execução com `origin: { kind: "task-notification", subkind: "scheduled-trigger", fireReason: "scheduled" }` e sem `isSynthetic`. Claude Code ignora a declaração em um processo iniciado sem essa variável. Ele também a ignora em um processo cujo ambiente carrega [`CLAUDECODE`](/docs/pt/env-vars) ou `CLAUDE_CODE_CHILD_SESSION`. Claude Code mantém `fireReason` apenas quando o valor é 1 a 32 letras minúsculas ou underscores. Requer TypeScript Agent SDK v0.3.280 ou posterior.

<h3 id="peer-origin-fields">
  Campos de origem de par
</h3>

Uma origem `peer` identifica qual agente enviou a mensagem: um [colega de equipe](/docs/pt/agent-teams) em processo enviando para `main` com `SendMessage`, ou um [par entre sessões](/docs/pt/cross-session-messaging), outra de suas sessões Claude Code. Pares entre sessões requerem Claude Code v2.1.224 ou posterior em macOS e Linux; veja [disponibilidade de mensagens entre sessões](/docs/pt/cross-session-messaging#availability) para o requisito nativo do Windows. Um par entre sessões pode executar na mesma máquina, ou em [outra de suas máquinas](/docs/pt/cross-session-messaging#message-sessions-on-other-machines) ou [Claude Code na web](/docs/pt/claude-code-on-the-web) quando sua mensagem chega através de Remote Control. Os dois tipos de remetente preenchem os campos diferentemente:

* `from`: o nome do colega de equipe, ou o endereço do remetente para um par entre sessões. Para uma [mensagem entre máquinas unidirecional](/docs/pt/cross-session-messaging#message-sessions-on-other-machines), o remetente não tem endereço de resposta e `from` é `"unknown"`. O valor é criado pelo remetente; `verifiedPeerPid` é a identidade verificada.
*

`fromMode`: a classe de permissão da sessão de envio, `bypass` ou `prompting`, declarada por um host que retransmite uma mensagem de par entre suas sessões, como o [aplicativo de desktop](/docs/pt/desktop#work-across-sessions). Claude Code a lê na sessão receptora quando aplica os [controles de entrada](/docs/pt/cross-session-messaging#control-inbound-messages). Requer Agent SDK v0.3.234 ou posterior.

* `senderTaskId`: o ID de tarefa do colega de equipe. Ausente para um par entre sessões.
*

`name`: o nome de exibição do remetente, normalizado por Claude Code: remove pontos de código de controle, formato, substituto e separador de linha ou parágrafo Unicode, depois corta o resultado e o limita a 64 pontos de código com reticências. Requer Claude Code v2.1.205 ou posterior.

*

`body`: o corpo da mensagem decodificado com o envelope de par removido, byte-exato com o que o modelo vê. Sempre presente para uma mensagem de colega de equipe; para um par entre sessões, presente apenas quando o turno é exatamente um envelope de par formado por Claude Code. Renderize `name` e `body` em vez de reanalisar o texto da mensagem. Requer Claude Code v2.1.205 ou posterior.

*

`fromSession`: o ID de sessão do remetente que pode ser aberto pelo host, definido pelo host do remetente para que sua interface do usuário possa vincular de volta à sessão de envio. Como `from`, é afirmado pelo remetente: use-o como um alvo de navegação apenas, e não o trate como prova da identidade do remetente. Requer Claude Code v2.1.216 ou posterior.

*

`verifiedPeerPid`: o ID do processo do processo que se conectou ao socket de mensagens entre sessões desta sessão, verificado pelo kernel e lido da conexão a si mesma, nunca do payload. Use-o, não `from`, para identificar o remetente: `from` é forjável por qualquer processo do mesmo usuário. O campo está ausente quando Claude Code não consegue verificá-lo, como no Windows ou entrada não-socket, portanto um valor ausente significa que o remetente não é verificado. Para tráfego retransmitido identifica o retransmissor em vez do autor da mensagem, e IDs de processo são recicláveis, portanto trate-o como proveniência em vez de um token de autenticação. Requer Claude Code v2.1.216 ou posterior.

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
  | "UserPromptExpansion"
  | "SessionStart"
  | "SessionEnd"
  | "Stop"
  | "StopFailure"
  | "SubagentStart"
  | "SubagentStop"
  | "PreCompact"
  | "PostCompact"
  | "PreModelSwitch"
  | "PostModelSwitch"
  | "PermissionRequest"
  | "PermissionDenied"
  | "Setup"
  | "TeammateIdle"
  | "TaskCreated"
  | "TaskCompleted"
  | "Elicitation"
  | "ElicitationResult"
  | "ConfigChange"
  | "DirectoryAdded"
  | "WorktreeCreate"
  | "WorktreeRemove"
  | "InstructionsLoaded"
  | "CwdChanged"
  | "FileChanged"
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
  | PermissionDeniedHookInput
  | NotificationHookInput
  | UserPromptSubmitHookInput
  | UserPromptExpansionHookInput
  | SessionStartHookInput
  | SessionEndHookInput
  | StopHookInput
  | StopFailureHookInput
  | SubagentStartHookInput
  | SubagentStopHookInput
  | PreCompactHookInput
  | PostCompactHookInput
  | PreModelSwitchHookInput
  | PostModelSwitchHookInput
  | PermissionRequestHookInput
  | SetupHookInput
  | TeammateIdleHookInput
  | TaskCreatedHookInput
  | TaskCompletedHookInput
  | ElicitationHookInput
  | ElicitationResultHookInput
  | ConfigChangeHookInput
  | InstructionsLoadedHookInput
  | DirectoryAddedHookInput
  | WorktreeCreateHookInput
  | WorktreeRemoveHookInput
  | CwdChangedHookInput
  | FileChangedHookInput
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
  mcp_server?: McpServerProvenance;
};
```

`mcp_server` está presente quando a ferramenta vem de um servidor MCP; veja [`McpServerProvenance`](#mcpserverprovenance). As entradas `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` e `PermissionDenied` carregam o mesmo campo. O campo requer Agent SDK v0.3.274 ou posterior.

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
  mcp_server?: McpServerProvenance;
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
  mcp_server?: McpServerProvenance;
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

<h4 id="permissiondeniedhookinput">
  `PermissionDeniedHookInput`
</h4>

```typescript theme={null}
type PermissionDeniedHookInput = BaseHookInput & {
  hook_event_name: "PermissionDenied";
  tool_name: string;
  tool_input: unknown;
  tool_use_id: string;
  reason: string;
  mcp_server?: McpServerProvenance;
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
  session_title?: string;
};
```

<h4 id="userpromptexpansionhookinput">
  `UserPromptExpansionHookInput`
</h4>

```typescript theme={null}
type UserPromptExpansionHookInput = BaseHookInput & {
  hook_event_name: "UserPromptExpansion";
  expansion_type: "slash_command" | "mcp_prompt";
  command_name: string;
  command_args: string;
  command_source?: string;
  prompt: string;
};
```

<h4 id="sessionstarthookinput">
  `SessionStartHookInput`
</h4>

```typescript theme={null}
type SessionStartHookInput = BaseHookInput & {
  hook_event_name: "SessionStart";
  source: "startup" | "resume" | "clear" | "compact" | "fork";
  agent_type?: string;
  model?: string;
  session_title?: string;
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

<h4 id="stopfailurehookinput">
  `StopFailureHookInput`
</h4>

```typescript theme={null}
type StopFailureHookInput = BaseHookInput & {
  hook_event_name: "StopFailure";
  error: SDKAssistantMessageError;
  error_details?: string;
  last_assistant_message?: string;
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

<h4 id="postcompacthookinput">
  `PostCompactHookInput`
</h4>

```typescript theme={null}
type PostCompactHookInput = BaseHookInput & {
  hook_event_name: "PostCompact";
  trigger: "manual" | "auto";
  compact_summary: string;
};
```

<h4 id="premodelswitchhookinput">
  `PreModelSwitchHookInput`
</h4>

Dispara antes de uma mudança de modelo solicitada entrar em vigor. `context_tokens` e os campos após ele estimam o custo de reenviar a conversa para o novo modelo. Para as descrições completas de campos e semântica de bloqueio, veja [PreModelSwitch](/docs/pt/hooks#premodelswitch).

```typescript theme={null}
type PreModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PreModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
};
```

<h4 id="postmodelswitchhookinput">
  `PostModelSwitchHookInput`
</h4>

Dispara após o modelo da sessão mudar. Ele carrega os mesmos campos que `PreModelSwitchHookInput`, com dois valores de `source` adicionais. Veja [PostModelSwitch](/docs/pt/hooks#postmodelswitch).

```typescript theme={null}
type PostModelSwitchHookInput = BaseHookInput & {
  hook_event_name: "PostModelSwitch";
  from_model: string;
  to_model: string;
  requested_model: string | null;
  source: "command" | "picker" | "sdk" | "auto" | "resume";
  context_tokens: number;
  prompt_cache_warm: boolean;
  cache_ttl: "5m" | "1h";
  estimated_cache_write_usd: number;
  pricing: "configured" | "catalog" | "default";
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
  mcp_server?: McpServerProvenance;
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

<h4 id="taskcreatedhookinput">
  `TaskCreatedHookInput`
</h4>

```typescript theme={null}
type TaskCreatedHookInput = BaseHookInput & {
  hook_event_name: "TaskCreated";
  task_id: string;
  task_subject: string;
  task_description?: string;
  teammate_name?: string;
  /** @deprecated desde v2.1.178. Carrega o nome da equipe derivado da sessão; será removido. */
  team_name?: string;
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

<h4 id="elicitationhookinput">
  `ElicitationHookInput`
</h4>

```typescript theme={null}
type ElicitationHookInput = BaseHookInput & {
  hook_event_name: "Elicitation";
  mcp_server_name: string;
  message: string;
  mode?: "form" | "url";
  url?: string;
  elicitation_id?: string;
  requested_schema?: Record<string, unknown>;
};
```

<h4 id="elicitationresulthookinput">
  `ElicitationResultHookInput`
</h4>

```typescript theme={null}
type ElicitationResultHookInput = BaseHookInput & {
  hook_event_name: "ElicitationResult";
  mcp_server_name: string;
  elicitation_id?: string;
  mode?: "form" | "url";
  action: "accept" | "decline" | "cancel";
  content?: Record<string, unknown>;
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

<h4 id="instructionsloadedhookinput">
  `InstructionsLoadedHookInput`
</h4>

```typescript theme={null}
type InstructionsLoadedHookInput = BaseHookInput & {
  hook_event_name: "InstructionsLoaded";
  file_path: string;
  memory_type: "User" | "Project" | "Local" | "Managed";
  load_reason:
    | "session_start"
    | "nested_traversal"
    | "path_glob_match"
    | "include"
    | "compact";
  globs?: string[];
  trigger_file_path?: string;
  parent_file_path?: string;
};
```

<h4 id="directoryaddedhookinput">
  `DirectoryAddedHookInput`
</h4>

```typescript theme={null}
type DirectoryAddedHookInput = BaseHookInput & {
  hook_event_name: "DirectoryAdded";
  directory: string;
  source: "slash_command" | "register_repo_root";
};
```

`directory` é o caminho absoluto do diretório que foi adicionado. `source` é `"slash_command"` quando `/add-dir` o adicionou e `"register_repo_root"` quando a solicitação de controle do SDK o fez.

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

<h4 id="cwdchangedhookinput">
  `CwdChangedHookInput`
</h4>

```typescript theme={null}
type CwdChangedHookInput = BaseHookInput & {
  hook_event_name: "CwdChanged";
  old_cwd: string;
  new_cwd: string;
};
```

<h4 id="filechangedhookinput">
  `FileChangedHookInput`
</h4>

```typescript theme={null}
type FileChangedHookInput = BaseHookInput & {
  hook_event_name: "FileChanged";
  file_path: string;
  event: "change" | "add" | "unlink";
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
  /**
   * Uma sequência de escape de terminal (por exemplo, OSC 9 / OSC 777 desktop-notification)
   * para Claude Code emitir em seu nome. Apenas OSCs de notificação/título
   * (0, 1, 2, 9, 99, 777) e BEL são permitidos; um valor contendo
   * qualquer outra coisa é ignorado como um todo. Apenas o CLI interativo o emite;
   * o SDK ignora o campo.
   */
  terminalSequence?: string;
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
        sessionTitle?: string;
        /** Quando a decisão é "block", omita o prompt original da mensagem de bloqueio. */
        suppressOriginalPrompt?: boolean;
      }
    | {
        hookEventName: "UserPromptExpansion";
        additionalContext?: string;
      }
    | {
        hookEventName: "SessionStart";
        additionalContext?: string;
        initialUserMessage?: string;
        sessionTitle?: string;
        watchPaths?: string[];
        /**
         * Rescaneie os diretórios de skill e comando após a conclusão dos hooks SessionStart,
         * para que skills instaladas pelo hook estejam disponíveis na
         * mesma sessão.
         */
        reloadSkills?: boolean;
      }
    | {
        hookEventName: "Setup";
        additionalContext?: string;
      }
    | {
        hookEventName: "PreModelSwitch";
        /**
         * Mesmo contrato que PreToolUse: "allow" prossegue, "deny" cancela
         * a mudança, "ask" pede ao usuário para confirmar. Apenas /model em uma
         * sessão interativa mostra esse prompt; todas as outras superfícies,
         * incluindo solicitações set_model, tratam "ask" como uma recusa.
         */
        permissionDecision?: "allow" | "deny" | "ask";
        permissionDecisionReason?: string;
      }
    | {
        hookEventName: "PostModelSwitch";
        /** Chega ao modelo com a próxima solicitação que o novo modelo atende. */
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStart";
        additionalContext?: string;
      }
    | {
        hookEventName: "PostToolUse";
        additionalContext?: string;
        /**
         * Nota breve sobre o resultado desta chamada de ferramenta para o classificador
         * de permissão do modo automático. Limitado a 2000 caracteres, compartilhado entre
         * todos os hooks que respondem à mesma chamada; honrado apenas em respostas
         * de hook síncrono. Não copie saída de ferramenta não confiável para ele.
         */
        classifierContext?: string;
        updatedToolOutput?: unknown;
        /** @deprecated Use `updatedToolOutput`, que funciona para todas as ferramentas. */
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
        hookEventName: "Stop";
        additionalContext?: string;
      }
    | {
        hookEventName: "SubagentStop";
        additionalContext?: string;
      }
    | {
        hookEventName: "PermissionDenied";
        retry?: boolean;
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
      }
    | {
        hookEventName: "Elicitation";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "ElicitationResult";
        action?: "accept" | "decline" | "cancel";
        content?: Record<string, unknown>;
      }
    | {
        hookEventName: "CwdChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "FileChanged";
        watchPaths?: string[];
      }
    | {
        hookEventName: "WorktreeCreate";
        worktreePath: string;
      }
    | {
        hookEventName: "MessageDisplay";
        /** Texto exibido no lugar do delta. Omita (ou retorne o delta inalterado) para exibir o original. */
        displayContent?: string;
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

União de tipos de entrada de ferramenta exportados de `@anthropic-ai/claude-agent-sdk`; os membros incluem:

```typescript theme={null}
type ToolInputSchemas =
  | AgentInput
  | ArtifactInput
  | AskUserQuestionInput
  | BashInput
  | CronCreateInput
  | CronDeleteInput
  | CronListInput
  | EnterPlanModeInput
  | EnterWorktreeInput
  | ExitPlanModeInput
  | ExitWorktreeInput
  | FileEditInput
  | FileReadInput
  | FileWriteInput
  | GlobInput
  | GrepInput
  | ListMcpResourcesInput
  | McpInput
  | MonitorInput
  | NotebookEditInput
  | ProjectsInput
  | PushNotificationInput
  | ReadMcpResourceDirInput
  | ReadMcpResourceInput
  | RefreshMcpToolsInput
  | RemoteTriggerInput
  | ReportFindingsInput
  | ScheduleWakeupInput
  | ShowOnboardingRolePickerInput
  | TaskCreateInput
  | TaskGetInput
  | TaskListInput
  | TaskStopInput
  | TaskUpdateInput
  | TodoWriteInput
  | WebFetchInput
  | WebSearchInput
  | WorkflowInput;
```

<h3 id="agent">
  Agent
</h3>

**Nome da ferramenta:** `Agent`. O nome anterior `Task` ainda é aceito como um alias, e o array `tools` na mensagem de inicialização [`SDKSystemMessage`](#sdksystemmessage) atualmente lista essa ferramenta como `Task` para compatibilidade com versões anteriores.

<Note>
  O campo `mode` está descontinuado e ignorado no Claude Code v2.1.212 ou posterior. Um subagente é executado no modo de permissão da sessão pai ou na [`permissionMode`](#agentdefinition) de sua definição, e as [regras de herança de subagente](/docs/pt/agent-sdk/permissions#available-modes) decidem qual.
</Note>

```typescript theme={null}
type AgentInput = {
  description: string;
  prompt: string;
  subagent_type?: string;
  model?: "sonnet" | "opus" | "haiku" | "fable";
  run_in_background?: boolean;
  name?: string;
  team_name?: string; // Descontinuado; ignorado
  mode?: "acceptEdits" | "auto" | "bypassPermissions" | "default" | "dontAsk" | "plan"; // Descontinuado; ignorado. As regras de herança de subagente decidem o modo de permissão de um subagente
  isolation?: "worktree" | "remote";
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
  answers?: Record<string, string>;
  annotations?: Record<string, { preview?: string; notes?: string }>;
  metadata?: { source?: string };
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

Executa comandos Bash com timeout opcional e execução em background. O diretório de trabalho persiste entre comandos, incluindo comandos executados em turnos posteriores de uma sessão multi-turno; o estado do shell, como variáveis de ambiente exportadas, não persiste. Para os limites sobre quais mudanças de diretório são mantidas, veja [O que persiste entre comandos](/docs/pt/tools-reference#what-persists-between-commands).

<h3 id="monitor">
  Monitor
</h3>

**Nome da ferramenta:** `Monitor`

```typescript theme={null}
type MonitorInput = {
  description: string;
  timeout_ms: number;
  command?: string;
  ws?: {
    url: string;
    protocols?: string[];
  };
};
```

Executa uma fonte de background e entrega cada evento para Claude para que possa reagir sem polling: `command` executa um script e emite um evento por linha stdout, e `ws` abre um WebSocket e emite um evento por frame de texto. Forneça exatamente um de `command` ou `ws`. A fonte `ws` requer Claude Code v2.1.195 ou posterior.

`timeout_ms` é o prazo do watch em milissegundos. O padrão é 300000 e aceita valores até 3600000. O prazo efetivo é no máximo 1800000, que é 30 minutos, então um valor aceito maior é encurtado para isso. No prazo, o watch termina e Claude recebe um aviso para que possa iniciar um novo watch se ainda precisar de um.

O tipo exportado marca `timeout_ms` como obrigatório porque o esquema preenche o padrão; uma chamada que o omite valida.

Quando Monitor executa um comando, ele segue as mesmas regras de permissão que Bash; um watch de WebSocket solicita aprovação separadamente. Veja a [referência da ferramenta Monitor](/docs/pt/tools-reference#monitor-tool) para comportamento e disponibilidade de provedor.

<h3 id="taskoutput">
  TaskOutput
</h3>

Removido no Claude Code v2.1.277, junto com seu tipo `TaskOutputInput`. Anteriormente recuperava saída de uma tarefa de background em execução ou concluída; Claude lê o arquivo de saída de uma tarefa de background com `Read` em vez disso.

Uma entrada `disallowedTools` ou uma regra de negação que ainda nomeia `TaskOutput` é ignorada sem um aviso.

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

Para um PDF, Claude recebe o conteúdo do arquivo dentro do `tool_result` da chamada Read. Uma leitura que retorna a saída `pdf` [output](#tool-output-types) carrega um bloco `text` de resumo seguido por um bloco `document`. Uma que retorna a saída `parts` carrega o bloco `text` de resumo seguido por um bloco por página extraída: um bloco `image`, ou um bloco `text` nomeando a página quando Claude Code não conseguiu renderizá-la como uma imagem. Antes do Agent SDK v0.3.242, Claude Code entregava o conteúdo do arquivo como uma mensagem `user` separada após o resultado da ferramenta.

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
  "-o"?: boolean; // print only the matched parts of each line; requires output_mode: "content"
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
  args?: unknown; // any JSON value; the published typings render this as an object map
  resumeFromRunId?: string;
  title?: string; // ignored; the script's meta block sets the title
  description?: string; // ignored; the script's meta block sets the description
};
```

Executa um [workflow dinâmico](/docs/pt/workflows): um script que orquestra muitos subagentes em background e retorna um resultado consolidado. A ferramenta `Workflow` está disponível no Agent SDK v0.3.149 e posterior. Pelo menos um de `script`, `name` ou `scriptPath` é obrigatório.

| Campo             | Tipo      | Descrição                                                                                                                                                                                                                                                                                                                               |
| ----------------- | --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `script`          | `string`  | Script de workflow inline. Deve começar com `export const meta = { name, description }` como um literal, seguido pelo corpo do script usando `agent()`, `parallel()`, `pipeline()` e `phase()`. Um array `phases` opcional em `meta` agrupa agentes sob estágios nomeados na visualização de progresso                                  |
| `name`            | `string`  | Nome de um workflow integrado ou um salvo em `.claude/workflows/`. Resolvido para um script                                                                                                                                                                                                                                             |
| `scriptPath`      | `string`  | Caminho para um arquivo de script de workflow no disco. Tem precedência sobre `script` e `name`. Claude Code persiste cada invocação do script e retorna o caminho no resultado, para que você possa editar esse arquivo e reinvocar com o mesmo `scriptPath` para iterar                                                               |
| `args`            | `unknown` | Valor de entrada exposto ao script como o `args` global, para workflows nomeados parametrizados, como uma pergunta de pesquisa ou uma lista de caminhos de arquivo. Passe arrays e objetos como valores JSON reais, não como uma string codificada em JSON                                                                              |
| `resumeFromRunId` | `string`  | ID de execução de uma invocação anterior de `Workflow` para retomar. Chamadas `agent()` concluídas com entradas inalteradas geralmente retornam resultados em cache; o resto é executado ao vivo. [Retomar após uma pausa](/docs/pt/workflows#resume-after-a-pause) cobre quais chamadas concluídas são re-executadas. Apenas a mesma sessão |
| `title`           | `string`  | Ignorado; o bloco `meta` do script define o título                                                                                                                                                                                                                                                                                      |
| `description`     | `string`  | Ignorado; o bloco `meta` do script define a descrição                                                                                                                                                                                                                                                                                   |

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
  [k: string]: unknown;
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

<h3 id="exitworktree">
  ExitWorktree
</h3>

**Nome da ferramenta:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeInput = {
  action: "keep" | "remove";
  discard_changes?: boolean;
};
```

Sai do worktree git atual e retorna ao diretório de trabalho original. A ação `keep` deixa o worktree e a branch no disco, enquanto `remove` deleta ambos. `discard_changes` deve ser `true` ao remover um worktree que tem arquivos não confirmados ou commits não mesclados.

<h3 id="enterplanmode">
  EnterPlanMode
</h3>

**Nome da ferramenta:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeInput = {};
```

Entra no modo de planejamento, onde Claude pesquisa e apresenta um plano antes de fazer alterações.

<h3 id="croncreate">
  CronCreate
</h3>

**Nome da ferramenta:** `CronCreate`

```typescript theme={null}
type CronCreateInput = {
  cron: string;
  prompt: string;
  recurring?: boolean;
  durable?: boolean;
};
```

Agenda um prompt para ser executado em um cronograma cron de 5 campos em hora local. Defina `recurring` para `false` para disparar uma vez na próxima correspondência. Os trabalhos são escopo de sessão por padrão: iniciar uma conversa nova os limpa, e retomar com `--resume` ou `--continue` restaura trabalhos que não expiraram. Veja [Tarefas agendadas](/docs/pt/scheduled-tasks).

Definir `durable` para `true` solicita persistência para `.claude/scheduled_tasks.json` para que o trabalho sobreviva a reinicializações. O agendamento durável não está disponível em todas as sessões: quando não está, Claude Code aceita `durable: true` mas cria o trabalho apenas para a sessão. Leia o campo `durable` da saída para ver se o trabalho persistiu.

<h3 id="crondelete">
  CronDelete
</h3>

**Nome da ferramenta:** `CronDelete`

```typescript theme={null}
type CronDeleteInput = {
  id: string;
};
```

Deleta um trabalho cron agendado pelo ID retornado de `CronCreate`.

<h3 id="cronlist">
  CronList
</h3>

**Nome da ferramenta:** `CronList`

```typescript theme={null}
type CronListInput = {};
```

Lista os trabalhos cron agendados: trabalhos duráveis de `.claude/scheduled_tasks.json` e trabalhos apenas de sessão da sessão atual.

<h3 id="schedulewakeup">
  ScheduleWakeup
</h3>

**Nome da ferramenta:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupInput = {
  delaySeconds?: number;
  reason?: string;
  prompt?: string;
  noop?: boolean;
  stop?: boolean;
};
```

Agenda um wake-up único que dispara o prompt fornecido após um atraso. Esta ferramenta respalda o comando `/loop` auto-paced. O runtime fixa `delaySeconds` entre 60 e 3600 segundos. Os campos `delaySeconds`, `reason`, `prompt` e `noop` são obrigatórios a menos que `stop` seja true. `noop: true` relata um wake-up onde nada mudou. Definir `stop: true` cancela o wakeup pendente e encerra o `/loop` auto-paced. O campo `stop` requer Claude Code v2.1.202 ou posterior. Veja a [linha ScheduleWakeup na referência de ferramentas](/docs/pt/tools-reference).

<h3 id="remotetrigger">
  RemoteTrigger
</h3>

**Nome da ferramenta:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerInput = {
  action:
    | "list"
    | "get"
    | "create"
    | "update"
    | "run"
    | "create_webhook_trigger"
    | "list_runs"
    | "get_run_log";
  trigger_id?: string;
  session_id?: string;
  cursor?: string;
  body?: {
    [k: string]: unknown;
  };
};
```

Gerencia [Rotinas](/docs/pt/routines), as execuções do Claude Code agendadas e acionadas hospedadas na nuvem. Esta ferramenta respalda o comando `/schedule`. `trigger_id` é obrigatório para as ações `get`, `update`, `run` e `list_runs`. `body` é obrigatório para `create`, `update` e `create_webhook_trigger`, e opcional para `run`.

`create_webhook_trigger` anexa uma fonte de evento a uma rotina existente, como um [evento GitHub](/docs/pt/routines#add-a-github-trigger) que a dispara. O `body` nomeia a fonte, os eventos e a rotina a disparar. Requer Claude Code v2.1.225 ou posterior.

`list_runs` lista as execuções recentes de uma rotina, e `get_run_log` lê o log de uma execução. `session_id` nomeia a execução a ler, de um resultado `list_runs`, e `cursor` pagina através dos resultados de qualquer ação. Ambas as ações requerem Claude Code v2.1.227 ou posterior.

Esta ferramenta está disponível apenas quando a sessão é autenticada com uma conta claude.ai em um plano com Rotinas habilitadas, e está ausente quando a política da sua organização desabilita [Claude Code na web](/docs/pt/claude-code-on-the-web). No Claude Code v2.1.227 ou posterior, a ferramenta também está ausente quando um Proprietário [desativou rotinas para a organização](/docs/pt/routines#routines-are-disabled-by-your-organizations-policy). Antes de v2.1.227, uma sessão com apenas o toggle de rotinas desativado ainda mostrava a ferramenta, e o servidor negava suas chamadas.

<h3 id="pushnotification">
  PushNotification
</h3>

**Nome da ferramenta:** `PushNotification`

```typescript theme={null}
type PushNotificationInput = {
  message: string;
  status: "proactive";
};
```

Envia uma notificação push proativa para o usuário. Mantenha `message` com menos de 200 caracteres porque os sistemas operacionais móveis truncam texto mais longo. Veja a [linha PushNotification na referência de ferramentas](/docs/pt/tools-reference) para disponibilidade de provedor; a entrega de push é executada através de infraestrutura hospedada pela Anthropic que não é acessível do Amazon Bedrock, Claude Platform no AWS, Agent Platform do Google Cloud ou Microsoft Foundry.

<h3 id="repl">
  REPL
</h3>

Removido em v2.1.275. Através de v2.1.274, uma ferramenta `REPL` experimental poderia ser ativada com `CLAUDE_CODE_REPL=1` na [opção `env`](#options).

<h3 id="reportfindings">
  ReportFindings
</h3>

**Nome da ferramenta:** `ReportFindings`

```typescript theme={null}
type ReportFindingsInput = {
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Relata descobertas de revisão de código como uma lista estruturada para que Claude Code possa renderizá-las em vez de imprimi-las como texto. `level` é o nível de esforço em que a revisão foi executada. As descobertas são ordenadas mais graves primeiro, com no máximo 32 por chamada, e o array está vazio quando nenhuma sobreviveu. Requer Claude Code v2.1.196 ou posterior.

Cada descoberta carrega esses campos:

* `file`: caminho relativo ao repositório em que a descoberta está. O `line` opcional é a linha 1-indexada à qual ela se ancora.
* `summary`: declaração de uma sentença do defeito. `failure_scenario` descreve as entradas concretas e o estado que levam à saída errada ou crash.
* `short_summary`: rótulo comprimido opcional de no máximo 60 caracteres para exibição compacta. Requer Claude Code v2.1.212 ou posterior.
* `category`: slug kebab-case curto opcional do tipo de descoberta, como `correctness` ou `test-coverage`. Requer Claude Code v2.1.199 ou posterior.
* `verdict`: definido quando uma passagem de verificação foi executada; ausente em revisões apenas inline.
* `outcome`: definido apenas ao relatar novamente após aplicar correções.

<h3 id="artifact">
  Artifact
</h3>

**Nome da ferramenta:** `Artifact`

```typescript theme={null}
type ArtifactInput = {
  action?: "publish" | "list";
  file_path?: string;
  favicon?: string;
  icon?: string;
  limit?: number;
  scope?: "mine" | "shared" | "all";
  title?: string;
  description?: string;
  label?: string;
  url?: string;
  force?: boolean;
  capabilities?: Record<string, unknown>;
  contract?: "latest" | string;
};
```

Publica um arquivo `.html` ou `.md` local como uma página de artefato hospedada, ou lista os artefatos publicados do usuário. Omita `action` ou passe `"publish"` para publicar `file_path`, que é obrigatório para a ação de publicação. Cada campo abaixo se aplica a uma publicação:

* `icon`: uma palavra genérica curta para o ícone da aba do navegador do artefato, como `chart` ou `map`. Claude o inclui em uma primeira publicação e o omite em uma atualização, o que mantém o ícone armazenado do artefato.
* `favicon`: descontinuado, e Claude o omite.
* `title`: nomeia a página publicada na aba do navegador e galeria quando o arquivo HTML não tem uma tag `<title>`.
* `url`: visa um artefato existente para atualizar no local em vez de criar um novo.

`force` é uma sobrescrita de último recurso que descarta uma versão mais nova que outra sessão publicou. Em um conflito, a publicação falhada retorna o conteúdo mais novo; Claude mescla suas alterações nesse conteúdo, ou relê o artefato, e publica novamente. Passe `force` apenas quando o usuário explicitamente pedir para descartar essa versão.

Passe `"list"` para enumerar os artefatos publicados do usuário; apenas `limit` e `scope` podem acompanhá-lo. `scope` padrão é `"mine"`, que lista artefatos que o usuário possui; `"shared"` lista artefatos que outras pessoas compartilharam com o usuário, e `"all"` lista ambos.

* `capabilities`: as capacidades de tempo de execução que a página publicada usa, codificadas por nome de capacidade, como os [conectores que a página pode chamar](/docs/pt/artifacts#pull-live-data-with-mcp-connectors). O serviço de artefato valida a declaração e rejeita uma publicação que nomeia uma capacidade que a conta não pode usar ou dá uma configuração inválida. Passe `{}` para limpar uma declaração armazenada, e omita o campo em uma reimplantação para mantê-lo. Requer Agent SDK v0.3.235 ou posterior.
* `contract`: a versão de tempo de execução contra a qual a página publicada é executada. Omita-a para manter a versão atual do artefato, passe `"latest"` para atualizar, ou passe uma versão específica para fixar ou reverter. Requer Agent SDK v0.3.235 ou posterior.

Os tipos são exportados, mas a ferramenta está desativada por padrão em sessões do Agent SDK. A publicação também requer todas as condições na [tabela de disponibilidade de artefatos](/docs/pt/artifacts#availability), que sessões autenticadas com uma chave de API não atendem.

<h3 id="projects">
  Projects
</h3>

**Nome da ferramenta:** `Projects`

```typescript theme={null}
type ProjectsInput = {
  method:
    | "project_info"
    | "project_read"
    | "project_search"
    | "project_write"
    | "project_delete";
  path?: string;
  content?: string;
  local_path?: string;
  present_to_user?: boolean;
  query?: string;
  n?: number;
};
```

Lê e escreve o Projeto claude.ai anexado à sessão. Despacha em `method`:

* `project_info`: retorna metadados do projeto e a lista de documentos.
* `project_read`: lê um documento por `path`.
* `project_search`: consulta a base de conhecimento do projeto com `query`. `n` limita os acertos e padrão é 5.
* `project_write`: cria ou substitui um documento em `path` de exatamente um de `content`, que carrega texto inline, ou `local_path`, que nomeia um arquivo dentro do diretório de trabalho. `present_to_user: true` marca o documento escrito como o entregável que o usuário precisa ver.
* `project_delete`: deleta um documento por `path`.

<h3 id="readmcpresourcedir">
  ReadMcpResourceDir
</h3>

**Nome da ferramenta:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirInput = {
  server: string;
  uri: string;
};
```

Lista os filhos diretos de um recurso de diretório em um servidor MCP. Apenas utilizável contra um servidor que declarou suporte para listagem de diretório; a listagem não é recursiva. A listagem de diretório não está habilitada em todas as sessões: quando está desativada, a chamada retorna uma lista `resources` vazia e o campo `error` relata que a listagem de diretório não está habilitada.

<h3 id="refreshmcptools">
  RefreshMcpTools
</h3>

**Nome da ferramenta:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsInput = {
  server?: string; // refresh only this server; omit to refresh all connected servers
};
```

Re-consulta a lista de ferramentas de servidores MCP conectados e aplica quaisquer alterações. Os tipos são exportados, mas Claude Code registra a ferramenta apenas quando você define `CLAUDE_CODE_ENABLE_REFRESH_MCP_TOOLS=1` na [opção `env`](#options), e apenas em sessões com pelo menos um servidor MCP. Requer Claude Code v2.1.211 ou posterior.

<h3 id="showonboardingrolepicker">
  ShowOnboardingRolePicker
</h3>

**Nome da ferramenta:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerInput = {};
```

Renderiza uma linha de chip seletor de função clicável durante o onboarding do Cowork para que o usuário possa escolher sua função e obter um plugin correspondente instalado. Não leva argumentos; a lista de funções é definida pelo cliente. A chamada bloqueia até que o usuário responda.

<h3 id="mcpinput">
  McpInput
</h3>

**Nome da ferramenta:** nomes de ferramenta MCP dinâmicos da forma `mcp__<server>__<tool>`

```typescript theme={null}
type McpInput = {
  [k: string]: unknown;
};
```

Os argumentos da ferramenta MCP são um objeto aberto: cada servidor define seus próprios parâmetros, então o tipo não coloca restrições em nomes de campo ou valores. Consulte o esquema de ferramenta do servidor para os campos que uma ferramenta específica aceita.

<h2 id="tool-output-types">
  Tipos de Saída de Ferramenta
</h2>

Documentação de esquemas de saída para todas as ferramentas integradas do Claude Code. Esses tipos são exportados de `@anthropic-ai/claude-agent-sdk` e representam os dados de resposta reais retornados por cada ferramenta.

<h3 id="tooloutputschemas">
  `ToolOutputSchemas`
</h3>

União de tipos de saída de ferramenta exportados de `@anthropic-ai/claude-agent-sdk`; os membros incluem:

```typescript theme={null}
type ToolOutputSchemas =
  | AgentOutput
  | ArtifactOutput
  | AskUserQuestionOutput
  | BashOutput
  | CronCreateOutput
  | CronDeleteOutput
  | CronListOutput
  | EnterPlanModeOutput
  | EnterWorktreeOutput
  | ExitPlanModeOutput
  | ExitWorktreeOutput
  | FileEditOutput
  | FileReadOutput
  | FileWriteOutput
  | GlobOutput
  | GrepOutput
  | ListMcpResourcesOutput
  | McpOutput
  | MonitorOutput
  | NotebookEditOutput
  | ProjectsOutput
  | PushNotificationOutput
  | ReadMcpResourceDirOutput
  | ReadMcpResourceOutput
  | RefreshMcpToolsOutput
  | RemoteTriggerOutput
  | ReportFindingsOutput
  | ScheduleWakeupOutput
  | ShowOnboardingRolePickerOutput
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

**Nome da ferramenta:** `Agent`. O nome anterior `Task` ainda é aceito como um alias, e o array `tools` na mensagem de inicialização [`SDKSystemMessage`](#sdksystemmessage) atualmente lista essa ferramenta como `Task` para compatibilidade com versões anteriores.

```typescript theme={null}
type AgentOutput =
  | {
      status: "completed";
      agentId: string;
      agentType?: string;
      content: Array<{ type: "text"; text: string; citations?: unknown[] | null }>;
      resolvedModel?: string;
      modelsUsed?: string[];
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
        output_tokens_details?: {
          thinking_tokens?: number | null;
        } | null;
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
      modelsUsed?: string[];
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

Na variante `completed`, `resolvedModel` nomeia o modelo em que o subagente iniciou, que pode diferir do input `model` solicitado quando [`availableModels`](/docs/pt/model-config#restrict-model-selection) ou outra substituição se aplica. Este campo requer Claude Code v2.1.174 ou posterior. Em `async_launched`, nomeia o modelo em uso quando a tarefa passou para o background.

`modelsUsed` lista os modelos que o subagente usou, em ordem. O campo está presente apenas quando uma troca no meio da execução aconteceu, e um modelo aparece novamente quando a execução voltou para ele. Em `async_launched`, a lista cobre os modelos usados antes de passar para o background. Tanto `modelsUsed` quanto o comportamento de backgrounding de `resolvedModel` requerem Claude Code v2.1.212 ou posterior.

Se o Claude Code [manteve o worktree isolado do subagente](/docs/pt/worktrees#isolate-subagents-with-worktrees), `worktreePath` no resultado `completed` é onde encontrá-lo. `worktreeBranch` é seu branch, presente quando o Claude Code criou o worktree com git.

O Claude Code preenche `usage` e `totalTokens` a partir da solicitação final da API do subagente, não de toda a execução, então `usage.service_tier` é a string de nível de serviço que a API relatou nessa solicitação. Quando presente, `usage.output_tokens_details.thinking_tokens` é o número de tokens de saída dessa solicitação que eram tokens de pensamento. O campo `output_tokens_details` requer TypeScript SDK v0.3.228 ou posterior, que agrupa Claude Code v2.1.228.

`usage.output_tokens_details` corresponde a [`Usage.output_tokens_details`](#usage) em significado, escopo para essa solicitação final, mas cada nível dele é opcional aqui. Proteja tanto o objeto quanto o campo, por exemplo `usage.output_tokens_details?.thinking_tokens ?? 0`, em vez de lê-lo diretamente.

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
  annotations?: Record<string, { preview?: string; notes?: string }>;
  afkTimeoutMs?: number;
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
  timedOutAfterMs?: number;
  backgroundCwdHint?: string;
  backgroundEndsWithFinalResponse?: true;
  dangerouslyDisableSandbox?: boolean;
  returnCodeInterpretation?: string;
  noOutputExpected?: boolean;
  structuredContent?: unknown[];
  persistedOutputPath?: string;
  persistedOutputSize?: number;
  staleReadFileStateHint?: string;
  ghRateLimitHint?: string;
  gitOperation?: {
    commit?: { sha: string; kind: "committed" | "amended" | "cherry-picked"; branch?: string };
    push?: { branch: string };
    branch?: { ref: string; action: "merged" | "rebased" };
    pr?: {
      number: number;
      url?: string;
      action: "created" | "edited" | "merged" | "commented" | "closed" | "reopened" | "ready" | "draft" | "auto-merge-enabled" | "auto-merge-disabled";
    };
  };
};
```

Os campos `stdout`, `stderr` e `backgroundTaskId` carregam:

| Campo              | O que carrega                                                                                                             |
| ------------------ | ------------------------------------------------------------------------------------------------------------------------- |
| `stdout`           | A stdout e stderr do comando, mescladas em um fluxo intercalado                                                           |
| `stderr`           | Avisos que a própria ferramenta adiciona, como uma redefinição de diretório de trabalho do shell, não a stderr do comando |
| `backgroundTaskId` | Presente para comandos em background                                                                                      |

`timedOutAfterMs` é o tempo limite em milissegundos, definido quando o comando atingiu seu tempo limite e passou para o background em vez de começar lá explicitamente. `backgroundCwdHint` é definido quando o comando em background continha um builtin de mudança de diretório como `cd`, `pushd`, `popd` ou `chdir`, e observa que o diretório de trabalho da sessão não mudou. Ambos os campos requerem Claude Code v2.1.210 ou posterior.

Quando um subagente em execução em primeiro plano possui um comando em background, o Claude Code encerra o comando quando esse subagente fornece sua resposta final. O Claude Code define `backgroundEndsWithFinalResponse` como `true` em tais comandos e omite o campo quando o comando sobrevive ao turno, como comandos iniciados pela conversa principal ou por subagentes em background. O campo requer Claude Code v2.1.227 ou posterior.

O Claude Code define `gitOperation.commit.branch` para o branch nomeado na linha de resumo do commit do git e o omite para um commit feito em um HEAD desanexado. O campo requer Agent SDK v0.3.227 ou posterior. O Claude Code relata um comando `gh pr reopen` como a ação PR `reopened`, que requer Agent SDK v0.3.234 ou posterior.

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
  originalFile: string | null;
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
    repository?: string | null;
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
        /** True quando uma leitura de arquivo inteiro foi paginada automaticamente porque excedeu o limite de tokens (o conteúdo é uma primeira página parcial). */
        truncatedByTokenCap?: boolean;
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
      /** Número de página do documento da primeira página extraída; rotula as imagens de página no conteúdo tool_result. */
      firstPage?: number;
      /** Apenas em processo: os bytes da imagem de página são entregues como blocos de imagem no conteúdo tool_result e não são retidos no tool_use_result emitido, então essa chave está ausente lá. */
      pages?: {
        base64: string;
        mediaType: "image/jpeg" | "image/png" | "image/gif" | "image/webp";
        error?: string;
      }[];
    }
  | {
      type: "file_unchanged";
      file: {
        filePath: string;
      };
      /** Definido quando a dedup correspondeu a uma entrada com seed de inicialização (CLAUDE.md / memória aninhada) em vez de um resultado anterior de ferramenta Read. */
      source?: "seeded";
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
    repository?: string | null;
  };
  userModified?: boolean;
};
```

Retorna o resultado da escrita com informações de diff estruturado. O que `originalFile` e `structuredPatch` carregam depende da escrita:

* Para um arquivo recém-criado, `originalFile` é null e `structuredPatch` está vazio
* Em uma sobrescrita, `originalFile` carrega o conteúdo anterior, exceto quando esse conteúdo é maior que cerca de 10 MB: o Claude Code então pula o diff e retorna `originalFile` null e `structuredPatch` vazio
* `structuredPatch` também está vazio quando a escrita não mudou nada ou o diff expirou

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
  totalMatches?: number;
  countIsComplete?: boolean;
};
```

Retorna caminhos de arquivo correspondentes ao padrão glob, classificados por tempo de modificação.

`totalMatches` e `countIsComplete` requerem Claude Code v2.1.191 ou posterior. `totalMatches` relata o número de arquivos correspondentes antes da truncagem. Quando `countIsComplete` é false, `totalMatches` é um limite inferior porque a busca subjacente truncou sua própria saída.

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
  totalFiles?: number;
  totalLines?: number;
  appliedLimit?: number;
  appliedOffset?: number;
};
```

Retorna resultados de busca. A forma varia por `mode`: lista de arquivo, conteúdo com correspondências ou contagens de correspondência. Em modo `count`, `numFiles` e `numMatches` são totais sobre o conjunto de resultados completo, não a fatia paginada. Antes da v2.1.208, um `head_limit` ou `offset` que truncava as entradas listadas também truncava esses totais.

`totalFiles` requer Claude Code v2.1.208 ou posterior e relata o número total de resultados antes da paginação `head_limit` e `offset` em modo `files_with_matches`. `totalLines` requer Claude Code v2.1.210 ou posterior e relata o número total de linhas antes da paginação em modo `content`.

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
  old_source?: string;
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
  artifactRead?: {
    slug: string;
    ver?: string;
    seeded?: false;
  };
};
```

Retorna o conteúdo buscado com status HTTP e metadados.

`artifactRead` é o próprio registro do Claude Code de uma leitura de artefato, presente apenas quando Claude buscou um artefato que a sessão pode publicar. O Claude Code o lê novamente quando uma sessão é retomada para que uma publicação posterior seja construída na versão correta; seu código não precisa agir sobre isso. `slug` nomeia o artefato, `ver` é a versão que a leitura registrou e está ausente quando não registrou nenhuma, e `seeded: false` marca uma leitura cuja fonte completa não chegou ao Claude. O campo `seeded` requer Agent SDK v0.3.239 ou posterior.

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
  searchCount?: number;
};
```

Retorna resultados de busca da web.

<h3 id="workflow-2">
  Workflow
</h3>

**Nome da ferramenta:** `Workflow`

```typescript theme={null}
type WorkflowOutput = {
  status: "async_launched" | "remote_launched";
  taskId: string;
  taskType?: "local_workflow" | "remote_agent";
  workflowName?: string;
  runId?: string;
  summary?: string;
  transcriptDir?: string;
  scriptPath?: string;
  sessionUrl?: string; // definido quando o workflow foi lançado como uma sessão remota
  warning?: string;
  error?: string;
};
```

Retorna imediatamente após a ferramenta aceitar a invocação. O resultado final chega mais tarde como uma conclusão de tarefa. Verifique `error` antes de tratar a execução como iniciada: um script que falha sua verificação de sintaxe retorna `status: "async_launched"` com `error` definido e nunca é executado.

| Campo           | Tipo                                    | Descrição                                                                                                                                                                                                  |
| --------------- | --------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `status`        | `"async_launched" \| "remote_launched"` | A ferramenta aceitou a invocação. `"async_launched"` para execuções em processo, `"remote_launched"` para execuções despachadas para uma sessão remota em vez de executar em processo                      |
| `taskId`        | `string`                                | Identificador de tarefa em background para a execução                                                                                                                                                      |
| `taskType`      | `"local_workflow" \| "remote_agent"`    | Tipo de tarefa da tarefa em background registrada, correspondendo ao braço `status`                                                                                                                        |
| `workflowName`  | `string`                                | O `meta.name` do script de workflow                                                                                                                                                                        |
| `runId`         | `string`                                | Identificador de execução de workflow para passar como `resumeFromRunId` em uma invocação posterior. Ausente para execuções `remote_launched`, onde a URL da sessão em nuvem é o identificador de retomada |
| `summary`       | `string`                                | Descrição de uma linha do que o workflow faz                                                                                                                                                               |
| `transcriptDir` | `string`                                | Diretório onde transcrições de subagente são escritas durante a execução                                                                                                                                   |
| `scriptPath`    | `string`                                | Caminho para o script de workflow persistido para esta execução. Edite-o e passe de volta como `scriptPath` para executar novamente sem reenviar o script                                                  |
| `sessionUrl`    | `string`                                | URL da sessão em nuvem, definida quando `status` é `"remote_launched"`                                                                                                                                     |
| `warning`       | `string`                                | Aviso não bloqueante, como estado git local divergindo do branch enviado que uma sessão em nuvem clonará                                                                                                   |
| `error`         | `string`                                | Definido quando o script falha sua verificação de sintaxe. Quando presente, a execução não foi iniciada apesar do status lançado                                                                           |

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
  planWasEdited?: boolean;
  awaitingLeaderApproval?: boolean;
  requestId?: string;
};
```

Retorna o estado do plano após sair do Plan Mode.

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
    blobSavedTo?: string;
  }>;
  error?: string;
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

Retorna informações sobre o git worktree.

<h3 id="exitworktree-2">
  ExitWorktree
</h3>

**Nome da ferramenta:** `ExitWorktree`

```typescript theme={null}
type ExitWorktreeOutput = {
  action: "keep" | "remove";
  originalCwd: string;
  worktreePath: string;
  worktreeBranch?: string;
  tmuxSessionName?: string;
  discardedFiles?: number;
  discardedCommits?: number;
  message: string;
};
```

Retorna a ação tomada e detalhes sobre o worktree que foi encerrado.

<h3 id="enterplanmode-2">
  EnterPlanMode
</h3>

**Nome da ferramenta:** `EnterPlanMode`

```typescript theme={null}
type EnterPlanModeOutput = {
  message: string;
};
```

Retorna uma confirmação de que o Plan Mode foi inserido.

<h3 id="croncreate-2">
  CronCreate
</h3>

**Nome da ferramenta:** `CronCreate`

```typescript theme={null}
type CronCreateOutput = {
  id: string;
  humanSchedule: string;
  recurring: boolean;
  durable?: boolean; // true quando persistido em .claude/scheduled_tasks.json; false quando apenas sessão
};
```

Retorna o ID do trabalho e uma descrição legível por humanos do cronograma.

<h3 id="crondelete-2">
  CronDelete
</h3>

**Nome da ferramenta:** `CronDelete`

```typescript theme={null}
type CronDeleteOutput = {
  id: string;
};
```

Retorna o ID do trabalho deletado.

<h3 id="cronlist-2">
  CronList
</h3>

**Nome da ferramenta:** `CronList`

```typescript theme={null}
type CronListOutput = {
  jobs: {
    id: string;
    cron: string;
    humanSchedule: string;
    prompt: string;
    recurring?: boolean;
    durable?: boolean;
  }[];
};
```

Retorna os trabalhos cron agendados: trabalhos duráveis de `.claude/scheduled_tasks.json` e trabalhos apenas de sessão da sessão atual. Um trabalho apenas de sessão carrega `durable: false`; trabalhos lidos do disco omitem o campo.

<h3 id="schedulewakeup-2">
  ScheduleWakeup
</h3>

**Nome da ferramenta:** `ScheduleWakeup`

```typescript theme={null}
type ScheduleWakeupOutput = {
  scheduledFor: number;
  clampedDelaySeconds: number;
  wasClamped: boolean;
  stopped?: boolean;
  cancelledWakeups?: number;
};
```

Retorna quando o wake-up será acionado como um timestamp de época em milissegundos, o atraso realmente usado e se o atraso solicitado foi fixado. O campo `stopped` é `true` quando a chamada encerrou o loop com `stop: true`. Requer Claude Code v2.1.202 ou posterior. O campo `cancelledWakeups` conta quantos wake-ups pendentes uma chamada `stop: true` cancelou. Um valor de 0 significa que nada estava pendente, e um cron `/loop` recorrente não é cancelado por `stop: true`. Requer Claude Code v2.1.206 ou posterior.

<h3 id="remotetrigger-2">
  RemoteTrigger
</h3>

**Nome da ferramenta:** `RemoteTrigger`

```typescript theme={null}
type RemoteTriggerOutput = {
  status: number;
  json: string;
  summary?: string;
};
```

Retorna o status de resposta da API e o corpo para a operação de acionamento.

<h3 id="pushnotification-2">
  PushNotification
</h3>

**Nome da ferramenta:** `PushNotification`

```typescript theme={null}
type PushNotificationOutput = {
  message: string;
  pushSent?: boolean;
  localSent?: boolean;
  disabledReason?: "config_off" | "user_present" | "no_transport";
  sentAt?: string;
};
```

Retorna detalhes de entrega, incluindo se uma notificação push ou local foi enviada e por que a entrega foi ignorada.

<h3 id="reportfindings-2">
  ReportFindings
</h3>

**Nome da ferramenta:** `ReportFindings`

```typescript theme={null}
type ReportFindingsOutput = {
  count: number;
  level?: "low" | "medium" | "high" | "xhigh" | "max";
  findings: Array<{
    file: string;
    line?: number;
    summary: string;
    failure_scenario: string;
    short_summary?: string;
    category?: string;
    verdict?: "CONFIRMED" | "PLAUSIBLE";
    outcome?: "fixed" | "skipped" | "no_change_needed";
  }>;
};
```

Retorna o número de descobertas relatadas, o nível de esforço em que a revisão foi executada e as descobertas ecoadas de volta para o corpo do resultado. Requer Claude Code v2.1.196 ou posterior. O campo `short_summary` ecoado requer Claude Code v2.1.212 ou posterior.

<h3 id="artifact-2">
  Artifact
</h3>

**Nome da ferramenta:** `Artifact`

```typescript theme={null}
type ArtifactOutput =
  | {
      url: string;
      path: string;
      title?: string;
      version?: string;
      capabilities?: unknown;
      stored?: {
        contract: string;
        capabilities?: Record<string, unknown>;
      };
      warnings?: string[];
      contract?: string;
      updated?: boolean;
      liveSubscription?: string;
    }
  | {
      artifacts: Array<{
        title: string;
        url: string;
        updatedAt?: string;
        rel?: "mine" | "shared";
      }>;
      truncated?: boolean;
      scope?: "shared" | "all";
    };
```

Retorna a `url` da página publicada e o `path` local que foi publicado para a ação de publicação, com `updated` definido como true quando a publicação reimplantou um artefato existente, e `warnings` carregando quaisquer avisos de tempo de publicação. A ação de lista retorna as linhas `artifacts` em vez disso, com `truncated` definido quando mais artefatos existem do que o limite solicitado. Em listagens cujo escopo não é `"mine"`, cada linha carrega `rel` marcando se o usuário possui o artefato ou foi compartilhado com ele, e a `scope` da saída registra qual escopo não padrão produziu a listagem; ambos estão ausentes em listagens padrão.

<h3 id="projects-2">
  Projects
</h3>

**Nome da ferramenta:** `Projects`

```typescript theme={null}
type ProjectsOutput =
  | {
      method: "project_info";
      notice?: string;
      name: string;
      description: string;
      instructions: string;
      docs: Array<{ path: string; created_at: string | null }>;
      files?: Array<{
        path: string;
        file_kind: string;
        created_at: string | null;
      }>;
      sync_sources?: Array<{
        type: string | null;
        config: Record<string, unknown>;
      }>;
      knowledge: {
        knowledge_size: number;
        max_knowledge_size: number;
      };
    }
  | {
      method: "project_read";
      notice?: string;
      path: string;
      file_kind?: string;
      content?: string;
      local_file?: string;
      created_at: string | null;
    }
  | {
      method: "project_search";
      notice?: string;
      rag: boolean;
      hits?: Array<{ name?: string; doc_uuid?: string; text?: string }>;
      docs?: string[];
    }
  | {
      method: "project_write";
      notice?: string;
      path: string;
      doc_uuid: string;
      replaced: boolean;
      present_to_user?: boolean;
      local_path?: string;
    }
  | {
      method: "project_delete";
      notice?: string;
      path: string;
      deleted: boolean;
    };
```

Discriminado no campo `method`, espelhando a entrada. `project_read` retorna pequenos documentos de texto inline em `content` e escreve documentos maiores em um caminho `local_file` em vez disso; `project_search` retorna `hits` RAG com `rag: true` quando o índice do projeto está disponível e volta para uma lista de caminho `docs` caso contrário.

<h3 id="readmcpresourcedir-2">
  ReadMcpResourceDir
</h3>

**Nome da ferramenta:** `ReadMcpResourceDirTool`

```typescript theme={null}
type ReadMcpResourceDirOutput = {
  resources: Array<{
    uri: string;
    name: string;
    mimeType?: string;
  }>;
  error?: string;
};
```

Retorna os filhos diretos do recurso de diretório. Subdiretórios aparecem com mimeType `"inode/directory"`; `error` carrega uma mensagem legível por humanos quando o servidor não conseguiu listar o diretório.

<h3 id="refreshmcptools-2">
  RefreshMcpTools
</h3>

**Nome da ferramenta:** `RefreshMcpTools`

```typescript theme={null}
type RefreshMcpToolsOutput = Array<{
  server: string;
  status: "refreshed" | "error" | "not_connected";
  toolCount?: number; // ferramentas agora disponíveis deste servidor
  added?: string[]; // nomes de ferramentas que esta atualização adicionou
  removed?: string[]; // nomes de ferramentas que esta atualização removeu
  error?: string; // por que a atualização falhou ou o servidor estava indisponível
}>;
```

Retorna uma entrada por servidor: `refreshed` significa que a lista de ferramentas re-consultada foi aplicada, `error` significa que a re-consulta falhou e o conjunto de ferramentas anterior foi mantido, e `not_connected` significa que o servidor não tem conexão ativa para consultar.

<h3 id="showonboardingrolepicker-2">
  ShowOnboardingRolePicker
</h3>

**Nome da ferramenta:** `ShowOnboardingRolePicker`

```typescript theme={null}
type ShowOnboardingRolePickerOutput = {
  role?: string;
  dismissed?: boolean;
};
```

Retorna a seleção do usuário: `role` quando ele escolheu um chip de função ou digitou um, e `dismissed: true` quando fechou o seletor. Um objeto vazio significa que o usuário aprovou a chamada sem escolher uma função.

<h3 id="mcpoutput">
  McpOutput
</h3>

**Nome da ferramenta:** nomes de ferramentas MCP dinâmicas da forma `mcp__<server>__<tool>`

```typescript theme={null}
type McpOutput =
  | string
  | {
      type: string;
      [k: string]: unknown;
    }[]
  | {
      [k: string]: unknown;
    };
```

Os resultados das ferramentas MCP são retornados como uma string ou um array de blocos de conteúdo, dependendo do servidor. O ramo de objeto simples à direita no tipo exportado é um artefato de geração de esquema: o SDK não retorna um objeto simples, porque uma saída estruturada de um servidor é serializada para uma string JSON antes de ser retornada. Em tempo de execução, o valor também pode ser `undefined`, embora o tipo exportado não modele isso.

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

De onde a chave de API para as requisições da sessão veio, relatada como `apiKeySource` na mensagem init [`SDKSystemMessage`](#sdksystemmessage).

```typescript theme={null}
type ApiKeySource =
  | "ANTHROPIC_API_KEY"
  | "apiKeyHelper"
  | "/login managed key"
  | "none"
  | "user"
  | "project"
  | "org"
  | "temporary"
  | "oauth";
```

Claude Code relata um de quatro valores:

| Valor                | Chave em uso                                                                                                                             |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `ANTHROPIC_API_KEY`  | A chave na variável de ambiente `ANTHROPIC_API_KEY`                                                                                      |
| `apiKeyHelper`       | A chave retornada pelo seu comando [`apiKeyHelper`](/docs/pt/settings-reference#apikeyhelper)                                                 |
| `/login managed key` | A chave que Claude Code armazenou quando você fez login com uma [conta Claude Console](/docs/pt/authentication#claude-console-authentication) |
| `none`               | Nenhuma chave de API. A sessão se autentica de outra forma, como um login claude.ai, um token bearer ou um provedor de nuvem             |

Agent SDK v0.3.234 e posterior listam esses quatro valores no tipo. O tipo também mantém `user`, `project`, `org`, `temporary` e `oauth` para que código mais antigo ainda compile, e Claude Code não os relata.

<h3 id="sdkbeta">
  `SdkBeta`
</h3>

Recursos beta disponíveis que podem ser ativados via opção `betas`. Veja [Beta headers](https://platform.claude.com/docs/en/api/beta-headers) para mais informações.

```typescript theme={null}
type SdkBeta = "context-1m-2025-08-07";
```

<Warning>
  O beta `context-1m-2025-08-07` foi descontinuado a partir de 30 de abril de 2026. Passar este valor com Claude Sonnet 4.5 ou Sonnet 4 não tem efeito, e requisições que excedem a janela de contexto padrão de 200k-token retornam um erro. Para usar uma janela de contexto de 1M-token, migre para [Claude Opus 5.5, Claude Opus 5, Claude Sonnet 5, Claude Sonnet 4.6, Claude Opus 4.6, Claude Opus 4.7 ou Claude Opus 4.8](https://platform.claude.com/docs/en/about-claude/models/overview), que incluem contexto de 1M a preço padrão sem header beta necessário.
</Warning>

<h3 id="slashcommand">
  `SlashCommand`
</h3>

Informações sobre um comando disponível.

```typescript theme={null}
type SlashCommand = {
  name: string;
  description: string;
  argumentHint: string;
  aliases?: string[];
  builtin?: boolean;
};
```

`builtin` é `true` em uma linha quando o comando é próprio do Claude Code e digitar `/name` o executa. Está ausente para um comando definido por um usuário, projeto, plugin ou servidor MCP, e para um comando agrupado que um desses [substitui por nome](/docs/pt/skills#resolve-skills-that-share-a-name). Requer Agent SDK v0.3.277 ou posterior.

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

| Campo         | Tipo                  | Descrição                                                                                                                                                                                                      |
| :------------ | :-------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`        | `string`              | Identificador de tipo de agente (por exemplo, `"Explore"`, `"general-purpose"`)                                                                                                                                |
| `description` | `string`              | Descrição de quando usar este agente                                                                                                                                                                           |
| `model`       | `string \| undefined` | Modelo que este agente usa: um alias ou ID de modelo, ou `'inherit'` para o modelo do pai. Quando é `undefined`, Claude Code escolhe o modelo na [ordem de modelo de subagente](/docs/pt/sub-agents#choose-a-model) |

<h3 id="mcpserverprovenance">
  `McpServerProvenance`
</h3>

O servidor MCP que serve uma ferramenta `mcp__*`, e de onde a definição desse servidor veio. As entradas de hook [`PreToolUse`](#pretoolusehookinput), `PostToolUse`, `PostToolUseFailure`, `PermissionRequest` e `PermissionDenied` a carregam como `mcp_server`, e as opções [`CanUseTool`](#canusetool) a carregam como `mcpServer`. Ambas a omitem para ferramentas que não vêm de um servidor MCP.

```typescript theme={null}
type McpServerProvenance = {
  name: string;
  source: string;
};
```

| Campo    | Tipo     | Descrição                                                                                                            |
| :------- | :------- | :------------------------------------------------------------------------------------------------------------------- |
| `name`   | `string` | O nome sob o qual o servidor está registrado, o mesmo valor que [`mcpServerStatus()`](#query-object) relata para ele |
| `source` | `string` | De onde a definição do servidor veio: `sdk`, `plugin` ou um escopo de configuração                                   |

`source` assume um dos seguintes valores. O conjunto é aberto, então trate um valor que você não reconheça como uma fonte configurada, nunca como `sdk`:

* **`sdk`**: um servidor em processo que sua aplicação registrou. Apenas a aplicação host do SDK pode registrar um, então um servidor configurado nunca relata `sdk`, qualquer que seja seu nome.
* **`plugin`**: um servidor que um [plugin](/docs/pt/agent-sdk/plugins) fornece. Seu `name` é a forma `plugin:<plugin-name>:<server-name>` com escopo descrita em [servidores MCP fornecidos por plugin](/docs/pt/mcp#plugin-provided-mcp-servers).
* **Um escopo de configuração**: `user`, `project`, `local`, `dynamic`, `managed`, `enterprise`, `claudeai` ou `agent`. Um servidor `.mcp.json` relata `project`, e [escopos de instalação MCP](/docs/pt/mcp#mcp-installation-scopes) define `local`, `project` e `user`. Servidores que sua aplicação passa na opção [`mcpServers`](#options), outros que servidores SDK em processo, relatam `dynamic`.

Baseie decisões de confiança em `source`, não em `name` ou no prefixo de nome de ferramenta `mcp__<server>__`. Para qualquer fonte que não seja `sdk`, `name` é texto não confiável: escape-o antes de exibir.

`McpServerProvenance` e os campos que a carregam requerem Agent SDK v0.3.274 ou posterior.

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
  source?: string;
  tools?: {
    name: string;
    description?: string;
    annotations?: {
      readOnly?: boolean;
      destructive?: boolean;
      openWorld?: boolean;
    };
    _meta?: Record<string, unknown>;
  }[];
};
```

`source` diz de onde a definição do servidor veio, com os mesmos valores e regra de confiança que o `source` de [`McpServerProvenance`](#mcpserverprovenance). O campo requer Agent SDK v0.3.274 ou posterior e está ausente em versões anteriores.

`_meta` em uma entrada `tools` carrega os membros MCP Apps de `_meta` dessa ferramenta, para que sua aplicação possa encontrar o recurso `ui://` para renderizar com [`readMcpResource()`](#query-object). Claude Code passa através do objeto `ui` e da string `ui/resourceUri` plana descontinuada, e retém todas as outras chaves. Dentro de `ui`, `resourceUri` é uma string `ui://` e `visibility` um array de `"model"` e `"app"` quando o servidor os define, e qualquer outro membro passa através inalterado. Claude Code descarta qualquer chave quando o valor é malformado, e omite `_meta` de uma ferramenta que não declara nenhum. O campo está presente apenas quando o [`capabilities`](#sdksystemmessage) da mensagem init incluem `mcp_tool_ui_meta_v1`, e requer TypeScript Agent SDK v0.3.280 ou posterior.

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
  thinkingTokens?: number;
  cacheReadInputTokens: number;
  cacheCreationInputTokens: number;
  webSearchRequests: number;
  costUSD: number;
  contextWindow: number;
  maxOutputTokens: number;
  canonicalModel?: string;
  provider?: string;
  costBasis?: 'list' | 'managed' | 'unknown';
};
```

`thinkingTokens` conta os tokens de pensamento que este modelo gerou. `outputTokens` já os inclui, então não adicione os dois juntos. O campo está ausente até que uma volta seja executada em uma versão de Claude Code que o registra, então uma sessão retomada que começou em uma versão anterior relata uma contagem parcial. `thinkingTokens` requer Agent SDK v0.3.257 ou posterior.

Os campos `canonicalModel` e `provider` requerem Claude Code v2.1.218 ou posterior. `canonicalModel` é o ID de modelo canônico que a busca de preço usa; pode diferir da string de modelo bruto que chave a entrada, por exemplo quando essa string é um ID específico do provedor ou um alias.

`provider` nomeia o backend de API que serviu o modelo, como `firstParty`, `bedrock`, `vertex`, `foundry`, `anthropicAws`, `mantle` ou `gateway`.

`costBasis` nomeia a tabela de preço que precificou a requisição mais recente do modelo: `list` para preço de lista, `managed` para uma tabela [`modelPricing`](/docs/pt/settings-reference#modelpricing), ou `unknown` quando nenhuma correspondeu ao ID do modelo. O campo requer Claude Code v2.1.246 ou posterior.

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
  output_tokens_details: BetaOutputTokensDetails | null;
};
```

`BetaServerToolUsage`, `BetaIterationsUsage` e `BetaOutputTokensDetails` são definidos em `@anthropic-ai/sdk`.

`output_tokens_details` divide a saída faturada por categoria. Atualmente carrega um campo, `thinking_tokens: number`, contando os tokens de saída que o modelo gerou como raciocínio interno, incluindo os delimitadores de bloco de pensamento. O campo `output_tokens_details` requer TypeScript SDK v0.3.228 ou posterior, que agrupa Claude Code v2.1.228.

* **Faturamento**: leia a divisão para observabilidade, não para faturamento. `output_tokens` permanece o total autoritário, e `output_tokens - thinking_tokens` aproxima a saída não-raciocínio.
* **O que a contagem cobre**: o raciocínio bruto que o modelo produziu, que pode ser mais longo do que o texto de pensamento retornado no corpo da resposta. A API o computa re-tokenizando esse texto bruto, então pode diferir da contagem exata de geração do modelo por alguns tokens.
* **Streaming**: em mensagens de assistente transmitidas, essa divisão, como `output_tokens`, é um placeholder `message_start` e não carrega contagem real, então leia-a da mensagem de resultado `usage` como [Ler tokens de saída da mensagem de resultado](/docs/pt/agent-sdk/cost-tracking#read-output-tokens-from-the-result-message) descreve. Na mensagem de resultado, `thinking_tokens` lê `0` quando o modelo ou provedor não relata divisão.
* **Casos `null`**: `output_tokens_details` em si é `null` em mensagens de assistente que Claude Code sintetiza, como mensagens de erro de API.

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

<h3 id="sdkmcpresourcelink">
  `SDKMcpResourceLink`
</h3>

Um arquivo que uma ferramenta MCP retornou por referência. Claude Code constrói cada entrada a partir de um bloco `resource_link` no resultado da ferramenta e entrega a lista como `resourceLinks` em [`SDKUserMessage.tool_use_result`](#sdkusermessage), ou como `resource_links` em [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) quando a chamada terminou em background. Requer Agent SDK v0.3.257 ou posterior.

```typescript theme={null}
type SDKMcpResourceLink = {
  uri: string;
  name: string;
  title?: string;
  description?: string;
  mimeType?: string;
  size?: number;
  annotations?: Record<string, unknown>;
};
```

Claude Code descarta um bloco cujo `uri` ou `name` não é uma string, e omite um campo opcional cujo valor não é do tipo listado.

| Campo         | Tipo                                   | Descrição                                                      |
| :------------ | :------------------------------------- | :------------------------------------------------------------- |
| `uri`         | `string`                               | URI do recurso, conforme o servidor o retornou                 |
| `name`        | `string`                               | Nome que o servidor deu ao recurso                             |
| `title`       | `string \| undefined`                  | Título de exibição, quando o servidor definiu um               |
| `description` | `string \| undefined`                  | Descrição, quando o servidor definiu uma                       |
| `mimeType`    | `string \| undefined`                  | Tipo MIME, quando o servidor definiu um                        |
| `size`        | `number \| undefined`                  | Tamanho em bytes, quando o servidor definiu um                 |
| `annotations` | `Record<string, unknown> \| undefined` | Objeto de anotações MCP do bloco, quando o servidor definiu um |

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

O campo `display` opcional controla se o texto de pensamento é retornado `"summarized"` ou `"omitted"`. No Claude Opus 4.7 e posterior, o padrão da API é `"omitted"`, então defina `"summarized"` para receber conteúdo de pensamento em blocos `thinking`. Claude Code não envia `display` para Amazon Bedrock ou Google Cloud's Agent Platform, então nesses provedores Opus 4.7 e posterior retornam blocos `thinking` vazios mesmo quando você define `display` para `"summarized"`.

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

Quando você chama `setMcpServers()`, Claude Code aplica estas regras:

* **Servidores que a chamada não nomeia**: Claude Code mantém servidores fornecidos por plugin em execução. Requer Agent SDK v0.3.210 ou posterior.
* **Servidores que a chamada nomeia**: exceto para servidores integrados que a CLI iniciou na inicialização, Claude Code substitui um servidor em execução apenas quando sua configuração difere da que você passou.
* **Servidores integrados que a CLI iniciou na inicialização**: se a chamada nomear um, Claude Code descarta essa entrada e a relata em `errors`.

A promise é resolvida após novos servidores stdio, HTTP e SSE adicionados se conectarem ou falharem, então ferramentas de servidores que se conectaram estão disponíveis na próxima volta.

`added` lista os servidores que Claude Code adicionou ou substituiu, independentemente de terem se conectado. Um servidor que falhou ao se conectar aparece em `added` e `errors`, com o texto de falha em `errors` e uma linha `failed` em [`mcpServerStatus()`](#methods). Antes de Claude Code v2.1.257, um servidor cuja tentativa de conexão lançou uma exceção era relatado apenas em `errors`.

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
  skippedLinks?: number;
};
```

`skippedLinks` conta os caminhos rastreados que o rewind recusou restaurar ou deletar por segurança de link: um symlink, hard link ou outro arquivo não-regular no caminho rastreado, um diretório pai que não mais resolve para onde apontava quando o checkpoint foi tirado, ou um backup que não pôde ser lido com segurança. O campo requer Claude Code v2.1.216 ou posterior. Uma chamada de visualização com `rewindFiles(userMessageId, { dryRun: true })` nunca o define.

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

Notificação quando uma tarefa de background é concluída, falha ou é parada. Tarefas de background incluem comandos Bash `run_in_background`, watches [Monitor](#monitor) e subagentes de background. Para o campo `ambient`, veja [`SDKTaskStartedMessage`](#sdktaskstartedmessage), que o define e seu requisito de versão.

```typescript theme={null}
type SDKTaskNotificationMessage = {
  type: "system";
  subtype: "task_notification";
  task_id: string;
  tool_use_id?: string;
  status: "completed" | "failed" | "stopped";
  output_file: string;
  summary: string;
  ambient?: boolean;
  usage?: {
    total_tokens: number;
    tool_uses: number;
    duration_ms: number;
  };
  resource_links?: SDKMcpResourceLink[];
  uuid: UUID;
  session_id: string;
};
```

Quando Claude Code [move uma chamada de ferramenta MCP longa para background](/docs/pt/mcp#automatic-backgrounding-of-long-tool-calls), o bloco `tool_result` para essa chamada contém apenas um placeholder e o resultado real da chamada chega nesta notificação. Corresponda a notificação à chamada com `tool_use_id`. Em uma notificação `completed`, `resource_links` lista os arquivos que a ferramenta retornou por referência como entradas [`SDKMcpResourceLink`](#sdkmcpresourcelink), com os mesmos limites de 50 links e 64 KiB que [`tool_use_result.resourceLinks`](#sdkusermessage). Claude Code omite `resource_links` quando o resultado não tinha links e em notificações para tarefas que não são chamadas de ferramenta MCP. `resource_links` requer Agent SDK v0.3.257 ou posterior.

Claude Code prepara um aviso a cada notificação de tarefa que envia ao modelo, exceto entregas marcadas com a [subkind `scheduled-trigger`](#task-notification-subkinds), que carregam um enquadramento de tarefa atribuída. O aviso afirma que nenhuma entrada humana ocorreu, então o modelo não trata a notificação como uma instrução ou aprovação do usuário.

Para detectar uma volta de notificação de tarefa, verifique `origin.kind === "task-notification"` em [`SDKUserMessage`](#sdkusermessage) ou [`SDKResultMessage`](#sdkresultmessage) em vez de corresponder ao texto do aviso. Leia `subkind` do mesmo campo se precisar saber o que o levantou. Antes de v2.1.205, Claude Code deixava o aviso fora de notificações que chegavam enquanto a sessão estava ociosa.

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

Claude Code entrega esta mensagem, [`SDKHookProgressMessage`](#sdkhookprogressmessage) e [`SDKHookResponseMessage`](#sdkhookresponsemessage) para o fluxo de mensagens imediatamente, incluindo enquanto um hook `SessionStart` ou `Setup` ainda está em execução durante a inicialização da sessão. Claude Code v2.1.169 através de v2.1.203 entregou estas mensagens em um lote após um hook `SessionStart` ou `Setup` ser concluído; v2.1.204 restaurou a entrega ao vivo.

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
  heartbeat?: boolean;
  subagent_type?: string;
  subagent_retry?: {
    agent_id: string;
    attempt: number;
    max_retries: number;
    retry_delay_ms: number;
    error_status: number | null;
    error_category: string;
  };
  uuid: UUID;
  session_id: string;
};
```

Enquanto uma chamada de ferramenta é executada na conversa principal, Claude Code emite uma mensagem `tool_progress` a cada 30 segundos com `heartbeat: true`. Cada heartbeat carrega o nome da ferramenta e segundos decorridos, para que você possa distinguir uma chamada de longa duração de uma sessão travada. Claude Code não emite heartbeats para chamadas de ferramenta dentro de um subagente. O campo `heartbeat` requer Agent SDK v0.3.214 ou posterior. Antes de v2.1.257, Claude Code também não emitia heartbeats para uma chamada de ferramenta Agent em primeiro plano.

Em mensagens `tool_progress` para a ferramenta Agent que não são heartbeats, `subagent_type` nomeia o tipo de subagente em execução, como `general-purpose`. `subagent_retry` está presente enquanto esse subagente aguarda um backoff de erro de API, como um limite de taxa ou sobrecarga, com uma mensagem por tentativa de retry. Ambos os campos requerem Agent SDK v0.3.214 ou posterior.

Para renderizar um indicador de retry de `subagent_retry`:

* Rastreie o indicador por `parent_tool_use_id`, que é único por subagente. `tool_use_id` é compartilhado por subagentes paralelos de uma volta de assistente, então rastrear por ele deixaria a atualização de um subagente limpar o indicador de outro.
* Limpe o indicador quando um `tool_progress` posterior para o mesmo `parent_tool_use_id` chegar sem `subagent_retry` nem `heartbeat: true`, ou quando a mensagem de resultado da ferramenta chegar. Frames com `heartbeat: true` relatam apenas vivacidade, então mantenha o indicador quando um chegar. `attempt` pode exceder `max_retries` sob retry persistente, então não derive limpeza dos contadores.
* Trate `error_category` como um token para escolher seu próprio texto de mensagem, não como texto de exibição. Os valores são `rate_limit`, `overloaded`, `authentication_failed`, `server_error`, `cloud_credential_error` e `unknown`. Manipule um valor que você não reconheça da forma que manipula `unknown`, porque versões posteriores podem adicionar valores.

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

Emitido quando uma tarefa começa. O campo `task_type` é `"local_bash"` para comandos Bash e watches [Monitor](#monitor), `"local_agent"` para subagentes, ou `"remote_agent"`.

```typescript theme={null}
type SDKTaskStartedMessage = {
  type: "system";
  subtype: "task_started";
  task_id: string;
  tool_use_id?: string;
  description: string;
  task_type?: string;
  is_backgrounded?: boolean;
  spawn_depth?: number;
  ambient?: boolean;
  uuid: UUID;
  session_id: string;
};
```

`ambient` é `true` para tarefas que não fazem parte do trabalho da sessão, como tarefas que Claude Code executa para sua própria operação. Watchers de atualização ao vivo também são ambient, incluindo watchers que o usuário pediu. Exclua tarefas ambient de indicadores de atividade. O campo requer Agent SDK v0.3.247 ou posterior.

`ambient` também aparece em [`SDKTaskNotificationMessage`](#sdktasknotificationmessage) e em entradas [`SDKBackgroundTasksChangedMessage`](#sdkbackgroundtaskschangedmessage).

`is_backgrounded` e `spawn_depth` descrevem como Claude Code iniciou a tarefa. Ambos os campos requerem Agent SDK v0.3.238 ou posterior.

* `is_backgrounded`: Claude Code o define em tarefas `"local_agent"` e `"local_bash"`. `true` significa que a tarefa é executada em background. `false` significa que a tarefa é executada em primeiro plano, e a chamada de ferramenta que a iniciou permanece bloqueada até que a tarefa termine ou se mude para background.
* `spawn_depth`: Claude Code o define apenas em tarefas `"local_agent"`. Um subagente que a thread principal gerou tem profundidade `1`. Um subagente que um subagente de profundidade `1` gerou tem profundidade `2`, e assim por diante.

Um [subagente retomado](/docs/pt/agent-sdk/subagents#resume-subagents) sempre relata `is_backgrounded: true`, porque Claude Code executa cada subagente retomado em background. Quando uma tarefa em primeiro plano se move para background depois, Claude Code relata o novo valor `is_backgrounded` em uma mensagem [`task_updated`](#sdktaskupdatedmessage) em vez de enviar um segundo `task_started`.

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

Emitido sempre que o conjunto de tarefas de background ativas muda: uma tarefa inicia, é concluída, é eliminada, um agente em primeiro plano é colocado em background, ou o campo `description` ou `ambient` de uma tarefa muda.

O array `tasks` é o conjunto completo ativo. Substitua qualquer conjunto em cache por cada payload em vez de emparelhar eventos `task_started` e `task_notification`, para que a próxima mudança de associação corrija qualquer evento que você tenha perdido.

A ordenação relativa a esses eventos por tarefa é não especificada, então não correlacione os dois fluxos.

Nada é emitido na inicialização. Redefina para um conjunto vazio sempre que o processo CLI da sessão inicia ou reinicia e deixe a próxima mudança de associação repopulá-lo.

Quando você envia uma requisição de controle `initialize` repetida para uma sessão em execução, como com [`reinitialize()`](#query-object) após uma lacuna de transporte, Claude Code segue a resposta com um snapshot do conjunto ativo atual, mesmo quando está vazio. Um host reconectando portanto aprende o que está em execução sem esperar pela próxima mudança de associação. Antes de Agent SDK v0.3.239, Claude Code não enviava snapshot após um `initialize` repetido.

Requer Claude Code v2.1.203 ou posterior.

```typescript theme={null}
type SDKBackgroundTasksChangedMessage = {
  type: "system";
  subtype: "background_tasks_changed";
  tasks: {
    task_id: string;
    task_type: string;
    description: string;
    ambient?: boolean;
  }[];
  uuid: UUID;
  session_id: string;
};
```

<h3 id="sdkthinkingtokensmessage">
  `SDKThinkingTokensMessage`
</h3>

Emitido enquanto Claude está produzindo um bloco de pensamento, incluindo um redatado. `estimated_tokens` é uma estimativa em execução dos tokens de pensamento gerados até agora no bloco atual, e `estimated_tokens_delta` é o incremento carregado por este frame. Use essas estimativas para exibição de progresso.

Quando o modelo ou provedor relata uma divisão, a contagem final para o loop de agente de nível superior é [`usage.output_tokens_details.thinking_tokens`](#usage) da mensagem de resultado, que [não inclui tokens de subagente](/docs/pt/agent-sdk/cost-tracking#get-the-total-cost-of-a-query).

Requer Claude Code v2.1.153 ou posterior.

```typescript theme={null}
type SDKThinkingTokensMessage = {
  type: "system";
  subtype: "thinking_tokens";
  estimated_tokens: number;
  estimated_tokens_delta: number;
  user_message_uuid?: string;
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

Claude Code não emite este tipo de mensagem. Quando você envia um comando como `/context` ou `/usage` como um prompt, sua saída chega como um [`SDKAssistantMessage`](#sdkassistantmessage).

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

Emitido quando o conjunto de comandos disponíveis muda durante a sessão, como quando Claude Code descobre skills conforme o agente entra em um subdiretório. O array `commands` é a lista completa atualizada, então substitua qualquer lista de comandos em cache por este payload. Chamar [`supportedCommands()`](#query-object) após esta mensagem retorna a mesma lista atualizada, porque o método rastreia o push mais recente; isso requer Agent SDK v0.3.216 ou posterior. Em versões anteriores do SDK, `supportedCommands()` retorna o snapshot capturado na inicialização e nunca reflete mudanças durante a sessão.

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

Emitido após uma volta quando [`promptSuggestions`](#options) está ativado e Claude Code gerou uma sugestão para essa volta. Contém o prompt de usuário previsto. Para as voltas que não recebem nenhuma, veja [Quando Claude Code pula sugestões](/docs/pt/interactive-mode#when-claude-code-skips-suggestions).

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

Emitido quando a conversa da sessão é substituída sem encerrar a sessão. Em uma chamada `query()`, apenas `/clear` e seus aliases produzem esta mensagem. Monte uma transcrição vazia sob `new_conversation_id` e descarte qualquer título de sessão em cache.

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

`AbortError` é a única classe de erro na API tipada do SDK. Outras falhas, como o processo Claude Code saindo ou falhando ao iniciar, rejeitam a iteração de mensagem com erros que não carregam nenhuma classe SDK para corresponder. [Troubleshooting](/docs/pt/agent-sdk/troubleshooting) chave esses erros por mensagem, com a causa e correção para cada.

<h2 id="sandbox-configuration">
  Configuração de Sandbox
</h2>

<h3 id="sandboxsettings">
  `SandboxSettings`
</h3>

Configuração para o comportamento do sandbox. Use isso para habilitar sandboxing de comandos e configurar restrições de rede programaticamente.

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

| Property                    | Type                                                  | Default     | Description                                                                                                                                                                                                                                                      |
| :-------------------------- | :---------------------------------------------------- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `enabled`                   | `boolean`                                             | `false`     | Habilitar modo sandbox para execução de comandos                                                                                                                                                                                                                 |
| `failIfUnavailable`         | `boolean`                                             | `true`      | Parar na inicialização se `enabled` for `true` mas o sandbox não conseguir iniciar. Defina como `false` para fazer fallback para execução sem sandbox com um aviso em stderr                                                                                     |
| `autoAllowBashIfSandboxed`  | `boolean`                                             | `true`      | Aprovar automaticamente comandos Bash quando o sandbox está habilitado                                                                                                                                                                                           |
| `excludedCommands`          | `string[]`                                            | `[]`        | Comandos que contornam restrições de sandbox, como `['docker *']`. Esses são executados sem sandbox automaticamente sem envolvimento do modelo; [`sandbox.excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands) cobre quando uma entrada se aplica |
| `allowUnsandboxedCommands`  | `boolean`                                             | `true`      | Permitir que o modelo solicite executar comandos fora do sandbox. Quando `true`, o modelo pode definir `dangerouslyDisableSandbox` na entrada da ferramenta, que faz fallback para o [sistema de permissões](#permissions-fallback-for-unsandboxed-commands)     |
| `network`                   | [`SandboxNetworkConfig`](#sandboxnetworkconfig)       | `undefined` | Configuração de sandbox específica de rede                                                                                                                                                                                                                       |
| `filesystem`                | [`SandboxFilesystemConfig`](#sandboxfilesystemconfig) | `undefined` | Configuração de sandbox específica do sistema de arquivos para restrições de leitura/escrita                                                                                                                                                                     |
| `ignoreViolations`          | `Record<string, string[]>`                            | `undefined` | Mapa de substrings de comando, ou `*` para cada comando, para substrings do texto de violação a ignorar, como `{ "*": ['/etc/hosts'] }`; veja [`sandbox.ignoreViolations`](/docs/pt/settings-reference#sandbox-ignoreviolations)                                      |
| `enableWeakerNestedSandbox` | `boolean`                                             | `false`     | Habilitar um sandbox aninhado mais fraco para compatibilidade                                                                                                                                                                                                    |
| `ripgrep`                   | `{ command: string; args?: string[] }`                | `undefined` | Configuração de binário ripgrep personalizado para ambientes sandbox                                                                                                                                                                                             |

<Note>
  O sandbox depende do suporte da plataforma e, no Linux, de ferramentas como `bubblewrap` e `socat`. Quando `enabled` é `true` e o sandbox não consegue iniciar, `query()` relata uma mensagem `result` com `subtype: "error_during_execution"` e o motivo em `errors`. Para uma única chamada de mensagem `query()`, o SDK lança uma exceção após gerar esse resultado de erro, então envolva o loop em um bloco try para continuar além dele. Veja [Handle the result](/docs/pt/agent-sdk/agent-loop#handle-the-result) para o contrato de erro.

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
  // A single-shot query() throws after yielding an error result,
  // such as when the sandbox can't start (failIfUnavailable defaults to true).
  console.log(`Session ended with an error: ${error}`);
}
```

<Warning>
  **Segurança de socket Unix:** A opção `allowUnixSockets` pode conceder acesso a serviços do sistema que alcançam fora do sandbox. Por exemplo, permitir `/var/run/docker.sock` efetivamente concede acesso completo ao sistema host através da API Docker, contornando o isolamento do sandbox. Permita apenas sockets Unix que são estritamente necessários e compreenda as implicações de segurança de cada um.
</Warning>

<h3 id="sandboxnetworkconfig">
  `SandboxNetworkConfig`
</h3>

Configuração específica de rede para modo sandbox. Essas configurações se aplicam a comandos Bash em sandbox quando `enabled` é `true` na [`SandboxSettings`](#sandboxsettings) pai. Elas não restringem a ferramenta WebFetch, que usa [regras de permissão](/docs/pt/permissions#webfetch) em vez disso.

```typescript theme={null}
type SandboxNetworkConfig = {
  allowedDomains?: string[];
  deniedDomains?: string[];
  strictAllowlist?: boolean;
  allowManagedDomainsOnly?: boolean;
  allowLocalBinding?: boolean;
  allowUnixSockets?: string[];
  allowAllUnixSockets?: boolean;
  httpProxyPort?: number;
  socksProxyPort?: number;
};
```

| Property                  | Type       | Default     | Description                                                                                                                                                                                                                                                                                                                                                                                                                   |
| :------------------------ | :--------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowedDomains`          | `string[]` | `[]`        | Nomes de domínio que processos em sandbox podem acessar                                                                                                                                                                                                                                                                                                                                                                       |
| `deniedDomains`           | `string[]` | `[]`        | Nomes de domínio que processos em sandbox não podem acessar. Tem precedência sobre `allowedDomains`                                                                                                                                                                                                                                                                                                                           |
| `strictAllowlist`         | `boolean`  | `false`     | Negar acesso de comandos em sandbox a hosts fora da [lista de permissões de rede](/docs/pt/sandboxing#network-isolation) em vez de solicitar. Aplicado apenas para comandos em sandbox; ferramentas em processo como WebFetch não são controladas por isso. Apenas honrado a partir de configurações de usuário, gerenciadas ou CLI `--settings`; configurações de projeto são ignoradas. Requer Claude Code v2.1.219 ou posterior |
| `allowManagedDomainsOnly` | `boolean`  | `false`     | Apenas configurações gerenciadas. Quando definido em [configurações gerenciadas](/docs/pt/managed-settings), apenas entradas `allowedDomains` e regras de permissão `WebFetch(domain:...)` de configurações gerenciadas são honradas, e entradas de permissão de configurações de usuário, projeto ou local são ignoradas. Não tem efeito quando definido via opções SDK                                                           |
| `allowLocalBinding`       | `boolean`  | `false`     | Permitir que processos se vinculem a portas locais (por exemplo, para servidores de desenvolvimento)                                                                                                                                                                                                                                                                                                                          |
| `allowUnixSockets`        | `string[]` | `[]`        | Caminhos de socket Unix que processos podem acessar (por exemplo, socket Docker)                                                                                                                                                                                                                                                                                                                                              |
| `allowAllUnixSockets`     | `boolean`  | `false`     | Permitir acesso a todos os sockets Unix                                                                                                                                                                                                                                                                                                                                                                                       |
| `httpProxyPort`           | `number`   | `undefined` | Porta de proxy HTTP para requisições de rede                                                                                                                                                                                                                                                                                                                                                                                  |
| `socksProxyPort`          | `number`   | `undefined` | Porta de proxy SOCKS para requisições de rede                                                                                                                                                                                                                                                                                                                                                                                 |

<Note>
  O proxy de sandbox integrado aplica `allowedDomains` com base no nome de host solicitado e não encerra ou inspeciona tráfego TLS, então técnicas como [domain fronting](https://en.wikipedia.org/wiki/Domain_fronting) podem potencialmente contorná-lo. Veja [Limitações de segurança do sandboxing](/docs/pt/sandboxing#security-limitations) para detalhes e [Implantação segura](/docs/pt/agent-sdk/secure-deployment#traffic-forwarding) para configurar um proxy que encerra TLS.
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

| Property     | Type       | Default | Description                                                   |
| :----------- | :--------- | :------ | :------------------------------------------------------------ |
| `allowWrite` | `string[]` | `[]`    | Padrões de caminho de arquivo para permitir acesso de escrita |
| `denyWrite`  | `string[]` | `[]`    | Padrões de caminho de arquivo para negar acesso de escrita    |
| `denyRead`   | `string[]` | `[]`    | Padrões de caminho de arquivo para negar acesso de leitura    |

<h3 id="permissions-fallback-for-unsandboxed-commands">
  Fallback de Permissões para Comandos Sem Sandbox
</h3>

Quando `allowUnsandboxedCommands` está habilitado, o modelo pode solicitar executar comandos fora do sandbox definindo `dangerouslyDisableSandbox: true` na entrada da ferramenta. Essas solicitações fazem fallback para o sistema de permissões existente, significando que seu manipulador `canUseTool` é invocado, permitindo que você implemente lógica de autorização personalizada.

Suas entradas `excludedCommands` em vez disso contornam o sandbox com nenhum envolvimento do modelo; [`sandbox.excludedCommands`](/docs/pt/settings-reference#sandbox-excludedcommands) cobre quando uma entrada se aplica.

No exemplo abaixo, `isCommandAuthorized` representa uma verificação de autorização que você define.

```typescript theme={null}
import { query } from "@anthropic-ai/claude-agent-sdk";

for await (const message of query({
  prompt: "Deploy my application",
  options: {
    sandbox: {
      enabled: true,
      allowUnsandboxedCommands: true // Model can request unsandboxed execution
    },
    permissionMode: "default",
    canUseTool: async (tool, input) => {
      // Check if the model is requesting to bypass the sandbox
      if (tool === "Bash" && input.dangerouslyDisableSandbox) {
        // The model is requesting to run this command outside the sandbox
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

<Warning>
  Comandos em execução com `dangerouslyDisableSandbox: true` têm acesso completo ao sistema. Certifique-se de que seu manipulador `canUseTool` valida essas solicitações cuidadosamente.

  Se `permissionMode` for definido como `bypassPermissions` e `allowUnsandboxedCommands` estiver habilitado, o modelo pode executar autonomamente comandos fora do sandbox sem prompts de aprovação, exceto pelas [ações que nenhum modo aprova automaticamente](/docs/pt/permission-modes#actions-no-mode-auto-approves). Essa combinação efetivamente permite que o modelo escape do isolamento do sandbox silenciosamente.
</Warning>

<h2 id="see-also">
  Veja também
</h2>

* [Visão geral do SDK](/docs/pt/agent-sdk/overview) - Conceitos gerais do SDK
* [Referência do SDK Python](/docs/pt/agent-sdk/python) - Documentação do SDK Python
* [Referência da CLI](/docs/pt/cli-reference) - Interface de linha de comando
* [Fluxos de trabalho comuns](/docs/pt/common-workflows) - Guias passo a passo
