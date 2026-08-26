> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Encontre bugs com ultrareview

> Execute uma revisão de código profunda e multi-agente na nuvem com /code-review ultra para encontrar e verificar bugs antes de fazer merge.

<Note>
  Ultrareview é um recurso de visualização de pesquisa. O recurso, preços e disponibilidade podem mudar com base no feedback. O comando agora é invocado como `/code-review ultra`, e `/ultrareview` permanece como um alias.
</Note>

Ultrareview é uma revisão de código profunda que é executada no Claude Code na infraestrutura web. Quando você executa `/code-review ultra`, Claude Code inicia uma frota de agentes revisores em um sandbox remoto para encontrar bugs em sua branch ou pull request.

Comparado a um `/code-review` ou `/review` local, ultrareview oferece:

* **Sinal mais alto**: cada descoberta relatada é reproduzida e verificada independentemente, portanto os resultados se concentram em bugs reais em vez de sugestões de estilo
* **Cobertura mais ampla**: uma frota maior de agentes revisores explora a mudança em paralelo, o que expõe problemas que uma revisão local pode perder
* **Sem uso de recursos locais**: a revisão é executada inteiramente em um sandbox remoto, portanto seu terminal permanece livre para outro trabalho enquanto é executada

Ultrareview requer autenticação com uma conta Claude.ai porque é executado no Claude Code na infraestrutura web. Se você está conectado apenas com uma chave de API, execute `/login` e autentique-se com Claude.ai primeiro. Ultrareview não está disponível ao usar Claude Code com Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry, e não está disponível para organizações que habilitaram Zero Data Retention.

<h2 id="run-ultrareview-from-the-cli">
  Execute ultrareview a partir da CLI
</h2>

Inicie uma revisão de qualquer repositório git no Claude Code CLI.

```text theme={null}
/code-review ultra
```

Sem argumentos, ultrareview revisa o diff entre sua branch atual e a branch padrão, incluindo quaisquer mudanças não confirmadas e preparadas em sua árvore de trabalho. Claude Code agrupa o estado do repositório e o carrega em um sandbox remoto para a revisão.

Para revisar uma pull request do GitHub, passe o número da PR.

```text theme={null}
/code-review ultra 1234
```

No modo PR, o sandbox remoto clona a pull request diretamente do host em vez de agrupar sua árvore de trabalho local. O modo PR funciona com repositórios em `github.com` e em instâncias do [GitHub Enterprise Server](/docs/pt/github-enterprise-server) que um proprietário conectou ao Claude Code.

<Tip>
  Se seu repositório for muito grande para agrupar, Claude Code o solicita a usar o modo PR. Envie sua branch e abra uma PR de rascunho, depois execute `/code-review ultra <PR-number>`.

  Se o diff da pull request for muito grande, Claude Code recusa a revisão com uma dica de escopo antes de qualquer trabalho de revisão ser executado.
</Tip>

Antes de iniciar, Claude Code mostra um diálogo de confirmação com o escopo da revisão (incluindo a contagem de arquivos e linhas ao revisar uma branch), suas execuções gratuitas restantes e o custo estimado. Depois que você confirmar, a revisão continua em segundo plano e você pode continuar usando sua sessão. O comando é executado apenas quando você o invoca com `/code-review ultra`; Claude não inicia um ultrareview por conta própria.

<h2 id="pricing-and-free-runs">
  Preços e execuções gratuitas
</h2>

Ultrareview é um recurso premium que é cobrado contra créditos de uso em vez do uso incluído em seu plano.

