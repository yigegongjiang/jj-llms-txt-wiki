> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Référence des erreurs

> Consultez les messages d'erreur d'exécution de Claude Code avec leur signification et comment les corriger.

Cette page répertorie les erreurs d'exécution que Claude Code affiche et comment récupérer de chacune d'elles, ainsi que ce qu'il faut vérifier lorsque les réponses semblent incorrectes sans erreur. Pour les erreurs d'installation telles que `command not found` ou les défaillances TLS lors de la configuration, consultez [Dépannage de l'installation et de la connexion](/docs/fr/troubleshoot-install).

Ces erreurs et les commandes de récupération s'appliquent sur l'ensemble de l'interface CLI, l'[application Desktop](/docs/fr/desktop) et [Claude Code sur le web](/docs/fr/claude-code-on-the-web), car les trois encapsulent le même CLI Claude Code. Pour les problèmes spécifiques à une surface, consultez la section dépannage sur la page de cette surface.

<Note>
  Claude Code appelle l'API Claude pour les réponses du modèle, donc la plupart des erreurs d'exécution correspondent à un code d'erreur API sous-jacent. Cette page couvre ce que chaque erreur signifie dans Claude Code et comment récupérer. Pour les définitions brutes du code de statut HTTP, consultez la [référence des erreurs de la plateforme Claude](https://platform.claude.com/docs/en/api/errors).
</Note>

<h2 id="find-your-error">
  Trouvez votre erreur
</h2>

Faites correspondre le message que vous voyez dans votre terminal à une section ci-dessous.

| Message                                                                                            | Section                                                                                                                        |
| :------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------- |
| `API Error: 500 Internal server error`                                                             | [Erreurs serveur](#api-error-500-internal-server-error)                                                                        |
| `API Error: Repeated 529 Overloaded errors`                                                        | [Erreurs serveur](#api-error-repeated-529-overloaded-errors)                                                                   |
| `Request timed out`                                                                                | [Erreurs serveur](#request-timed-out), ou [Réseau](#unable-to-connect-to-api) si le message mentionne votre connexion Internet |
| `Server error mid-response. The response above may be incomplete.`                                 | [Erreurs serveur](#the-response-above-may-be-incomplete)                                                                       |
| `Connection closed mid-response` / `Response stalled mid-stream`                                   | [Erreurs serveur](#the-response-above-may-be-incomplete)                                                                       |
| `<model> is temporarily unavailable, so auto mode cannot determine the safety of...`               | [Erreurs serveur](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Auto mode could not evaluate this action and is blocking it for safety`                           | [Erreurs serveur](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Auto mode classifier transcript exceeded context window`                                          | [Erreurs serveur](#auto-mode-cannot-determine-the-safety-of-an-action)                                                         |
| `Agent terminated early due to an API error`                                                       | [Erreurs serveur](#agent-terminated-early-due-to-an-api-error)                                                                 |
| `You've hit your session limit` / `You've hit your weekly limit`                                   | [Limites d'utilisation](#youve-hit-your-session-limit)                                                                         |
| `Usage credits required for 1M context`                                                            | [Limites d'utilisation](#usage-credits-required-for-1m-context)                                                                |
| `Server is temporarily limiting requests`                                                          | [Limites d'utilisation](#server-is-temporarily-limiting-requests)                                                              |
| `Request rejected (429)`                                                                           | [Limites d'utilisation](#request-rejected-429)                                                                                 |
| `Credit balance is too low`                                                                        | [Limites d'utilisation](#credit-balance-is-too-low)                                                                            |
| `Not logged in · Please run /login`                                                                | [Authentification](#not-logged-in)                                                                                             |
| `Could not resolve authentication method`                                                          | [Authentification](#could-not-resolve-authentication-method)                                                                   |
| `Invalid API key`                                                                                  | [Authentification](#invalid-api-key)                                                                                           |
| `Your apiKeyHelper script is failing`                                                              | [Authentification](#your-apikeyhelper-script-is-failing)                                                                       |
| `This organization has been disabled`                                                              | [Authentification](#this-organization-has-been-disabled)                                                                       |
| `Your organization has disabled API key authentication`                                            | [Authentification](#your-organization-has-disabled-api-key-authentication)                                                     |
| `Your organization has disabled Claude subscription access`                                        | [Authentification](#your-organization-has-disabled-claude-subscription-access)                                                 |
| `Routines are disabled by your organization's policy`                                              | [Authentification](#routines-are-disabled-by-your-organizations-policy)                                                        |
| `Remote Control is only available when using Claude via api.anthropic.com`                         | [Authentification](#remote-control-requires-the-anthropic-api)                                                                 |
| `OAuth token revoked` / `OAuth token has expired`                                                  | [Authentification](#oauth-token-revoked-or-expired)                                                                            |
| `Login expired · Please run /login`                                                                | [Authentification](#login-expired)                                                                                             |
| `Failed to authenticate: OAuth session expired and could not be refreshed`                         | [Authentification](#login-expired)                                                                                             |
| `does not meet scope requirement user:profile`                                                     | [Authentification](#oauth-scope-requirement)                                                                                   |
| `AWS credentials expired or invalid`                                                               | [Authentification](#aws-credentials-expired-or-invalid)                                                                        |
| `AWS authentication failed`                                                                        | [Authentification](#aws-authentication-failed)                                                                                 |
| `AWS default-chain credential resolve timed out`                                                   | [Authentification](#aws-default-chain-credential-resolve-timed-out)                                                            |
| `Unable to connect to API`                                                                         | [Réseau](#unable-to-connect-to-api)                                                                                            |
| `Waiting for API response · will retry in`                                                         | [Tentatives automatiques](#automatic-retries), ou [Réseau](#unable-to-connect-to-api) si le problème persiste                  |
| `Bedrock streaming response has content-type "..."; expected "application/vnd.amazon.eventstream"` | [Réseau](#bedrock-streaming-response-has-an-unexpected-content-type)                                                           |
| `SSL certificate verification failed`                                                              | [Réseau](#ssl-certificate-errors)                                                                                              |
| `SSL certificate error (...)` during login or startup                                              | [Réseau](#ssl-certificate-errors)                                                                                              |
| `403` with `x-deny-reason: host_not_allowed` in a cloud or routine session                         | [Réseau](#host-not-allowed-in-a-cloud-session)                                                                                 |
| `Couldn't reconnect to your Remote Control session`                                                | [Réseau](#couldnt-reconnect-to-your-remote-control-session)                                                                    |
| `Prompt is too long`                                                                               | [Erreurs de requête](#prompt-is-too-long)                                                                                      |
| `Error during compaction: Conversation too long`                                                   | [Erreurs de requête](#error-during-compaction-conversation-too-long)                                                           |
| `Request too large`                                                                                | [Erreurs de requête](#request-too-large)                                                                                       |
| `Image was too large`                                                                              | [Erreurs de requête](#image-was-too-large)                                                                                     |
| `Unable to resize image`                                                                           | [Erreurs de requête](#unable-to-resize-image)                                                                                  |
| `PDF too large` / `PDF is password protected`                                                      | [Erreurs de requête](#pdf-errors)                                                                                              |
| `Extra inputs are not permitted`                                                                   | [Erreurs de requête](#extra-inputs-are-not-permitted)                                                                          |
| `There's an issue with the selected model`                                                         | [Erreurs de requête](#theres-an-issue-with-the-selected-model)                                                                 |
| `Model ... is not a recognized model id`                                                           | [Erreurs de requête](#model-is-not-a-recognized-model-id)                                                                      |
| `Claude Opus is not available with the Claude Pro plan`                                            | [Erreurs de requête](#claude-opus-is-not-available-with-the-claude-pro-plan)                                                   |
| `Model ... is restricted by your organization's settings`                                          | [Erreurs de requête](#model-is-restricted-by-your-organizations-settings)                                                      |
| `thinking.type.enabled is not supported for this model`                                            | [Erreurs de requête](#thinking-type-enabled-is-not-supported-for-this-model)                                                   |
| `max_tokens must be greater than thinking.budget_tokens`                                           | [Erreurs de requête](#thinking-budget-exceeds-output-limit)                                                                    |
| `API Error: 400 due to tool use concurrency issues`                                                | [Erreurs de requête](#tool-use-or-thinking-block-mismatch)                                                                     |
| `Claude Code is unable to respond to this request, which appears to violate our Usage Policy`      | [Erreurs de requête](#usage-policy-refusal)                                                                                    |
| `<model> has safety measures that flagged this message for a cybersecurity topic`                  | [Erreurs de requête](#safety-measures-flagged-a-cybersecurity-topic)                                                           |
| `Installation was killed before it could finish (exit code 137)`                                   | [Erreurs d'installation](#installation-was-killed-before-it-could-finish)                                                      |
| `The connection dropped while downloading the update`                                              | [Erreurs d'installation](#the-connection-dropped-while-downloading-the-update)                                                 |
| `Download timed out: exceeded the total deadline`                                                  | [Erreurs d'installation](#the-connection-dropped-while-downloading-the-update)                                                 |
| `--bg and --print conflict`                                                                        | [Erreurs de ligne de commande](#command-line-errors)                                                                           |
| `Error: --json-schema is not a valid JSON Schema`                                                  | [Erreurs de ligne de commande](#command-line-errors)                                                                           |
| `Could not import <server>: <reason>`                                                              | [Erreurs de ligne de commande](#could-not-import-a-server-from-claude-desktop)                                                 |
| `Error: MCP tool <name> (passed via --permission-prompt-tool) not found`                           | [Erreurs de ligne de commande](#mcp-permission-prompt-tool-not-found)                                                          |
| `Marketplace "<name>" is registered from an untrusted source`                                      | [Erreurs de plugin](#marketplace-is-registered-from-an-untrusted-source)                                                       |
| `references ${user_config.*} in a shell-form command`                                              | [Erreurs de plugin](#plugin-command-references-user-config)                                                                    |
| `Monitor "<name>" from plugin <plugin> references ${user_config.*} in its command`                 | [Erreurs de plugin](#plugin-command-references-user-config)                                                                    |
| `headersHelper for MCP server '<name>' references ${user_config.*}`                                | [Erreurs de plugin](#plugin-command-references-user-config)                                                                    |
| `would be spawned with zero tools — refusing`                                                      | [Erreurs d'outil](#agent-would-be-spawned-with-zero-tools)                                                                     |
| `File is covered by a Read deny rule in your permission settings`                                  | [Erreurs d'outil](#file-is-covered-by-a-read-deny-rule)                                                                        |
| `Can't open MCP settings in a background session`                                                  | [Erreurs de session en arrière-plan](#commands-refused-in-a-background-session)                                                |
| `CLAUDE_CODE_PROCESS_WRAPPER: launcher ...`                                                        | [Erreurs de session en arrière-plan](#claude_code_process_wrapper-launcher-errors)                                             |
| `Ignoring N permissions.allow entries from ... this workspace has not been trusted`                | [Avertissements de configuration](#workspace-has-not-been-trusted)                                                             |
| Responses seem lower quality than usual                                                            | [Qualité des réponses](#responses-seem-lower-quality-than-usual)                                                               |

<h2 id="automatic-retries">
  Tentatives automatiques
</h2>

Claude Code réessaie les défaillances transitoires avant de vous afficher une erreur. Les erreurs serveur, les réponses surchargées, les délais d'attente des requêtes, les accélérateurs 429 temporaires et les connexions interrompues sont tous réessayés jusqu'à 10 fois avec un backoff exponentiel. À partir de la v2.1.198, cela couvre les connexions qui se déconnectent au milieu d'une réponse avant que toute sortie visible n'ait été diffusée : Claude Code réédite la requête avec le même backoff et le tour continue au lieu de s'arrêter avec une erreur de connexion. À partir de la v2.1.199, les accélérateurs 429 temporaires qui ne portent pas les en-têtes de quota de votre plan sont également réessayés lorsque vous êtes connecté avec un abonnement claude.ai ; les versions antérieures les réessayaient uniquement pour les connexions par clé API et Enterprise.

Certaines classes de défaillances ne sont pas réessayées, car une tentative ne peut pas réussir :

* À partir de la v2.1.199, une défaillance de validation de certificat TLS, telle qu'un proxy inspectant TLS, un bundle `NODE_EXTRA_CA_CERTS` manquant ou un certificat expiré, échoue à la première tentative pour que le correctif apparaisse immédiatement au lieu d'après le budget de tentative complet. Consultez [Erreurs de certificat SSL](#ssl-certificate-errors). Les conditions TLS transitoires telles qu'un délai d'attente de poignée de main réessaient toujours.
* À partir de la v2.1.199, une erreur serveur qui arrive après que Claude a déjà diffusé une sortie visible conserve la réponse partielle et ajoute un [avis de réponse incomplète](#the-response-above-may-be-incomplete) au lieu de réessayer, car réexécuter la requête pourrait exécuter les mêmes appels d'outils deux fois. Les versions antérieures ont rejeté la sortie partielle et ont signalé le tour comme une erreur.
* Une [réponse de streaming Amazon Bedrock avec un type de contenu inattendu](#bedrock-streaming-response-has-an-unexpected-content-type) échoue à la première tentative, car la passerelle ou le proxy réécrivant la réponse réécrirait la tentative de la même manière. Nécessite Claude Code v2.1.208 ou version ultérieure.

Lors des tentatives, le spinner affiche un compte à rebours `Retrying in Ns · attempt x/y` après une étiquette d'erreur. L'étiquette nomme la raison spécifique de la première tentative pour les défaillances sur lesquelles vous pouvez agir immédiatement : le réseau est en panne, une poignée de main TLS a échoué, ou vous avez atteint une limite de débit. Pour les autres erreurs, elle lit `API error` au début. À partir de la v2.1.198, elle bascule vers la raison spécifique de la troisième tentative, ou à la tentative finale lorsque `CLAUDE_CODE_MAX_RETRIES` permet moins de trois ; les versions antérieures ne basculent qu'à la tentative finale.

À partir de la v2.1.198, l'indice de spinner habituel est supprimé lors des tentatives. Une fois que la raison de l'erreur est révélée, si la défaillance est une surcharge 529, la ligne sous le compte à rebours nomme également où vérifier l'état du service : `status.claude.com` sur l'API Anthropic, ou l'hôte du fournisseur ou de la passerelle nommé dans le message sur d'autres configurations.

Si aucune donnée n'arrive sur le flux de réponse pendant 20 secondes alors qu'une requête est toujours en attente, le spinner affiche `Waiting for API response · will retry in … · check your network` avant que toute tentative n'ait commencé. La requête n'a pas encore échoué : le compte à rebours s'exécute jusqu'au point où Claude Code abandonne la connexion bloquée et réessaie, de sorte que la bannière s'efface d'elle-même une fois que les données reprennent ou que la tentative réussit. À partir de la v2.1.185, le seuil est de 20 secondes ; les versions antérieures affichent la bannière après 10 secondes avec un libellé différent. Si elle réapparaît à chaque tentative, traitez-la comme un [problème réseau](#unable-to-connect-to-api).

Lorsque vous voyez l'une des erreurs de cette page, ces tentatives ont déjà été épuisées, sauf si elle appartient à une classe qui n'est pas réessayée, telle qu'une défaillance de validation de certificat. Vous pouvez ajuster le comportement avec ces variables d'environnement :

| Variable                                     | Par défaut | Effet                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| :------------------------------------------- | :--------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`CLAUDE_CODE_MAX_RETRIES`](/docs/fr/env-vars)    | 10         | Nombre de tentatives de réessai. Limité à 15 à partir de la v2.1.186 ; à partir de la v2.1.199 `CLAUDE_CODE_RETRY_WATCHDOG` augmente la valeur par défaut et supprime le plafond. Réduisez-le pour afficher les défaillances plus rapidement dans les scripts.                                                                                                                                                                                                                                                                                                                    |
| [`CLAUDE_CODE_RETRY_WATCHDOG`](/docs/fr/env-vars) | non défini | Définissez sur `1` dans les sessions sans surveillance telles que les tâches CI pour réessayer les erreurs de capacité `429` et `529` indéfiniment au lieu d'échouer après `CLAUDE_CODE_MAX_RETRIES` tentatives. À partir de la v2.1.199, il augmente également le nombre de tentatives par défaut pour les autres erreurs transitoires, telles que les erreurs serveur, les délais d'attente et les connexions interrompues, à 300, environ trois heures de backoff, et supprime le plafond de 15 sur `CLAUDE_CODE_MAX_RETRIES` si vous définissez cette variable explicitement. |
| [`API_TIMEOUT_MS`](/docs/fr/env-vars)             | 600000     | Délai d'attente par requête en millisecondes. Augmentez-le pour les réseaux lents ou les proxies.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

<h2 id="server-errors">
  Erreurs serveur
</h2>

Ces erreurs proviennent du fournisseur d'inférence plutôt que de votre compte ou de votre demande. Sur l'API Anthropic, cela signifie l'infrastructure Anthropic. Sur Amazon Bedrock, la plateforme Agent de Google Cloud, Microsoft Foundry ou une passerelle personnalisée, cela signifie l'infrastructure de ce fournisseur.

<h3 id="api-error-500-internal-server-error">
  Erreur API : 500 Erreur serveur interne
</h3>

Claude Code affiche le code d'état et le message d'erreur de l'API pour toute réponse 5xx. L'exemple ci-dessous montre une réponse 500 sur l'API Anthropic :

```text theme={null}
API Error: 500 Internal server error. This is a server-side issue, usually temporary — try again in a moment. If it persists, check https://status.claude.com.
```

La phrase finale indique où vérifier l'état du service et varie selon le fournisseur. Les configurations Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry indiquent l'état du service de ce fournisseur. Une `ANTHROPIC_BASE_URL` personnalisée indique l'hôte de la passerelle.

Cela indique une défaillance inattendue à l'intérieur de l'API. Elle n'est pas causée par votre prompt, vos paramètres ou votre compte.

**Que faire :**

* Vérifiez [status.claude.com](https://status.claude.com), ou la page d'état du fournisseur nommée dans le message, pour les incidents actifs
* Attendez une minute, puis renvoyez votre message. Votre message original est toujours dans la conversation, donc pour un long prompt vous pouvez taper `try again` au lieu de coller le tout.
* Si l'erreur persiste sans incident signalé, exécutez `/feedback` pour qu'Anthropic puisse enquêter avec les détails de votre demande. Consultez [Report an error](#report-an-error) si `/feedback` n'est pas disponible dans votre environnement.

<h3 id="api-error-repeated-529-overloaded-errors">
  Erreur API : Erreurs 529 Overloaded répétées
</h3>

L'API est temporairement à capacité pour tous les utilisateurs. Claude Code a déjà réessayé plusieurs fois avant d'afficher ce message :

```text theme={null}
API Error: Repeated 529 Overloaded errors. The API is at capacity — this is usually temporary. Try again in a moment. If it persists, check https://status.claude.com.
```

La phrase finale varie selon le fournisseur de la même manière que l'erreur 500 ci-dessus.

Un 529 n'est pas votre limite d'utilisation et ne compte pas contre votre quota.

**Que faire :**

* Vérifiez [status.claude.com](https://status.claude.com), ou la page d'état du fournisseur nommée dans le message, pour les avis de capacité
* Réessayez dans quelques minutes
* Exécutez `/model` et basculez vers un modèle différent pour continuer à travailler, car la capacité est suivie par modèle. Claude Code vous invite à le faire quand un modèle est sous une charge particulièrement élevée, par exemple `Opus is experiencing high load, please use /model to switch to Sonnet`.

<h3 id="request-timed-out">
  Délai d'attente de la demande dépassé
</h3>

L'API n'a pas répondu avant la limite de connexion.

```text theme={null}
Request timed out
```

Cela peut se produire pendant les périodes de charge élevée ou quand le modèle génère une réponse très volumineuse. Le délai d'attente de demande par défaut est de 10 minutes.

**Que faire :**

* Réessayez la demande
* Pour les tâches longues, divisez le travail en prompts plus petits
* Si un réseau lent ou un proxy en est la cause, augmentez `API_TIMEOUT_MS` comme décrit dans [Automatic retries](#automatic-retries)
* Si les délais d'attente sont fréquents et votre réseau est par ailleurs sain, consultez [Network and connection errors](#network-and-connection-errors) ci-dessous

<h3 id="the-response-above-may-be-incomplete">
  La réponse ci-dessus peut être incomplète
</h3>

Une réponse en streaming a échoué après que Claude ait déjà produit une sortie visible. Le renvoi de la demande pourrait exécuter les mêmes appels d'outils deux fois, donc Claude Code conserve ce qui a déjà été diffusé en streaming et ajoute cet avis à la place de rejeter le tour. La variante que vous voyez indique la cause :

```text theme={null}
API Error: Server error mid-response. The response above may be incomplete.
API Error: Connection closed mid-response. The response above may be incomplete.
API Error: Response stalled mid-stream. The response above may be incomplete.
```

* `Server error mid-response` : une erreur serveur surchargée ou 5xx en milieu de flux. Cette variante nécessite Claude Code v2.1.199 ou ultérieur ; avant cela, ce cas rejetait la sortie partielle et signalait le tour entier comme une erreur.
* `Connection closed mid-response` : la connexion a été interrompue.
* `Response stalled mid-stream` : le flux a cessé d'envoyer des données.

**Que faire :**

* Lisez la réponse qui a été diffusée en streaming. Rien n'a été perdu, mais les dernières phrases ou appels d'outils peuvent manquer.
* Répondez avec `continue` pour que Claude reprenne là où il s'est arrêté
* Si la même erreur apparaît avant toute sortie visible, Claude Code réessaye la demande au lieu de la finaliser. Consultez [Automatic retries](#automatic-retries).

<h3 id="auto-mode-cannot-determine-the-safety-of-an-action">
  Le mode auto ne peut pas déterminer la sécurité d'une action
</h3>

Le modèle que [le mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) utilise pour classer les actions n'a pas pu produire une décision, donc le mode auto n'a pas approuvé l'action automatiquement. Le message que vous voyez dépend de la raison de l'échec du classificateur.

Les lectures, recherches et modifications à l'intérieur de votre répertoire de travail ignorent le classificateur, donc elles continuent à fonctionner dans tous ces cas.

Quand le modèle classificateur est surchargé :

```text theme={null}
<model> is temporarily unavailable, so auto mode cannot determine the safety of <tool> right now. Wait briefly and then try this action again.
```

**Que faire :**

* Réessayez après quelques secondes ; Claude voit le même message et réessaye généralement de lui-même
* Si les tentatives continuent d'échouer, continuez avec les tâches en lecture seule et revenez à l'action bloquée plus tard
* C'est transitoire et sans rapport avec [l'admissibilité du mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode) ; vous n'avez pas besoin de modifier les paramètres

Quand le classificateur a renvoyé une réponse non analysable :

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — run with --debug for details
```

**Que faire :**

* Réessayez l'action ; cela réussit généralement à la tentative suivante
* Exécutez `claude --debug` et répétez l'action pour voir la réponse du classificateur sous-jacente dans le journal de débogage

Quand une vérification de sécurité API distincte a bloqué la demande du classificateur en raison du contenu de la conversation antérieure :

```text theme={null}
Auto mode could not evaluate this action and is blocking it for safety — a safety check separate from auto mode blocked this request because of earlier conversation content — it isn't about the action itself — run with --debug for details
```

**Que faire :**

* Ce n'est pas une décision concernant votre action. Le contenu déjà dans votre conversation a déclenché un filtre de sécurité sur l'API quand le mode auto a envoyé la conversation au classificateur
* Le renvoi n'aidera pas ; le même contenu de conversation déclenchera le filtre à nouveau
* Basculez vers un [mode de permission](/docs/fr/permission-modes) différent pour pouvoir approuver l'action quand vous y êtes invité, ou commencez une nouvelle conversation sans le contenu déclencheur

Quand la conversation a grandi au-delà de la fenêtre de contexte du classificateur :

```text theme={null}
Auto mode classifier transcript exceeded context window — falling back to manual approval (try /compact to reduce conversation size)
```

Dans une session interactive, le mode auto revient à une invite de permission normale pour cette action afin que vous puissiez l'approuver ou la refuser manuellement. En [mode non-interactif](/docs/fr/headless), l'exécution s'arrête car la transcription ne fait que croître et le renvoi ne peut pas réussir.

**Que faire :**

* Approuvez ou refusez l'action dans l'invite qui apparaît
* Exécutez `/compact` pour réduire la taille de la conversation afin que les actions suivantes s'adaptent à nouveau à la fenêtre du classificateur

<h3 id="agent-terminated-early-due-to-an-api-error">
  Agent terminé prématurément en raison d'une erreur API
</h3>

La demande API d'un [sous-agent](/docs/fr/sub-agents) a échoué de manière terminale, par exemple parce qu'une limite d'utilisation a été atteinte ou que les tentatives pour une erreur serveur ont épuisé, donc le sous-agent s'est arrêté avant de terminer sa tâche. Ce message nécessite Claude Code v2.1.199 ou ultérieur ; avant cela, le texte d'erreur API était renvoyé à Claude comme s'il s'agissait du résultat du sous-agent.

```text theme={null}
Agent terminated early due to an API error: <error detail>
```

**Que faire :**

* Faites correspondre le détail de l'erreur après les deux points à sa propre section sur cette page, telle que [Usage limits](#usage-limits) ou [Server errors](#server-errors), et suivez les étapes de cette section
* Une fois que l'erreur sous-jacente est résolue, demandez à Claude de réessayer la tâche ou de [reprendre le sous-agent](/docs/fr/sub-agents#resume-subagents)

Quand une limite de débit, une surcharge ou une erreur serveur interrompt un sous-agent au premier plan qui a déjà produit une sortie textuelle, Claude reçoit cette sortie partielle marquée comme incomplète au lieu de cette erreur. Un sous-agent dont la seule sortie était des appels d'outils reçoit également cette erreur ; en v2.1.199, cela renvoyait un résultat partiel vide à la place. Consultez [API errors in subagents](/docs/fr/sub-agents#api-errors-in-subagents).

<h2 id="usage-limits">
  Limites d'utilisation
</h2>

Ces erreurs signifient qu'un quota lié à votre compte ou à votre plan a été atteint. Elles sont distinctes des [erreurs serveur](#server-errors), qui affectent tout le monde.

<h3 id="youve-hit-your-session-limit">
  Vous avez atteint votre limite de session
</h3>

Les plans d'abonnement incluent une allocation d'utilisation glissante. Quand elle s'épuise, vous voyez l'un de ces messages :

```text theme={null}
You've hit your session limit · resets 3:45pm
You've hit your weekly limit · resets Mon 12:00am
You've hit your Opus limit · resets 3:45pm
```

Claude Code bloque les demandes supplémentaires jusqu'à l'heure de réinitialisation indiquée dans le message. Les limites de session et hebdomadaires sont partagées entre tous les modèles, donc changer de modèle ne restaure pas l'accès. La limite Opus s'applique uniquement aux demandes Opus, donc passer à un autre modèle avec `/model` vous permet de continuer à travailler.

L'utilisation compte par rapport aux allocations de session et hebdomadaires en même temps. Une seule rafale d'activité intensive, comme un grand fanout de flux de travail, peut épuiser l'allocation hebdomadaire avant que la fenêtre de session se réinitialise.

**À faire :**

* Attendez l'heure de réinitialisation indiquée dans l'erreur
* Pour la limite Opus, exécutez `/model` et passez à un autre modèle pour continuer à travailler
* Exécutez `/usage` pour voir les limites de votre plan et quand elles se réinitialisent
* Exécutez `/usage-credits` pour acheter une utilisation supplémentaire sur Pro et Max, ou pour la demander à votre administrateur sur Team et Enterprise. Consultez [usage credits for paid plans](https://support.claude.com/en/articles/12429409-extra-usage-for-paid-claude-plans) pour savoir comment cela est facturé.
* Pour mettre à niveau votre plan vers des limites de base plus élevées, consultez [claude.com/pricing](https://claude.com/pricing)

Pour surveiller votre allocation restante avant d'atteindre la limite, ajoutez les champs `rate_limits` à une [ligne d'état personnalisée](/docs/fr/statusline#rate-limit-usage), ou dans l'application Desktop, cliquez sur l'[anneau d'utilisation](/docs/fr/desktop#check-usage) à côté du sélecteur de modèle.

<h3 id="usage-credits-required-for-1m-context">
  Crédits d'utilisation requis pour le contexte 1M
</h3>

Le modèle sélectionné utilise la fenêtre de contexte étendue 1M-token, et votre plan ne l'inclut que par le biais de crédits d'utilisation.

```text theme={null}
API Error: Usage credits required for 1M context · run /usage-credits to turn them on, or /model to switch to standard context
```

Il s'agit d'une vérification de droit, et non d'un épuisement de quota. Elle se déclenche même quand vos allocations de session et hebdomadaires ont de la capacité restante. Consultez [Extended context](/docs/fr/model-config#extended-context) pour voir quels plans incluent directement le contexte 1M et lesquels nécessitent des crédits d'utilisation.

Quand cette erreur apparaît au milieu d'une conversation parce que le contexte a dépassé 200K tokens, Claude Code compacte automatiquement la conversation en dessous de la limite de contexte standard et maintient la session à cette limite par la suite, donc aucune action n'est nécessaire. Sur les versions antérieures à v2.1.172, l'erreur s'est répétée à chaque demande ultérieure, y compris `/compact` ; exécutez `/clear` sur ces versions pour récupérer. Les étapes ci-dessous s'appliquent quand vous avez explicitement sélectionné un modèle `[1m]`.

**À faire :**

* Exécutez `/model` et sélectionnez la variante sans le suffixe `[1m]` pour revenir à la fenêtre de contexte standard
* Exécutez `/usage-credits` pour activer la facturation mesurée pour la variante 1M sur Pro et Max, ou pour la demander à votre administrateur sur Team et Enterprise
* Si l'erreur persiste après `/model`, un ID de modèle 1M peut être défini ailleurs. Consultez [There's an issue with the selected model](#theres-an-issue-with-the-selected-model) pour les emplacements de configuration à vérifier par ordre de priorité.
* Pour supprimer entièrement les variantes 1M du sélecteur de modèle, définissez [`CLAUDE_CODE_DISABLE_1M_CONTEXT=1`](/docs/fr/env-vars)

<h3 id="server-is-temporarily-limiting-requests">
  Le serveur limite temporairement les demandes
</h3>

L'API a appliqué un throttle de courte durée qui n'est pas lié à votre quota de plan.

```text theme={null}
API Error: Server is temporarily limiting requests (not your usage limit)
```

Claude Code les distingue de votre limite de plan par l'absence des en-têtes de quota unifiés qu'une réponse de limite réelle porte. À partir de v2.1.199, ceci est [réessayé automatiquement](#automatic-retries) avec backoff avant d'être affiché, quelle que soit votre méthode d'authentification. Sur les versions antérieures, une session connectée avec un abonnement claude.ai a échoué le tour à la première occurrence ; seules les authentifications par clé API et Enterprise l'ont réessayé.

**À faire :**

* Attendez brièvement et réessayez
* Vérifiez [status.claude.com](https://status.claude.com) si cela persiste

<h3 id="request-rejected-429">
  Demande rejetée (429)
</h3>

Vous avez atteint la limite de débit configurée pour votre clé API, votre projet Amazon Bedrock ou votre projet Google Cloud.

```text theme={null}
API Error: Request rejected (429) · this may be a temporary capacity issue. If it persists, check https://status.claude.com.
```

La phrase de fin indique où vérifier la santé du service et varie selon le fournisseur. Les configurations Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry nomment la page d'état du service de ce fournisseur au lieu de la page d'état Anthropic. Une `ANTHROPIC_BASE_URL` personnalisée nomme l'hôte de la passerelle.

**À faire :**

* Exécutez `/status` et confirmez que les identifiants actifs sont ceux que vous attendez. Une `ANTHROPIC_API_KEY` égarée dans votre environnement peut acheminer les demandes via une clé de niveau inférieur au lieu de votre abonnement.
* Vérifiez votre console de fournisseur pour les limites actives et demandez un niveau supérieur si nécessaire
* Pour les clés API Anthropic, consultez la [référence des limites de débit](https://platform.claude.com/docs/en/api/rate-limits) pour savoir comment fonctionnent les niveaux et comment définir des plafonds par espace de travail
* Réduisez la concurrence : abaissez [`CLAUDE_CODE_MAX_TOOL_USE_CONCURRENCY`](/docs/fr/env-vars), évitez d'exécuter de nombreux sous-agents parallèles, ou passez à un modèle plus petit avec `/model` pour les exécutions scriptées à haut volume

<h3 id="credit-balance-is-too-low">
  Le solde de crédit est trop faible
</h3>

Votre organisation Console a épuisé ses crédits prépayés.

```text theme={null}
Credit balance is too low
```

**À faire :**

* Ajoutez des crédits sur [platform.claude.com/settings/billing](https://platform.claude.com/settings/billing), et envisagez d'activer le rechargement automatique pour que le solde se reconstitue avant d'atteindre zéro
* Passez à l'authentification par abonnement avec `/login` si vous avez un plan Pro, Max, Team ou Enterprise
* Définissez des plafonds de dépenses par espace de travail dans la Console pour empêcher un seul projet de drainer le solde de l'organisation. Consultez [Manage costs effectively](/docs/fr/costs).

<h2 id="authentication-errors">
  Erreurs d'authentification
</h2>

Ces erreurs signifient que Claude Code ne peut pas prouver votre identité à l'API. Exécutez `/status` à tout moment pour voir quelle credential est actuellement active.

<h3 id="not-logged-in">
  Non connecté
</h3>

Aucune credential valide n'est disponible pour cette session.

```text theme={null}
Not logged in · Please run /login
```

**À faire :**

* Exécutez `/login` pour vous authentifier avec votre abonnement Claude ou votre compte Console
* Si vous vous attendiez à ce qu'une variable d'environnement vous authentifie, confirmez que `ANTHROPIC_API_KEY` est définie et exportée dans le shell où vous avez lancé `claude`
* Pour l'intégration continue ou l'automatisation où la connexion interactive n'est pas possible, configurez un script [`apiKeyHelper`](/docs/fr/settings#available-settings) qui récupère une clé au démarrage
* Consultez [Précédence d'authentification](/docs/fr/authentication#authentication-precedence) pour comprendre quelle credential Claude Code utilise quand plusieurs sont présentes

Si vous êtes invité à vous connecter à plusieurs reprises, consultez [Non connecté ou token expiré](/docs/fr/troubleshoot-install#not-logged-in-or-token-expired) pour les corrections d'horloge système et de Keychain macOS.

<h3 id="could-not-resolve-authentication-method">
  Impossible de résoudre la méthode d'authentification
</h3>

La session a atteint le client API sans aucune credential. Cela apparaît dans les [sessions en arrière-plan](/docs/fr/agent-view), les sessions cloud et les contextes Agent SDK où la vérification de connexion interactive ne s'exécute pas avant la première requête.

```text theme={null}
Could not resolve authentication method. Expected one of apiKey, authToken, credentials, config, or profile to be set. Or for one of the "X-Api-Key" or "Authorization" headers to be explicitly omitted
```

Avant la v2.1.174, une session en arrière-plan ou cloud assignée à un worker pré-initialisé inactif pouvait échouer de cette façon même quand des credentials valides étaient configurées. Mettez à jour pour récupérer. Sur les versions actuelles, l'erreur signifie qu'aucune credential n'était disponible pour le processus worker.

**À faire :**

* Mettez à jour vers la v2.1.174 ou ultérieure si cela apparaît dans une session en arrière-plan ou cloud et que vos credentials sont déjà configurées
* Confirmez que `ANTHROPIC_API_KEY`, `CLAUDE_CODE_OAUTH_TOKEN` ou vos credentials du fournisseur cloud sont définis dans l'environnement qui lance le worker, pas seulement dans votre shell interactif
* Pour l'Agent SDK, consultez [configuration de l'authentification](/docs/fr/agent-sdk/overview#get-started)
* Exécutez `/status` dans une session interactive dans le même environnement pour confirmer quelle source de credential se résout

<h3 id="invalid-api-key">
  Clé API invalide
</h3>

La variable d'environnement `ANTHROPIC_API_KEY` ou le script `apiKeyHelper` a retourné une clé que l'API a rejetée.

```text theme={null}
Invalid API key · Fix external API key
```

**À faire :**

* Vérifiez les fautes de frappe et confirmez que la clé n'a pas été révoquée dans la [Console](https://platform.claude.com/settings/keys)
* Exécutez `env | grep ANTHROPIC` dans le même shell. Des outils comme direnv, les plugins shell dotenv et les terminaux IDE peuvent charger une clé obsolète à partir d'un fichier `.env` dans votre projet sans que vous la définissiez explicitement.
* Déconfigurez `ANTHROPIC_API_KEY` et exécutez `/login` pour utiliser l'authentification par abonnement à la place
* Si la clé provient d'un script [`apiKeyHelper`](/docs/fr/settings#available-settings), exécutez le script directement pour confirmer qu'il imprime une clé valide sur stdout
* Exécutez `/status` pour confirmer quelle source de credential Claude Code utilise réellement

<h3 id="your-apikeyhelper-script-is-failing">
  Votre script apiKeyHelper échoue
</h3>

La commande configurée dans le paramètre [`apiKeyHelper`](/docs/fr/settings#available-settings) s'est terminée avec une erreur, a expiré ou n'a rien imprimé sur stdout. Sans une clé du script, la requête atteint l'API avec une credential d'espace réservé, et l'API la rejette avec `401`.

```text theme={null}
Your apiKeyHelper script is failing · This usually means you need to re-authenticate with your provider · Run /status to see the script's error output
```

Claude Code réexécute le script et réessaie la requête jusqu'à deux fois de plus avant d'afficher ce message, donc l'échec apparaît dans trois tentatives. Avant la v2.1.208, Claude Code dépensait le [budget de retry](#automatic-retries) complet en renvoyant la requête avec la credential d'espace réservé et signalait ensuite une erreur d'authentification générique `401` au lieu de l'échec du script.

Exécuter `/login` n'aide pas ici : la sortie du helper [prend la priorité](/docs/fr/authentication#authentication-precedence) sur une connexion enregistrée tant que le paramètre est présent.

**À faire :**

* Exécutez la commande configurée dans `apiKeyHelper` directement dans votre shell pour reproduire l'échec
* Si la commande signale une session expirée, réauthentifiez-vous auprès de votre fournisseur de credential, par exemple en vous reconnectant à votre SSO ou à votre coffre-fort de secrets
* Corrigez la commande pour qu'elle imprime la clé sur stdout et se termine avec le code 0. Consultez [rotation des credentials avec apiKeyHelper](/docs/fr/llm-gateway-connect#rotate-credentials-with-apikeyhelper) pour une configuration fonctionnelle.
* Exécutez `/status` pour confirmer que `apiKeyHelper` est la source de credential active. Chaque fois que la commande échoue, son code de sortie et sa sortie d'erreur apparaissent dans un panneau `Cloud authentication` dans le terminal.

<h3 id="this-organization-has-been-disabled">
  Cette organisation a été désactivée
</h3>

Une `ANTHROPIC_API_KEY` obsolète d'une organisation Console désactivée remplace votre connexion par abonnement.

```text theme={null}
Your ANTHROPIC_API_KEY belongs to a disabled organization · Unset the environment variable to use your other credentials
API Error: 400 ... This organization has been disabled.
```

Les variables d'environnement ont la priorité sur `/login`, donc une clé exportée dans votre profil shell ou chargée à partir d'un fichier `.env` est utilisée même quand vous avez un abonnement Pro ou Max fonctionnant. En mode non-interactif (`-p`), la clé est toujours utilisée quand elle est présente.

**À faire :**

* Déconfigurez `ANTHROPIC_API_KEY` dans le shell actuel et supprimez-la de votre profil shell, puis relancez `claude`
* Exécutez `/status` après pour confirmer que la credential active est votre abonnement
* Si aucune variable d'environnement n'est définie et que l'erreur persiste, l'organisation désactivée est celle liée à votre `/login`. Contactez le support ou connectez-vous avec un compte différent.

<h3 id="your-organization-has-disabled-api-key-authentication">
  Votre organisation a désactivé l'authentification par clé API
</h3>

Ce message nécessite Claude Code v2.1.169 ou ultérieur. L'administrateur de votre organisation Console a désactivé l'authentification par clé API, donc l'API rejette la clé que Claude Code envoie. L'indice de récupération après le `·` varie selon d'où provient la clé :

```text theme={null}
Your organization has disabled API key authentication · Run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY to use your claude.ai account instead
Your organization has disabled API key authentication · Unset ANTHROPIC_API_KEY and run /login to sign in with your claude.ai account
Your organization has disabled API key authentication · Unset the apiKeyHelper setting and run /login to sign in with your claude.ai account
```

Les variables d'environnement et `apiKeyHelper` ont la priorité sur `/login`, donc exécuter `/login` seul n'aide pas tant que l'un ou l'autre fournit toujours une clé. Consultez [Précédence d'authentification](/docs/fr/authentication#authentication-precedence).

**À faire :**

* Si le message nomme `ANTHROPIC_API_KEY`, déconfigurez-la dans le shell actuel et supprimez-la de votre profil shell ou fichier `.env`, puis relancez `claude`
* Si le message nomme `apiKeyHelper`, supprimez le paramètre [`apiKeyHelper`](/docs/fr/settings#available-settings) de votre `settings.json`
* Exécutez `/login` pour vous connecter avec votre compte claude.ai
* Exécutez `/status` après pour confirmer que la credential active est votre abonnement plutôt qu'une clé API
* Si vous avez besoin de l'authentification par clé API pour l'automatisation, demandez à votre administrateur d'organisation de la réactiver dans la Console

<h3 id="your-organization-has-disabled-claude-subscription-access">
  Votre organisation a désactivé l'accès à l'abonnement Claude
</h3>

Votre organisation Claude ne permet pas de se connecter à Claude Code avec une connexion par abonnement. Exécuter `/login` à nouveau avec le même compte retourne la même erreur.

```text theme={null}
Your organization has disabled Claude subscription access for Claude Code · Use an Anthropic API key instead, or ask your admin to enable access
```

C'est un paramètre d'organisation côté serveur, donc il ne peut pas être remplacé à partir des paramètres locaux, des variables d'environnement ou des drapeaux CLI.

L'Agent SDK et le mode non-interactif `-p` présentent cela comme le code d'erreur `oauth_org_not_allowed`.

**À faire :**

* Demandez à votre administrateur d'activer l'accès à Claude Code pour votre organisation
* Authentifiez-vous avec une clé API Console au lieu de votre abonnement. Consultez [Authentification Claude Console](/docs/fr/authentication#claude-console-authentication) pour la configuration.
* Si vous êtes l'administrateur et que vous ne voyez pas d'option pour activer l'accès, contactez le [support Anthropic](https://support.claude.com)

<h3 id="routines-are-disabled-by-your-organizations-policy">
  Les routines sont désactivées par la politique de votre organisation
</h3>

Un propriétaire de votre organisation Team ou Enterprise a désactivé les routines au niveau de l'organisation. L'erreur apparaît quand vous essayez de créer ou d'exécuter une routine, y compris à partir de `/schedule` et de l'interface utilisateur [Routines](/docs/fr/routines) sur claude.ai/code.

```text theme={null}
Routines are disabled by your organization's policy.
```

C'est un paramètre côté serveur, donc il ne peut pas être remplacé à partir des paramètres locaux, des variables d'environnement ou des drapeaux CLI.

**À faire :**

* Demandez à un propriétaire de votre organisation d'activer le bouton bascule **Routines** sur [claude.ai/admin-settings/claude-code](https://claude.ai/admin-settings/claude-code)
* Pour un travail programmé ponctuel qui ne nécessite pas de routines au niveau de l'organisation, consultez [tâches programmées](/docs/fr/scheduled-tasks)

<h3 id="remote-control-requires-the-anthropic-api">
  Remote Control nécessite l'API Anthropic
</h3>

La session ne parle pas directement à l'API Anthropic, donc il n'y a pas de backend claude.ai pour que [Remote Control](/docs/fr/remote-control) s'apparie avec.

```text theme={null}
Remote Control is only available when using Claude via api.anthropic.com.
```

Cela apparaît sur Amazon Bedrock, la plateforme Agent de Google Cloud et Microsoft Foundry. À partir de la v2.1.196, cela apparaît également quand [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) pointe vers un hôte autre que `api.anthropic.com`, comme une [passerelle LLM](/docs/fr/llm-gateway) ou un proxy, même quand vous vous connectez avec claude.ai.

**À faire :**

* Déconfigurez `ANTHROPIC_BASE_URL` et redémarrez la session, ou démarrez Remote Control à partir d'une session qui parle directement à l'API Anthropic
* Pour ce message et les autres messages de démarrage de Remote Control, consultez [Dépannage de Remote Control](/docs/fr/remote-control#troubleshooting)

<h3 id="oauth-token-revoked-or-expired">
  Token OAuth révoqué ou expiré
</h3>

Votre connexion enregistrée n'est plus valide. Un token révoqué signifie que vous vous êtes déconnecté partout ou qu'un administrateur a supprimé l'accès ; un token expiré signifie que l'actualisation automatique a échoué en cours de session.

Les deux messages signalent un rejet que l'API a retourné pour une requête que Claude Code a envoyée. Quand la connexion enregistrée a déjà été effacée après un échec d'actualisation, vous voyez [Connexion expirée](#login-expired) à la place.

```text theme={null}
OAuth token revoked · Please run /login
OAuth token has expired · Please run /login
API Error: 401 ... authentication_error
```

**À faire :**

* Exécutez `/login` pour vous reconnecter
* Si l'erreur revient dans la même session après réauthentification, exécutez d'abord `/logout` pour effacer complètement le token stocké, puis `/login`
* Pour les invites répétées à vous connecter entre les lancements, consultez les vérifications d'horloge système et de Keychain macOS dans [Dépannage](/docs/fr/troubleshoot-install#not-logged-in-or-token-expired)
* Pour les autres défaillances incluant `403 Forbidden` et les problèmes de navigateur OAuth, consultez [Connexion et authentification](/docs/fr/troubleshoot-install#login-and-authentication)

<h3 id="login-expired">
  Connexion expirée
</h3>

Claude Code a essayé de renouveler votre connexion claude.ai ou Claude Console enregistrée et le service OAuth a rejeté le token d'actualisation stocké, donc Claude Code a effacé les credentials enregistrées. Après cela, chaque requête s'arrête localement avant d'atteindre l'API, car seul `/login` peut créer de nouvelles credentials. Avant la v2.1.206, Claude Code envoyait la requête de toute façon avec quelle que soit la credential restante dans l'environnement, et chaque modèle échouait ensuite avec [Il y a un problème avec le modèle sélectionné](#theres-an-issue-with-the-selected-model) ou un 401 au lieu d'une invite à se connecter.

```text theme={null}
Login expired · Please run /login
```

En [mode non-interactif](/docs/fr/headless) (`-p`) et l'[Agent SDK](/docs/fr/agent-sdk/overview), le message se lit comme suit, et le code d'erreur structuré est `authentication_failed` :

```text theme={null}
Failed to authenticate: OAuth session expired and could not be refreshed
```

Ce n'est pas le même état que [Token OAuth révoqué ou expiré](#oauth-token-revoked-or-expired). Ces messages signalent un 401 que l'API a retourné. Claude Code lui-même produit `Login expired` pour une connexion qu'il a déjà échoué à renouveler, donc il n'envoie aucune requête.

Les sessions authentifiées avec une clé API, [`CLAUDE_CODE_OAUTH_TOKEN`](/docs/fr/env-vars) ou un fournisseur tiers n'utilisent pas la connexion enregistrée et ne voient jamais ce message.

**À faire :**

* Exécutez `/login` pour vous reconnecter. Réessayer sans vous connecter affiche le même message à chaque requête.
* En mode non-interactif, exécutez `claude` dans le même environnement, complétez `/login`, puis réexécutez votre commande. Pour l'automatisation qui ne peut pas se connecter de manière interactive, authentifiez-vous avec `ANTHROPIC_API_KEY` ou [générez un token de longue durée avec `claude setup-token`](/docs/fr/authentication#generate-a-long-lived-token).
* Si la connexion continue d'échouer, consultez [Connexion et authentification](/docs/fr/troubleshoot-install#login-and-authentication)

<h3 id="oauth-scope-requirement">
  Exigence de portée OAuth
</h3>

Le token stocké est antérieur à une portée de permission qu'une fonctionnalité plus récente nécessite. Vous voyez cela le plus souvent à partir de `/usage` et de l'indicateur d'utilisation de la ligne d'état :

```text theme={null}
OAuth token does not meet scope requirement: user:profile
```

**À faire :**

* Exécutez `/login` pour obtenir un nouveau token avec les portées actuelles. Vous n'avez pas besoin de vous déconnecter d'abord.

<h3 id="aws-credentials-expired-or-invalid">
  Les credentials AWS ont expiré ou sont invalides
</h3>

Ce message nécessite Claude Code v2.1.198 ou ultérieur et n'apparaît que quand [`awsAuthRefresh`](/docs/fr/amazon-bedrock#advanced-credential-configuration) est défini dans votre fichier de paramètres. Votre token de session AWS a expiré ou a été rejeté, et l'actualisation automatique que Claude Code a déjà exécutée n'a pas produit une credential que l'API accepte. Cela apparaît sur un 401 de [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) ou du [point de terminaison Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint), c'est ainsi que ces fournisseurs signalent un token de sécurité expiré.

L'indice d'action au milieu nomme la commande `awsAuthRefresh` de vos paramètres, donc il varie. La partie stable est le début `AWS credentials expired or invalid` :

```text theme={null}
AWS credentials expired or invalid · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · API Error: 401 ...
```

Sans `awsAuthRefresh` configuré, le même 401 affiche le message générique `Please run /login` à la place, qui ne peut pas actualiser les credentials AWS.

**À faire :**

* Exécutez la commande `awsAuthRefresh` nommée dans le message, comme `aws sso login --profile myprofile`, dans un autre terminal et complétez la connexion au navigateur, puis réessayez
* Dans une session interactive, exécutez `/login`, choisissez **plateforme tierce**, puis sélectionnez **Claude Platform on AWS · refresh credentials** sous **Utilisation de plateformes tierces** pour exécuter la même commande sans redémarrer Claude Code. Consultez [Configurer les credentials AWS](/docs/fr/claude-platform-on-aws#1-configure-aws-credentials)
* Si l'erreur se répète après que la commande d'actualisation réussisse, confirmez que l'identité est valide en dehors de Claude Code avec `aws sts get-caller-identity` dans le même shell et profil

<h3 id="aws-authentication-failed">
  L'authentification AWS a échoué
</h3>

Ce message nécessite Claude Code v2.1.198 ou ultérieur et n'apparaît que quand [`awsAuthRefresh`](/docs/fr/amazon-bedrock#advanced-credential-configuration) est défini dans votre fichier de paramètres. Votre fournisseur AWS a retourné un 403, ou [Amazon Bedrock](/docs/fr/amazon-bedrock) a retourné un 401.

Claude Code ne peut pas dire quelle cause vous avez atteinte. Amazon Bedrock signale un token de sécurité expiré comme un 403, mais un 403 est aussi comment il signale un refus d'autorisation, comme une `AccessDeniedException` d'une permission IAM manquante ou d'un modèle qui n'est pas activé pour votre compte.

Un 401 d'Amazon Bedrock atterrit également ici plutôt que sous [Les credentials AWS ont expiré ou sont invalides](#aws-credentials-expired-or-invalid), car Amazon Bedrock ne signale pas un token expiré comme un 401. Un 401 de ce point de terminaison provient généralement de quelque chose d'autre dans le chemin de la requête, comme un proxy d'entreprise.

Une actualisation de credential corrige un token expiré et ne peut pas corriger les autres causes, donc le message offre les deux :

```text theme={null}
AWS authentication failed · run /login and select "Claude Platform on AWS · refresh credentials", or run `aws sso login --profile myprofile` in another terminal · if credentials are current, check AWS permissions and model access · API Error: 403 ...
```

L'indice d'action au milieu nomme la commande `awsAuthRefresh` de vos paramètres, donc il varie. La partie stable est le début `AWS authentication failed`.

**À faire :**

* Exécutez la commande `awsAuthRefresh` nommée dans le message, ou `aws sso login`, au cas où une credential expirée serait la cause
* Si vos credentials sont actuelles, confirmez que les permissions IAM dans [Configuration IAM](/docs/fr/amazon-bedrock#iam-configuration) sont attachées à l'identité que vous utilisez et que le modèle sélectionné est activé pour votre compte et région
* Exécutez `aws sts get-caller-identity` pour confirmer quelle identité vos requêtes utilisent ; un `AWS_PROFILE` obsolète ou un profil par défaut est une cause courante d'une incompatibilité de permission

<h3 id="aws-default-chain-credential-resolve-timed-out">
  Le délai d'expiration de la résolution des credentials de la chaîne par défaut AWS a expiré
</h3>

Le fournisseur de credentials de la chaîne par défaut AWS n'a pas produit de credentials dans les 60 secondes, donc Claude Code a arrêté la résolution et a échoué la requête. L'échec est une résolution de credentials locale : la requête n'a jamais atteint [Amazon Bedrock](/docs/fr/amazon-bedrock), [Claude Platform on AWS](/docs/fr/claude-platform-on-aws) ou le [point de terminaison Mantle](/docs/fr/amazon-bedrock#use-the-mantle-endpoint). Claude Code efface son [cache de credentials](/docs/fr/amazon-bedrock#credential-caching-and-resolution-timeout) et réessaie avant que cette erreur ne fasse surface, donc au moment où vous la voyez, la chaîne s'est bloquée sur des tentatives répétées.

```text theme={null}
API Error: AWS default-chain credential resolve timed out
```

Les causes courantes sont une commande `credential_process` dans votre profil AWS qui attend une entrée qu'elle ne peut pas recevoir, et un conteneur ou une VM dont le service de métadonnées d'instance (IMDS) ne répond jamais à la sonde de la chaîne. Avant la v2.1.207, une chaîne bloquée laissait la requête en attente indéfiniment au lieu d'échouer avec ce message.

**À faire :**

* Exécutez `aws sts get-caller-identity` dans le même shell avec le même `AWS_PROFILE`. S'il se bloque également, corrigez le profil ; une commande `credential_process` qui demande de manière interactive est une cause courante.
* Complétez l'étape de connexion avant de démarrer Claude Code, par exemple `aws sso login --profile myprofile`, pour que la chaîne se résolve à partir du cache SSO local au lieu d'attendre un flux de navigateur
* Si votre chaîne exécute une connexion interactive qui a légitimement besoin de plus de 60 secondes, comme SSO avec MFA via un wrapper comme `aws-vault`, augmentez la limite en millisecondes avec [`CLAUDE_CODE_AWS_CHAIN_RESOLVE_TIMEOUT_MS`](/docs/fr/env-vars)

<h2 id="network-and-connection-errors">
  Erreurs de réseau et de connexion
</h2>

Ces erreurs signifient qu'une requête réseau de Claude Code n'a pas pu atteindre sa destination, ou que quelque chose entre Claude Code et l'API a modifié la réponse en chemin. Elles proviennent généralement de votre réseau local, d'un proxy, d'un pare-feu, ou de la politique réseau de l'environnement cloud.

<h3 id="unable-to-connect-to-api">
  Impossible de se connecter à l'API
</h3>

La connexion TCP à l'API a échoué ou ne s'est jamais complétée.

```text theme={null}
Unable to connect to API. Check your internet connection
Unable to connect to API (ECONNREFUSED)
Unable to connect to API (ECONNRESET)
Unable to connect to API (ETIMEDOUT)
fetch failed
Request timed out. Check your internet connection and proxy settings
```

Les causes courantes incluent l'absence d'accès à Internet, un VPN qui bloque `api.anthropic.com`, ou un proxy d'entreprise requis qui n'est pas configuré.

**Que faire :**

* Confirmez que vous pouvez atteindre l'hôte API depuis le même shell en exécutant `curl -I https://api.anthropic.com`. Sur Windows PowerShell, utilisez `curl.exe -I https://api.anthropic.com` pour que l'alias `Invoke-WebRequest` intégré ne soit pas utilisé.
* Si vous êtes derrière un proxy d'entreprise, définissez `HTTPS_PROXY` avant de lancer Claude Code et consultez [Configuration réseau](/docs/fr/network-config)
* Si vous routez via une passerelle LLM ou un relais, définissez [`ANTHROPIC_BASE_URL`](/docs/fr/env-vars) sur son adresse. Consultez [Connecter Claude Code à une passerelle LLM](/docs/fr/llm-gateway-connect) pour la configuration.
* Assurez-vous que votre pare-feu autorise les hôtes listés dans [Exigences d'accès réseau](/docs/fr/network-config#network-access-requirements)
* Les défaillances intermittentes sont [réessayées automatiquement](#automatic-retries) ; les défaillances persistantes indiquent un problème réseau local

Si `curl` réussit mais que Claude Code échoue toujours, la cause est généralement quelque chose entre le runtime et le réseau plutôt que le réseau lui-même :

* Sur Linux et WSL, vérifiez `/etc/resolv.conf` pour un serveur de noms inaccessible. WSL en particulier peut hériter d'un résolveur cassé de l'hôte.
* Sur macOS, un client VPN qui a été déconnecté ou désinstallé peut laisser une interface tunnel ou une règle de routage. Vérifiez `ifconfig` pour les interfaces `utun` obsolètes et supprimez l'extension réseau du VPN dans Paramètres système.
* Docker Desktop et les runtimes de conteneurs similaires peuvent intercepter le trafic sortant. Quittez-les et réessayez pour exclure cette possibilité.

<h3 id="bedrock-streaming-response-has-an-unexpected-content-type">
  La réponse de streaming Bedrock a un content-type inattendu
</h3>

Une passerelle ou un proxy entre Claude Code et [Amazon Bedrock](/docs/fr/amazon-bedrock) transforme le corps de la réponse de streaming ou son en-tête `Content-Type`. Amazon Bedrock diffuse les réponses en tant que `application/vnd.amazon.eventstream`, et Claude Code rejette une réponse de streaming réussie qui signale un content-type différent au lieu de décoder un corps qu'il ne peut pas lire. La requête n'est pas réessayée.

```text theme={null}
Bedrock streaming response has content-type "text/event-stream"; expected "application/vnd.amazon.eventstream". A gateway or proxy between Claude Code and Bedrock is likely transforming the response body — Bedrock's binary event-stream format must be passed through unmodified. Set CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1 to suppress this check while the gateway is being fixed.
```

Avant la v2.1.208, la même mauvaise configuration s'affichait sous la forme `API Error: Truncated event message received` après que la réponse entière ait été mise en mémoire tampon.

**Que faire :**

* Configurez la passerelle pour transmettre le corps de la réponse `InvokeModelWithResponseStream` et son en-tête `Content-Type` sans modification. Un intermédiaire qui réemet le flux sous forme d'événements envoyés par le serveur est une cause courante.
* Si la passerelle réécrit uniquement l'en-tête et transmet le corps binaire intact, définissez [`CLAUDE_CODE_DISABLE_BEDROCK_CONTENT_TYPE_GUARD=1`](/docs/fr/env-vars) pour ignorer la vérification jusqu'à ce que la passerelle soit corrigée. Consultez [Erreurs de streaming derrière une passerelle ou un proxy](/docs/fr/amazon-bedrock#streaming-errors-behind-a-gateway-or-proxy).

<h3 id="ssl-certificate-errors">
  Erreurs de certificat SSL
</h3>

Un proxy ou un appareil de sécurité sur votre réseau intercepte le trafic TLS avec son propre certificat, et Claude Code ne lui fait pas confiance.

```text theme={null}
Unable to connect to API: SSL certificate verification failed. Check your proxy or corporate SSL certificates
Unable to connect to API: Self-signed certificate detected
```

À partir de la v2.1.199, une défaillance de validation de certificat n'est pas réessayée, donc cette erreur apparaît à la première tentative au lieu d'après le [budget de retry](#automatic-retries) complet. Les versions antérieures ont passé quelques minutes à réessayer avant de l'afficher. Les conditions TLS transitoires, telles qu'un délai d'expiration de poignée de main, sont toujours réessayées.

Pendant `/login` et la vérification de connectivité au démarrage, la même défaillance est signalée avec le code OpenSSL et le correctif en ligne :

```text theme={null}
SSL certificate error (UNABLE_TO_GET_ISSUER_CERT_LOCALLY). If you are behind a corporate proxy or TLS-intercepting firewall, set NODE_EXTRA_CA_CERTS to your CA bundle path, or ask IT to allowlist *.anthropic.com. Run `claude doctor` for details.
```

**Que faire :**

* Exportez le bundle CA de votre organisation et pointez Claude Code dessus avec `NODE_EXTRA_CA_CERTS=/path/to/ca-bundle.pem`
* Consultez [Configuration réseau](/docs/fr/network-config#custom-ca-certificates) pour les instructions de configuration complètes
* Ne définissez pas `NODE_TLS_REJECT_UNAUTHORIZED=0`, qui désactive entièrement la validation de certificat

<h3 id="host-not-allowed-in-a-cloud-session">
  Hôte non autorisé dans une session cloud
</h3>

Une requête HTTP sortante d'une session cloud ou d'une routine a été bloquée par la politique réseau de l'environnement.

```text theme={null}
HTTP 403
x-deny-reason: host_not_allowed
```

Vous pouvez également voir un certificat TLS qui ne correspond pas au certificat réel de la destination. L'environnement cloud route le trafic sortant via un proxy qui applique la politique réseau, donc un certificat non correspondant signifie que le proxy a terminé la connexion, pas la destination.

Ce n'est pas un problème réseau côté client. Les sessions cloud et les [routines](/docs/fr/routines) s'exécutent dans un environnement en sandbox dont le trafic sortant est filtré selon la liste d'autorisation de l'environnement. L'environnement **Default** utilise l'accès **Trusted**, qui autorise la [liste d'autorisation par défaut](/docs/fr/claude-code-on-the-web#default-allowed-domains) des registres de paquets, des API de fournisseurs cloud, des registres de conteneurs et des domaines de développement courants, mais bloque tout le reste.

**Que faire :**

* Ouvrez la routine pour la modifier, ou démarrez une session cloud. Sélectionnez l'icône cloud affichant le nom de votre environnement, tel que **Default**, pour ouvrir le sélecteur. Survolez votre environnement et cliquez sur l'icône des paramètres.
* Dans la boîte de dialogue **Update cloud environment**, changez **Network access** de **Trusted** à **Custom**, puis ajoutez le domaine bloqué à **Allowed domains**. Entrez un domaine par ligne. Cochez **Also include default list of common package managers** pour conserver la [liste d'autorisation par défaut](/docs/fr/claude-code-on-the-web#default-allowed-domains) aux côtés de vos domaines personnalisés. Sélectionnez **Full** à la place si vous souhaitez un accès sans restriction.
* Cliquez sur **Save changes**. La prochaine exécution utilise la liste d'autorisation mise à jour.

Consultez [Network access](/docs/fr/claude-code-on-the-web#network-access) pour les niveaux d'accès et la liste d'autorisation par défaut. Les sessions CLI locales ne sont pas affectées par cette politique.

<h3 id="couldnt-reconnect-to-your-remote-control-session">
  Impossible de se reconnecter à votre session Remote Control
</h3>

```text theme={null}
Couldn't reconnect to your Remote Control session. Retry, or start a fresh session without --resume.
```

La reprise avec `claude --resume` ou `claude --continue` se reconnecte à la session [Remote Control](/docs/fr/remote-control) enregistrée dans cette conversation. Ce message signifie que la reconnexion a échoué pour une raison qui peut être temporaire, telle qu'une interruption réseau ou une erreur serveur, donc Claude Code ne peut pas confirmer si la session distante existe toujours. Votre session locale continue de s'exécuter sans Remote Control.

**Que faire :**

* Exécutez `/remote-control` pour réessayer la connexion
* Démarrez Claude Code sans `--resume` pour créer une nouvelle session Remote Control
* Pour les autres messages de démarrage Remote Control, consultez [Dépanner Remote Control](/docs/fr/remote-control#troubleshooting)

Vous ne verrez pas ce message lorsque le serveur confirme que la session précédente n'existe plus ; Claude Code en crée une nouvelle dans ce cas. Avant la v2.1.200, toute défaillance de reconnexion créait une nouvelle session Remote Control, ce qui laissait des sessions supplémentaires dans la liste des sessions sur claude.ai/code.

<h2 id="request-errors">
  Erreurs de requête
</h2>

Ces erreurs concernent le contenu de votre requête. La plupart proviennent de l'API après qu'elle ait rejeté la requête ; quelques-unes sont produites localement par Claude Code avant l'envoi de toute requête.

<h3 id="prompt-is-too-long">
  Le prompt est trop long
</h3>

La conversation plus les fichiers joints dépassent la fenêtre de contexte du modèle.

```text theme={null}
Prompt is too long
```

**À faire :**

* Exécutez `/compact` pour résumer les tours précédents et libérer de l'espace, ou `/clear` pour recommencer à zéro
* Exécutez `/context` pour voir une ventilation de ce qui consomme la fenêtre : prompt système, outils, fichiers mémoire et messages
* Désactivez les serveurs MCP que vous n'utilisez pas avec `/mcp disable <name>` pour supprimer leurs définitions d'outils du contexte
* Réduisez les fichiers mémoire `CLAUDE.md` volumineux, ou déplacez les instructions dans les [règles limitées au chemin](/docs/fr/memory#path-specific-rules) qui se chargent uniquement lorsqu'elles sont pertinentes
* Les sous-agents héritent de chaque définition d'outil MCP de la session parent, ce qui peut remplir leur fenêtre de contexte avant le premier tour. Désactivez les serveurs MCP que vous n'utilisez pas avant de générer des sous-agents.
* L'auto-compact est activé par défaut et empêche normalement cette erreur. Si vous avez défini [`DISABLE_AUTO_COMPACT`](/docs/fr/env-vars), réactivez-le ou exécutez `/compact` manuellement avant que la fenêtre ne se remplisse.

Consultez [Explorez la fenêtre de contexte](/docs/fr/context-window) pour une vue interactive de la façon dont le contexte se remplit.

<h3 id="error-during-compaction-conversation-too-long">
  Erreur lors de la compaction : Conversation trop longue
</h3>

`/compact` lui-même a échoué car il n'y a pas assez d'espace libre dans le contexte pour contenir le résumé qu'il produit.

```text theme={null}
Error during compaction: Conversation too long. Press esc twice to go up a few messages and try again.
```

Cela peut se produire lorsque la fenêtre est déjà pleine au moment où l'auto-compact se déclenche, ou lorsque vous exécutez `/compact` après avoir vu `Prompt is too long`.

**À faire :**

* Appuyez deux fois sur Échap pour ouvrir la liste des messages et revenir plusieurs tours en arrière. Cela supprime les messages les plus récents du contexte. Ensuite, exécutez `/compact` à nouveau.
* Si revenir en arrière ne libère pas assez d'espace, exécutez `/clear` pour démarrer une nouvelle session. Votre conversation précédente est conservée et peut être rouverte avec `/resume`.

<h3 id="request-too-large">
  Requête trop volumineuse
</h3>

Le corps de la requête brute a dépassé la limite d'octets de l'API avant la tokenisation, généralement en raison d'un fichier volumineux collé ou d'une pièce jointe.

```text theme={null}
Request too large (max 30 MB). Double press esc to go back and remove or shrink the attached content.
```

Il s'agit d'une limite de taille sur la requête HTTP, distincte de la [limite de la fenêtre de contexte](#prompt-is-too-long).

**À faire :**

* Appuyez deux fois sur Échap et revenez en arrière au-delà du tour qui a ajouté le contenu surdimensionné
* Référencez les fichiers volumineux par chemin au lieu de coller leur contenu, afin que Claude puisse les lire par morceaux
* Pour les images, consultez [L'image était trop volumineuse](#image-was-too-large) ci-dessous

<h3 id="image-was-too-large">
  L'image était trop volumineuse
</h3>

Une image collée ou jointe dépasse les limites de taille ou de dimension de l'API.

```text theme={null}
Image was too large. Double press esc to go back and try again with a smaller image.
API Error: 400 ... image dimensions exceed max allowed size
```

Claude Code remplace l'image non traitée par un espace réservé textuel et réessaie, de sorte que les messages suivants réussissent. Sur les versions antérieures à 2.1.142, une image collée pouvait rester dans la conversation et répéter la même erreur à chaque message suivant. Pour récupérer sur ces versions, appuyez deux fois sur Échap et revenez en arrière au-delà du tour où l'image a été ajoutée.

**À faire :**

* Redimensionnez l'image avant de la coller. L'API accepte les images jusqu'à 8 000 pixels sur le côté le plus long pour une seule image, ou 2 000 pixels lorsque de nombreuses images sont en contexte.
* Prenez une capture d'écran plus serrée de la région pertinente au lieu de l'écran complet

<h3 id="unable-to-resize-image">
  Impossible de redimensionner l'image
</h3>

Claude Code n'a pas pu réduire une image jointe avant de l'envoyer à l'API.

```text theme={null}
Unable to resize image — image processing is unavailable and dimensions could not be read from the file header. Please convert the image to PNG, JPEG, GIF, or WebP.
Unable to resize image — dimensions exceed the 2000x2000px limit and image processing failed. Please resize the image to reduce its pixel dimensions.
Unable to resize image (… raw, … base64). The image exceeds the … API limit and compression failed. Please resize the image manually or use a smaller image.
Unable to resize image — could not verify image dimensions are within the 2000x2000px API limit.
```

Claude Code redimensionne normalement les grandes images automatiquement. Ces erreurs signifient que le processeur d'image natif n'a pas pu se charger ou a renvoyé une erreur, de sorte que l'image n'a pas pu être redimensionnée pour s'adapter aux limites de l'API.

**À faire :**

* Si le message vous demande de convertir l'image, convertissez-la en PNG, JPEG, GIF ou WebP et joignez-la à nouveau. Claude Code peut vérifier les dimensions de ces formats sans le processeur d'image.
* Si le message signale une limite de dimension ou de taille, redimensionnez ou recompressez l'image en dessous de cette limite avant de la joindre.

<h3 id="pdf-errors">
  Erreurs PDF
</h3>

Le PDF que vous avez joint n'a pas pu être traité.

```text theme={null}
PDF too large (max 100 pages, 32 MB). Try splitting it or extracting text first.
PDF is password protected. Try removing protection or extracting text first.
The PDF file was not valid. Try converting to a different format first.
```

**À faire :**

* Pour les PDF surdimensionnés, demandez à Claude de lire une plage de pages avec l'outil Read au lieu de joindre le fichier entier, ou extrayez le texte avec un outil comme `pdftotext` et référencez le fichier de sortie par chemin
* Pour les PDF protégés ou invalides, supprimez le mot de passe ou réexportez le fichier depuis son application source, puis réessayez

<h3 id="extra-inputs-are-not-permitted">
  Les entrées supplémentaires ne sont pas autorisées
</h3>

Un proxy ou une passerelle LLM entre Claude Code et l'API a supprimé l'en-tête de requête `anthropic-beta`, de sorte que l'API a rejeté les champs qui en dépendent.

```text theme={null}
API Error: 400 ... Extra inputs are not permitted ... context_management
API Error: 400 ... Extra inputs are not permitted ... tools.0.custom.input_examples
API Error: 400 ... Unexpected value(s) for the `anthropic-beta` header
```

Claude Code envoie des champs bêta uniquement tels que `context_management`, `effort` et les `input_examples` d'outils aux côtés d'un en-tête `anthropic-beta` qui les active. Lorsqu'une passerelle transfère le corps mais supprime l'en-tête, l'API voit des champs qu'elle ne reconnaît pas.

**À faire :**

* Configurez votre passerelle pour transférer l'en-tête `anthropic-beta`. Consultez [feature pass-through](/docs/fr/llm-gateway-protocol#feature-pass-through) pour savoir ce que les passerelles doivent transférer.
* En dernier recours, définissez [`CLAUDE_CODE_DISABLE_EXPERIMENTAL_BETAS=1`](/docs/fr/env-vars) avant de lancer. Cela désactive les fonctionnalités qui nécessitent l'en-tête bêta afin que les requêtes réussissent via une passerelle qui ne peut pas le transférer.

<h3 id="theres-an-issue-with-the-selected-model">
  Il y a un problème avec le modèle sélectionné
</h3>

Le nom du modèle configuré n'a pas été reconnu ou votre compte n'a pas accès à celui-ci. À partir de v2.1.160, l'indice de fin, montré ici sous sa forme interactive, varie selon la surface.

```text theme={null}
There's an issue with the selected model (claude-...). It may not exist or you may not have access to it. Run /model to pick a different model.
```

**À faire :**

* **CLI interactif** : exécutez `/model` pour choisir parmi les modèles disponibles pour votre compte.
* **Mode non interactif (`-p`)** : passez `--model` avec un alias ou un ID valide, ou définissez [`ANTHROPIC_MODEL`](/docs/fr/env-vars). Le texte d'erreur affiche `Run --model` sur cette surface.
* **Agent SDK** : le texte d'erreur omet l'indice car le modèle est défini par programmation. Définissez [`model` sur `Options`](/docs/fr/agent-sdk/typescript#options) en TypeScript ou [`ClaudeAgentOptions(model=...)`](/docs/fr/agent-sdk/python#claudeagentoptions) en Python, et gérez l'erreur structurée `model_not_found` pour afficher votre propre retry ou sélecteur de modèle.
* Utilisez un alias tel que `sonnet` ou `opus` au lieu d'un ID complet avec version. Les alias se résolvent en une valeur par défaut maintenue afin qu'ils ne deviennent pas obsolètes. Consultez [Configuration du modèle](/docs/fr/model-config).
* Si le mauvais modèle continue à revenir dans la CLI, un ID obsolète est défini quelque part. Vérifiez dans [l'ordre de priorité](/docs/fr/model-config#setting-your-model) : l'indicateur `--model`, la variable d'environnement `ANTHROPIC_MODEL`, puis le champ `model` dans `.claude/settings.local.json`, le `.claude/settings.json` de votre projet, et `~/.claude/settings.json`. Supprimez la valeur obsolète et Claude Code revient à la valeur par défaut de votre compte.
* Claude Code signale une connexion claude.ai expirée comme [Connexion expirée](#login-expired), pas comme cette erreur. Avant v2.1.206, une connexion expirée qui ne pouvait plus être actualisée échouait avec chaque modèle avec cette erreur ; exécutez `/login` si vous voyez cela sur une version plus ancienne.
* Pour les déploiements Google Cloud's Agent Platform, consultez [Dépannage de Google Cloud's Agent Platform](/docs/fr/google-vertex-ai#troubleshooting).

<h3 id="model-is-not-a-recognized-model-id">
  Le modèle n'est pas un ID de modèle reconnu
</h3>

La chaîne de modèle que vous avez transmise à un changement de modèle n'est pas un alias de modèle, un ID de modèle que cette version de Claude Code connaît, ou un ID qui commence par `claude-`. Les causes habituelles sont une faute de frappe dans l'ID, un nom d'affichage tel que `Sonnet 5` où l'ID `claude-sonnet-5` est attendu, ou un alias que seules les versions plus récentes de Claude Code reconnaissent. Claude Code rejette immédiatement le changement. Avant v2.1.200, Claude Code enregistrait la chaîne et échouait à la requête suivante avec [Il y a un problème avec le modèle sélectionné](#theres-an-issue-with-the-selected-model).

```text theme={null}
Model "claud-sonnet-5" is not a recognized model id. Did you mean 'claude-sonnet-5'?
```

L'indice de fin nomme l'alias ou l'ID de modèle le plus proche. Lorsque rien n'est assez proche, il lit `Run /model to see available models.` à la place.

Claude Code produit cette erreur localement au moment où le changement est demandé, avant toute requête API. Elle s'applique lorsqu'un modèle est défini via la méthode [Agent SDK](/docs/fr/agent-sdk/typescript) `setModel()` ou par une application telle que l'[application de bureau](/docs/fr/desktop) qui exécute la CLI Claude Code pour vous.

**À faire :**

* Exécutez `/model` sans argument pour ouvrir le sélecteur et choisir parmi les modèles disponibles pour votre compte, puis transmettez l'alias ou l'ID affiché là
* Si vous avez utilisé un alias qu'une version plus récente de Claude Code supporte, exécutez `claude update`. Un ID complet qui commence par `claude-` réussit cette vérification même lorsque le modèle est plus récent que votre version de Claude Code, donc la mise à niveau n'est pas nécessaire pour ceux-ci.
* Un modèle enregistré avant v2.1.200 n'est pas réparé par cette vérification. Si une valeur obsolète continue à revenir, supprimez-la des emplacements listés sous [Il y a un problème avec le modèle sélectionné](#theres-an-issue-with-the-selected-model).
* La vérification s'exécute uniquement sur l'API Anthropic. Sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry, [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), et derrière une [passerelle LLM](/docs/fr/llm-gateway) ou une `ANTHROPIC_BASE_URL` personnalisée, votre fournisseur ou passerelle définit les noms de modèles, de sorte que Claude Code accepte n'importe quelle chaîne et la transmet.

<h3 id="claude-opus-is-not-available-with-the-claude-pro-plan">
  Claude Opus n'est pas disponible avec le plan Claude Pro
</h3>

Votre plan d'abonnement actif n'inclut pas le modèle que vous avez sélectionné.

```text theme={null}
Claude Opus is not available with the Claude Pro plan · Select a different model in /model
```

**À faire :**

* Exécutez `/model` et sélectionnez un modèle que votre plan inclut
* Si vous avez récemment mis à niveau votre plan et voyez toujours ceci, exécutez `/logout` puis `/login`. Le jeton stocké reflète votre plan au moment où vous vous êtes connecté, de sorte que la mise à niveau sur le web ne prend effet dans une session existante que lorsque vous vous réauthentifiez.
* Consultez [claude.com/pricing](https://claude.com/pricing) pour voir quels modèles chaque plan inclut

<h3 id="model-is-restricted-by-your-organizations-settings">
  Le modèle est restreint par les paramètres de votre organisation
</h3>

Votre administrateur d'organisation a désactivé ce modèle dans la console d'administration claude.ai, ou il est exclu par une liste d'autorisation [`availableModels`](/docs/fr/model-config#restrict-model-selection) dans les paramètres gérés. Lorsque le modèle restreint a été défini avec `--model`, `ANTHROPIC_MODEL`, ou le paramètre `model`, Claude Code substitue un modèle autorisé et continue. Taper `/model <name>` pour un modèle restreint est rejeté avec `Run /model to choose a different model.` et la session conserve son modèle actuel.

```text theme={null}
Model "claude-opus-4-8" is restricted by your organization's settings. Using claude-sonnet-4-6 instead.
```

Claude Code traite un alias de famille de modèles, l'un de `opus`, `sonnet`, `haiku`, ou `fable`, comme une demande de cette famille plutôt que de sa version la plus récente. Sur l'API Anthropic et sur [Claude Platform on AWS](/docs/fr/claude-platform-on-aws), un alias de famille restreint se résout à la version la plus récente de la famille que votre organisation et la liste d'autorisation `availableModels` permettent, et l'avis de substitution nomme cette version. Claude Code rejette `/model <alias>` uniquement lorsque chaque version de la famille est restreinte. Avant v2.1.205, un alias de famille était substitué ou rejeté en fonction de sa version la plus récente seule, même lorsqu'une version plus ancienne de la même famille était autorisée.

**À faire :**

* Exécutez `/model` pour choisir parmi les modèles que votre organisation autorise. Les modèles restreints sont masqués du sélecteur.
* Si le modèle restreint a été défini dans `--model`, `ANTHROPIC_MODEL`, ou le champ `model` d'un fichier de paramètres, supprimez ou mettez à jour cette valeur afin que l'avis ne se reproduise pas à chaque lancement
* Si vous avez besoin d'accès au modèle restreint, demandez à votre administrateur d'organisation de l'activer. Consultez [Restrictions de modèle d'organisation](/docs/fr/model-config#organization-model-restrictions).

<h3 id="thinking-type-enabled-is-not-supported-for-this-model">
  thinking.type.enabled n'est pas supporté pour ce modèle
</h3>

Votre version de Claude Code est plus ancienne que le minimum pour Sonnet 5, Opus 4.8, ou Opus 4.7. La CLI a envoyé une configuration de réflexion que le modèle n'accepte plus.

```text theme={null}
API Error: 400 ... "thinking.type.enabled" is not supported for this model. Use "thinking.type.adaptive" and "output_config.effort" to control thinking behavior.
```

**À faire :**

* Exécutez `claude update` et redémarrez Claude Code. Opus 4.7 nécessite v2.1.111 ou ultérieur. Opus 4.8 nécessite v2.1.154 ou ultérieur. Sonnet 5 nécessite v2.1.197 ou ultérieur
* Si vous ne pouvez pas mettre à niveau, exécutez `/model` et sélectionnez Opus 4.6 ou Sonnet 4.6 à la place
* Si vous rencontrez ceci dans l'[Agent SDK](/docs/fr/agent-sdk/overview), mettez à niveau le package SDK à la place. Opus 4.8 nécessite TypeScript SDK v0.3.154 ou ultérieur et Python SDK v0.2.88 ou ultérieur. Sonnet 5 nécessite TypeScript SDK v0.3.197 ou ultérieur

<h3 id="thinking-budget-exceeds-output-limit">
  Le budget de réflexion dépasse la limite de sortie
</h3>

Le budget de réflexion étendue configuré dépasse la longueur de réponse maximale, il n'y a donc pas de place pour la réponse réelle.

```text theme={null}
API Error: 400 ... max_tokens must be greater than thinking.budget_tokens
```

Claude Code ajuste ces valeurs automatiquement sur l'API Anthropic. Vous voyez généralement cette erreur sur Amazon Bedrock ou Google Cloud's Agent Platform lorsque [`MAX_THINKING_TOKENS`](/docs/fr/env-vars) est défini plus haut que la limite de sortie du fournisseur, ou lorsque le mode plan augmente le budget de réflexion.

**À faire :**

* Abaissez `MAX_THINKING_TOKENS`, ou augmentez [`CLAUDE_CODE_MAX_OUTPUT_TOKENS`](/docs/fr/env-vars) au-dessus du budget de réflexion
* Consultez [Réflexion étendue](/docs/fr/model-config#extended-thinking) pour savoir comment le budget interagit avec la longueur de sortie

<h3 id="tool-use-or-thinking-block-mismatch">
  Décalage de bloc d'utilisation d'outil ou de réflexion
</h3>

L'historique de conversation a atteint l'API dans un état incohérent, généralement après qu'un appel d'outil ait été interrompu ou qu'un tour ait été modifié en cours de flux.

```text theme={null}
API Error: 400 due to tool use concurrency issues. Run /rewind to recover the conversation.
API Error: 400 ... unexpected `tool_use_id` found in `tool_result` blocks
API Error: 400 ... thinking blocks ... cannot be modified
```

Les trois variantes signifient la même chose : la séquence de blocs `tool_use`, `tool_result` et `thinking` dans l'historique ne correspond plus à ce que l'API attend.

**À faire :**

* Si vous utilisez Opus 4.7 ou Opus 4.8, exécutez d'abord `claude update`. Les versions antérieures à v2.1.156 peuvent déclencher cette erreur lors de l'utilisation normale d'outils, et `/rewind` ne la supprime pas.
* Exécutez `/rewind`, ou appuyez deux fois sur Échap, pour revenir à un point de contrôle avant le tour corrompu et continuer à partir de là. Consultez [Points de contrôle](/docs/fr/checkpointing) pour savoir comment les points de contrôle sont créés et restaurés.

<h3 id="usage-policy-refusal">
  Refus de la politique d'utilisation
</h3>

L'API a refusé de répondre car le contenu de la conversation a déclenché une vérification de la [Politique d'utilisation](https://www.anthropic.com/legal/aup). Le message inclut un ID de requête que vous pouvez citer au support si vous pensez que le refus est incorrect.

```text theme={null}
API Error: Claude Code is unable to respond to this request, which appears to violate our Usage Policy (https://www.anthropic.com/legal/aup). Please double press esc to edit your last message or start a new session for Claude Code to assist with a different task.
```

La vérification évalue la conversation complète, pas seulement votre dernier prompt, de sorte que l'envoi d'un nouveau message dans la même session réactive généralement le même refus. Il en va de même après la sortie et la réouverture de la session avec `--continue` ou `--resume`, puisque la transcription sur disque contient toujours le contenu déclencheur. Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), et [Microsoft Foundry](/docs/fr/microsoft-foundry), ce message couvre également les requêtes que les mesures de sécurité du modèle ont signalées comme un sujet de cybersécurité. Consultez [Les mesures de sécurité ont signalé un sujet de cybersécurité](#safety-measures-flagged-a-cybersecurity-topic).

**À faire :**

* Appuyez deux fois sur Échap ou exécutez `/rewind` pour revenir à un point de contrôle avant le tour qui a déclenché le refus, puis reformulez ou adoptez une approche différente. Consultez [Points de contrôle](/docs/fr/checkpointing).
* Si vous ne pouvez pas identifier quel tour l'a causé, exécutez `/clear` pour démarrer une nouvelle conversation dans le même projet. Votre conversation précédente est conservée sur disque et reste disponible dans `/resume`.
* En [mode non interactif](/docs/fr/headless) (`-p`), où la rembobinage n'est pas disponible, réessayez avec un prompt reformulé dans une nouvelle session sans `--continue`. Les vérifications de politique varient selon le modèle, de sorte que le passage à un modèle différent avec `--model` peut également résoudre le refus dans certains cas.

<h3 id="safety-measures-flagged-a-cybersecurity-topic">
  Les mesures de sécurité ont signalé un sujet de cybersécurité
</h3>

Les mesures de sécurité du modèle ont signalé le contenu de la conversation comme un sujet de cybersécurité. Le message nomme le modèle qui a signalé la requête :

```text theme={null}
API Error: Opus 4.8 has safety measures that flagged this message for a cybersecurity topic. To learn about the Cyber Verification Program and apply for access, visit our help center: https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude.

If you were not engaging in a cybersecurity topic, please send feedback via /feedback.
```

Le message renvoie au [Programme de vérification de la cybersécurité](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude), qui accorde l'accès pour les travaux de cybersécurité légitimes. La protection elle-même est côté serveur et antérieure à v2.1.203 ; cette version a changé uniquement la formulation du message et la page vers laquelle il renvoie.

Ce que vous voyez dépend de votre fournisseur et de votre mode :

* Sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai), et [Microsoft Foundry](/docs/fr/microsoft-foundry), un drapeau de cybersécurité produit le message [Refus de la politique d'utilisation](#usage-policy-refusal) à la place.
* [Le mode non interactif](/docs/fr/headless) omet la phrase `/feedback`.

Avant v2.1.203, le message lisait `<model>'s safeguards flagged this message for a cybersecurity topic. If your work requires this access, you can apply for an exemption:` suivi d'un lien de formulaire d'exemption.

**À faire :**

* Si votre travail nécessite ce contenu, demandez l'accès via le [Programme de vérification de la cybersécurité](https://support.claude.com/en/articles/14604842-real-time-cyber-safeguards-on-claude)
* Si votre requête n'était pas sur un sujet de cybersécurité, exécutez `/feedback` pour signaler le faux positif
* Pour continuer à travailler dans la même session, appuyez deux fois sur Échap ou exécutez `/rewind` pour revenir à un point de contrôle avant le tour qui a déclenché le drapeau, puis adoptez une approche différente. Consultez [Points de contrôle](/docs/fr/checkpointing).

<h2 id="installation-errors">
  Erreurs d'installation
</h2>

Ces erreurs apparaissent lors de l'installation ou de la mise à jour de Claude Code, à partir du [script d'installation](/docs/fr/setup#install-claude-code), `claude install`, ou `claude update`. Pour les problèmes de `command not found`, PATH, permission et TLS lors de la configuration, consultez [Dépannage de l'installation et de la connexion](/docs/fr/troubleshoot-install).

<h3 id="installation-was-killed-before-it-could-finish">
  L'installation a été interrompue avant de pouvoir se terminer
</h3>

Le script d'installation signale quand l'étape `claude install` est terminée par un signal. Sur Linux, le code de sortie 137 signifie que le processus a reçu SIGKILL, et sur un hôte à faible mémoire, c'est généralement le tueur de mémoire insuffisante (OOM) du noyau. Le script affiche cette explication et se termine avec le code 137 :

```text theme={null}
Installation was killed before it could finish (exit code 137). This usually means the system ran out of memory.
Claude Code needs roughly 512MB of free memory to install. Free up memory, then run this script again.
```

Pour tout autre signal fatal, et pour le code de sortie 137 sur macOS, le script affiche `Installation was killed before it could finish (exit code <N>)` avec le code de sortie réel et omet l'explication de mémoire insuffisante. Le message provient du script d'installation que macOS et Linux utilisent, qui couvre également les installations à l'intérieur de WSL ; les scripts d'installation Windows natifs ne l'affichent jamais. Avant la v2.1.200, le script se terminait avec seulement la ligne `Killed` du shell.

**Que faire :**

* Arrêtez les autres processus pour libérer de la mémoire, puis relancez le programme d'installation
* Ajoutez de l'espace d'échange ou passez à une instance plus grande. Consultez [Installation interrompue sur les serveurs Linux à faible mémoire](/docs/fr/troubleshoot-install#install-killed-on-low-memory-linux-servers) pour les commandes de fichier d'échange.

<h3 id="the-connection-dropped-while-downloading-the-update">
  La connexion s'est interrompue lors du téléchargement de la mise à jour
</h3>

La connexion au serveur de téléchargement s'est fermée pendant que `claude install`, `claude update`, ou le [programme de mise à jour automatique](/docs/fr/setup#auto-updates) récupérait le binaire Claude Code, et les tentatives n'ont pas récupéré. Claude Code réessaie le téléchargement quand la connexion s'interrompt, le transfert s'arrête, ou le fichier téléchargé échoue sa somme de contrôle, jusqu'à trois tentatives au total. Une erreur HTTP complétée, comme un 404, n'est pas réessayée car le serveur a déjà répondu. Avant la v2.1.202, une seule connexion interrompue échouait le téléchargement immédiatement avec l'erreur simple `aborted` au lieu de réessayer.

```text theme={null}
The connection dropped while downloading the update (attempt 3/3: aborted). Check your network — proxies sometimes cut off large downloads.
```

Le texte entre parenthèses nomme quelle tentative a échoué et l'erreur réseau sous-jacente. `claude update` précède le message avec `Error: Failed to install native update` sur stderr.

Un téléchargement qui reste connecté mais ne se termine pas dans les 10 minutes échoue avec `Download timed out: exceeded the total deadline` à la place. Claude Code ne réessaie pas un téléchargement qui a expiré, car une connexion trop lente pour se terminer dans le délai ne se terminera pas lors d'une tentative immédiate non plus. Les étapes ci-dessous s'appliquent aux deux messages. Avant la v2.1.205, le même délai de 10 minutes était signalé comme le générique `timeout of 600000ms exceeded` du client HTTP.

La cause habituelle est un proxy ou une passerelle qui ferme un long transfert avant qu'il ne se termine. Le binaire Claude Code est un grand téléchargement, donc une limite de connexion proxy qui n'affecte jamais le trafic API normal peut quand même l'interrompre.

**Que faire :**

* Exécutez `claude update` à nouveau. Sur un réseau par ailleurs sain, le téléchargement réussit généralement à la prochaine exécution. Pour le message d'expiration, exécutez-le à nouveau à partir d'un réseau plus rapide ou moins limité.
* Si votre réseau nécessite un proxy, définissez `HTTPS_PROXY` avant d'exécuter le programme d'installation ou `claude update`. Consultez [Vérifier la connectivité réseau](/docs/fr/troubleshoot-install#check-network-connectivity).
* Si un proxy d'entreprise continue de fermer le transfert, demandez à votre équipe réseau d'autoriser le téléchargement complet depuis `downloads.claude.ai`. Consultez [Exigences d'accès réseau](/docs/fr/network-config#network-access-requirements).
* Exécutez `claude doctor` à partir de votre shell pour les diagnostics d'installation

<h2 id="command-line-errors">
  Erreurs de ligne de commande
</h2>

Ces erreurs proviennent de la ligne de commande `claude` et de ses sous-commandes. Claude Code les affiche avant d'exécuter votre prompt ou d'envoyer une requête API.

<h3 id="conflict-between-bg-and-print">
  Conflit entre --bg et --print
</h3>

Ce message nécessite Claude Code v2.1.198 ou version ultérieure. Vous avez combiné `--bg` avec `-p` ou `--print` dans la même invocation `claude`. `--bg` démarre une [session en arrière-plan](/docs/fr/agent-view#from-your-shell) à laquelle vous vous connectez ultérieurement avec `claude agents`, tandis que `--print` s'exécute [de manière non interactive](/docs/fr/headless) et ne démarre jamais la session interactive à laquelle `claude agents` se connecte. Avant la v2.1.198, cette combinaison créait silencieusement une tâche en arrière-plan qui ne pouvait jamais être attachée.

```text theme={null}
--bg and --print conflict: --print never starts the interactive session that `claude agents` attaches to, so the job would be unattachable. The prompt is the positional — drop --print: `claude --bg '<task>'`.
```

**À faire :**

* Supprimez `-p` ou `--print`. `--bg` prend le prompt comme argument positionnel, donc `claude --bg "<task>"` est la commande complète. Voir [Dispatch new agents from your shell](/docs/fr/agent-view#from-your-shell).
* Pour exécuter le prompt de manière non interactive et imprimer le résultat au lieu de créer une session en arrière-plan, supprimez `--bg` et exécutez `claude -p "<task>"`

<h3 id="the-json-schema-value-is-not-a-valid-json-schema">
  La valeur --json-schema n'est pas un JSON Schema valide
</h3>

Le schéma que vous avez transmis à [`--json-schema`](/docs/fr/cli-reference#cli-flags) en [mode non interactif](/docs/fr/headless#get-structured-output) a échoué la compilation JSON Schema, donc `claude` se termine avec le code 1 au lieu d'exécuter le prompt. Avant la v2.1.205, un schéma invalide produisait une sortie non structurée sans erreur, et tout schéma utilisant le mot-clé `format` était traité comme invalide.

```text theme={null}
Error: --json-schema is not a valid JSON Schema: data/type must be equal to one of the allowed values
```

Le texte après le deuxième deux-points est le diagnostic du validateur et nomme le mot-clé ou l'emplacement qui a échoué. Les schémas qui utilisent le mot-clé `format`, comme `"format": "email"`, sont valides : Claude Code accepte `format` comme annotation et ne l'applique pas.

Claude Code exécute deux vérifications avant la compilation du schéma : il rejette une valeur qui n'est pas du JSON analysable avec `Error: --json-schema is not valid JSON`, et du JSON valide qui n'est pas un objet avec `Error: --json-schema must be a JSON object`.

**À faire :**

* Corrigez la partie du schéma que le diagnostic nomme, puis réexécutez la commande
* Si le diagnostic est `schema too large`, réduisez l'imbrication du schéma et la réutilisation de `$ref`
* Voir [Get structured output](/docs/fr/headless#get-structured-output) pour un schéma et une commande fonctionnels

<h3 id="could-not-import-a-server-from-claude-desktop">
  Impossible d'importer un serveur depuis Claude Desktop
</h3>

Claude Code n'a pas pu ajouter l'un des serveurs que vous avez sélectionnés dans `claude mcp add-from-claude-desktop`. La commande importe toujours les autres serveurs sélectionnés et affiche une ligne par serveur qu'elle n'a pas pu ajouter. Avant la v2.1.205, le premier serveur qui a échoué a arrêté l'importation et aucun des serveurs sélectionnés n'a été ajouté.

```text theme={null}
Could not import my server: Invalid name my server. Names can only contain letters, numbers, hyphens, and underscores.
```

Le texte après le nom du serveur est la raison. La plus courante est la vérification du nom : Claude Desktop autorise les caractères dans les noms de serveurs, tels que les espaces et les points, que `claude mcp` restreint aux lettres, chiffres, traits d'union et traits de soulignement. D'autres raisons incluent une configuration de serveur qui échoue la validation et un serveur bloqué par la [politique MCP](/docs/fr/managed-mcp) de votre organisation.

**À faire :**

* Renommez le serveur dans `claude_desktop_config.json` pour utiliser uniquement des lettres, des chiffres, des traits d'union et des traits de soulignement, puis exécutez `claude mcp add-from-claude-desktop` à nouveau
* Ajoutez ce serveur directement avec `claude mcp add` ou `claude mcp add-json` sous un nom valide. Voir [Import MCP servers from Claude Desktop](/docs/fr/mcp#import-mcp-servers-from-claude-desktop).

<h3 id="mcp-permission-prompt-tool-not-found">
  Outil de prompt de permission MCP introuvable
</h3>

L'outil que vous avez transmis à [`--permission-prompt-tool`](/docs/fr/cli-reference#cli-flags) ne figurait pas parmi les outils MCP connectés lorsque l'exécution a d'abord eu besoin d'une décision de permission, soit parce que son serveur ne s'est jamais connecté, soit parce qu'aucun serveur connecté n'expose un outil portant ce nom. Claude Code envoie toujours votre prompt : l'exécution [non interactive](/docs/fr/headless) se termine avec cette erreur et le code de sortie 1 au premier appel d'outil qui nécessite une approbation, donc elle ne produit aucune réponse même si la requête a été effectuée. Avant le premier prompt, Claude Code attend jusqu'à 30 secondes, le délai d'expiration de la connexion par serveur défini par [`MCP_TIMEOUT`](/docs/fr/env-vars), pour que ce serveur se connecte. Avant la v2.1.206, le démarrage n'attendait pas que le serveur finisse de se connecter, donc un serveur qui démarre lentement mais sain produisait également cette erreur.

```text theme={null}
Error: MCP tool mcp__permissions__approve (passed via --permission-prompt-tool) not found. Available MCP tools: none
```

La liste après `Available MCP tools:` nomme les outils MCP qui étaient connectés lorsque l'attente s'est terminée.

**À faire :**

* Vérifiez que le serveur démarre et reste connecté : exécutez `claude mcp list` dans le même répertoire et confirmez que le serveur est listé comme connecté
* Confirmez que le nom de l'outil correspond au nom `mcp__<server>__<tool>` que le serveur expose
* Si le serveur a besoin de plus de 30 secondes pour démarrer, augmentez [`MCP_TIMEOUT`](/docs/fr/env-vars)

<h2 id="plugin-errors">
  Erreurs de plugin
</h2>

Ces erreurs proviennent de la configuration des [plugins](/docs/fr/plugins) et des [marketplaces](/docs/fr/plugin-marketplaces). Pour les problèmes de plugin qui ne produisent pas l'un des messages de cette page, comme une URL de marketplace qui ne se charge pas ou un plugin qui s'installe mais n'apparaît pas, consultez [Dépannage des plugins](/docs/fr/discover-plugins#troubleshooting).

<h3 id="marketplace-is-registered-from-an-untrusted-source">
  Marketplace enregistrée à partir d'une source non fiable
</h3>

La marketplace est enregistrée sous un nom qui est [réservé aux marketplaces officielles d'Anthropic](/docs/fr/plugin-marketplaces#marketplace-schema), mais sa source enregistrée n'est pas un référentiel GitHub `anthropics`. Claude Code revérifie les noms réservés chaque fois qu'il charge ou actualise une marketplace, de sorte que la marketplace et les plugins installés à partir de celle-ci cessent de se charger. Avant la v2.1.205, le nom n'était vérifié que lors de l'ajout de la marketplace, de sorte qu'une entrée enregistrée avant que son nom ne soit réservé continuait à se charger.

```text theme={null}
Marketplace "claude-community" is registered from an untrusted source: The name 'claude-community' is reserved for official Anthropic marketplaces. Only repositories from 'github.com/anthropics/' can use this name. To fix it, remove the marketplace and re-add it from the official source.
```

**À faire :**

* Exécutez `claude plugin marketplace remove <name>`, puis ajoutez à nouveau la marketplace à partir du référentiel officiel `github.com/anthropics`
* Si vous publiez une marketplace tierce qui utilisait le nom avant qu'il ne soit réservé, renommez-la et demandez aux utilisateurs de la rajouter à partir de votre source
* Consultez la liste des noms réservés sous [Schéma de marketplace](/docs/fr/plugin-marketplaces#marketplace-schema)

<h3 id="plugin-command-references-user-config">
  La commande du plugin référence user\_config dans une commande shell
</h3>

Un hook de plugin, [monitor](/docs/fr/plugins-reference#monitors), ou une commande MCP [`headersHelper`](/docs/fr/mcp#use-dynamic-headers-for-custom-authentication) référence une [option de plugin](/docs/fr/plugins-reference#user-configuration) `${user_config.KEY}`, et la chaîne substituée serait transmise à un shell. Une valeur configurée contenant `$(...)`, des backticks ou `;` s'exécuterait en tant que code là-bas, de sorte que Claude Code refuse de démarrer le composant au lieu de substituer la valeur. La vérification s'exécute sur le modèle de commande, de sorte que l'erreur apparaît même quand aucune valeur n'est encore configurée. Avant la v2.1.207, la valeur était substituée dans la commande shell.

La formulation dépend de la surface qui a référencé l'option. Un hook de forme shell rapporte :

```text theme={null}
Hook from plugin formatter@acme-tools references ${user_config.*} in a shell-form command. The substituted value would be re-parsed by the shell. Use exec form instead — {"command": "<executable>", "args": ["${user_config.KEY}", ...]} — or read $CLAUDE_PLUGIN_OPTION_<KEY> from the hook's environment. Command: ./scripts/notify.sh ${user_config.webhook_url}
```

Un monitor rapporte :

```text theme={null}
Monitor "deploy-status" from plugin deploy-tools references ${user_config.*} in its command. The substituted value would be passed to a shell. Monitor commands cannot safely reference ${user_config.*}; have the monitor script read the value from a config file or prompt instead.
```

Un MCP `headersHelper` rapporte :

```text theme={null}
headersHelper for MCP server 'internal-api' references ${user_config.*}. The substituted value would be passed to a shell; read the value inside the helper script instead (e.g. from an env var set in the server's "env" block).
```

**À faire :**

* Pour un hook, ajoutez un tableau `args` afin qu'il s'exécute en [forme exec](/docs/fr/hooks#exec-form-and-shell-form), où chaque `${user_config.KEY}` devient un argument sans shell entre les deux. Ou supprimez la référence et lisez la variable d'environnement `$CLAUDE_PLUGIN_OPTION_<KEY>` à l'intérieur du script
* Pour un monitor, supprimez la référence et faites en sorte que le script monitor lise la valeur à partir d'un fichier de configuration
* Pour un `headersHelper`, déplacez `${user_config.KEY}` dans le champ `headers` du serveur, qui n'est pas analysé par shell, ou lisez la valeur à l'intérieur du script helper

<h2 id="tool-errors">
  Erreurs d'outils
</h2>

Ces erreurs proviennent des outils intégrés de Claude qui refusent une entrée. Claude corrige la plupart des erreurs d'outils de lui-même ; les deux ci-dessous nécessitent une modification de votre part, car elles proviennent d'une définition de sous-agent ou d'une règle de permission que vous contrôlez.

<h3 id="agent-would-be-spawned-with-zero-tools">
  Un agent serait lancé sans outils
</h3>

Rien dans la [liste `tools` d'un sous-agent](/docs/fr/sub-agents#supported-frontmatter-fields) n'a été résolu en un outil, donc Claude Code refuse de lancer le sous-agent plutôt que de démarrer un qui ne peut pas agir. Le message regroupe les entrées par la raison pour laquelle elles n'ont pas été résolues : outil non reconnu, outil qui n'est pas disponible pour les sous-agents, ou reconnu mais ne correspondant à aucun outil dans la session actuelle. L'omission du champ `tools` ne déclenche jamais ce refus. Un modèle de serveur MCP tel que `mcp__github__*` n'est pas exempté : quand aucun outil connecté ne provient de ce serveur, le lancement est refusé avec le modèle dans le groupe sans correspondance. Avant la v2.1.208, le sous-agent était lancé sans outils et retournait un résultat vide ou confus.

```text theme={null}
Agent 'code-reviewer' would be spawned with zero tools — refusing. Its tools list resolved to nothing: unrecognized [Grpe]. Fix the agent's tools frontmatter or pass a different subagent_type.
```

**Ce qu'il faut faire :**

* Corrigez chaque entrée que l'erreur nomme par rapport aux [outils disponibles pour les sous-agents](/docs/fr/sub-agents#available-tools)
* Supprimez les entrées pour les outils que la session n'a pas, comme les outils MCP d'un serveur qui n'est pas connecté
* Pour donner au sous-agent tous les outils que le parent a, supprimez le champ `tools` au lieu de lister les outils

<h3 id="file-is-covered-by-a-read-deny-rule">
  Le fichier est couvert par une règle de refus Read
</h3>

L'outil Edit a été appelé sur un chemin correspondant à une [règle de refus `Read`](/docs/fr/permissions#read-and-edit), y compris la création d'un nouveau fichier à ce chemin. L'édition réécrit le contenu que Claude doit pouvoir relire, donc l'appel est refusé avant tout accès au fichier. La règle bloque uniquement l'outil Edit : Write et NotebookEdit ne sont pas couverts par les règles de refus `Read`. Avant la v2.1.208, seule une règle de refus `Edit` bloquait les éditions, et une règle de refus `Read` seule ne le faisait pas.

```text theme={null}
File is covered by a Read deny rule in your permission settings and cannot be edited.
```

**Ce qu'il faut faire :**

* Si Claude doit pouvoir éditer le fichier, supprimez ou réduisez la règle de refus `Read` dans `/permissions` ou dans les [paramètres](/docs/fr/settings#permission-settings)
* Si le fichier doit rester intact, conservez la règle et ajoutez une règle de refus `Edit` pour le même chemin afin que les outils Write et NotebookEdit soient également bloqués

<h2 id="background-session-errors">
  Erreurs de session en arrière-plan
</h2>

Les [sessions en arrière-plan](/docs/fr/agent-view) s'exécutent sans terminal interactif propre, donc les commandes qui en ont besoin se comportent différemment là. Ces messages apparaissent dans la transcription d'une session en arrière-plan, en vue agent ou après attachement.

<h3 id="commands-refused-in-a-background-session">
  Commandes refusées dans une session en arrière-plan
</h3>

Les commandes qui ouvrent une boîte de dialogue interactive sont refusées dans une session en arrière-plan avec un message nommant un formulaire qui fonctionne là ou vous indiquant d'exécuter la commande à partir d'un terminal régulier. `/install-github-app`, la liste des paramètres `/mcp`, et les actions d'authentification dans le menu du serveur MCP sont tous refusés de cette façon. Avant la v2.1.208, ils ouvraient leur boîte de dialogue à l'intérieur de la session en arrière-plan.
Dans la v2.1.208 uniquement, le sélecteur `/model` a également été refusé dans une session en arrière-plan, et `/upgrade` a imprimé l'URL de mise à niveau au lieu d'ouvrir un navigateur.

La formulation nomme la commande qui a été refusée. La liste des paramètres `/mcp` rapporte :

```text theme={null}
Can't open MCP settings in a background session — use `/mcp enable|disable|reconnect <server>` to steer, or run /mcp from an interactive terminal to authenticate.
```

**Ce qu'il faut faire :**

* Utilisez le formulaire que le message nomme, tel que `/mcp reconnect <server>`, `/mcp enable`, ou `/mcp disable`
* Pour les flux de connexion et d'autorisation, exécutez la commande à partir d'une session `claude` régulière dans un terminal

<h3 id="claude_code_process_wrapper-launcher-errors">
  Erreurs du lanceur CLAUDE\_CODE\_PROCESS\_WRAPPER
</h3>

[`CLAUDE_CODE_PROCESS_WRAPPER`](/docs/fr/corporate-launcher) est défini, et sa valeur ne peut pas être utilisée, donc Claude Code refuse de démarrer le processus affecté plutôt que de l'exécuter sans le lanceur. Les problèmes de configuration sont signalés avec un message qui commence par le nom de la variable et indique la raison, par exemple :

```text theme={null}
CLAUDE_CODE_PROCESS_WRAPPER: launcher `/opt/corp/launcher` is not an executable regular file
```

Un lanceur qui démarre mais se termine sans se remplacer par Claude Code échoue la session qu'il démarrait, et la ligne de la session dans la vue agent rapporte que le lanceur `must exec, not daemonize`, suivi de tout ce que le lanceur a imprimé. Une session qui ne peut pas démarrer ou atteindre le service en arrière-plan en raison du lanceur rapporte le problème du lanceur comme raison à l'intérieur de `Couldn't reach the background service (...)`.

**Ce qu'il faut faire :**

* Définissez la variable sur le chemin absolu d'un exécutable qui se termine en appelant `exec "$@"`. Voir [le contrat du lanceur](/docs/fr/corporate-launcher#the-launcher-contract) pour le contrat complet
* Vérifiez `/status`, qui affiche la commande de lancement résolue dans son entrée Self-exec et avertit lorsque le service en arrière-plan en cours d'exécution ne correspond pas, ou exécutez `claude daemon status` à partir d'un shell
* Après avoir corrigé la valeur dans le bloc `env` des [paramètres](/docs/fr/corporate-launcher#set-up-the-launcher), redémarrez le service en arrière-plan avec `claude daemon stop --any` afin que la prochaine expédition démarre un service enveloppé

<h2 id="configuration-warnings">
  Avertissements de configuration
</h2>

Claude Code écrit ces messages sur stderr au démarrage plutôt que d'afficher une erreur dans la conversation. Ils signalent la configuration qu'il a lue mais n'a pas appliquée.

<h3 id="workspace-has-not-been-trusted">
  L'espace de travail n'a pas été approuvé
</h3>

Claude Code a trouvé des règles `permissions.allow` ou des entrées `permissions.additionalDirectories` dans le fichier `.claude/settings.json` ou `.claude/settings.local.json` du projet et ne les a pas appliquées, car [les règles d'autorisation des paramètres du projet nécessitent l'approbation de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust). Le nombre, le nom du paramètre et le fichier nommé dans le message varient selon votre configuration. Les règles `deny` et `ask` ne sont pas affectées.

```text theme={null}
Ignoring 2 permissions.allow entries from .claude/settings.local.json: this workspace has not been trusted. Run Claude Code interactively here once and accept the trust dialog, or set projects["/Users/you/project"].hasTrustDialogAccepted: true in /Users/you/.claude.json.
```

**Que faire :**

* Exécutez `claude` dans le répertoire et acceptez la boîte de dialogue d'approbation. La boîte de dialogue s'affiche même si un répertoire parent est déjà approuvé, répertorie les règles retenues et vous permet de refuser et de continuer à travailler sans elles. Avant la v2.1.200, aucune boîte de dialogue n'apparaissait dans cette situation, donc cette étape ne pouvait pas être complétée là.
* En [mode non interactif](/docs/fr/headless) avec `-p`, aucune boîte de dialogue n'est affichée. Définissez l'entrée `hasTrustDialogAccepted` dans `~/.claude.json` en utilisant la clé `projects` exacte que le message affiche.
* Si le message nomme `.claude/settings.local.json` et que vous avez démarré Claude Code en dehors d'un référentiel git ou dans votre répertoire personnel, mettez à jour vers la v2.1.200 ou ultérieure. Les versions 2.1.196 à 2.1.199 ont traité votre propre `.claude/settings.local.json` comme fourni par le référentiel dans ces espaces de travail. Sur la v2.1.207 et ultérieure, la mise à jour ne suffit pas en dehors d'un référentiel git si vous n'avez pas approuvé le dossier : déterminer qu'un dossier ne se trouve pas dans un référentiel exécute git, et Claude Code n'exécute cette vérification qu'après que vous acceptiez la boîte de dialogue d'approbation, donc utilisez la première étape. Votre répertoire personnel et tout autre [répertoire de configuration](/docs/fr/permissions#project-allow-rules-and-workspace-trust) sont exempts et n'attendent pas la boîte de dialogue. Voir [Règles d'autorisation du projet et approbation de l'espace de travail](/docs/fr/permissions#project-allow-rules-and-workspace-trust).

<h2 id="responses-seem-lower-quality-than-usual">
  Les réponses semblent de qualité inférieure à la normale
</h2>

Si les réponses de Claude semblent moins performantes que prévu mais qu'aucune erreur n'est affichée, la cause est généralement l'état de la conversation plutôt que le modèle lui-même. Claude Code ne change pas silencieusement les versions de modèle. Il peut basculer vers un modèle de secours dans trois cas spécifiques :

* Un [`--fallback-model`](/docs/fr/cli-reference#cli-flags) configuré prend le relais après une erreur de disponibilité, pour ce tour uniquement, avec un avis dans la transcription
* Une vérification de démarrage d'Amazon Bedrock ou de la plateforme Agent de Google Cloud détecte que votre modèle par défaut n'est pas disponible
* Le [basculement automatique du modèle](/docs/fr/model-config#automatic-model-fallback) sur Fable 5 déplace la session vers le modèle Opus par défaut et affiche un avis dans la transcription

La vérification de sélection du modèle ci-dessous détecte les deuxième et troisième cas ; le premier apparaît comme un avis de transcription plutôt qu'un changement de `/model`. La [configuration du modèle](/docs/fr/model-config) explique quand chaque basculement s'applique.

Vérifiez d'abord ceci :

* **Sélection du modèle** : exécutez `/model` pour confirmer que vous êtes sur le modèle attendu. Un choix `/model` précédent ou une variable d'environnement `ANTHROPIC_MODEL` peut vous placer sur un modèle plus petit que prévu.
* **Niveau d'effort** : exécutez `/effort` pour vérifier le niveau de raisonnement actuel et l'augmenter pour le débogage difficile ou le travail de conception. Les valeurs par défaut varient selon le modèle, vérifiez donc avant de supposer que vous êtes en dessous du maximum. Consultez [Ajuster le niveau d'effort](/docs/fr/model-config#adjust-effort-level) pour les valeurs par défaut par modèle et le raccourci `ultrathink`.
* **Pression contextuelle** : exécutez `/context` pour voir le remplissage de la fenêtre. S'il est proche de la capacité, exécutez `/compact` à un point naturel ou `/clear` pour recommencer. Consultez [Explorer la fenêtre contextuelle](/docs/fr/context-window) pour voir comment auto-compact affecte les tours précédents.
* **Instructions obsolètes** : les fichiers `CLAUDE.md` volumineux ou obsolètes et les définitions d'outils MCP consomment du contexte et peuvent orienter les réponses. La vérification `/doctor` signale les fichiers mémoire surdimensionnés et les extensions inutilisées, et `/context` affiche l'utilisation des jetons des outils MCP. Avant la v2.1.205, `/doctor` ouvrait un écran de diagnostics qui signalait les fichiers mémoire surdimensionnés et les définitions de sous-agents.

Quand une réponse s'avère incorrecte, revenir en arrière fonctionne généralement mieux que de répondre avec des corrections. Appuyez deux fois sur Échap ou exécutez `/rewind` pour revenir avant le mauvais tour, puis reformulez l'invite avec plus de détails. Corriger dans le fil de discussion garde la mauvaise tentative en contexte, ce qui peut ancrer les réponses ultérieures à celle-ci. Consultez [Checkpointing](/docs/fr/checkpointing).

Si la qualité semble toujours mauvaise après vérification des éléments ci-dessus, exécutez `/feedback` et décrivez ce que vous attendiez par rapport à ce que vous avez obtenu. Les commentaires soumis de cette manière incluent la transcription de la conversation, ce qui est le moyen le plus rapide pour Anthropic de diagnostiquer une véritable régression. Consultez [Signaler une erreur](#report-an-error) si `/feedback` n'est pas disponible dans votre environnement.

Si Claude vous avertit d'une injection d'invite suspectée, ou refuse une demande en raison d'une injection suspectée, et que le texte nommé par l'avertissement est un contexte que Claude Code ajoute automatiquement à la conversation plutôt que du contenu de fichier ou web, exécutez `claude update` et réessayez. Si l'avertissement se répète après la mise à jour, [signalez-le](#report-an-error) plutôt que de coller le contenu signalé dans l'invite. Avant la v2.1.201, Sonnet 5 refusait certaines demandes de la même manière.

<h2 id="report-an-error">
  Signaler une erreur
</h2>

Pour les erreurs provenant de composants non couverts par cette page, consultez le guide pertinent :

* Le serveur MCP n'a pas pu se connecter ou s'authentifier : [MCP](/docs/fr/mcp)
* Le script hook a échoué ou a bloqué un outil : [Déboguer les hooks](/docs/fr/hooks#debug-hooks)
* Erreur de permission ou erreurs du système de fichiers lors de l'installation : [Dépanner l'installation et la connexion](/docs/fr/troubleshoot-install)

Si une erreur n'est pas répertoriée ici ou si la correction suggérée ne vous aide pas :

* Exécutez `/feedback` dans Claude Code pour envoyer la transcription et une description à Anthropic. La commande propose également d'ouvrir un problème GitHub prérempli. L'envoi à Anthropic nécessite une [authentification](/docs/fr/authentication). Sur Amazon Bedrock, Google Cloud's Agent Platform, Microsoft Foundry et d'autres fournisseurs tiers, ou lorsqu'aucune identifiant Anthropic n'est configuré, `/feedback` enregistre une archive locale que vous pouvez envoyer à votre représentant de compte Anthropic à la place.
* Exécutez `claude doctor` depuis votre shell pour un diagnostic en lecture seule de votre installation, ou exécutez la vérification `/doctor` dans Claude Code pour trouver et corriger les problèmes de configuration
* Vérifiez [status.claude.com](https://status.claude.com) pour les incidents actifs
* Recherchez les [problèmes existants](https://github.com/anthropics/claude-code/issues) sur GitHub
