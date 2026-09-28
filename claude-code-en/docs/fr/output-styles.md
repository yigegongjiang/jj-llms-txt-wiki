> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Styles de sortie

> Modifiez le rôle, le ton et le format de réponse de Claude Code avec un style de sortie intégré tel que Concis ou Explicatif, ou écrivez un style personnalisé.

Un style de sortie est un ensemble d'instructions qui définit le rôle, le ton et le format de réponse de Claude pour chaque réponse dans une session. Claude Code inclut quatre styles intégrés en plus de son style par défaut, et vous pouvez écrire le vôtre.

Utilisez un style de sortie pour modifier la façon dont Claude répond et travaille avec vous pendant toute une session, afin que vous ne répétiez pas la demande dans chaque invite. Par exemple, un style intégré peut rendre les réponses plus courtes, ajouter une explication de chaque modification, ou faire en sorte que Claude commence le travail sans poser de questions de routine. Un style personnalisé peut également transformer Claude en quelque chose d'autre qu'un ingénieur logiciel, comme un assistant d'écriture ou un analyste de données.

* Pour utiliser un style intégré, choisissez-en un parmi les [styles de sortie intégrés](#built-in-output-styles) et [basculez vers celui-ci](#change-your-output-style).
* Pour écrire vos propres instructions, [créez un style de sortie personnalisé](#create-a-custom-output-style).

<Note>
  Un style de sortie donne à Claude des instructions à suivre. Il ne garantit pas que quelque chose se produit toujours ou ne se produit jamais. Certains besoins correspondent à une fonctionnalité différente :

  * Pour ce que Claude doit savoir sur votre projet, utilisez [CLAUDE.md](/docs/fr/memory).
  * Pour quelque chose qui doit se produire à chaque fois, comme le formatage après chaque modification ou le blocage d'une commande, utilisez un [hook](/docs/fr/hooks-guide).
  * Pour les skills, les sous-agents et les autres options, consultez [Choisir entre un style de sortie et d'autres fonctionnalités](#choose-between-an-output-style-and-other-features).
</Note>

<h2 id="built-in-output-styles">
  Styles de sortie intégrés
</h2>

Claude Code démarre dans le style [**Default**](#default), ses instructions standard pour accomplir les tâches d'ingénierie logicielle. Chacun des quatre autres styles intégrés conserve ces instructions et ajoute les siennes.

Ce tableau montre ce que chaque style change dans une session et quand il convient :

| Style                       | Ce qui change                                                                                                               | Utilisez-le quand                                                                                                                                 |
| :-------------------------- | :-------------------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------ |
| [Proactive](#proactive)     | Claude commence le travail immédiatement et fait des hypothèses raisonnables plutôt que de demander des décisions courantes | Vous voulez que Claude continue à travailler à travers les décisions courantes, et vous corrigerez la trajectoire si une hypothèse est incorrecte |
| [Concise](#concise)         | Les réponses commencent par le résultat et omettent le préambule, la narration et les récapitulatifs                        | Les réponses par défaut sont plus longues que vous le souhaitez                                                                                   |
| [Explanatory](#explanatory) | Claude ajoute des blocs `Insight` courts qui expliquent les choix derrière le code qu'il écrit                              | Vous découvrez une base de code ou vous voulez le raisonnement avec la modification                                                               |
| [Learning](#learning)       | Claude explique ses choix et laisse de petits morceaux de code pour que vous les écriviez vous-même                         | Vous voulez une pratique de codage pratique tandis que la tâche est toujours accomplissable                                                       |

<h3 id="default">
  Default
</h3>

Default signifie qu'aucun style de sortie n'est sélectionné. Claude Code n'ajoute aucune instruction de style, et Claude fonctionne à partir de l'invite système standard de Claude Code, qui est écrite pour les tâches d'ingénierie logicielle.

`default` apparaît dans la liste `/output-style` avec les autres styles, donc vous [le sélectionnez de la même manière](#change-your-output-style).

<h3 id="proactive">
  Proactive
</h3>

Dans le style Proactive, Claude commence à implémenter dès que vous envoyez une tâche. Il fait des hypothèses raisonnables sur les décisions courantes plutôt que de s'arrêter pour demander, et il ne bascule pas en mode plan à moins que vous ne demandiez un plan. Vous pouvez le rediriger à tout moment.

Les instructions du style indiquent également à Claude de vérifier avec vous dans la conversation avant une action qui supprime des données ou modifie un système partagé ou de production. Cette vérification est une instruction que Claude suit et est distincte des invites de permission.

Basculer vers le style Proactive ne change pas votre [mode de permission](/docs/fr/permission-modes). Votre mode de permission décide toujours quels appels d'outils s'exécutent sans vous demander, donc les invites de permission apparaissent de la même manière qu'avant le basculement.

<h3 id="concise">
  Concise
</h3>

Dans le style Concise, la première phrase d'une réponse indique ce qui s'est passé ou quelle est la réponse. Claude omet l'introduction, la narration étape par étape et le récapitulatif de fermeture, et répond à une question simple en une à trois phrases. Il effectue le travail d'ingénierie aussi minutieusement que dans le style Default. Nécessite Claude Code v2.1.237 ou ultérieur.

Claude écrit toujours à longueur complète dans ces cas :

* **Tout ce que vous demandez** : quand vous demandez une explication ou plus de détails, Claude répond complètement.
* **Tout ce dont vous avez besoin pour agir en toute sécurité** : les rapports d'erreur, la sortie des tests échoués, les avertissements de sécurité et les confirmations pour les actions destructrices conservent leur contenu complet.

<h3 id="explanatory">
  Explanatory
</h3>

Dans le style Explanatory, Claude effectue la tâche de la même manière que dans le style Default et ajoute de courtes explications sur les raisons pour lesquelles il a fait les choix qu'il a faits. Chaque explication apparaît dans la conversation, avant ou après le code auquel elle se rapporte, dans un bloc étiqueté `Insight`. Les explications ne sont pas écrites dans vos fichiers en tant que commentaires.

Un bloc `Insight` porte deux ou trois points sur votre base de code ou le code que Claude a écrit, comme celui-ci après l'ajout d'un point de terminaison API :

```text theme={null}
★ Insight ─────────────────────────────────────
- Every route in this repo goes through the withAuth wrapper, so the new endpoint gets session checks without its own middleware.
- Rate limits are set per route in limits.ts, which is why this change adds an entry there rather than a global default.
─────────────────────────────────────────────────
```

<h3 id="learning">
  Learning
</h3>

Dans le style Learning, Claude ajoute les mêmes blocs `Insight` que le [style Explanatory](#explanatory) et vous demande également d'écrire une partie du code. Claude gère lui-même l'implémentation courante. Quand il atteint une partie avec une véritable décision de conception, comme la gestion des erreurs, une structure de données ou une logique métier avec plus d'une approche valide, il laisse quelques lignes pour vous.

Claude marque l'endroit avec un commentaire `TODO(human)` dans le fichier, puis envoie une demande qui indique ce qui est déjà construit, ce qu'il faut écrire et ce qu'il faut peser :

```text theme={null}
● Learn by Doing

Context: The upload form is in place and calls validateFile() before accepting a file. Size and type checks work for images, but the switch statement has no handling for documents yet.

Your Task: In upload.js, implement the case "document" branch inside validateFile(). Look for TODO(human).

Guidance: Decide on a size limit for documents and whether the file extension has to match the MIME type. Return {valid: boolean, error?: string}.
```

Claude s'arrête alors et attend. Écrivez votre code au commentaire `TODO(human)` et dites à Claude quand vous avez terminé. Claude répond avec un `Insight` sur votre code et continue la tâche.

<h2 id="change-your-output-style">
  Modifier votre style de sortie
</h2>

Choisissez un style avec la commande, un menu ou un fichier de paramètres. La commande et les deux menus enregistrent votre choix dans `.claude/settings.local.json` au [niveau du projet local](/docs/fr/settings).

* **Commande `/output-style`** : exécutez `/output-style <style>` pour basculer, par exemple `/output-style concise`. Sans argument, la commande liste les styles que vous pouvez choisir et marque celui en cours.

  La commande fonctionne également en [mode non interactif](/docs/fr/headless) et dans les sessions Agent SDK, et depuis l'application mobile ou le web via [Contrôle à distance](/docs/fr/remote-control#limitations), où vous pouvez lister et sélectionner uniquement les [styles intégrés](#built-in-output-styles). Nécessite Claude Code v2.1.269 ou ultérieur.
* **Menu Terminal** : exécutez `/config` et sélectionnez **Output style** pour choisir un style dans un menu.
* **Extension VS Code** : ouvrez le [menu de commandes](/docs/fr/vs-code#use-the-prompt-box) avec `/` et sélectionnez **Output styles** pour choisir un style, y compris vos styles personnalisés. Nécessite Claude Code v2.1.257 ou ultérieur.
* **Application de bureau** : définissez le champ `outputStyle` dans un fichier de paramètres, par exemple `.claude/settings.local.json`, le fichier que le menu terminal écrit. Lorsque vous exécutez `/config` là-bas, Claude Code [ouvre **Paramètres > Claude Code**](/docs/fr/desktop#what%E2%80%99s-not-available-in-desktop) plutôt qu'un menu.

Pour définir un style sans le menu, modifiez directement le champ `outputStyle` dans un fichier de paramètres :

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

La valeur est sensible à la casse, donc écrivez les noms intégrés comme `Proactive`, `Concise`, `Explanatory` et `Learning`. Une valeur qui ne correspond pas exactement à un nom de style, comme `explanatory`, vous donne le style par défaut. La commande `/output-style` ignore la casse.

Pour faire d'un style votre style par défaut dans tous les projets, définissez `outputStyle` dans `~/.claude/settings.json`. Les fichiers de paramètres propres à un projet [ont la priorité](/docs/fr/settings#settings-precedence) sur cette valeur.

Lorsque vous changez de style en cours de session, Claude utilise le nouveau style à partir de votre message suivant. Pour le coût de ce premier message en mise en cache des invites, consultez [Modifier le style de sortie](/docs/fr/prompt-caching#changing-output-style). Avant la v2.1.251, le nouveau style s'appliquait uniquement après que vous ayez exécuté `/clear` ou démarré une nouvelle session.

<h2 id="create-a-custom-output-style">
  Créer un style de sortie personnalisé
</h2>

Un style de sortie personnalisé est un fichier Markdown : frontmatter pour les métadonnées, puis les instructions pour Claude.

Dans l'extension VS Code, vous pouvez également créer le fichier à partir du menu [**Output styles**](/docs/fr/vs-code#use-the-prompt-box) plutôt que de l'écrire à la main. Cela nécessite Claude Code v2.1.261 ou version ultérieure.

<Steps>
  <Step title="Créer un fichier Markdown">
    Enregistrez-le à l'un des trois niveaux. Le nom du fichier devient le nom du style sauf si vous définissez `name` dans le frontmatter.

    * Utilisateur : `~/.claude/output-styles`
    * Projet : `.claude/output-styles`
    * Politique gérée : `.claude/output-styles` à l'intérieur du [répertoire des paramètres gérés](/docs/fr/managed-settings#delivery-mechanisms)

    Les styles de sortie de projet se chargent à partir de chaque `.claude/output-styles/` entre le répertoire de travail et la racine du référentiel. Lorsque plusieurs de ces répertoires imbriqués définissent un style portant le même nom, Claude Code utilise celui le plus proche du répertoire de travail.
  </Step>

  <Step title="Ajouter le frontmatter et les instructions">
    Décidez si vous souhaitez conserver les instructions d'ingénierie logicielle de Claude Code. Définissez `keep-coding-instructions: true` si vous modifiez la façon dont Claude communique mais que vous voulez qu'il code de la même manière. Omettez-le si Claude ne fera pas d'ingénierie logicielle.

    Cet exemple commence chaque explication par un diagramme tout en conservant le comportement de codage de Claude :

    ```markdown theme={null}
    ---
    name: Diagrams first
    description: Lead every explanation with a diagram
    keep-coding-instructions: true
    ---

    When explaining code, architecture, or data flow, start with a Mermaid diagram showing the structure, then explain in prose.

    ## Diagram conventions

    Use `flowchart TD` for control flow and `sequenceDiagram` for request paths. Keep diagrams under 15 nodes.
    ```
  </Step>

  <Step title="Basculer vers votre style">
    Exécutez `/output-style <style>` dans le terminal, ou exécutez `/config` et sélectionnez votre style sous **Output style**. Claude utilise le nouveau style à partir de votre message suivant. Dans le terminal, Claude Code lit les fichiers de style au démarrage, donc si vous en créez ou en modifiez un pendant une session en cours, redémarrez Claude Code pour appliquer la modification.
  </Step>
</Steps>

Les [Plugins](/docs/fr/plugins/manifest-reference) peuvent également fournir des styles de sortie dans un répertoire `output-styles/`.

<h3 id="frontmatter">
  Référence du frontmatter
</h3>

Configurez un style de sortie avec le [frontmatter](/docs/fr/glossary#frontmatter) YAML entre les marqueurs `---` en haut du fichier. Tous les champs sont facultatifs, et les noms de champs utilisent des mots minuscules séparés par des tirets. Un champ mal orthographié est ignoré sans erreur. Si le YAML n'analyse pas correctement, le style se charge quand même sous son nom de fichier sans aucun champ défini ; exécutez `claude --debug` pour voir l'erreur d'analyse.

| Champ                      | Obligatoire | Description                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------- | :---------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                     | Non         | Nom du style de sortie, affiché dans le sélecteur `/config`. Par défaut : le nom du fichier                                                                                                                                                                                                                                                                 |
| `description`              | Non         | Description du style de sortie, affichée dans le sélecteur `/config`                                                                                                                                                                                                                                                                                        |
| `keep-coding-instructions` | Non         | Définissez sur `true` pour conserver les instructions d'ingénierie logicielle intégrées de Claude Code aux côtés de votre style. Par défaut : `false`                                                                                                                                                                                                       |
| `force-for-plugin`         | Non         | Styles de sortie de plugin uniquement. Définissez sur `true` pour appliquer ce style automatiquement chaque fois que le plugin est activé, sans nécessiter une sélection de l'utilisateur. Remplace le paramètre `outputStyle` de l'utilisateur. Si plusieurs plugins activés définissent ceci, Claude Code utilise le premier chargé. Par défaut : `false` |

<span id="comparisons-to-related-features" />

<h2 id="choose-between-an-output-style-and-other-features">
  Choisir entre un style de sortie et d'autres fonctionnalités
</h2>

Un style de sortie s'applique à chaque réponse dans une session. C'est une instruction que Claude suit, donc rien ne l'impose. Lorsque ce que vous voulez est plus étroit que chaque réponse, ou doit se produire sans faute, une autre fonctionnalité convient mieux.

Ce tableau fait correspondre ce que vous voulez à la fonctionnalité qui le fait :

| Vous voulez                                                                                                                                | Utiliser                                                          | Pourquoi cela convient                                                                                                              |
| :----------------------------------------------------------------------------------------------------------------------------------------- | :---------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------- |
| Chaque réponse dans une certaine voix, longueur ou format, ou Claude dans un rôle différent                                                | Un style de sortie                                                | Il s'applique à la session entière, et vous changez de styles avec une seule commande                                               |
| Claude connaître les conventions, commandes et structure de votre projet                                                                   | [CLAUDE.md](/docs/fr/memory)                                           | Il contient ce que Claude devrait savoir sur la base de code, et il reste chargé quel que soit le style que vous choisissez         |
| Instructions pour un type de tâche, comme une liste de contrôle de version ou une procédure d'examen                                       | Un [skill](/docs/fr/skills)                                            | Claude le charge uniquement lorsque vous l'invoquez ou que la tâche correspond, donc il ne façonne pas les réponses non liées       |
| Quelque chose qui doit se produire à chaque fois sans exception, comme le formatage après chaque modification ou le blocage d'une commande | Un [hook](/docs/fr/hooks-guide)                                        | Claude Code exécute un hook lui-même lors d'un événement du cycle de vie, donc cela ne dépend pas de Claude suivant une instruction |
| Un assistant avec ses propres instructions, modèle et outils pour une tâche ciblée                                                         | Un [subagent](/docs/fr/sub-agents)                                     | Il s'exécute dans un contexte séparé avec sa propre invite système et retourne un résumé à votre conversation                       |
| Un ajout aux instructions de Claude que vous transmettez au démarrage de Claude Code                                                       | [`--append-system-prompt`](/docs/fr/cli-reference#system-prompt-flags) | Il ajoute à l'invite système sans rien supprimer                                                                                    |

Ces fonctionnalités se combinent. Par exemple, vous pouvez utiliser CLAUDE.md pour ce que Claude devrait savoir, un style de sortie pour la façon dont il répond, et un hook pour tout ce qui doit être garanti. [Étendre Claude Code](/docs/fr/features-overview) compare le reste des fonctionnalités de l'extension.

<h2 id="how-output-styles-work">
  Fonctionnement des styles de sortie
</h2>

Un style de sortie modifie les instructions que Claude Code donne à Claude.

* Claude Code envoie les instructions du style actif avec chaque demande.
* Les styles de sortie personnalisés omettent les instructions d'ingénierie logicielle intégrées de Claude Code, telles que la façon de délimiter les modifications, d'écrire des commentaires et de vérifier le travail, sauf si `keep-coding-instructions` est défini sur `true`.

Les styles de sortie s'appliquent à la conversation principale et à un [fork](/docs/fr/sub-agents#fork-the-current-conversation), qui hérite de la conversation complète et de l'invite système du parent. Les autres [sous-agents exécutent leur propre invite système](/docs/fr/sub-agents#what-loads-at-startup), donc les styles ne modifient pas leur façon de répondre.

L'utilisation des tokens dépend du style. Les instructions d'un style ajoutent des tokens d'entrée, bien que la mise en cache des invites réduise ce coût après la première demande d'une session.

Les styles Explanatory et Learning intégrés produisent des réponses plus longues que Default par conception, ce qui augmente les tokens de sortie. Le style Concise fait l'inverse en instruisant Claude de garder les réponses courtes par défaut. Pour les styles personnalisés, l'utilisation des tokens de sortie dépend de ce que vos instructions demandent à Claude de produire.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Settings](/docs/fr/settings) : où se trouve le champ `outputStyle` et comment fonctionne la précédence des paramètres
* [Permission modes](/docs/fr/permission-modes) : comment le style Proactive se compare au mode auto
* [Plugins](/docs/fr/plugins/overview) : empaquetez et distribuez les styles de sortie aux côtés des skills, des hooks et des agents
* [Debug your configuration](/docs/fr/debug-your-config) : diagnostiquez pourquoi un style de sortie ne prend pas effet
