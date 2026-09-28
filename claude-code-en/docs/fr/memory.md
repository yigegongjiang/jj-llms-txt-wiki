> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Comment Claude se souvient de votre projet

> Donnez à Claude des instructions persistantes avec les fichiers CLAUDE.md ou AGENTS.md, et laissez Claude accumuler automatiquement les apprentissages avec la mémoire automatique.

Chaque session Claude Code commence avec une fenêtre de contexte vierge. Deux mécanismes transportent les connaissances d'une session à l'autre :

* **Fichiers CLAUDE.md** : instructions que vous écrivez pour donner à Claude un contexte persistant. Claude peut également lire les fichiers [`AGENTS.md`](#agents-md) d'un référentiel, seuls ou aux côtés de CLAUDE.md
* **Mémoire automatique** : notes que Claude écrit lui-même en fonction de vos corrections et préférences

Cette page couvre comment :

* [Écrire et organiser les fichiers CLAUDE.md](#claude-md-files)
* [Utiliser un fichier AGENTS.md existant](#agents-md) comme instructions de votre projet, seul ou aux côtés de CLAUDE.md
* [Limiter les règles à des types de fichiers spécifiques](#organize-rules-with-claude/rules/) avec `.claude/rules/`
* [Configurer la mémoire automatique](#auto-memory) pour que Claude prenne des notes automatiquement
* [Dépanner](#troubleshoot-memory-issues) quand les instructions ne sont pas suivies

<h2 id="claude-md-vs-auto-memory">
  CLAUDE.md vs mémoire automatique
</h2>

Claude Code dispose de deux systèmes de mémoire complémentaires. Les deux sont chargés au début de chaque conversation. Claude les traite comme du contexte, pas comme une configuration appliquée. Pour bloquer une action indépendamment de ce que Claude décide, utilisez un [hook PreToolUse](/docs/fr/hooks-guide) à la place. Plus vos instructions sont spécifiques et concises, plus Claude les suit régulièrement.

|                       | Fichiers CLAUDE.md                                        | Mémoire automatique                                                                                              |
| :-------------------- | :-------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------- |
| **Qui l'écrit**       | Vous                                                      | Claude                                                                                                           |
| **Ce qu'il contient** | Instructions et règles                                    | Apprentissages et modèles                                                                                        |
| **Portée**            | Projet, utilisateur ou organisation                       | Par référentiel, partagé entre les worktrees                                                                     |
| **Chargé dans**       | Chaque session                                            | Chaque session (premières 200 lignes ou 25 KB)                                                                   |
| **À utiliser pour**   | Normes de codage, flux de travail, architecture du projet | Vos préférences, corrections que vous donnez à Claude, contexte du projet que Claude ne peut pas dériver du code |

Utilisez les fichiers CLAUDE.md quand vous voulez guider le comportement de Claude. La mémoire automatique permet à Claude d'apprendre de vos corrections sans effort manuel.

Les subagents peuvent également maintenir leur propre mémoire automatique. Consultez la [configuration des subagents](/docs/fr/sub-agents#enable-persistent-memory) pour plus de détails.

<h2 id="claude-md-files">
  Fichiers CLAUDE.md
</h2>

Les fichiers CLAUDE.md sont des fichiers markdown qui donnent à Claude des instructions persistantes pour un projet, votre flux de travail personnel ou toute votre organisation. Vous écrivez ces fichiers en texte brut ; Claude les lit au début de chaque session. Si votre référentiel utilise `AGENTS.md` à la place, consultez [AGENTS.md](#agents-md).

<h3 id="when-to-add-to-claude-md">
  Quand ajouter à CLAUDE.md
</h3>

Traitez CLAUDE.md comme l'endroit où vous écrivez ce que vous auriez autrement dû réexpliquer. Ajoutez-y quand :

* Claude fait la même erreur une deuxième fois
* Une revue de code détecte quelque chose que Claude aurait dû savoir sur cette base de code
* Vous tapez la même correction ou clarification dans le chat que vous aviez tapée la session précédente
* Un nouveau coéquipier aurait besoin du même contexte pour être productif

Limitez-vous aux faits que Claude doit retenir à chaque session : commandes de compilation, conventions, disposition du projet, règles « toujours faire X ». Si une entrée est une procédure multi-étapes ou ne concerne qu'une partie de la base de code, déplacez-la vers une [compétence](/docs/fr/skills) ou une [règle scoped par chemin](#organize-rules-with-claude/rules/) à la place. L'[aperçu des extensions](/docs/fr/features-overview#build-your-setup-over-time) couvre quand utiliser chaque mécanisme.

<h3 id="choose-where-to-put-claude-md-files">
  Choisir où placer les fichiers CLAUDE.md
</h3>

Les fichiers CLAUDE.md peuvent se trouver à plusieurs emplacements, chacun avec une portée différente. Le tableau ci-dessous les énumère dans l'ordre de chargement, de la portée la plus large à la plus spécifique, de sorte qu'une instruction de projet apparaît en contexte après une instruction utilisateur.

| Portée                       | Emplacement                                                                                                                                                               | Objectif                                                                    | Exemples de cas d'usage                                                           | Partagé avec                                  |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | --------------------------------------------- |
| **Politique gérée**          | • macOS : `/Library/Application Support/ClaudeCode/CLAUDE.md`<br />• Linux et WSL : `/etc/claude-code/CLAUDE.md`<br />• Windows : `C:\Program Files\ClaudeCode\CLAUDE.md` | Instructions à l'échelle de l'organisation gérées par l'informatique/DevOps | Normes de codage de l'entreprise, politiques de sécurité, exigences de conformité | Tous les utilisateurs de l'organisation       |
| **Instructions utilisateur** | `~/.claude/CLAUDE.md`                                                                                                                                                     | Préférences personnelles pour tous les projets                              | Préférences de style de code, raccourcis d'outils personnels                      | Juste vous (tous les projets)                 |
| **Instructions de projet**   | `./CLAUDE.md` ou `./.claude/CLAUDE.md`. Consultez [AGENTS.md](#agents-md) pour savoir quand `./AGENTS.md` se charge à la place ou aux côtés de ceux-ci                    | Instructions partagées par l'équipe pour le projet                          | Architecture du projet, normes de codage, flux de travail courants                | Membres de l'équipe via le contrôle de source |
| **Instructions locales**     | `./CLAUDE.local.md`                                                                                                                                                       | Préférences personnelles spécifiques au projet ; ajouter à `.gitignore`     | Vos URL de sandbox, données de test préférées                                     | Juste vous (projet actuel)                    |

Les fichiers CLAUDE.md et CLAUDE.local.md dans la hiérarchie de répertoires au-dessus du répertoire de travail sont chargés au lancement. Les fichiers dans les sous-répertoires se chargent à la demande quand Claude lit les fichiers de ces répertoires. Consultez [Comment les fichiers CLAUDE.md se chargent](#how-claude-md-files-load) pour l'ordre de résolution complet.

Pour les grands projets, vous pouvez diviser les instructions en fichiers spécifiques à un sujet en utilisant [les règles de projet](#organize-rules-with-claude/rules/). Les règles vous permettent de limiter les instructions à des types de fichiers ou des sous-répertoires spécifiques.

<h3 id="set-up-a-project-claude-md">
  Configurer un CLAUDE.md de projet
</h3>

Un CLAUDE.md de projet peut être stocké dans `./CLAUDE.md` ou `./.claude/CLAUDE.md`. Créez ce fichier et ajoutez des instructions qui s'appliquent à quiconque travaille sur le projet : commandes de compilation et de test, normes de codage, décisions architecturales, conventions de nommage et flux de travail courants. Ces instructions sont partagées avec votre équipe via le contrôle de source, donc concentrez-vous sur les normes au niveau du projet plutôt que sur les préférences personnelles. Pour confirmer que le fichier a été chargé, exécutez `/context` dans une session et vérifiez la liste sous **Fichiers de mémoire**.

<Tip>
  Exécutez `/init` pour générer automatiquement un CLAUDE.md de démarrage. Claude analyse votre base de code et crée un fichier avec les commandes de compilation, les instructions de test et les conventions de projet qu'il découvre. Si un CLAUDE.md existe déjà, `/init` suggère des améliorations plutôt que de le remplacer. Affinez-le à partir de là avec des instructions que Claude ne découvrirait pas de lui-même.

  Pour un flux interactif multi-phases à la place, définissez la variable d'environnement `CLAUDE_CODE_NEW_INIT` à `1` avant d'exécuter `/init`. Définissez-la dans votre shell ou dans le bloc `env` d'un fichier de paramètres, comme indiqué dans [Définir les variables d'environnement](/docs/fr/env-vars#set-environment-variables). Avec elle définie, `/init` demande quels artefacts configurer : fichiers CLAUDE.md, compétences et hooks. Il explore ensuite votre base de code avec un sous-agent, comble les lacunes via des questions de suivi et présente une proposition vérifiable avant d'écrire des fichiers. La variable ne change que la façon dont `/init` s'exécute, de sorte que vous pouvez la laisser définie.
</Tip>

<h3 id="write-effective-instructions">
  Écrire des instructions efficaces
</h3>

Les fichiers CLAUDE.md sont chargés dans la fenêtre de contexte au début de chaque session, consommant des jetons aux côtés de votre conversation. La [visualisation de la fenêtre de contexte](/docs/fr/context-window) montre où CLAUDE.md se charge par rapport au reste du contexte de démarrage. Parce qu'ils sont du contexte plutôt qu'une configuration appliquée, la façon dont vous écrivez les instructions affecte la fiabilité avec laquelle Claude les suit. Les instructions spécifiques, concises et bien structurées fonctionnent mieux.

**Taille** : visez moins de 200 lignes par fichier CLAUDE.md. Les fichiers plus longs consomment plus de contexte et réduisent l'adhérence. Si vos instructions deviennent trop volumineuses, utilisez [les règles scoped par chemin](#path-specific-rules) pour que les instructions ne se chargent que quand Claude travaille avec des fichiers correspondants. Vous pouvez également diviser le contenu en [imports](#import-additional-files) pour l'organisation, bien que les fichiers importés se chargent toujours et entrent dans la fenêtre de contexte au lancement.

**Structure** : utilisez les en-têtes markdown et les puces pour regrouper les instructions connexes. Claude scanne la structure de la même manière que les lecteurs : les sections organisées sont plus faciles à suivre que les paragraphes denses.

**Spécificité** : écrivez des instructions suffisamment concrètes pour être vérifiables. Par exemple :

* « Utiliser l'indentation à 2 espaces » au lieu de « Formater le code correctement »
* « Exécuter `npm test` avant de valider » au lieu de « Testez vos modifications »
* « Les gestionnaires d'API se trouvent dans `src/api/handlers/` » au lieu de « Gardez les fichiers organisés »

**Cohérence** : si deux règles se contredisent, Claude peut en choisir une arbitrairement. Examinez régulièrement vos fichiers CLAUDE.md, les fichiers CLAUDE.md imbriqués dans les sous-répertoires et [`.claude/rules/`](#organize-rules-with-claude/rules/) pour supprimer les instructions obsolètes ou conflictuelles. Dans les monorepos, utilisez [`claudeMdExcludes`](#exclude-specific-claude-md-files) pour ignorer les fichiers CLAUDE.md d'autres équipes qui ne sont pas pertinents pour votre travail.

<h3 id="import-additional-files">
  Importer des fichiers supplémentaires
</h3>

Les fichiers CLAUDE.md peuvent importer des fichiers supplémentaires en utilisant la syntaxe `@path/to/import`. Les fichiers importés sont développés et chargés en contexte au lancement aux côtés du CLAUDE.md qui les référence.

Les chemins relatifs et absolus sont autorisés. Les chemins relatifs se résolvent par rapport au fichier contenant l'import, pas au répertoire de travail. Les fichiers importés peuvent importer récursivement d'autres fichiers, avec une profondeur maximale de quatre sauts.

L'analyse d'import ignore les étendues de code Markdown et les blocs de code clôturés. Pour mentionner un chemin dans votre CLAUDE.md sans l'importer, enveloppez-le dans des backticks : écrire `` `@README` `` garde le texte littéral, tandis que `@README` en dehors des backticks importe le fichier.

Pour intégrer un README, package.json et un guide de flux de travail, référencez-les avec la syntaxe `@` n'importe où dans votre CLAUDE.md :

```text theme={null}
Consultez @README pour un aperçu du projet et @package.json pour les commandes npm disponibles pour ce projet.

# Instructions supplémentaires
- flux de travail git @docs/git-instructions.md
```

Pour les préférences personnelles privées par projet qui ne doivent pas être validées dans le contrôle de source, créez un `CLAUDE.local.md` à la racine du projet. Il se charge aux côtés de `CLAUDE.md` et est traité de la même manière. Ajoutez `CLAUDE.local.md` à votre `.gitignore` pour qu'il ne soit pas validé. Avec `CLAUDE_CODE_NEW_INIT=1` défini, l'exécution de `/init` et le choix de l'option personnelle le font pour vous.

Si vous travaillez sur plusieurs git worktrees du même référentiel, un `CLAUDE.local.md` ignoré par git n'existe que dans le worktree où vous l'avez créé. Pour partager des instructions personnelles entre worktrees, importez plutôt un fichier de votre répertoire personnel :

```text theme={null}
# Préférences individuelles
- @~/.claude/my-project-instructions.md
```

<Warning>
  Un import dans un fichier de mémoire au niveau du projet est externe quand son chemin se résout en dehors de votre répertoire de travail, comme l'import du répertoire personnel ci-dessus. La première fois que Claude Code rencontre des imports externes dans un projet, il affiche une boîte de dialogue d'approbation listant les fichiers. Si vous refusez, les imports restent désactivés et la boîte de dialogue n'apparaît plus.

  Claude Code affiche la boîte de dialogue pour vous protéger des fichiers que d'autres personnes valident dans un projet partagé. Les fichiers de mémoire au niveau utilisateur, tels que `~/.claude/CLAUDE.md` et `~/.claude/rules/`, sont des fichiers que vous avez écrit vous-même. Sauf dans les sessions [Cowork](https://claude.com/product/cowork) sur votre bureau, Claude Code charge leurs imports sans la boîte de dialogue et les fait confiance comme le reste de votre configuration personnelle.

  Dans les sessions Cowork sur votre bureau, Claude Code ignore tout import dans un fichier au niveau utilisateur qui se résout en un chemin en dehors du répertoire de travail de la session et charge le reste du fichier. Dans ces sessions, il ignore également un `~/.claude/CLAUDE.md` qui est lui-même un lien symbolique ou un lien physique, et un répertoire `~/.claude/rules/` symlinké ou un fichier de règle qui pointe en dehors du répertoire de travail.
</Warning>

<h3 id="how-claude-md-files-load">
  Comment les fichiers CLAUDE.md se chargent
</h3>

Claude Code charge `CLAUDE.md` et `CLAUDE.local.md` à partir de votre répertoire de travail actuel et de chaque répertoire au-dessus. Exécutez Claude Code dans `foo/bar/` et il charge les instructions de `foo/bar/CLAUDE.md`, `foo/CLAUDE.md` et tous les fichiers `CLAUDE.local.md` qui les accompagnent.

Tous les fichiers découverts sont concaténés en contexte plutôt que de se remplacer les uns les autres. Dans l'arborescence des répertoires, le contenu est ordonné de la racine du système de fichiers jusqu'à votre répertoire de travail. Pour l'exemple `foo/bar/`, `foo/CLAUDE.md` apparaît en contexte avant `foo/bar/CLAUDE.md`, de sorte que les instructions plus proches de l'endroit où vous avez lancé Claude sont lues en dernier. Dans chaque répertoire, `CLAUDE.local.md` est ajouté après `CLAUDE.md`, de sorte que vos notes personnelles sont la dernière chose que Claude lit à ce niveau.

Claude découvre également les fichiers `CLAUDE.md` et `CLAUDE.local.md` dans les sous-répertoires sous votre répertoire de travail actuel. Au lieu de les charger au lancement, ils sont inclus quand Claude lit les fichiers de ces sous-répertoires.

Si vous travaillez dans un grand monorepo où les fichiers CLAUDE.md d'autres équipes sont détectés, utilisez [`claudeMdExcludes`](#exclude-specific-claude-md-files) pour les ignorer. Pour la disposition complète des fichiers CLAUDE.md racine et par répertoire et des règles, consultez [Monorepos et grands référentiels](/docs/fr/large-codebases).

Les commentaires HTML au niveau des blocs (`<!-- maintainer notes -->`) dans les fichiers CLAUDE.md sont supprimés avant que le contenu ne soit injecté dans le contexte de Claude. Utilisez-les pour laisser des notes aux responsables humains sans dépenser de jetons de contexte. Les commentaires à l'intérieur des blocs de code sont conservés. Quand vous ouvrez un fichier CLAUDE.md directement avec l'outil Read, les commentaires restent visibles.

<h4 id="load-from-additional-directories">
  Charger à partir de répertoires supplémentaires
</h4>

Le drapeau `--add-dir` donne à Claude accès à des répertoires supplémentaires en dehors de votre répertoire de travail principal. Par défaut, les fichiers CLAUDE.md de ces répertoires ne sont pas chargés.

Pour charger également les fichiers de mémoire à partir de répertoires supplémentaires, définissez la variable d'environnement `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD` :

```bash theme={null}
CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1 claude --add-dir ../shared-config
```

La forme en ligne définit la variable pour ce seul lancement dans Bash ou Zsh. Pour la garder activée pour chaque session, ajoutez-la au bloc `env` dans `~/.claude/settings.json` comme indiqué dans [Définir les variables d'environnement](/docs/fr/env-vars#set-environment-variables).

Cela charge `CLAUDE.md`, `.claude/CLAUDE.md`, `.claude/rules/*.md` et `CLAUDE.local.md` à partir du répertoire supplémentaire. `CLAUDE.local.md` est ignoré si vous excluez `local` de [`--setting-sources`](/docs/fr/cli-reference).

<h3 id="organize-rules-with-claude/rules/">
  Organiser les règles avec `.claude/rules/`
</h3>

Pour les projets plus importants, vous pouvez organiser les instructions en plusieurs fichiers en utilisant le répertoire `.claude/rules/`. Cela garde les instructions modulaires et plus faciles à maintenir pour les équipes. Les règles peuvent également être [scoped à des chemins de fichiers spécifiques](#path-specific-rules), de sorte qu'elles ne se chargent en contexte que quand Claude travaille avec des fichiers correspondants, réduisant le bruit et économisant l'espace de contexte.

<Note>
  Les règles se chargent en contexte à chaque session ou quand des fichiers correspondants sont ouverts. Pour les instructions spécifiques à une tâche qui n'ont pas besoin d'être en contexte tout le temps, utilisez plutôt les [compétences](/docs/fr/skills), qui ne se chargent que quand vous les invoquez ou quand Claude détermine qu'elles sont pertinentes pour votre invite.
</Note>

<h4 id="set-up-rules">
  Configurer les règles
</h4>

Placez les fichiers markdown dans le répertoire `.claude/rules/` de votre projet. Chaque fichier doit couvrir un sujet, avec un nom de fichier descriptif comme `testing.md` ou `api-design.md`. Tous les fichiers `.md` sont découverts récursivement, de sorte que vous pouvez organiser les règles en sous-répertoires comme `frontend/` ou `backend/` :

```text theme={null}
your-project/
├── .claude/
│   ├── CLAUDE.md           # Instructions principales du projet
│   └── rules/
│       ├── code-style.md   # Directives de style de code
│       ├── testing.md      # Conventions de test
│       └── security.md     # Exigences de sécurité
```

Les règles sans [frontmatter `paths`](#path-specific-rules) sont chargées au lancement avec la même priorité que `.claude/CLAUDE.md`.

Les règles de projet sont ignorées si vous excluez `project` de [`--setting-sources`](/docs/fr/cli-reference). Avant v2.1.211, les règles qui se chargent à la demande, y compris les règles scoped par chemin et les règles dans les répertoires `.claude/rules/` imbriqués, se chargeaient même quand `project` était exclu.

<h4 id="path-specific-rules">
  Règles spécifiques au chemin
</h4>

Les règles peuvent être scoped à des fichiers spécifiques en utilisant le frontmatter YAML avec le champ `paths`. Ces règles conditionnelles ne s'appliquent que quand Claude travaille avec des fichiers correspondant aux modèles spécifiés.

```markdown theme={null}
---
paths:
  - "src/api/**/*.ts"
---

# Règles de développement d'API

- Tous les points de terminaison d'API doivent inclure la validation des entrées
- Utiliser le format de réponse d'erreur standard
- Inclure les commentaires de documentation OpenAPI
```

Les règles sans champ `paths` sont chargées sans condition et s'appliquent à tous les fichiers. Les règles scoped par chemin se déclenchent quand Claude lit les fichiers correspondant au modèle, pas à chaque utilisation d'outil. À partir de v2.1.198, la correspondance fonctionne également quand Claude atteint un fichier via un chemin symlinké vers le répertoire du projet, par exemple dans un checkout symlinké.

Utilisez les modèles glob dans le champ `paths` pour faire correspondre les fichiers par extension, répertoire ou toute combinaison :

| Modèle                 | Correspond à                                                |
| ---------------------- | ----------------------------------------------------------- |
| `**/*.ts`              | Tous les fichiers TypeScript dans n'importe quel répertoire |
| `src/**/*`             | Tous les fichiers sous le répertoire `src/`                 |
| `*.md`                 | Fichiers Markdown à la racine du projet                     |
| `src/components/*.tsx` | Composants React dans un répertoire spécifique              |

Vous pouvez spécifier plusieurs modèles et utiliser l'expansion entre accolades pour faire correspondre plusieurs extensions dans un modèle :

```markdown theme={null}
---
paths:
  - "src/**/*.{ts,tsx}"
  - "lib/**/*.ts"
  - "tests/**/*.test.ts"
---
```

Chaque groupe entre accolades multiplie le nombre de modèles développés : `src/*.{ts,tsx}` se développe en deux modèles, et `{a,b}/{c,d}/*.{ts,tsx}` en huit. Pour garder l'expansion limitée, la liste `paths` entière d'une règle partage un budget de 1 000 modèles développés et 4 MiB, et les modèles sans accolades ne comptent pas contre lui.

Claude Code utilise tout modèle qui dépasserait le budget non développé, et ses accolades littérales ne correspondent à aucun fichier. Avant v2.1.217, une valeur `paths` avec de nombreux groupes entre accolades bloquait ou plantait le CLI au démarrage.

La syntaxe Glob traite `[` comme le début d'une expression entre crochets telle que `[abc]`. Un modèle avec un `[` qui ne peut pas être lu comme une expression entre crochets, tel que `photos [2024/**`, est invalide : il ne correspond à rien, et les autres modèles de la règle continuent de fonctionner. Pour faire correspondre un `[` littéral dans un nom de fichier, échappez-le comme `photos \[2024/**`. Avant v2.1.207, un modèle invalide faisait échouer l'outil Read pour chaque fichier sur lequel la règle était évaluée, au lieu de ne correspondre à rien.

<h4 id="rules-frontmatter-reference">
  Référence frontmatter des règles
</h4>

Configurez une règle avec le [frontmatter](/docs/fr/glossary#frontmatter) YAML entre les marqueurs `---` en haut du fichier. `paths` est le seul champ que Claude Code lit à partir d'une règle ; tout autre champ est ignoré sans erreur. Claude Code supprime le frontmatter avant de charger la règle en contexte.

| Champ   | Requis | Description                                                                                                                                         |
| :------ | :----- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `paths` | Non    | Modèles glob qui [scoped la règle aux fichiers correspondants](#path-specific-rules). Accepte une liste YAML ou une chaîne séparée par des virgules |

Si le YAML entre les marqueurs ne s'analyse pas, Claude Code ignore le frontmatter et charge la règle comme si elle n'avait pas de `paths`. Exécutez `claude --debug` pour voir l'erreur d'analyse.

<h4 id="share-rules-across-projects-with-symlinks">
  Partager les règles entre les projets avec des symlinks
</h4>

Le répertoire `.claude/rules/` supporte les symlinks, de sorte que vous pouvez maintenir un ensemble partagé de règles et les lier dans plusieurs projets. Les symlinks circulaires sont détectés et gérés correctement.

Claude Code traite un symlink dont la cible se trouve en dehors de votre répertoire de travail comme un [import externe](#import-additional-files). Les règles liées ne se chargent pas jusqu'à ce que vous approuviez les imports externes pour le projet, et après cela seulement celles sans champ [`paths`](#path-specific-rules) se chargent. Claude Code demande cette approbation uniquement quand un fichier de mémoire de projet importe un fichier en dehors du répertoire de travail avec `@path`, pas pour les symlinks seuls. Pour charger les règles partagées sans cette approbation, gardez-les dans [`~/.claude/rules/`](#user-level-rules), où elles s'appliquent à chaque projet sur votre machine.

Cet exemple lie à la fois un répertoire partagé et un fichier individuel :

```bash theme={null}
ln -s ~/shared-claude-rules .claude/rules/shared
ln -s ~/company-standards/security.md .claude/rules/security.md
```

<h4 id="user-level-rules">
  Règles au niveau utilisateur
</h4>

Les règles personnelles dans `~/.claude/rules/` s'appliquent à chaque projet sur votre machine. Utilisez-les pour les préférences qui ne sont pas spécifiques au projet :

```text theme={null}
~/.claude/rules/
├── preferences.md    # Vos préférences de codage personnelles
└── workflows.md      # Vos flux de travail préférés
```

Claude Code charge les règles au niveau utilisateur avant les règles de projet, de sorte qu'une règle de projet apparaît plus tard dans le contexte de Claude qu'une règle utilisateur. Aucun ensemble ne remplace l'autre : si une règle utilisateur et une règle de projet entrent en conflit, Claude peut suivre l'une ou l'autre, donc gardez les deux cohérentes.

<h3 id="manage-claude-md-for-large-teams">
  Gérer CLAUDE.md pour les grandes équipes
</h3>

Pour les organisations déployant Claude Code sur plusieurs équipes, vous pouvez centraliser les instructions et contrôler quels fichiers CLAUDE.md sont chargés.

<h4 id="deploy-organization-wide-claude-md">
  Déployer un CLAUDE.md à l'échelle de l'organisation
</h4>

Les organisations peuvent déployer un CLAUDE.md géré de manière centralisée qui s'applique à tous les utilisateurs sur une machine. Ce fichier ne peut pas être exclu par les paramètres individuels.

<Steps>
  <Step title="Créer le fichier à l'emplacement de la politique gérée">
    * macOS : `/Library/Application Support/ClaudeCode/CLAUDE.md`
    * Linux et WSL : `/etc/claude-code/CLAUDE.md`
    * Windows : `C:\Program Files\ClaudeCode\CLAUDE.md`
  </Step>

  <Step title="Déployer avec votre système de gestion de configuration">
    Utilisez MDM, Group Policy, Ansible ou des outils similaires pour distribuer le fichier sur les machines des développeurs. Consultez [paramètres gérés](/docs/fr/managed-settings) pour d'autres options de configuration à l'échelle de l'organisation.
  </Step>
</Steps>

La clé `claudeMd` vous permet de placer le contenu CLAUDE.md géré directement dans `managed-settings.json` au lieu de déployer un fichier séparé.

**Portée** : chaque session Claude Code sur la machine, dans chaque référentiel. Pour des conseils spécifiques au référentiel, validez un CLAUDE.md de projet à la place.

**Précédence** : identique à un fichier CLAUDE.md géré. Se charge avant CLAUDE.md utilisateur et projet.

**Où c'est honoré** : paramètres gérés et politiques uniquement. Définir `claudeMd` dans les paramètres utilisateur, projet ou locaux n'a aucun effet.

L'exemple ci-dessous ajoute des instructions comportementales directement dans un fichier de paramètres gérés :

```json theme={null}
{
  "claudeMd": "Always run `make lint` before committing.\nNever push directly to main."
}
```

Un CLAUDE.md géré et les [paramètres gérés](/docs/fr/managed-settings) servent des objectifs différents. Utilisez les paramètres pour l'application technique et CLAUDE.md pour les conseils comportementaux :

| Préoccupation                                                    | Configurer dans                                            |
| :--------------------------------------------------------------- | :--------------------------------------------------------- |
| Bloquer des outils, commandes ou chemins de fichiers spécifiques | Paramètres gérés : `permissions.deny`                      |
| Appliquer l'isolation du sandbox                                 | Paramètres gérés : `sandbox.enabled`                       |
| Variables d'environnement et routage du fournisseur d'API        | Paramètres gérés : `env`                                   |
| Méthode de connexion et restrictions d'organisation              | Paramètres gérés : `forceLoginMethod`, `forceLoginOrgUUID` |
| Directives de style de code et de qualité                        | CLAUDE.md géré                                             |
| Rappels de gestion des données et de conformité                  | CLAUDE.md géré                                             |
| Instructions comportementales pour Claude                        | CLAUDE.md géré                                             |

Les règles de paramètres sont appliquées par le client indépendamment de ce que Claude décide de faire. Les instructions CLAUDE.md façonnent le comportement de Claude mais ne constituent pas une couche d'application stricte.

<h4 id="exclude-specific-claude-md-files">
  Exclure des fichiers CLAUDE.md spécifiques
</h4>

Dans les grands monorepos, les fichiers CLAUDE.md ancêtres peuvent contenir des instructions qui ne sont pas pertinentes pour votre travail. Le paramètre `claudeMdExcludes` vous permet d'ignorer des fichiers spécifiques par chemin ou modèle glob.

Cet exemple exclut un CLAUDE.md de niveau supérieur et un répertoire de règles d'un dossier parent. Ajoutez-le à `.claude/settings.local.json` pour que l'exclusion reste locale à votre machine :

```json theme={null}
{
  "claudeMdExcludes": [
    "**/monorepo/CLAUDE.md",
    "/home/user/monorepo/other-team/.claude/rules/**"
  ]
}
```

Les modèles sont mis en correspondance avec les chemins de fichiers absolus en utilisant la syntaxe glob. Vous pouvez configurer `claudeMdExcludes` à n'importe quelle [couche de paramètres](/docs/fr/settings#where-settings-live) : utilisateur, projet, local ou politique gérée. Les tableaux fusionnent entre les couches.

Pour exclure un fichier de règles que vous atteignez via un [symlink](#share-rules-across-projects-with-symlinks), que le fichier ou son répertoire soit le lien, écrivez le modèle par rapport à l'un ou l'autre chemin : le chemin du fichier sous `.claude/rules/` ou sa cible de lien. Un modèle qui correspond à l'un ou l'autre chemin exclut le fichier. Avant v2.1.239, seul un modèle qui correspondait à la cible du lien excluait le fichier.

Les fichiers CLAUDE.md de politique gérée ne peuvent pas être exclus. Cela garantit que les instructions à l'échelle de l'organisation s'appliquent toujours indépendamment des paramètres individuels.

<h2 id="agents-md">
  AGENTS.md
</h2>

Claude Code peut lire [`AGENTS.md`](/docs/fr/glossary#agents-md) comme vos instructions de projet, donc un référentiel déjà configuré pour d'autres agents de codage fonctionne sans ajouter un `CLAUDE.md`, une importation ou un paramètre. Ce tableau montre ce que Claude lit par défaut pour chaque combinaison de fichiers d'instructions dans votre référentiel :

| Votre référentiel contient                                                                              | Claude lit                                                   |
| :------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------- |
| Un `AGENTS.md`, et aucun `CLAUDE.md` ou `CLAUDE.local.md` dans votre répertoire de travail ou au-dessus | Votre `AGENTS.md`                                            |
| Un `AGENTS.md` et un `CLAUDE.md` ou `CLAUDE.local.md` dans votre répertoire de travail ou au-dessus     | Vos fichiers `CLAUDE.md` uniquement                          |
| Un `CLAUDE.md` qui [importe déjà `AGENTS.md`](#share-one-file-with-other-coding-tools)                  | Votre `CLAUDE.md`, avec `AGENTS.md` inclus via l'importation |

Pour modifier le comportement par défaut, par exemple pour que Claude lise toujours les deux fichiers, lise uniquement `CLAUDE.md`, ou lise uniquement les instructions gérées de votre organisation, [modifiez le paramètre **Instructions du projet**](#choose-which-instruction-files-load).

<Note>
  La lecture directe de `AGENTS.md` nécessite Claude Code v2.1.277 ou une version ultérieure. Dans certaines sessions, Claude [ne peut pas lire `AGENTS.md`](#when-agents-md-support-is-unavailable), donc [importez-le à partir d'un `CLAUDE.md`](#share-one-file-with-other-coding-tools) à la place.
</Note>

<h3 id="when-claude-code-reads-agents-md">
  Quand Claude Code lit AGENTS.md
</h3>

Par défaut, Claude lit `AGENTS.md` uniquement quand vous n'avez pas de `CLAUDE.md` dans votre répertoire de travail ou au-dessus. Voici lesquels de vos fichiers comptent pour cette vérification :

* **Comptent, donc Claude les lit à la place de `AGENTS.md`** : un `CLAUDE.md`, `.claude/CLAUDE.md`, ou `CLAUDE.local.md` dans votre répertoire de travail ou n'importe quel répertoire au-dessus
* **Ne comptent pas, et continuent à se charger aux côtés de `AGENTS.md`** : votre `~/.claude/CLAUDE.md`, le `CLAUDE.md` géré de votre organisation, et les fichiers `.claude/rules/`

Quand aucun ne compte, voici ce que Claude lit et comment vous pouvez le dire :

* **Au démarrage de la session** : tous les `AGENTS.md` et `.claude/AGENTS.md` dans votre répertoire de travail et les répertoires au-dessus. Dans une session interactive, vous voyez une ligne comme `no CLAUDE.md found; AGENTS.md loaded: /home/you/repo/AGENTS.md` dans la conversation
* **Quand Claude travaille dans les sous-répertoires** : le `AGENTS.md` d'un sous-répertoire, quand Claude ouvre un fichier là avec l'outil Read et ce sous-répertoire n'a aucun des trois fichiers `CLAUDE.md` qui lui sont propres
* **À l'intérieur de chaque `AGENTS.md`** : les importations [`@path`](#import-additional-files) sont développées, les motifs [`claudeMdExcludes`](#exclude-specific-claude-md-files) s'appliquent, et les sous-agents qui [ignorent les instructions du projet](/docs/fr/sub-agents#what-loads-at-startup) ignorent aussi ces fichiers
* **Non lus** : `AGENTS.local.md`, `AGENTS.override.md`, ou n'importe quoi sous un répertoire `.agents/`

<Note>
  Parce que `CLAUDE.local.md` compte, en ajouter un pour conserver vos propres instructions non validées dans un projet qui repose sur `AGENTS.md` arrête Claude de lire `AGENTS.md` pour vous. Pour conserver votre `CLAUDE.local.md` et toujours avoir Claude qui lit `AGENTS.md`, définissez **Instructions du projet** sur [`claude-md-and-agents-md`](#choose-which-instruction-files-load).
</Note>

<h3 id="choose-which-instruction-files-load">
  Choisir quels fichiers d'instructions charger
</h3>

Pour modifier les fichiers que Claude lit, tapez `/config` dans une session Claude Code pour ouvrir le panneau des paramètres, puis définissez **Instructions du projet** sur l'une de ces valeurs :

| Valeur                    | Ce que Claude lit                                                                                                                                                                                                                                                                                                                                                                                                                 |
| :------------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `claude-md-or-agents-md`  | Vos fichiers `CLAUDE.md`, ou vos fichiers `AGENTS.md` quand vous n'avez pas de `CLAUDE.md` ou `CLAUDE.local.md` dans votre répertoire de travail ou au-dessus. C'est la valeur par défaut                                                                                                                                                                                                                                         |
| `claude-md-and-agents-md` | Vos fichiers `CLAUDE.md` et `AGENTS.md` ensemble, le `CLAUDE.md` de chaque répertoire en premier et son `AGENTS.md` après. Claude Code ignore un `AGENTS.md` qu'il a déjà chargé, donc celui que votre `CLAUDE.md` importe ou crée un lien symbolique vers n'est pas lu deux fois                                                                                                                                                 |
| `claude-md`               | Vos fichiers `CLAUDE.md` uniquement                                                                                                                                                                                                                                                                                                                                                                                               |
| `managed-only`            | Uniquement le `CLAUDE.md` géré de votre organisation et la [mémoire automatique](#auto-memory) au lancement. Vos fichiers `CLAUDE.md` de projet, local et utilisateur, vos fichiers `.claude/rules/`, et tous les `AGENTS.md` sont exclus. Le `CLAUDE.md` et les fichiers `.claude/rules/` d'un sous-répertoire, et les [règles délimitées par chemin](#path-specific-rules), se chargent toujours quand Claude lit un fichier là |

Vous pouvez également définir la valeur dans un fichier de paramètres au lieu de `/config`. Ajoutez-la sous l'ID du plugin `agents-md` intégré dans [`pluginConfigs`](/docs/fr/settings-reference#pluginconfigs), dans `~/.claude/settings.json`, un fichier `--settings`, ou les [paramètres gérés](/docs/fr/managed-settings). Claude Code l'ignore dans les fichiers de paramètres de projet et locaux. Cet exemple fait que Claude lit les deux fichiers :

```json settings.json theme={null}
{
  "pluginConfigs": {
    "agents-md@builtin": {
      "options": { "instructionFiles": "claude-md-and-agents-md" }
    }
  }
}
```

Votre modification s'applique à partir du prochain message que vous envoyez et dans chaque nouvelle session.

<h3 id="when-agents-md-support-is-unavailable">
  Quand le support de AGENTS.md n'est pas disponible
</h3>

Dans ces sessions, Claude lit uniquement les fichiers `CLAUDE.md`, et **Instructions du projet** n'apparaît pas dans le panneau des paramètres `/config` :

* Vous utilisez une version de Claude Code antérieure à v2.1.277
* Vous avez désactivé le plugin `agents-md` intégré dans `/plugin`
* Dans certains cas, c'est votre [première session après la mise à niveau](/docs/fr/env-vars#first-session-after-an-install-or-upgrade) à partir de v2.1.276 ou antérieur. Claude lit `AGENTS.md` à partir de votre prochaine session

Avant v2.1.281, certaines sessions, comme celles sur Amazon Bedrock ou avec la télémétrie désactivée, lisaient uniquement les fichiers `CLAUDE.md`. Sur ces versions, mettez à jour Claude Code. Pour donner votre `AGENTS.md` à Claude dans l'une de ces sessions, [importez-le à partir d'un `CLAUDE.md`](#share-one-file-with-other-coding-tools).

<h3 id="where-agents-md-differs-from-claude-md">
  Où AGENTS.md diffère de CLAUDE.md
</h3>

Un `AGENTS.md` que Claude lit via le paramètre **Instructions du projet** diffère d'un `CLAUDE.md` à ces endroits :

|                                                                                                                                                         | `CLAUDE.md`                                                                                | `AGENTS.md` lu via le paramètre                                                                                                          |
| :------------------------------------------------------------------------------------------------------------------------------------------------------ | :----------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------- |
| [Hooks `InstructionsLoaded`](/docs/fr/hooks#instructionsloaded)                                                                                              | Se déclenchent                                                                             | Ne se déclenchent pas. Ils se déclenchent comme d'habitude pour un `AGENTS.md` qu'un `CLAUDE.md` importe ou crée un lien symbolique vers |
| Répertoires que vous ajoutez avec `--add-dir` tandis que [`CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD`](#load-from-additional-directories) est défini | Leur `CLAUDE.md` se charge                                                                 | Leur `AGENTS.md` ne se charge pas                                                                                                        |
| Une importation `@path` d'un fichier en dehors de votre répertoire de travail                                                                           | Claude Code vous demande d'approuver les [importations externes](#import-additional-files) | Se charge uniquement si vous avez déjà approuvé les importations externes pour ce projet, sans invite                                    |

<h3 id="remove-an-earlier-agents-md-workaround">
  Supprimer une ancienne solution de contournement AGENTS.md
</h3>

Si vous avez configuré Claude Code pour lire `AGENTS.md` avant qu'il ne le fasse de lui-même, voici ce qu'il faut faire avec chaque configuration courante :

* **Un `CLAUDE.md` contenant `@AGENTS.md`** : vous pouvez le laisser. Conserver l'importation ne fait jamais que Claude lise `AGENTS.md` deux fois, quelle que soit la valeur **Instructions du projet** que vous utilisez. Supprimez le `CLAUDE.md` s'il ne contient rien d'autre, ou conservez-le si certaines de vos sessions [ne peuvent pas charger `AGENTS.md` directement](#when-agents-md-support-is-unavailable).
* **Un `CLAUDE.md` qui dit à Claude en mots de lire `AGENTS.md`** : Claude ne voit `AGENTS.md` que s'il décide d'ouvrir le fichier. Supprimez le `CLAUDE.md` pour que Claude lise `AGENTS.md` directement, ou remplacez la phrase par une importation `@AGENTS.md`.
* **Un `CLAUDE.md` lié symboliquement à `AGENTS.md`** : rien, ou supprimez le lien symbolique. De toute façon, Claude lit le contenu une fois.
* **Un hook `SessionStart` qui imprime `AGENTS.md`** : supprimez-le. Une fois que Claude lit `AGENTS.md` directement, le hook ajoute une deuxième copie au contexte.

<h3 id="share-one-file-with-other-coding-tools">
  Partager un fichier avec d'autres outils de codage
</h3>

Quand Claude ne lit pas votre `AGENTS.md` directement, vous pouvez toujours le conserver comme le seul fichier que chaque outil partage en mettant une importation `@AGENTS.md` dans un `CLAUDE.md` à côté. Faites cela quand votre projet a aussi un `CLAUDE.md`, quand vous avez défini **Instructions du projet** sur `claude-md`, ou dans les sessions qui [ne peuvent pas charger `AGENTS.md`](#when-agents-md-support-is-unavailable). Ajoutez toutes les instructions spécifiques à Claude sous l'importation, et Claude lit le fichier importé en premier, puis le reste :

```markdown CLAUDE.md theme={null}
@AGENTS.md

## Claude Code

Use plan mode for changes under `src/billing/`.
```

Si vous n'avez pas besoin de contenu spécifique à Claude, un lien symbolique fonctionne aussi :

```bash theme={null}
ln -s AGENTS.md CLAUDE.md
```

La commande n'imprime rien en cas de succès. Avant de choisir le lien symbolique plutôt que l'importation, vérifiez ces contraintes :

* **Édition** : Claude lit `CLAUDE.md` via le lien, mais les outils Edit et Write [refusent d'écrire via un lien symbolique](/docs/fr/errors#refusing-after-a-symlink-changed), et le refus dirige Claude à éditer la cible du lien, `AGENTS.md`, à la place
* **Windows** : si vous ou quelqu'un qui clone le référentiel travaillez sur Windows, utilisez l'importation `@AGENTS.md` à la place. Créer un lien symbolique là nécessite les privilèges d'administrateur ou le mode développeur, et Git vérifie un lien symbolique validé comme un fichier texte brut sauf si `core.symlinks` est activé, ce qui laisse ce clone avec un `CLAUDE.md` d'une ligne à la place de vos instructions

Avec l'une ou l'autre approche, exécutez `/context` dans votre prochaine session et confirmez que `CLAUDE.md` apparaît sous **Fichiers de mémoire**.

<h3 id="migrate-instructions-from-other-tools">
  Migrer les instructions d'autres outils
</h3>

L'exécution de [`/init`](/docs/fr/commands) lit les fichiers d'instructions d'autres outils et incorpore les parties pertinentes dans le `CLAUDE.md` généré :

* Règles Cursor dans `.cursor/rules/` ou `.cursorrules`
* Règles Copilot dans `.github/copilot-instructions.md`
* Avec `CLAUDE_CODE_NEW_INIT=1` défini : `AGENTS.md`, `.devin/rules/`, `.windsurf/rules/` ou `.windsurfrules`, et `.clinerules`

Vous pouvez également exécuter [`/import`](/docs/fr/commands) pour apporter la configuration d'un agent de codage pris en charge dans Claude Code, qui ajoute une copie unique de fichiers d'instructions tels que `AGENTS.md` au `CLAUDE.md` correspondant et transporte les serveurs MCP, les commandes, les sous-agents et les compétences. Nécessite Claude Code v2.1.213 ou une version ultérieure.

<h2 id="auto-memory">
  Mémoire automatique
</h2>

La mémoire automatique permet à Claude d'accumuler des connaissances d'une session à l'autre sans que vous n'écriviez rien. Au fur et à mesure qu'il travaille, Claude enregistre quatre types de notes pour lui-même. Claude enregistre le type sous la forme d'un champ `type` dans le frontmatter du fichier de mémoire :

* `user` : votre rôle, expertise et préférences de travail
* `feedback` : les corrections que vous donnez à Claude et les approches que vous confirmez
* `project` : le travail en cours, les délais et les décisions que Claude ne peut pas déduire du code ou de l'historique git
* `reference` : où trouver des informations en dehors du projet, comme un suivi de problèmes ou un tableau de bord

Claude ignore tout ce qu'il peut déduire de la base de code, comme l'architecture, les chemins de fichiers ou les correctifs de débogage. Il ignore également tout ce que vos fichiers CLAUDE.md disent déjà.

Claude ne sauvegarde pas quelque chose à chaque session. Il décide ce qui vaut la peine d'être mémorisé en fonction de si l'information serait utile dans une conversation future.

<h3 id="enable-or-disable-auto-memory">
  Activer ou désactiver la mémoire automatique
</h3>

La mémoire automatique est activée par défaut. Pour la basculer, ouvrez `/memory` dans une session et utilisez le bouton bascule de mémoire automatique, qui enregistre `autoMemoryEnabled` dans vos paramètres utilisateur à `~/.claude/settings.json`. Pour la désactiver pour un seul projet, définissez `autoMemoryEnabled` dans les paramètres de ce projet :

```json theme={null}
{
  "autoMemoryEnabled": false
}
```

Pour désactiver la mémoire automatique via une variable d'environnement, définissez `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`.

<h3 id="storage-location">
  Emplacement de stockage
</h3>

Chaque projet obtient son propre répertoire de mémoire à `~/.claude/projects/<project>/memory/`. Le chemin `<project>` est dérivé du référentiel git, donc tous les worktrees et sous-répertoires dans le même référentiel partagent un répertoire de mémoire automatique. En dehors d'un référentiel git, la racine du projet est utilisée à la place.

Si vous définissez [`CLAUDE_CODE_PROJECT_DIR_NAME`](/docs/fr/sessions#name-the-project-directory-yourself) à côté de `CLAUDE_CONFIG_DIR`, Claude Code utilise ce nom comme répertoire `<project>` sous `<config dir>/projects/` quel que soit le référentiel dans lequel vous le lancez, donc les projets lancés avec ce répertoire de configuration partagent un répertoire de mémoire automatique. Nécessite Claude Code v2.1.234 ou ultérieur.

Pour stocker la mémoire automatique dans un emplacement différent, définissez `autoMemoryDirectory` dans votre `settings.json`. Il est lu à partir de n'importe quel [scope de paramètres](/docs/fr/settings#settings-precedence) : utilisateur, projet, local, politique, ou `--settings`.

```json theme={null}
{
  "autoMemoryDirectory": "~/my-custom-memory-dir"
}
```

La valeur doit être un chemin absolu ou commencer par `~/`.

Lorsqu'elle est définie dans le `.claude/settings.json` ou `.claude/settings.local.json` d'un projet, Claude Code l'honore selon la même [règle de confiance de l'espace de travail que les hooks dans les fichiers de paramètres](/docs/fr/permissions#what-runs-before-you-trust-a-folder). Tandis que [`permissions.blockReadsOutsideWorkingDirectories`](/docs/fr/settings-reference#permissions-blockreadsoutsideworkingdirectories) est activé, Claude Code ne charge aucune mémoire automatique à partir d'un répertoire qu'un [fichier de paramètres fourni par le référentiel](/docs/fr/permissions#when-your-local-settings-file-needs-trust) choisit et n'en sauvegarde aucune, où que ce répertoire se trouve.

Le répertoire contient un index `MEMORY.md` et un fichier de sujet par mémoire :

```text theme={null}
~/.claude/projects/<project>/memory/
├── MEMORY.md           # Index, une ligne par mémoire, chargé dans chaque session
├── user_role.md        # Une mémoire
├── feedback_testing.md # Une mémoire
└── ...                 # Tout autre fichier de sujet que Claude crée
```

`MEMORY.md` agit comme un index du répertoire de mémoire. Claude lit et écrit des fichiers dans ce répertoire tout au long de votre session, en utilisant `MEMORY.md` pour garder une trace de ce qui est stocké où.

La mémoire automatique est locale à la machine. Tous les worktrees et sous-répertoires dans le même référentiel git partagent un répertoire de mémoire automatique. Les fichiers ne sont pas partagés entre les machines ou les environnements cloud.

Claude Code supprime les anciennes transcriptions de session après la période de rétention [`cleanupPeriodDays`](/docs/fr/settings-reference#cleanupperioddays), mais exclut les fichiers de mémoire du répertoire de mémoire de ce [balayage de rétention](/docs/fr/claude-directory#cleaned-up-automatically). `MEMORY.md` et les fichiers de sujet restent jusqu'à ce que vous ou Claude les modifiiez ou les supprimiez.

<h3 id="how-it-works">
  Comment ça marche
</h3>

Les 200 premières lignes de `MEMORY.md`, ou les premiers 25 KB, selon ce qui vient en premier, sont chargés au début de chaque conversation. Le contenu au-delà de ce seuil n'est pas chargé au démarrage de la session. Claude garde `MEMORY.md` concis en déplaçant les notes détaillées dans des fichiers de sujet séparés.

Après que Claude écrive dans `MEMORY.md`, Claude Code mesure le fichier par rapport aux limites de lecture de 200 lignes et 25 KB. Si le fichier est proche d'une limite, Claude Code rappelle à Claude de le raccourcir : garder une ligne par entrée, déplacer les détails dans les fichiers de sujet, et fusionner ou supprimer les entrées obsolètes. Si le fichier dépasse une limite, l'écriture réussit toujours, mais Claude Code retourne une [erreur indiquant à Claude de réécrire l'index](/docs/fr/errors#memory-index-is-over-its-read-limit), car tout ce qui dépasse la limite est supprimé au prochain chargement.

Cette limite s'applique uniquement à `MEMORY.md`. Claude Code charge un fichier CLAUDE.md de jusqu'à 4 MiB en intégralité et ignore un fichier plus volumineux. Les fichiers plus courts produisent une meilleure adhérence.

Claude Code ne charge pas les fichiers de sujet tels que `user_role.md` ou `feedback_testing.md` au démarrage. Claude les lit à la demande en utilisant ses outils de fichiers standard quand il a besoin de l'information.

La mémoire automatique de la conversation principale n'est pas chargée dans les [sous-agents](/docs/fr/sub-agents#what-loads-at-startup) ; l'exception est un [fork](/docs/fr/sub-agents#fork-the-current-conversation), qui hérite de la conversation parent et de l'invite système. La propre mémoire automatique d'un sous-agent, activée avec le champ `memory` du sous-agent, est un répertoire séparé.

Claude lit et écrit les fichiers de mémoire pendant votre session. Quand vous voyez des messages comme « Saved 2 memories » ou « Recalled 2 memories » dans l'interface Claude Code, Claude met activement à jour ou lit à partir de `~/.claude/projects/<project>/memory/`.

Quand Claude écrit un fichier de mémoire qui commence par du frontmatter YAML, Claude Code enregistre l'heure d'écriture dans un champ frontmatter `modified` sous la forme d'un horodatage ISO 8601. L'horodatage montre à quel point le fait est actuel, à la fois pour vous et pour Claude quand il relit la mémoire. Tout fichier qui a du frontmatter obtient le champ la prochaine fois que Claude l'écrit, y compris les fichiers créés sur des versions antérieures ; Claude Code n'ajoute jamais de frontmatter à un fichier qui n'en a pas. Le champ `modified` nécessite Claude Code v2.1.214 ou ultérieur.

<h3 id="audit-and-edit-your-memory">
  Auditer et modifier votre mémoire
</h3>

Les fichiers de mémoire automatique sont du markdown brut que vous pouvez modifier ou supprimer à tout moment. Exécutez [`/memory`](#view-and-edit-with-%2Fmemory) pour parcourir et ouvrir les fichiers de mémoire à partir d'une session.

<h2 id="view-and-edit-with-/memory">
  Afficher et modifier avec `/memory`
</h2>

La commande `/memory` liste vos fichiers CLAUDE.md, CLAUDE.local.md et autres emplacements de fichiers de mémoire dans les portées utilisateur et projet, y compris les entrées CLAUDE.md utilisateur et projet pour les fichiers qui n'existent pas encore. Elle vous permet également de basculer la mémoire automatique activée ou désactivée et fournit une option pour ouvrir le dossier de mémoire automatique. Sélectionnez n'importe quel fichier pour l'ouvrir dans votre éditeur ; en sélectionner un qui n'existe pas encore le crée d'abord. Pour vérifier quels fichiers `CLAUDE.md` et fichiers de règles ont été chargés dans la session actuelle, exécutez `/context`.

Les éditeurs GUI tels que VS Code ouvrent le fichier dans une fenêtre séparée, et vous pouvez continuer à utiliser la session pendant qu'il est ouvert. Avant la v2.1.216, `/memory` attendait que vous fermiez le fichier avant de répondre. Les éditeurs de terminal tels que Vim prennent le contrôle du terminal jusqu'à ce que vous quittiez.

Quand vous demandez à Claude de se souvenir de quelque chose, comme « toujours utiliser pnpm, pas npm » ou « se souvenir que les tests d'API nécessitent une instance Redis locale », Claude l'enregistre dans la mémoire automatique. Pour ajouter des instructions à CLAUDE.md à la place, demandez directement à Claude, comme « ajouter ceci à CLAUDE.md », ou modifiez le fichier vous-même via `/memory`.

<h2 id="troubleshoot-memory-issues">
  Dépanner les problèmes de mémoire
</h2>

Ce sont les problèmes les plus courants avec CLAUDE.md et la mémoire automatique, ainsi que les étapes pour les déboguer.

<h3 id="claude-isn’t-following-my-claude-md">
  Claude ne suit pas mon CLAUDE.md
</h3>

Le contenu CLAUDE.md est livré en tant que message utilisateur après l'invite système, pas en tant que partie de l'invite système elle-même. Claude le lit et essaie de le suivre, mais il n'y a aucune garantie de conformité stricte, surtout pour les instructions vagues ou conflictuelles.

Pour déboguer :

* Exécutez `/context` et vérifiez la liste sous **Fichiers de mémoire** pour confirmer que vos fichiers CLAUDE.md et CLAUDE.local.md sont chargés. Si un fichier `CLAUDE.md` est manquant là, Claude ne peut pas le voir. Utilisez `/memory` pour ouvrir et modifier les fichiers.
* Vérifiez que le CLAUDE.md pertinent se trouve dans un emplacement qui se charge pour votre session (consultez [Choisir où placer les fichiers CLAUDE.md](#choose-where-to-put-claude-md-files)).
* Rendez les instructions plus spécifiques. « Utiliser l'indentation à 2 espaces » fonctionne mieux que « formater le code correctement ».
* Recherchez les instructions conflictuelles dans les fichiers CLAUDE.md. Si deux fichiers donnent des conseils différents pour le même comportement, Claude peut en choisir un arbitrairement.

Si l'instruction est quelque chose qui doit s'exécuter à un moment spécifique, comme avant chaque commit ou après chaque modification de fichier, écrivez-la plutôt comme un [hook](/docs/fr/hooks-guide). Les hooks s'exécutent en tant que commandes shell à des événements de cycle de vie fixes et s'appliquent indépendamment de ce que Claude décide de faire.

Pour les instructions que vous voulez au niveau de l'invite système, utilisez [`--append-system-prompt`](/docs/fr/cli-reference#system-prompt-flags). Vous le passez au lancement, donc c'est mieux adapté aux scripts et à l'automatisation qu'à l'utilisation interactive. Pour savoir comment il se comporte lorsque vous reprenez une conversation, consultez [Indicateurs d'invite système dans les conversations reprises](/docs/fr/cli-reference#system-prompt-flags-in-resumed-conversations).

<Tip>
  Utilisez le [hook `InstructionsLoaded`](/docs/fr/hooks#instructionsloaded) pour enregistrer exactement quels fichiers `CLAUDE.md` et fichiers de règles sont chargés, quand ils se chargent et pourquoi. C'est utile pour déboguer les règles spécifiques au chemin ou les fichiers chargés tardivement dans les sous-répertoires.
</Tip>

<h3 id="my-agents-md-isn’t-loading">
  Mon AGENTS.md ne se charge pas
</h3>

Si votre référentiel a un `AGENTS.md` et que Claude ne semble pas savoir ce qu'il dit, la cause habituelle est un `CLAUDE.md` quelque part sur le chemin du projet. Par défaut, Claude lit `AGENTS.md` uniquement lorsque vous n'avez pas de `CLAUDE.md` ou `CLAUDE.local.md` dans votre répertoire de travail ou au-dessus. Vérifiez ceux-ci dans l'ordre :

1. Recherchez un `CLAUDE.md`, `.claude/CLAUDE.md`, ou `CLAUDE.local.md` dans votre répertoire de travail ou dans n'importe quel répertoire au-dessus, autre que votre `~/.claude/CLAUDE.md`. Si vous en trouvez un, Claude le lit à la place de `AGENTS.md` sauf si vous définissez **Instructions du projet** sur `claude-md-and-agents-md`.
2. Exécutez `claude --version` et confirmez v2.1.277 ou ultérieure. Avant v2.1.281, certaines sessions, comme celles sur Amazon Bedrock ou avec la télémétrie désactivée, [ne pouvaient pas charger `AGENTS.md`](#when-agents-md-support-is-unavailable) non plus, donc sur ces versions, mettez à jour vers v2.1.281 ou ultérieure.
3. Tapez `/config` dans votre session pour ouvrir le panneau des paramètres et confirmez que **Instructions du projet** n'est pas défini sur `claude-md` ou `managed-only`. Si vous ne voyez pas le paramètre du tout, votre session est une session qui [ne peut pas charger `AGENTS.md`](#when-agents-md-support-is-unavailable).

Pour vérifier si Claude a lu votre `AGENTS.md`, exécutez `/memory` et recherchez son chemin dans la liste.

Avant v2.1.280, `/memory` et `/context` ne listaient pas un `AGENTS.md` que Claude lisait directement. Sur ces versions, demandez plutôt à Claude ce que ses instructions du projet disent.

Si vous voulez conserver le `CLAUDE.md` que vous avez trouvé, ou si votre session ne peut pas charger `AGENTS.md`, [ajoutez un `CLAUDE.md` à côté de votre `AGENTS.md` qui l'importe](#share-one-file-with-other-coding-tools).

<h3 id="i-don’t-know-what-auto-memory-saved">
  Je ne sais pas ce que la mémoire automatique a enregistré
</h3>

Exécutez `/memory` et sélectionnez le dossier de mémoire automatique pour parcourir ce que Claude a enregistré. Tout est du markdown brut que vous pouvez lire, modifier ou supprimer.

<h3 id="my-claude-md-is-too-large">
  Mon CLAUDE.md est trop volumineux
</h3>

Les fichiers de plus de 200 lignes consomment plus de contexte et peuvent réduire l'adhérence. Claude Code ignore un fichier de plus de 4 Mio. Utilisez les [règles spécifiques au chemin](#path-specific-rules) pour charger les instructions uniquement lorsque Claude travaille avec des fichiers correspondants, ou réduisez le contenu qui n'est pas nécessaire dans chaque session. La division en [imports `@path`](#import-additional-files) aide à l'organisation mais ne réduit pas le contexte, puisque les fichiers importés se chargent au lancement.

La vérification [`/doctor`](/docs/fr/commands#all-commands) propose des réductions pour un CLAUDE.md enregistré : elle supprime le contenu que Claude peut dériver de la base de code, comme les dispositions de répertoires, les listes de dépendances et les aperçus d'architecture, et conserve les pièges, la justification et les conventions qui diffèrent des paramètres par défaut des outils. La vérification de réduction nécessite Claude Code v2.1.206 ou version ultérieure.

<h3 id="instructions-seem-lost-after-/compact">
  Les instructions semblent perdues après `/compact`
</h3>

CLAUDE.md à la racine du projet survit à la compaction : après `/compact`, Claude relit votre CLAUDE.md à partir du disque et le réinjecte à nouveau dans la session. Les fichiers CLAUDE.md imbriqués dans les sous-répertoires et les règles avec [frontmatter `paths:`](#path-specific-rules) se rechargent lorsque Claude lit les fichiers auxquels elles s'appliquent.

Si une instruction a disparu après la compaction, elle a été donnée uniquement dans la conversation, se trouve dans un CLAUDE.md imbriqué qui ne s'est pas encore rechargé, ou est une règle spécifique au chemin qui n'a pas correspondu à un fichier depuis. Ajoutez les instructions données uniquement dans la conversation à CLAUDE.md pour les rendre persistantes. Consultez [Ce qui survit à la compaction](/docs/fr/context-window#what-survives-compaction) pour la répartition complète.

Consultez [Écrire des instructions efficaces](#write-effective-instructions) pour des conseils sur la taille, la structure et la spécificité.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Déboguer votre configuration](/docs/fr/debug-your-config) : diagnostiquez pourquoi CLAUDE.md ou les paramètres ne prennent pas effet
* [Skills](/docs/fr/skills) : empaquetez les flux de travail répétables qui se chargent à la demande
* [Paramètres](/docs/fr/settings) : configurez le comportement de Claude Code avec les fichiers de paramètres
* [Mémoire des subagents](/docs/fr/sub-agents#enable-persistent-memory) : laissez les subagents maintenir leur propre mémoire automatique
