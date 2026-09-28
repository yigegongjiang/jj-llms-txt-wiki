> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Mesurer le coût et l'utilisation d'un plugin

> Mesurez le coût en tokens d'un plugin Claude Code, découvrez si les gens l'utilisent toujours, et sélectionnez les événements de télémétrie pour les questions de plugins à l'échelle de l'organisation.

Chaque session où un plugin est activé inclut les noms et descriptions de ses skills, agents et commandes dans le contexte de Claude, et ces tokens comptent par rapport à l'utilisation de l'utilisateur, que le plugin soit utilisé ou non. Cette page montre comment voir ce nombre pour un plugin, comment le réduire si vous maintenez le plugin, et où l'utilisation s'affiche pour que vous puissiez dire si un plugin est toujours utilisé.

Cette page est destinée aux auteurs et mainteneurs de plugins. Si vous administrez Claude Code pour une organisation, [Mesurer sur une flotte](#measure-across-a-fleet) couvre les mêmes questions sur chaque machine.

<Note>
  Ces cas sont couverts sur d'autres pages :

  * **Tester la fiabilité avec laquelle le plugin change le comportement de Claude** : voir [Tester les plugins avec des évaluations](/docs/fr/plugin-evals)
  * **Réduire le contexte de votre propre session** : voir [Gérer les plugins installés](/docs/fr/plugins/install#manage-installed-plugins) et la page [fenêtre de contexte](/docs/fr/context-window)
</Note>

Commencez par [Mesurer le coût d'un plugin](#measure-what-a-plugin-costs).

<h2 id="measure-what-a-plugin-costs">
  Mesurer le coût d'un plugin
</h2>

Pour voir ce qu'un plugin ajoute au contexte de Claude, exécutez [`claude plugin details`](/docs/fr/plugins/cli-reference#plugin-details) avec le nom du plugin. Vous l'exécutez dans votre shell, pas à l'invite d'une session Claude Code en cours d'exécution. Le plugin doit être chargé : installé, dans un répertoire de skills, ou passé avec `--plugin-dir` dans la même commande, comme dans `claude --plugin-dir ./formatter plugin details formatter`.

Cet exemple lit un plugin installé nommé `formatter` qui a deux skills, une commande, un agent, un hook et un serveur MCP :

```bash theme={null}
claude plugin details formatter
```

```text theme={null}
formatter 1.0.0
  Description: Formats and lints code on save
  Source: formatter@my-marketplace

Component inventory
  Skills (3)  format-all, format-code, lint-fix
  Agents (1)  style-reviewer
  Hooks (1)  PostToolUse  (harness-only — no model context cost)
  MCP servers (1)  formatter-tools  (tool schemas resolved at runtime; not counted)
  LSP servers (0)

Projected token cost
  Always-on:   ~146 tok   added to every session

Per-component (rounded)
  component       always-on  on-invoke
  format-code           ~40        ~30
  lint-fix              ~50        ~30
  style-reviewer        ~40        ~40
  format-all           < 20        ~30

  On-invoke cost is paid each time a skill or agent fires.
  Token counts are estimates and may differ from actual usage.
```

Chaque partie de la sortie répond à une question différente :

* **Component inventory** : ce que Claude Code a trouvé dans le plugin. Les commandes sont comptées avec les skills, donc `format-all` apparaît sous `Skills`. Les hooks et les serveurs MCP n'obtiennent pas d'estimation de coût et pas de ligne par composant ; pour voir ce que les outils MCP d'un plugin ajoutent, exécutez `/context` dans une session avec le plugin activé et lisez la catégorie `MCP tools`.
* **Always-on** : les tokens que les noms et descriptions des skills, agents et commandes du plugin ajoutent à chaque session où le plugin est activé, que quelque chose s'exécute ou non. C'est le nombre que chaque utilisateur porte, et celui à réduire.
* **Per-component** : chaque ligne divise un skill, agent ou commande en sa part always-on et son coût on-invoke, qui est le corps qui se charge uniquement quand ce composant s'exécute. Utilisez la colonne always-on pour trouver quel composant contribue le plus.

<h3 id="lower-the-always-on-figure">
  Réduire le chiffre always-on
</h3>

Si vous maintenez le plugin, ces modifications réduisent ce qu'il ajoute à chaque session. Si vous l'utilisez seulement, vos options sont de le désactiver ou de le désinstaller ; voir [Gérer les plugins installés](/docs/fr/plugins/install#manage-installed-plugins).

Le chiffre always-on compte le nom de chaque composant plus sa `description` et son frontmatter `when_to_use`. Pour le réduire :

* Raccourcissez les descriptions des skills et des agents.
* Divisez un grand plugin pour que les utilisateurs n'installent que les composants dont ils ont besoin.

La description d'un skill est aussi ce que Claude fait correspondre à une demande, donc une description plus courte peut empêcher le skill de se déclencher. Après avoir réduit les descriptions, vérifiez le déclenchement avec un [grader `tool_used: Skill`](/docs/fr/plugin-evals#create-your-first-eval-suite) dans votre suite d'evals.

Pour ce que chaque type de composant contribue, voir [composants de plugin](/docs/fr/plugins/components).

<h3 id="cost-shown-to-users-before-install">
  Coût affiché aux utilisateurs avant l'installation
</h3>

Les plugins de la marketplace officielle affichent leur coût aux utilisateurs avant l'installation. Dans `/plugin`, quand un utilisateur parcourt la liste des plugins d'une marketplace et sélectionne un plugin, le volet de détails affiche une section **Context cost** avec une ligne `Every turn:` et une ligne `When invoked:`. Quand le chiffre always-on est de 2 000 tokens ou plus, la ligne `Every turn:` apparaît en surbrillance.

Un plugin dans votre propre marketplace n'a pas de section **Context cost**.

<h2 id="check-whether-a-plugin-is-used">
  Vérifier si un plugin est utilisé
</h2>

Claude Code ne signale pas l'utilisation d'un plugin à son auteur. L'utilisation est enregistrée sur la machine de chaque personne qui a installé le plugin, donc ce que vous pouvez apprendre dépend de votre relation avec ces personnes :

* **Vous administrez Claude Code pour leur organisation** : les événements OpenTelemetry et l'API Analytics comptent les installations et les activations de skills sur chaque machine. Voir [Mesurer sur une flotte](#measure-across-a-fleet).
* **Ce sont des coéquipiers que vous pouvez demander** : le propre Claude Code de chaque utilisateur leur montre s'il utilise toujours le plugin, en quatre endroits : le [panneau `/plugin`](#not-used-recently-in-/plugin), [`/skill-doctor`](#find-skills-that-never-run), [`/doctor`](#unused-plugins-in-/doctor), et [`/usage`](#usage-share-in-/usage). Les quatre sont des commandes que l'utilisateur exécute à l'invite Claude Code dans une session sur sa propre machine.
* **Aucun des deux** : vous n'avez aucun signal d'utilisation de Claude Code pour ce plugin.

<h3 id="not-used-recently-in-/plugin">
  Non utilisé récemment dans `/plugin`
</h3>

Sur l'onglet **Installed** de `/plugin`, un plugin que l'utilisateur a installé à partir d'une marketplace se déplace sous un en-tête **Not used recently** une fois qu'il n'a pas été utilisé pendant au moins 14 jours et 10 sessions. Les détails du plugin affichent également une ligne `Last used:`. Pour ce que les utilisateurs font avec cet en-tête et cette ligne, voir [Trouver les plugins que vous n'utilisez plus](/docs/fr/plugins/install#find-plugins-you-no-longer-use).

L'en-tête **Not used recently** n'apparaît jamais pour :

* Les plugins chargés avec `--plugin-dir` ou à partir d'un répertoire de skills
* Les plugins activés via les paramètres gérés, ou montés à partir d'un [répertoire seed](/docs/fr/plugins/org#seed-containers-and-ci)
* Les plugins qui incluent un thème, un style de sortie, un moniteur ou un workflow, car ceux-ci sont en cours d'utilisation sans invocation suivie

Le [serveur de langage](/docs/fr/plugins/components#lsp-servers) d'un plugin est compté comme utilisé quand il fournit des diagnostics ou répond à une demande de navigation de code, donc un plugin LSP dont le serveur est actif dans vos sessions n'est pas listé comme inutilisé.

Quand l'organisation de l'utilisateur définit [`strictKnownMarketplaces`](/docs/fr/plugins/org#restrict-what-users-can-install), ni l'en-tête ni la ligne `Last used:` n'apparaissent.

<h3 id="find-skills-that-never-run">
  Trouver les skills qui ne s'exécutent jamais
</h3>

Exécutez `/skill-doctor` pour voir ce que chacun de vos skills coûte et à quelle fréquence il est utilisé. Il signale les skills qui sont dans la liste des skills de Claude mais qui n'ont jamais été invoqués, y compris les skills des plugins.

Dans une session interactive, le rapport s'ouvre dans l'onglet **Stats** du gestionnaire `/plugin`. Voir [Trouver les skills inutilisés](/docs/fr/skills#find-unused-skills) pour ce que le rapport couvre et où il est disponible.

<h3 id="unused-plugins-in-/doctor">
  Plugins inutilisés dans `/doctor`
</h3>

La vérification `/doctor` liste chaque skill installé par l'utilisateur, serveur MCP et plugin, et recommande de désactiver ceux qui n'ont pas été utilisés. Voir [`/doctor` dans la référence des commandes](/docs/fr/commands#all-commands).

<h3 id="usage-share-in-/usage">
  Partage d'utilisation dans `/usage`
</h3>

Sur un plan Pro, Max, Team ou Enterprise, la ventilation `/usage` attribue l'utilisation récente aux skills, subagents, plugins et serveurs MCP en tant que part du total. Voir [Utiliser la commande `/usage`](/docs/fr/costs#using-the-/usage-command).

<h2 id="measure-across-a-fleet">
  Mesurer sur une flotte
</h2>

Si vous administrez Claude Code pour une organisation, vous pouvez mesurer le coût et l'utilisation des plugins sur chaque machine à partir de l'une de ces sources :

* **Événements OpenTelemetry** : Claude Code les exporte vers votre propre backend une fois que vous [configurez un exportateur](/docs/fr/monitoring-usage). Voir [Événements OpenTelemetry pour les installations et l'utilisation des plugins](#pick-the-opentelemetry-event-for-each-question).
* **API Analytics** : servie à partir des enregistrements d'Anthropic, sans exportateur nécessaire. Voir [Interroger l'API Analytics](#query-the-analytics-api).

<h3 id="pick-the-opentelemetry-event-for-each-question">
  Événements OpenTelemetry pour les installations et l'utilisation des plugins
</h3>

Ces événements OpenTelemetry et attributs répondent à chaque question de plugin depuis votre backend :

| Question                                           | Événement ou attribut OpenTelemetry                                                                                                                                  |
| :------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Quels plugins sont installés et d'où               | [`claude_code.plugin_installed`](/docs/fr/monitoring-usage#plugin-installed-event), un par installation                                                                   |
| Quels plugins sont actifs dans combien de sessions | [`claude_code.plugin_loaded`](/docs/fr/monitoring-usage#plugin-loaded-event), un par plugin activé au démarrage de la session                                             |
| Quels skills s'activent et quel plugin les possède | [`claude_code.skill_activated`](/docs/fr/monitoring-usage#skill-activated-event), avec `plugin.name` et `marketplace.name` pour les skills de plugin                      |
| Ce qu'un hook de plugin signale                    | [`claude_code.hook_plugin_metrics`](/docs/fr/monitoring-usage#hook-plugin-metrics-event), émis uniquement pour les hooks dans les plugins de la marketplace officielle    |
| Ce qu'un plugin coûte en dépenses API              | `plugin.name` et `marketplace.name` sur le [compteur de coûts](/docs/fr/monitoring-usage#cost-counter), défini quand le skill actif ou le subagent appartient à un plugin |

<h3 id="redacted-plugin-names-in-your-backend">
  Noms de plugins masqués dans votre backend
</h3>

Les plugins de la marketplace officielle signalent leur nom de plugin et leur nom de marketplace à votre backend littéralement. Tous les autres noms de plugins sont masqués ou omis par défaut, y compris un plugin de la propre marketplace de votre organisation. Le [niveau de confiance](/docs/fr/plugins/security#find-plugins-in-telemetry) du plugin décide lequel.

Pour obtenir des noms réels sur certains événements, définissez la variable d'environnement [`OTEL_LOG_TOOL_DETAILS`](/docs/fr/monitoring-usage#common-configuration-variables) à `1` sur les machines qui exportent la télémétrie, par exemple dans le bloc `env` des mêmes [paramètres gérés](/docs/fr/monitoring-usage#administrator-configuration) qui configurent l'exportateur :

| Événement                             | Par défaut                                                                                        | Avec `OTEL_LOG_TOOL_DETAILS=1`                          |
| :------------------------------------ | :------------------------------------------------------------------------------------------------ | :------------------------------------------------------ |
| `plugin_loaded`                       | `plugin.name` et `marketplace.name` sont la chaîne littérale `third-party`                        | Noms réels                                              |
| `plugin_installed`, `skill_activated` | `plugin.name` et `marketplace.name` omis ; sur `skill_activated`, `skill.name` est `custom_skill` | Noms réels                                              |
| Compteur de coûts                     | `plugin.name` est `third-party` ; `marketplace.name` absent                                       | `plugin.name` réel ; `marketplace.name` toujours absent |

Sur `plugin_loaded`, `plugin_id_hash` identifie toujours chaque plugin par défaut, vous pouvez donc compter les plugins tiers distincts.

<h3 id="query-the-analytics-api">
  Interroger l'API Analytics
</h3>

Sur le plan Enterprise, l'API Analytics répond à « quels plugins mon organisation installe et invoque » à partir des enregistrements d'Anthropic, sans exportateur nécessaire. [`GET /v1/organizations/analytics/plugins`](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) retourne les comptages d'installation et d'invocation par plugin, par jour, sur Claude Code et Cowork, que vous pouvez regrouper par utilisateur, groupe RBAC ou produit.

L'activité des plugins qui atteint Anthropic sans nom de plugin apparaît dans une ligne `third-party` agrégée. [Trouver les plugins dans la télémétrie](/docs/fr/plugins/security#find-plugins-in-telemetry) dit quels plugins Claude Code signale par nom.

Authentifiez la demande avec une clé API qui a la portée `read:analytics`, qu'un propriétaire principal crée comme décrit sous [Accéder aux données par programmation](/docs/fr/analytics#access-data-programmatically).

Voir la [référence du point de terminaison](https://platform.claude.com/docs/en/api/admin/analytics/plugins/list) pour les paramètres et les champs de réponse.

<h2 id="next-steps">
  Étapes suivantes
</h2>

* [Tester les plugins avec des evals](/docs/fr/plugin-evals) : mesurez la fiabilité avec laquelle le plugin oriente Claude, pas seulement ce qu'il coûte
* [Réduire le chiffre always-on](#lower-the-always-on-figure) : ce qu'il faut changer dans le plugin pour réduire son coût par tour
* [Sécurité et confiance des plugins](/docs/fr/plugins/security#find-plugins-in-telemetry) : quels champs de télémétrie portent les noms de plugins et quand ils sont masqués
* [Surveillance de l'utilisation](/docs/fr/monitoring-usage) : la référence complète des événements OpenTelemetry
