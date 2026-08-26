> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Styles de sortie

> Adaptez Claude Code pour des usages au-delà de l'ingénierie logicielle

Les styles de sortie modifient la façon dont Claude répond, non ce que Claude sait. Ils modifient l'invite système pour définir le rôle, le ton et le format de sortie. Utilisez-en un lorsque vous continuez à relancer avec la même voix ou le même format à chaque tour, ou lorsque vous voulez que Claude agisse comme quelque chose d'autre qu'un ingénieur logiciel.

Un style de sortie personnalisé ajoute vos instructions à l'invite système et vous permet de choisir si vous souhaitez conserver les instructions d'ingénierie logicielle intégrées de Claude Code. Conservez-les lorsque vous modifiez la façon dont Claude communique mais que vous codez toujours, comme répondre toujours avec un diagramme. Omettez-les lorsque Claude ne fait pas d'ingénierie logicielle du tout, comme un assistant d'écriture ou un analyste de données.

Pour les instructions concernant votre projet, les conventions ou votre base de code, utilisez [CLAUDE.md](/docs/fr/memory) à la place.

<h2 id="built-in-output-styles">
  Styles de sortie intégrés
</h2>

Le style de sortie **Default** de Claude Code est l'invite système existante, conçue pour vous aider à accomplir efficacement les tâches d'ingénierie logicielle.

Il existe trois styles de sortie intégrés supplémentaires :

* **Proactive** : Claude s'exécute immédiatement, fait des hypothèses raisonnables au lieu de s'arrêter pour les décisions courantes, et préfère l'action à la planification. Ceci applique des conseils d'exécution autonome plus forts que le [mode auto](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode), et cela fonctionne sans modifier votre mode de permission, vous voyez donc toujours les invites de permission avant l'exécution des outils.

* **Explanatory** : Fournit des « Insights » éducatifs entre les tâches d'ingénierie logicielle pour vous aider à les accomplir. Vous aide à comprendre les choix d'implémentation et les modèles de base de code.

* **Learning** : Mode collaboratif d'apprentissage par la pratique où Claude ne partagera pas seulement des « Insights » lors du codage, mais vous demandera également de contribuer à de petits éléments de code stratégiques. Claude Code ajoutera des marqueurs `TODO(human)` dans votre code pour que vous les implémentiez.

<h2 id="change-your-output-style">
  Modifier votre style de sortie
</h2>

Exécutez `/config` et sélectionnez **Output style** pour choisir un style dans un menu. Votre sélection est enregistrée dans `.claude/settings.local.json` au [niveau du projet local](/docs/fr/settings).

<Note>La commande autonome `/output-style` a été dépréciée dans la v2.1.73 et supprimée dans la v2.1.91. Utilisez `/config` ou modifiez directement le paramètre `outputStyle`.</Note>

Pour définir un style sans le menu, modifiez directement le champ `outputStyle` dans un fichier de paramètres :

```json theme={null}
{
  "outputStyle": "Explanatory"
}
```

Le style de sortie fait partie de l'invite système, que Claude Code lit une seule fois au démarrage de la session. Les modifications prennent effet après `/clear` ou une nouvelle session. Consultez [Comment Claude Code utilise la mise en cache des invites](/docs/fr/prompt-caching#changing-output-style) pour savoir ce qu'un changement de style de sortie fait au cache.

<h2 id="create-a-custom-output-style">
  Créer un style de sortie personnalisé
</h2>

Un style de sortie personnalisé est un fichier Markdown : frontmatter pour les métadonnées, puis les instructions à ajouter à l'invite système.

<Steps>
  <Step title="Créer un fichier Markdown">
    Enregistrez-le à l'un des trois niveaux. Le nom du fichier devient le nom du style sauf si vous définissez `name` dans le frontmatter.

    * Utilisateur : `~/.claude/output-styles`
    * Projet : `.claude/output-styles`
    * Politique gérée : `.claude/output-styles` à l'intérieur du [répertoire des paramètres gérés](/docs/fr/settings#settings-files)

    Les styles de sortie de projet se chargent à partir de chaque `.claude/output-styles/` entre le répertoire de travail et la racine du référentiel. À partir de la v2.1.178, lorsque plusieurs de ces répertoires imbriqués définissent un style portant le même nom, Claude Code utilise celui le plus proche du répertoire de travail.
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
    Exécutez `/config` et sélectionnez votre style sous **Output style**. Il prend effet après `/clear` ou la prochaine fois que vous démarrez une session.
  </Step>