| Plano             | Execuções gratuitas incluídas | Após execuções gratuitas                                                                                          |
| ----------------- | ----------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| Pro               | 3 execuções gratuitas         | cobrado como [créditos de uso](https://support.claude.com/pt/articles/12429409-extra-usage-for-paid-claude-plans) |
| Max               | 3 execuções gratuitas         | cobrado como [créditos de uso](https://support.claude.com/pt/articles/12429409-extra-usage-for-paid-claude-plans) |
| Team e Enterprise | nenhuma                       | cobrado como [créditos de uso](https://support.claude.com/pt/articles/12429409-extra-usage-for-paid-claude-plans) |

Os assinantes Pro e Max recebem três execuções ultrareview gratuitas para experimentar o recurso. Essas três execuções são uma alocação única por conta e não são renovadas. Depois de usar todas as três, ou após o período de execução gratuita terminar, cada revisão é cobrada para créditos de uso e normalmente custa \$5 a \$20 dependendo do tamanho da mudança. Uma execução é contada assim que a sessão remota é iniciada, portanto uma revisão que você interrompe no início ou que falha em ser concluída ainda usa uma execução gratuita. Para uma revisão paga, os créditos de uso são cobrados apenas pela parte que foi executada.

Como ultrareview sempre é cobrado como créditos de uso fora das execuções gratuitas, sua conta ou organização deve ter créditos de uso habilitados antes de poder iniciar uma revisão paga. Se os créditos de uso não estiverem habilitados, Claude Code bloqueia o lançamento e o vincula às configurações de faturamento onde você pode ativá-los. Você também pode executar `/usage-credits` para verificar ou alterar sua configuração atual.

<h2 id="track-a-running-review">
  Acompanhe uma revisão em execução
</h2>

Uma revisão normalmente leva 5 a 10 minutos. A revisão é executada como uma tarefa em segundo plano, portanto você pode continuar trabalhando em sua sessão, iniciar outros comandos ou fechar o terminal completamente.

Use `/tasks` para ver revisões em execução e concluídas, abrir a visualização de detalhes de uma revisão ou parar uma revisão em andamento. Parar uma revisão arquiva a sessão na nuvem e as descobertas parciais não são retornadas. Quando a revisão termina, as descobertas verificadas aparecem como uma notificação em sua sessão. Cada descoberta inclui a localização do arquivo e uma explicação do problema para que você possa pedir ao Claude para corrigi-lo diretamente.

<h2 id="run-ultrareview-non-interactively">
  Execute ultrareview de forma não interativa
</h2>

Use o subcomando `claude ultrareview` para iniciar um ultrareview a partir de CI ou um script sem uma sessão interativa. O subcomando inicia a mesma revisão que `/code-review ultra`, bloqueia até que a revisão remota termine, imprime as descobertas para stdout e sai com código 0 em caso de sucesso ou 1 em caso de falha.

```bash theme={null}
claude ultrareview
claude ultrareview 1234
claude ultrareview origin/main
```

Sem argumentos, o subcomando revisa o diff entre sua branch atual e a branch padrão. Passe um número de PR para revisar uma pull request, ou passe uma branch base para revisar o diff em relação a essa branch. Invocar o subcomando conta como consentimento para o aviso de faturamento e termos que o comando interativo mostra.

As mensagens de progresso e a URL da sessão ao vivo vão para stderr para que stdout permaneça analisável. Use esses sinalizadores para controlar a saída e o tempo limite:

| Sinalizador           | Descrição                                                                |
| --------------------- | ------------------------------------------------------------------------ |
| `--json`              | Imprima a carga útil bruta `bugs.json` em vez das descobertas formatadas |
| `--timeout <minutes>` | Minutos máximos para aguardar a conclusão da revisão. Padrão é 30        |

Executar `claude ultrareview` requer a mesma autenticação e configuração de uso extra que `/code-review ultra`. O subcomando sai com código 0 quando a revisão é concluída com ou sem descobertas, código 1 quando a revisão falha ao iniciar, a sessão remota apresenta erro ou o tempo limite decorre, e código 130 quando interrompido com Ctrl-C. A revisão remota continua em execução se você interromper o subcomando; siga a URL da sessão impressa em stderr para observá-la no navegador.

Para revisões automáticas em pull requests do GitHub, [Code Review](/docs/pt/code-review) integra-se diretamente com seu repositório e publica descobertas como comentários inline de PR sem uma etapa de CLI.

<h2 id="how-ultrareview-compares-to-/code-review-and-/review">
  Como ultrareview se compara a /code-review e /review
</h2>

Todos os três comandos revisam código, mas visam diferentes estágios do seu fluxo de trabalho.

|              | `/code-review`                    | `/review <pr>`                                     | `/code-review ultra`                                                                    |
| ------------ | --------------------------------- | -------------------------------------------------- | --------------------------------------------------------------------------------------- |
| Alvo         | seu diff de trabalho              | um pull request do GitHub                          | seu diff de trabalho ou um pull request                                                 |
| Execuções    | localmente em sua sessão          | localmente em sua sessão                           | remotamente em um sandbox na nuvem                                                      |
| Profundidade | escala com o argumento de esforço | uma revisão de passagem única no esforço da sessão | frota multi-agente com verificação independente                                         |
| Duração      | segundos a alguns minutos         | segundos a alguns minutos                          | aproximadamente 5 a 10 minutos                                                          |
| Custo        | conta para uso normal             | conta para uso normal                              | execuções gratuitas, depois aproximadamente \$5 a \$20 por revisão como créditos de uso |
| Melhor para  | feedback rápido durante iteração  | revisando um PR de um colega antes de aprovar      | confiança pré-merge em mudanças substanciais                                            |

Use `/code-review` para feedback rápido enquanto trabalha. Use `/review <pr>` para revisar um pull request da mesma forma que você faria antes de aprová-lo. Use `/code-review ultra` antes de fazer merge de uma mudança substancial quando você quer uma passagem mais profunda que capture problemas que uma revisão local pode perder.

<h2 id="related-resources">
  Recursos relacionados
</h2>

* [Claude Code na web](/docs/pt/claude-code-on-the-web): aprenda como funcionam as sessões remotas e os sandboxes na nuvem
* [Planeje mudanças complexas com ultraplan](/docs/pt/ultraplan): a contrapartida de planejamento para ultrareview para trabalho de design antecipado
* [Gerencie custos efetivamente](/docs/pt/costs): acompanhe o uso e defina limites de gastos
