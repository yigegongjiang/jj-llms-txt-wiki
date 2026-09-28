> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Segurança e confiança de plugins

> Decida se confia em um plugin antes de instalá-lo, desde o que um plugin pode fazer em sua máquina até como revisar um e removê-lo.

Um plugin Claude Code que você instala pode executar código arbitrário em sua máquina com seus privilégios de usuário.

Você instala um plugin de um marketplace, que é o catálogo do qual Claude Code o busca. Alguns nomes de marketplace são [reservados para os próprios marketplaces da Anthropic](#marketplace-tiers), e todos os outros marketplaces são de terceiros. O nome de um marketplace informa quem publica o catálogo, não o que cada plugin nele faz, portanto [revise um plugin antes de instalá-lo](#review-a-plugin-before-you-install) de qualquer marketplace de onde ele venha.

Leia esta página se você está decidindo se deve instalar um plugin ou se você revisa ferramentas antes que sua equipe possa usá-las.

<Note>
  Estes casos são cobertos em outras páginas:

  * **Modelo de segurança próprio do Claude Code**: veja [Segurança](/docs/pt/security)
  * **Restringir ou exigir plugins para uma organização**: veja [Gerenciar plugins para sua organização](/docs/pt/plugins/org)
  * **Os plugins `security-guidance` ou `claude-security`**: esta página não é sobre eles. Veja [`security-guidance`](/docs/pt/security-guidance) e [`claude-security`](/docs/pt/claude-security)
</Note>

Comece com [o que um plugin pode fazer](#understand-what-a-plugin-can-do) e [quais marketplaces são da Anthropic](#marketplace-tiers), depois [revise o plugin antes de instalá-lo](#review-a-plugin-before-you-install).

<h2 id="understand-what-a-plugin-can-do">
  Entenda o que um plugin pode fazer
</h2>

Um plugin pode conter conteúdo que executa código em sua máquina com seus privilégios de usuário e conteúdo que entra no contexto do Claude como instruções, portanto [revise um plugin antes de instalá-lo](#review-a-plugin-before-you-install). Aqui está o que um plugin instalado pode fazer:

* **Hooks**: os [hooks](/docs/pt/hooks) de um plugin são executados como comandos shell em pontos do ciclo de vida do Claude Code, como antes ou depois de uma chamada de ferramenta.
* **Servidores MCP e LSP**: Claude Code se conecta aos [servidores MCP](/docs/pt/mcp) que um plugin habilitado declara e fornece ao Claude suas ferramentas. Um servidor MCP stdio é executado como um processo que Claude Code inicia em sua máquina. Claude Code também inicia os servidores de linguagem que o plugin declara.
* **Diretório `bin/`**: Claude Code adiciona o diretório `bin/` de cada plugin habilitado ao `PATH` do shell da ferramenta Bash, para que os comandos Bash do Claude possam executar qualquer executável lá.
* **Skills, comandos e agentes**: estes entram no contexto do Claude como instruções, portanto influenciam o que Claude faz com as ferramentas que já possui.
* **Atualizações**: quando a atualização automática está ativada para o marketplace do qual você instalou um plugin, Claude Code atualiza esse plugin em segundo plano, portanto os arquivos que você revisou podem mudar no disco. [Quando auto-update é executado](/docs/pt/plugins/loading#when-auto-update-runs) tem o cronograma. Para ativar ou desativar a atualização automática por marketplace, veja [Manter plugins atualizados](/docs/pt/plugins/install#keep-plugins-updated).

As [regras de permissão](/docs/pt/permissions) e [sandbox](/docs/pt/sandboxing) do Claude Code cobrem as chamadas de ferramenta que Claude faz, não o código que um plugin executa por si só:

* **Hooks e processos de servidor**: command hooks executam comandos shell com suas permissões completas de usuário. Claude Code executa hooks e servidores MCP fora da sandbox.
* **Chamadas de ferramenta do Claude**: uma chamada para uma das ferramentas MCP do plugin e um comando Bash que executa um executável do `bin/` do plugin são chamadas de ferramenta, portanto suas regras de permissão se aplicam a elas.

Instalar um plugin também o habilita, a menos que seu manifesto ou entrada de marketplace defina [`defaultEnabled: false`](/docs/pt/plugins/install#choose-an-install-scope) e você não o tenha habilitado você mesmo.

Para remover um plugin em que você não confia mais, veja [Remover um plugin em que você não confia mais](#remove-a-plugin-you-no-longer-trust).

<h2 id="marketplace-tiers">
  Identifique os marketplaces da Anthropic pelo nome
</h2>

O nome de um marketplace o coloca em um de três níveis: oficial, comunidade ou terceiros. Claude Code aceita os nomes oficial e comunidade apenas para marketplaces originários de repositórios `github.com/anthropics/`, portanto um marketplace de terceiros não pode se apresentar como um da Anthropic. Um marketplace que um colega de trabalho ou sua organização publica é de terceiros.

A tabela lista quais nomes se enquadram em cada nível:

| Nível      | Quais marketplaces                                                                             |
| :--------- | :--------------------------------------------------------------------------------------------- |
| Oficial    | Os [nomes de marketplace oficial](#official-marketplace-names), como `claude-plugins-official` |
| Comunidade | `claude-community`, `claude-plugins-community` e `healthcare`                                  |
| Terceiros  | Todos os outros marketplaces                                                                   |

Onde o catálogo `claude-community` fixa um plugin em um SHA de commit, o que faz para quase todas as entradas, Claude Code recusa instalar um commit diferente.

<h3 id="official-marketplace-names">
  Nomes de marketplace oficial
</h3>

Estes nomes de marketplace compõem o nível oficial:

* `claude-plugins-official`
* `claude-code-marketplace`
* `claude-code-plugins`
* `anthropic-marketplace`
* `anthropic-plugins`
* `agent-skills`
* `anthropic-agent-skills`
* `life-sciences`
* `knowledge-work-plugins`
* `claude-for-legal`
* `claude-for-financial-services`
* `financial-services-plugins`
* `first-party-plugins`
* `claude-tag-plugins`

Para saber como os marketplaces oficial, comunidade e demo diferem e onde procurar o que cada um lista, veja [Marketplaces da Anthropic](/docs/pt/plugins/anthropic-marketplaces).

<h2 id="review-a-plugin-before-you-install">
  Revise um plugin antes de instalar
</h2>

Antes de instalar um plugin, veja o que ele adiciona e de onde vem.

<Steps>
  <Step title="Verifique a fonte do marketplace">
    Em seu shell, execute `claude plugin marketplace list` para imprimir a fonte de cada marketplace foi adicionado, como um repositório GitHub ou um diretório.
  </Step>

  <Step title="Leia o painel de detalhes">
    Em uma sessão Claude Code, execute `/plugin` e selecione o plugin. O painel de detalhes mostra uma seção **Will install** listando os comandos, agentes, skills, hooks e servidores MCP e LSP do plugin. Para um plugin para o qual a Anthropic não tem dados de componente publicados, a seção mostra o que a entrada do marketplace declara, ou uma nota: `Components will be discovered at installation` para um plugin armazenado dentro do marketplace, ou `Component summary not available for remote plugin` para um buscado de outro lugar.
  </Step>

  <Step title="Leia a fonte do plugin">
    No painel de detalhes, selecione **Open homepage** ou **View on GitHub** abaixo das opções de instalação. Se o painel não oferecer nenhum dos dois, abra o repositório do marketplace que você encontrou na primeira etapa. Encontre o diretório do plugin lá. A seção **Will install** mostra que um hook existe, mas não o que ele executa, portanto leia estes arquivos no diretório do plugin:

    * **`hooks/hooks.json`**: o comando que cada hook executa
    * **`.mcp.json`**: o comando ou URL de cada servidor
    * **`bin/`**: cada arquivo no diretório
  </Step>

  <Step title="Liste o que o plugin contém">
    Clone o repositório que contém o diretório do plugin, depois execute `claude --plugin-dir <plugin directory> plugin details <plugin name>` em seu shell para ver o que Claude Code encontra nele. O comando lê os arquivos do plugin sem iniciar uma sessão e imprime um `Component inventory` listando os skills e comandos do plugin, agentes, hooks com o evento de cada hook e servidores MCP e LSP.
  </Step>
</Steps>

Depois de instalar um plugin, execute `claude plugin details <plugin name>` em seu shell para imprimir o mesmo `Component inventory` para a cópia instalada em `~/.claude/plugins/cache/<marketplace>/<plugin>/<version>/`.

<h3 id="remove-a-plugin-you-no-longer-trust">
  Remover um plugin em que você não confia mais
</h3>

Em seu shell, execute [`claude plugin uninstall <plugin>`](/docs/pt/plugins/cli-reference#plugin-uninstall) com o `--scope` em que você o instalou. Depois verifique o que a desinstalação removeu e o que deixou:

* **Dados persistentes**: quando essa era a última escopo em que o plugin foi instalado, desinstalar também exclui o diretório de dados persistentes do plugin, a menos que você passe `--keep-data`.
* **Arquivos em cache**: os arquivos do plugin permanecem no disco em `~/.claude/plugins/cache/` por 14 dias antes de uma [varredura em segundo plano removê-los](/docs/pt/plugins/loading#cleanup-of-previous-versions). Depois de desinstalar seu último plugin, diretórios órfãos permanecem até você instalar outro. Para excluir os arquivos agora, remova o diretório do plugin em `~/.claude/plugins/cache/<marketplace>/<plugin>/` você mesmo.
* **O marketplace**: se você também não confia no proprietário do marketplace, [remova o marketplace](/docs/pt/plugins/install#manage-marketplaces) também, o que desinstala cada plugin que você instalou dele.

<h2 id="recognize-when-claude-code-refuses-or-warns">
  Reconheça quando Claude Code recusa ou avisa
</h2>

O painel de detalhes que você abre na aba **Discover** ou **Marketplaces** em `/plugin` mostra o mesmo aviso de confiança para cada plugin. Claude Code recusa em vez de avisar em casos como aqueles em [Fontes de marketplace não confiáveis e verificações de integridade falhadas](#untrusted-marketplace-sources-and-failed-integrity-checks).

<h3 id="trust-warning-before-you-install">
  Aviso de confiança antes de instalar
</h3>

O aviso lê o mesmo independentemente de qual marketplace o plugin vem:

```text theme={null}
Make sure you trust a plugin before installing, updating, or using it. Anthropic does not control what MCP servers, files, or other software are included in plugins and cannot verify that they will work as intended or that they won't change. See each plugin's homepage for more information.
```

Se sua organização define `pluginTrustMessage` em [configurações gerenciadas](/docs/pt/plugins/org), Claude Code anexa esse texto ao aviso.

<h3 id="untrusted-marketplace-sources-and-failed-integrity-checks">
  Fontes de marketplace não confiáveis e verificações de integridade falhadas
</h3>

Claude Code recusa carregar um marketplace ou instalar um plugin nestes casos, cada um com sua própria mensagem de erro:

* **Fonte de marketplace não confiável**: quando um marketplace usa um nome oficial ou comunidade, mas sua fonte está fora de `github.com/anthropics/`, Claude Code para de carregar o marketplace e os plugins que você instalou dele. O erro é [Marketplace is registered from an untrusted source](/docs/pt/errors#marketplace-is-registered-from-an-untrusted-source).
* **Integridade do arquivo**: quando uma entrada de marketplace fixa uma [`archive` source](/docs/pt/plugins/marketplace-reference#archive-plugin-source) para um digest `sha256` e o digest do arquivo baixado não corresponde, Claude Code recusa a instalação. O erro é [Plugin archive integrity check failed](/docs/pt/errors#plugin-archive-integrity-check-failed).

O pin `sha256` é separado do pin de SHA de commit do catálogo da comunidade, que seleciona o commit git para fazer checkout.

<h2 id="enforce-plugin-controls-for-your-organization">
  Aplique controles de plugin para sua organização
</h2>

Com [configurações gerenciadas](/docs/pt/plugins/org), um administrador pode aplicar estes controles de plugin:

* Lista de permissão ou bloqueio de fontes de marketplace
* Forçar habilitação de plugins
* Desativar os sinalizadores `--plugin-dir` e `--plugin-url` e a variável `CLAUDE_CODE_PLUGIN_DIRS`
* Limitar hooks aos das configurações gerenciadas e plugins forçados habilitados
* Impedir que plugins das contas claude.ai de membros sejam carregados em Claude Code, com [`syncClaudeAiPlugins`](/docs/pt/plugins/org#control-matrix)

A [matriz de controle](/docs/pt/plugins/org#control-matrix) diz o que cada chave faz e não cobre.

<h2 id="find-plugins-in-telemetry">
  Encontre plugins em telemetria
</h2>

Se sua organização exporta os [eventos OpenTelemetry](/docs/pt/monitoring-usage) do Claude Code para seu próprio backend, os [níveis de marketplace](#marketplace-tiers) decidem quais nomes de plugin aparecem lá:

* **[Evento de plugin carregado](/docs/pt/monitoring-usage#plugin-loaded-event)**: o evento relata nomes de plugin e marketplace de nível oficial como estão. Para os níveis comunidade e terceiros, `plugin.name` e `marketplace.name` são a string literal `third-party` a menos que você defina `OTEL_LOG_TOOL_DETAILS=1`.
* **Escopo do plugin**: o `plugin.scope` do evento carregado ainda relata de onde o plugin veio, como `org` para um plugin que suas configurações gerenciadas habilitam ou `user-local` para qualquer outro plugin de terceiros. O [evento de plugin carregado](/docs/pt/monitoring-usage#plugin-loaded-event) lista cada valor.
* **[Evento de plugin instalado](/docs/pt/monitoring-usage#plugin-installed-event)**: a menos que você defina `OTEL_LOG_TOOL_DETAILS=1`, o evento omite os campos de nome para plugins não oficiais em vez de relatar `third-party`.
* **[API de Análise do Claude Code](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list)**: Claude Code relata plugins dos níveis oficial e comunidade por nome e relata cada outro plugin como `third-party`.

<h2 id="next-steps">
  Próximas etapas
</h2>

* [Gerenciar plugins para sua organização](/docs/pt/plugins/org): restrinja quais marketplaces os usuários podem instalar e exija os em que você confia
* [Instalar e gerenciar plugins](/docs/pt/plugins/install): revise o painel de detalhes de um plugin antes de escolher um escopo
* [Marketplaces da Anthropic](/docs/pt/plugins/anthropic-marketplaces): quais nomes de marketplace são da Anthropic
* [Segurança](/docs/pt/security): modelo de segurança próprio do Claude Code