</Steps>

Les [Plugins](/docs/fr/plugins-reference) peuvent également fournir des styles de sortie dans un répertoire `output-styles/`.

<h3 id="frontmatter">
  Frontmatter
</h3>

Les fichiers de style de sortie prennent en charge ces champs frontmatter :

| Frontmatter                | Objectif                                                                                                                                                                                                                                                                                                     | Par défaut               |
| :------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------- |
| `name`                     | Nom du style de sortie, s'il ne s'agit pas du nom du fichier                                                                                                                                                                                                                                                 | Hérité du nom du fichier |
| `description`              | Description du style de sortie, affichée dans le sélecteur `/config`                                                                                                                                                                                                                                         | Aucun                    |
| `keep-coding-instructions` | Conserver les instructions d'ingénierie logicielle intégrées de Claude Code                                                                                                                                                                                                                                  | `false`                  |
| `force-for-plugin`         | Styles de sortie de plugin uniquement : appliquez ce style automatiquement chaque fois que le plugin est activé, sans nécessiter une sélection de l'utilisateur. Remplace le paramètre `outputStyle` de l'utilisateur. Si plusieurs plugins activés définissent ceci, Claude Code utilise le premier chargé. | `false`                  |

<h2 id="how-output-styles-work">
  Fonctionnement des styles de sortie
</h2>

Les styles de sortie modifient directement l'invite système de Claude Code.

* Tous les styles de sortie ont leurs propres instructions personnalisées ajoutées à la fin de l'invite système.
* Tous les styles de sortie déclenchent des rappels pour que Claude adhère aux instructions du style de sortie pendant la conversation.
* Les styles de sortie personnalisés omettent les instructions d'ingénierie logicielle intégrées de Claude Code, comme la façon de délimiter les modifications, d'écrire des commentaires et de vérifier le travail, sauf si `keep-coding-instructions` est défini sur `true`.

L'utilisation des tokens dépend du style. L'ajout d'instructions à l'invite système augmente les tokens d'entrée, bien que la mise en cache des invites réduise ce coût après la première requête d'une session. Les styles Explanatory et Learning intégrés produisent des réponses plus longues que Default par conception, ce qui augmente les tokens de sortie. Pour les styles personnalisés, l'utilisation des tokens de sortie dépend de ce que vos instructions demandent à Claude de produire.

<h2 id="comparisons-to-related-features">
  Comparaisons avec les fonctionnalités connexes
</h2>

Plusieurs fonctionnalités personnalisent le comportement de Claude Code. Les styles de sortie modifient directement l'invite système et s'appliquent à chaque réponse. Les autres ajoutent des instructions sans modifier l'invite système par défaut, ou les limitent à une tâche spécifique.

| Fonctionnalité           | Fonctionnement                                                                             | Utilisez-le quand                                                                                |
| :----------------------- | :----------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------- |
| Styles de sortie         | Modifie l'invite système                                                                   | Vous voulez un rôle, un ton ou un format de réponse par défaut différent à chaque tour           |
| [CLAUDE.md](/docs/fr/memory)  | Ajoute un message utilisateur après l'invite système                                       | Claude devrait toujours connaître vos conventions de projet et le contexte de votre base de code |
| `--append-system-prompt` | Ajoute à l'invite système sans rien supprimer                                              | Vous voulez un ajout ponctuel pour une seule invocation                                          |
| [Agents](/docs/fr/sub-agents) | Exécute un sous-agent avec sa propre invite système, son modèle et ses outils              | Vous voulez un assistant à portée séparée pour une tâche ciblée                                  |
| [Skills](/docs/fr/skills)     | Charge les instructions spécifiques à une tâche lorsqu'elles sont invoquées ou pertinentes | Vous avez un flux de travail réutilisable                                                        |

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Settings](/docs/fr/settings) : où se trouve le champ `outputStyle` et comment fonctionne la précédence des paramètres
* [Permission modes](/docs/fr/permission-modes) : comment le style Proactive se compare au mode auto
* [Plugins](/docs/fr/plugins) : empaquetez et distribuez les styles de sortie aux côtés des skills, des hooks et des agents
* [Debug your configuration](/docs/fr/debug-your-config) : diagnostiquez pourquoi un style de sortie ne prend pas effet
