> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code com GitHub Enterprise Server

> Conecte Claude Code à sua instância auto-hospedada do GitHub Enterprise Server para sessões web, revisão de código e marketplaces de plugins.

<Note>
  O suporte ao GitHub Enterprise Server está disponível para planos Team e Enterprise.
</Note>

O suporte ao GitHub Enterprise Server (GHES) permite que sua organização use Claude Code com repositórios hospedados em sua instância GitHub auto-gerenciada em vez de github.com. Depois que um Proprietário conecta sua instância GHES, os desenvolvedores podem executar sessões web e obter revisões de código automatizadas sem nenhuma configuração por repositório. Os marketplaces de plugins hospedados em sua instância também são suportados; os requisitos de credenciais variam por superfície, conforme descrito em [Plugin marketplaces on GHES](#plugin-marketplaces-on-ghes).

Para repositórios em github.com, consulte [Claude Code na web](/docs/pt/claude-code-on-the-web) e [Code Review](/docs/pt/code-review). Para executar Claude em sua própria infraestrutura de CI, consulte [GitHub Actions](/docs/pt/github-actions).

<h2 id="what-works-with-github-enterprise-server">
  O que funciona com GitHub Enterprise Server
</h2>

A tabela abaixo mostra quais recursos do Claude Code suportam GHES e quaisquer diferenças do comportamento do github.com.

| Recurso                  | Suporte GHES    | Notas                                                                                                                                                  |
| :----------------------- | :-------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code na web       | ✅ Suportado     | Um proprietário conecta a instância GHES uma vez; os desenvolvedores usam `claude --cloud` ou [claude.ai/code](https://claude.ai/code) como de costume |
| Code Review              | ✅ Suportado     | Mesmas revisões automatizadas de PR que github.com                                                                                                     |
| Claude Security          | ✅ Suportado     | Disponível em beta público para planos Enterprise em [claude.ai/security](https://claude.ai/security)                                                  |
| Sessões Teleport         | ✅ Suportado     | Mova sessões entre web e terminal com `--teleport`                                                                                                     |
| Marketplaces de plugins  | ✅ Suportado     | Os requisitos de credenciais diferem por superfície. Veja [Plugin marketplaces on GHES](#plugin-marketplaces-on-ghes)                                  |
| Métricas de contribuição | ✅ Suportado     | Entregues via webhooks para o [painel de análise](/docs/pt/analytics)                                                                                       |
| GitHub Actions           | ✅ Suportado     | Requer configuração manual de workflow; `/install-github-app` é apenas para github.com                                                                 |
| Servidor GitHub MCP      | ❌ Não suportado | O servidor GitHub MCP não funciona com instâncias GHES                                                                                                 |

<h2 id="admin-setup">
  Configuração do administrador
</h2>

Um Proprietário conecta sua instância GHES ao Claude Code uma vez. Depois disso, os desenvolvedores em sua organização podem usar repositórios GHES sem nenhuma configuração adicional. Você precisa da função Proprietário ou Proprietário Primário em sua organização Claude e permissão para criar GitHub Apps em sua instância GHES.

A configuração guiada gera um manifesto de GitHub App e o redireciona para sua instância GHES para criar o app em um clique. Se seu ambiente bloquear o fluxo de redirecionamento, uma [configuração manual alternativa](#manual-setup) está disponível.

<Steps>
  <Step title="Abra as configurações de administrador do Claude Code">
    Vá para [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) e encontre a seção GitHub Enterprise Server.
  </Step>

  <Step title="Inicie a configuração guiada">
    Clique em **Connect**. Digite um nome de exibição para a conexão e o nome do host GHES, por exemplo `github.example.com`. Se sua instância GHES usar um certificado auto-assinado ou autoridade de certificação privada, cole o certificado CA no campo opcional.
  </Step>

  <Step title="Crie o GitHub App">
    Clique em **Continue to GitHub Enterprise**. Seu navegador redireciona para sua instância GHES com um manifesto de app pré-preenchido. Revise a configuração e clique em **Create GitHub App**. GHES o redireciona de volta para Claude com as credenciais do app armazenadas automaticamente.
  </Step>

  <Step title="Instale o app em seus repositórios">
    Na página do GitHub App em sua instância GHES, instale o app nos repositórios ou organizações que você deseja que Claude acesse. Você pode começar com um subconjunto e adicionar mais depois.
  </Step>

  <Step title="Ative os recursos">
    Retorne a [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code) e ative [Code Review](/docs/pt/code-review#set-up-code-review), Claude Security e [métricas de contribuição](/docs/pt/analytics#enable-contribution-metrics) para seus repositórios GHES usando a mesma configuração que github.com.
  </Step>
</Steps>

<h3 id="github-app-permissions">
  Permissões do GitHub App
</h3>

O manifesto configura o GitHub App com as permissões e eventos de webhook que Claude precisa em sessões web, Code Review, Claude Security e métricas de contribuição:

| Permissão        | Acesso            | Usado para                                            |
| :--------------- | :---------------- | :---------------------------------------------------- |
| Contents         | Leitura e escrita | Clonagem de repositórios e push de branches           |
| Pull requests    | Leitura e escrita | Criação de PRs e postagem de comentários de revisão   |
| Issues           | Leitura e escrita | Resposta a menções de issues                          |
| Checks           | Leitura e escrita | Postagem de execuções de verificação do Code Review   |
| Actions          | Leitura           | Leitura do status de CI para auto-fix                 |
| Repository hooks | Leitura e escrita | Recebimento de webhooks para métricas de contribuição |
| Metadata         | Leitura           | Obrigatório pelo GitHub para todos os apps            |

O app se inscreve em eventos `pull_request`, `issue_comment`, `pull_request_review_comment`, `pull_request_review` e `check_run`.

<h3 id="manual-setup">
  Configuração manual
</h3>

Se o fluxo de redirecionamento guiado for bloqueado pela configuração de rede, clique em **Add manually** em vez de Connect. Crie um GitHub App em sua instância GHES com as [permissões e eventos acima](#github-app-permissions), depois insira as credenciais do app no formulário: nome do host, ID do cliente OAuth e segredo, ID do GitHub App, ID do cliente, segredo do cliente, segredo do webhook e chave privada.

<h3 id="network-requirements">
  Requisitos de rede
</h3>

Sua instância GHES deve ser acessível a partir da infraestrutura Anthropic para que Claude possa clonar repositórios e postar comentários de revisão. Se sua instância GHES estiver atrás de um firewall, coloque na lista de permissões os [endereços IP da API Anthropic](https://platform.claude.com/docs/en/api/ip-addresses).

<h2 id="developer-workflow">
  Fluxo de trabalho do desenvolvedor
</h2>

Depois que um Proprietário conectar a instância GHES, nenhuma configuração do lado do desenvolvedor é necessária. Claude Code detecta automaticamente o nome do host GHES a partir do git remote em seu diretório de trabalho.

Clone um repositório de sua instância GHES como você normalmente faria:

```bash theme={null}
git clone git@github.example.com:platform/api-service.git
cd api-service
```

Depois inicie uma sessão web. Claude detecta o host GHES a partir de seu git remote e roteia a sessão através da instância configurada de sua organização:

```bash theme={null}
claude --cloud "Add retry logic to the payment webhook handler"
```

A sessão é executada na infraestrutura Anthropic, clona seu repositório do GHES e envia alterações de volta para um branch. Monitore o progresso com `/tasks` ou em [claude.ai/code](https://claude.ai/code). Consulte [Claude Code na web](/docs/pt/claude-code-on-the-web) para o fluxo de trabalho completo de sessão remota, incluindo revisão de diff, auto-fix e rotinas.

<h3 id="teleport-sessions-to-your-terminal">
  Teleporte de sessões para seu terminal
</h3>

Puxe uma sessão web para seu terminal local com `claude --teleport`. Teleport verifica se você está em um checkout do mesmo repositório GHES antes de buscar o branch e carregar o histórico da sessão. Consulte [requisitos de teleport](/docs/pt/claude-code-on-the-web#teleport-requirements) para detalhes.

<h2 id="plugin-marketplaces-on-ghes">
  Marketplaces de plugins em GHES
</h2>

Hospede marketplaces de plugins em sua instância GHES para distribuir ferramentas internas em toda sua organização. A estrutura do marketplace é idêntica aos marketplaces hospedados em github.com, mas a instalação funciona de forma diferente dependendo de onde você adiciona o marketplace, e as credenciais diferem entre as superfícies:

| Superfície                                           | Como a instalação funciona                                                                                                                                                                                                                          | O que cada usuário precisa                                                                                                                                                                                                                 |
| :--------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Claude Code CLI e desktop                            | Claude Code clona o repositório do marketplace usando as credenciais git existentes da máquina                                                                                                                                                      | Acesso Git ao seu host GHES a partir de sua máquina                                                                                                                                                                                        |
| Configurações gerenciadas (`extraKnownMarketplaces`) | Claude Code registra a entrada e clona o repositório usando as credenciais git existentes da máquina                                                                                                                                                | Acesso Git ao seu host GHES a partir de sua máquina                                                                                                                                                                                        |
| Configurações de plugin da organização claude.ai     | Um Proprietário seleciona a instância GHES como a fonte; o backend da Anthropic busca e sincroniza o repositório usando o GitHub App de [configuração de administrador](#admin-setup)                                                               | Nada por usuário uma vez adicionado. O Proprietário que o adiciona precisa de sua própria conta GitHub Enterprise conectada como uma verificação de acesso, e o GitHub App deve estar instalado no repositório do marketplace              |
| Configurações de usuário claude.ai                   | O backend da Anthropic busca o repositório usando a conexão GitHub Enterprise do usuário que o envia                                                                                                                                                | Sua própria conta GitHub Enterprise conectada ao Claude                                                                                                                                                                                    |
| Claude Code na web                                   | As sessões em nuvem clonam marketplaces dentro da sandbox da sessão. A sandbox pode alcançar sua instância GHES apenas quando o repositório da sessão está nessa mesma instância, e suas credenciais git estão limitadas aos repositórios da sessão | Não é confiável para marketplaces hospedados em GHES: um host diferente do repositório da sessão não é alcançável, e até mesmo instalações na mesma instância podem falhar. Use a CLI, configurações gerenciadas ou claude.ai em vez disso |

<Warning>
  As conexões GitHub Enterprise em claude.ai são por usuário quando um marketplace é adicionado a partir das configurações de usuário. A [configuração de administrador](#admin-setup) conecta sua instância GHES à sua organização, mas não conecta contas de usuários individuais: cada usuário que adiciona um marketplace GHES a partir de suas próprias configurações deve primeiro conectar sua própria conta GitHub Enterprise, e a conexão de um usuário, incluindo a do Proprietário, não cobre ninguém mais. Os marketplaces adicionados por um Proprietário nas configurações de plugin da organização não colocam esse requisito nos usuários, porque as buscas contínuas usam o GitHub App da organização. O Proprietário que adiciona o marketplace ainda precisa de sua própria conta GitHub Enterprise conectada no momento da adição.
</Warning>

<h3 id="add-a-ghes-marketplace">
  Adicione um marketplace GHES
</h3>

O atalho `owner/repo` sempre resolve para github.com. Para marketplaces hospedados em GHES, use a URL git completa. URLs HTTPS são recomendadas:

```bash theme={null}
/plugin marketplace add https://github.example.com/platform/claude-plugins.git
```

URLs SSH funcionam se a máquina já confia em seu host GHES:

```bash theme={null}
/plugin marketplace add git@github.example.com:platform/claude-plugins.git
```

Claude Code executa git de forma não interativa e rejeita conexões SSH para hosts que não estão no arquivo `known_hosts` da máquina. Uma URL HTTPS com um auxiliar de credenciais git evita o requisito `known_hosts`.

Consulte [Criar e distribuir um marketplace de plugins](/docs/pt/plugin-marketplaces) para o guia completo de construção de marketplaces.

<h3 id="pre-register-ghes-marketplaces-with-managed-settings">
  Pré-registre marketplaces GHES com configurações gerenciadas
</h3>

A configuração `extraKnownMarketplaces` pré-registra um marketplace para que os desenvolvedores o obtenham sem configuração manual. Funciona a partir de [qualquer arquivo de configurações](/docs/pt/settings#extraknownmarketplaces), incluindo `.claude/settings.json` de um repositório; as configurações gerenciadas a entregam em toda a organização:

```json theme={null}
{
  "extraKnownMarketplaces": {
    "internal-tools": {
      "source": {
        "source": "git",
        "url": "https://github.example.com/platform/claude-plugins.git"
      }
    }
  }
}
```

Claude Code instala esses marketplaces localmente: registra cada entrada e clona o repositório com as credenciais git existentes da máquina. Este caminho não passa por claude.ai, portanto a conexão GitHub Enterprise por usuário não é necessária. Para um lançamento bem-sucedido:

* **Use uma URL git completa.** O atalho `owner/repo` sempre resolve para github.com e não pode referenciar um host GHES.
* **Prefira URLs HTTPS.** Clones SSH falham em máquinas que ainda não confiam na chave do host GHES. Uma URL HTTPS com o auxiliar de credenciais git padrão de sua organização funciona em qualquer máquina com credenciais configuradas.
* **Confirme que cada máquina pode clonar a partir de seu host GHES.** Se uma máquina não tiver credenciais, o marketplace é registrado mas nunca instalado, e seus plugins são relatados como não encontrados em vez de solicitar credenciais.
* **Confirme que a configuração atinge cada máquina.** Um arquivo de configurações gerenciadas só entra em vigor nas máquinas em que é implantado, por exemplo através de seu sistema de gerenciamento de dispositivos. Consulte [configurações gerenciadas](/docs/pt/settings#settings-files) para locais de arquivos.

<h3 id="allowlist-ghes-marketplaces-in-managed-settings">
  Coloque na lista de permissões marketplaces GHES em configurações gerenciadas
</h3>

Se sua organização usa [configurações gerenciadas](/docs/pt/settings) para restringir quais marketplaces os desenvolvedores podem adicionar, use o tipo de fonte `hostPattern` para permitir todos os marketplaces de sua instância GHES sem enumerar cada repositório:

```json theme={null}
{
  "strictKnownMarketplaces": [
    {
      "source": "hostPattern",
      "hostPattern": "^github\\.example\\.com$"
    }
  ]
}
```

Consulte a referência de configurações [strictKnownMarketplaces](/docs/pt/settings#strictknownmarketplaces) e [extraKnownMarketplaces](/docs/pt/settings#extraknownmarketplaces) para o esquema completo.

<h2 id="limitations">
  Limitações
</h2>

Alguns recursos se comportam de forma diferente em GHES do que em github.com. A [tabela de recursos](#what-works-with-github-enterprise-server) resume o suporte; esta seção cobre as soluções alternativas.

* **Comando `/install-github-app`**: siga o fluxo de [configuração do administrador](#admin-setup) em claude.ai. Se você também deseja workflows do GitHub Actions em GHES, adapte o [workflow de exemplo](https://github.com/anthropics/claude-code-action/blob/main/examples/claude.yml) manualmente.
* **Servidor GitHub MCP**: use o CLI `gh` configurado para seu host GHES. Execute `gh auth login --hostname github.example.com` para autenticar, depois Claude pode usar comandos `gh` em sessões.

<h2 id="troubleshooting">
  Troubleshooting
</h2>

<h3 id="web-session-fails-to-clone-repository">
  A sessão web falha ao clonar o repositório
</h3>

Se `claude --cloud` falhar com um erro de clone, verifique se um Owner concluiu a configuração para sua instância GHES e se o GitHub App está instalado no repositório em que você está trabalhando. Peça ao Owner que conectou a instância para confirmar que o nome do host registrado nas configurações do Claude corresponde ao nome do host em seu git remote.

<h3 id="marketplace-add-fails-with-a-policy-error">
  Falha ao adicionar marketplace com erro de política
</h3>

Se `/plugin marketplace add` for bloqueado para sua URL GHES, sua organização restringiu as fontes de marketplace. Peça ao seu administrador para adicionar uma entrada `hostPattern` para seu nome do host GHES em [configurações gerenciadas](#allowlist-ghes-marketplaces-in-managed-settings).

<h3 id="marketplace-add-on-claude-ai-fails-with-a-github-access-error">
  Falha ao adicionar marketplace em claude.ai com erro de acesso ao GitHub
</h3>

Se adicionar um marketplace GHES a partir de suas configurações de usuário falhar com um erro genérico como "Marketplace não pôde ser adicionado", verifique primeiro sua conexão GitHub Enterprise. Isso é o que aparece quando sua própria conta GitHub Enterprise não está conectada ao Claude, mesmo que a instância GHES de sua organização esteja configurada e outros usuários estejam conectados. O diálogo não aponta para o fluxo de conexão do GitHub Enterprise, e a opção "Conectar ao GitHub" na aba Browse faz login em github.com, o que não concede acesso aos repositórios GHES.

Para conectar sua conta GitHub Enterprise: o seletor de repositório em [claude.ai/code](https://claude.ai/code) oferece uma opção de conexão para cada instância GHES configurada, e Owners também podem conectar a partir da seção GitHub Enterprise das [configurações de administrador do Claude Code](https://claude.ai/admin-settings/claude-code). Em seguida, adicione o marketplace novamente. Alternativamente, peça a um Owner para adicionar o marketplace nas configurações de plugin da organização, o que remove o requisito de conexão por usuário.

Em outras superfícies do claude.ai, um erro "Repositório não encontrado. Se for privado, acesso ao GitHub é necessário" em um marketplace GHES geralmente indica a mesma conexão ausente. Conecte sua conta GitHub Enterprise através de um dos caminhos acima e tente novamente.

<h3 id="ghes-instance-not-reachable">
  Instância GHES não acessível
</h3>

Se revisões ou sessões web expirarem, sua instância GHES pode não ser acessível a partir da infraestrutura Anthropic. Confirme se seu firewall permite conexões de entrada dos [endereços IP da API Anthropic](https://platform.claude.com/docs/pt/api/ip-addresses).

<h2 id="related-resources">
  Recursos relacionados
</h2>

Estas páginas cobrem os recursos referenciados ao longo deste guia com mais profundidade:

* [Claude Code na web](/docs/pt/claude-code-on-the-web): execute sessões do Claude Code em infraestrutura em nuvem
* [Code Review](/docs/pt/code-review): revisões automatizadas de PR
* [Marketplaces de plugins](/docs/pt/plugin-marketplaces): construir e distribuir catálogos de plugins
* [Analytics](/docs/pt/analytics): rastrear uso e métricas de contribuição
* [Configurações gerenciadas](/docs/pt/settings): configuração de política em toda a organização
* [Configuração de rede](/docs/pt/network-config): requisitos de firewall e lista de permissões de IP
