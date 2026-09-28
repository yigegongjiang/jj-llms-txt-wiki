> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Étendre Claude Code

> Comprenez quand utiliser CLAUDE.md, Skills, subagents, hooks, MCP et plugins.

Claude Code combine un modèle qui raisonne sur votre code avec des [outils intégrés](/docs/fr/how-claude-code-works#tools) pour les opérations sur fichiers, la recherche, l'exécution et l'accès web. Les outils intégrés couvrent la plupart des tâches de codage. Ce guide couvre la couche d'extension : les fonctionnalités que vous ajoutez pour personnaliser ce que Claude connaît, le connecter à des services externes et automatiser les flux de travail.

<Note>
  Pour savoir comment fonctionne la boucle agentive principale, consultez [Comment fonctionne Claude Code](/docs/fr/how-claude-code-works).
</Note>

**Nouveau dans Claude Code ?** Commencez par [CLAUDE.md](/docs/fr/memory) pour les conventions de projet, puis ajoutez d'autres extensions [au fur et à mesure que des déclencheurs spécifiques se présentent](#build-your-setup-over-time).

<h2 id="overview">
  Aperçu
</h2>

Les extensions se connectent à différentes parties de la boucle agentive :

* **[CLAUDE.md](/docs/fr/memory)** ajoute un contexte persistant que Claude voit à chaque session
* **[Output styles](/docs/fr/output-styles)** définissent le rôle, le ton et le format de réponse de Claude pour chaque réponse dans une session
* **[Skills](/docs/fr/skills)** ajoutent des connaissances réutilisables et des flux de travail invocables
* **[Code intelligence](/docs/fr/tools-reference#lsp-tool-behavior)** connecte Claude à un serveur de langage pour la navigation au niveau des symboles et les erreurs de type en direct
* **[MCP](/docs/fr/mcp)** connecte Claude à des services et outils externes
* **[Subagents](/docs/fr/sub-agents)** exécutent leurs propres boucles dans un contexte isolé, en retournant des résumés
* **[Dynamic workflows](/docs/fr/workflows)** exécutent de nombreux subagents à partir d'un script que Claude écrit, en retournant un seul résultat
* **[Cross-session messaging](/docs/fr/cross-session-messaging)** permet à Claude de transmettre un message d'une de vos sessions à une autre
* **[Hooks](/docs/fr/hooks-guide)** exécutent votre script, requête HTTP, appel d'outil MCP, invite ou subagent quand Claude Code atteint un événement du cycle de vie
* **[Plugins](/docs/fr/plugins/overview)** et **[marketplaces](/docs/fr/plugins/overview)** empaquettent et distribuent ces fonctionnalités

[Skills](/docs/fr/skills) sont l'extension la plus flexible. Une skill est un fichier markdown contenant des connaissances, des flux de travail ou des instructions. Vous pouvez invoquer des skills avec une commande comme `/deploy`, ou Claude peut les charger automatiquement quand elles sont pertinentes. Les skills peuvent s'exécuter dans votre conversation actuelle ou dans un contexte isolé via des subagents.

<h2 id="match-features-to-your-goal">
  Associer les fonctionnalités à votre objectif
</h2>

Les fonctionnalités vont du contexte toujours actif que Claude voit à chaque session, aux capacités à la demande que vous ou Claude pouvez invoquer, à l'automatisation en arrière-plan qui s'exécute sur des événements spécifiques. Le tableau ci-dessous montre ce qui est disponible et quand chaque option a du sens.

| Fonctionnalité                                                 | Ce qu'elle fait                                                                                         | Quand l'utiliser                                                                                                                                    | Exemple                                                                                                                                   |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| **CLAUDE.md**                                                  | Contexte persistant chargé à chaque conversation                                                        | Conventions de projet, règles « toujours faire X »                                                                                                  | « Utilisez pnpm, pas npm. Exécutez les tests avant de valider. »                                                                          |
| **[Style de sortie](/docs/fr/output-styles)**                       | Instructions qui définissent le rôle, le ton et le format de réponse de Claude pour une session entière | Une voix, une longueur ou un format que vous voulez dans chaque réponse, ou Claude travaillant comme quelque chose d'autre qu'un ingénieur logiciel | Le style Concise intégré pour des réponses plus courtes ; un style personnalisé qui répond à chaque question avec un diagramme en premier |
| **Skill**                                                      | Instructions, connaissances et flux de travail que Claude peut utiliser                                 | Contenu réutilisable, documents de référence, tâches répétables                                                                                     | `/deploy` exécute votre liste de contrôle de déploiement ; skill de documentation API avec modèles de points de terminaison               |
| **Subagent**                                                   | Contexte d'exécution isolé qui retourne des résultats résumés                                           | Isolation du contexte, tâches parallèles, travailleurs spécialisés                                                                                  | Tâche de recherche qui lit de nombreux fichiers mais retourne uniquement les conclusions clés                                             |
| **[Flux de travail dynamique](/docs/fr/workflows)**                 | Script que Claude écrit qui exécute de nombreux subagents en arrière-plan                               | Travail qui dépasse une poignée de subagents, ou des conclusions que vous voulez vérifier en croisant                                               | Auditez un codebase entier, avec un deuxième ensemble d'agents vérifiant chaque conclusion                                                |
| **[Messagerie inter-sessions](/docs/fr/cross-session-messaging)**   | Claude livre un message d'une de vos sessions à une autre                                               | Sessions que vous exécutez vous-même qui ont besoin des conclusions les unes des autres en cours de tâche                                           | Une session avertit une autre qu'un changement qu'elle a apporté casse ce sur quoi l'autre s'appuie                                       |
| **[Code intelligence](/docs/fr/tools-reference#lsp-tool-behavior)** | Navigation et diagnostics du serveur de langage                                                         | Langages typés, grands codebases où grep est lent ou imprécis                                                                                       | Accédez à la définition d'un symbole au lieu de lire le fichier entier                                                                    |
| **MCP**                                                        | Connectez-vous à des services externes                                                                  | Données ou actions externes                                                                                                                         | Interrogez votre base de données, publiez sur Slack, contrôlez un navigateur                                                              |
| **Hook**                                                       | Script, requête HTTP, appel d'outil MCP, invite ou subagent déclenché par des événements                | Automatisation qui doit s'exécuter sur chaque événement correspondant                                                                               | Exécutez ESLint après chaque modification de fichier                                                                                      |
| **[Artifact](/docs/fr/artifacts)**                                  | Publiez la sortie de session en tant que page web privée et interactive                                 | Sortie que vous voulez voir ou partager visuellement plutôt que sous forme de texte terminal                                                        | Une chronologie d'incident qui se met à jour au fur et à mesure que Claude enquête                                                        |

**[Plugins](/docs/fr/plugins/overview)** sont la couche d'empaquetage. Un plugin regroupe des skills, des hooks, des subagents et des serveurs MCP dans une seule unité installable. Les skills de plugin sont espacées de noms (comme `/my-plugin:review`) afin que plusieurs plugins puissent coexister. Utilisez les plugins quand vous voulez réutiliser la même configuration sur plusieurs référentiels ou distribuer à d'autres via une **[marketplace](/docs/fr/plugins/overview)**.

<h3 id="build-your-setup-over-time">
  Construire votre configuration au fil du temps
</h3>

Vous n'avez pas besoin de tout configurer à l'avance. Chaque fonctionnalité a un déclencheur reconnaissable, et la plupart des équipes les ajoutent à peu près dans cet ordre :

| Déclencheur                                                                                                    | Ajouter                                                                                      |
| :------------------------------------------------------------------------------------------------------------- | :------------------------------------------------------------------------------------------- |
| Claude se trompe sur une convention ou une commande deux fois                                                  | Ajoutez-la à [CLAUDE.md](/docs/fr/memory)                                                         |
| Vous continuez à demander à Claude d'être plus court, d'expliquer davantage ou de répondre dans le même format | Définissez un [style de sortie](/docs/fr/output-styles)                                           |
| Vous continuez à taper la même invite pour démarrer une tâche                                                  | Enregistrez-la en tant que [skill](/docs/fr/skills) invocable par l'utilisateur                   |
| Vous collez le même playbook ou procédure multi-étapes dans le chat pour la troisième fois                     | Capturez-la en tant que [skill](/docs/fr/skills)                                                  |
| Vous continuez à copier des données d'un onglet de navigateur que Claude ne peut pas voir                      | Connectez ce système en tant que [serveur MCP](/docs/fr/mcp)                                      |
| Claude lit de nombreux fichiers pour trouver où un symbole est défini ou utilisé                               | Installez un [plugin de code intelligence](/docs/fr/plugins/code-intelligence) pour votre langage |
| Une tâche secondaire inonde votre conversation avec une sortie que vous ne référencerez plus                   | Acheminez-la via un [subagent](/docs/fr/sub-agents)                                               |
| Vous voulez que quelque chose se produise à chaque fois sans demander                                          | Écrivez un [hook](/docs/fr/hooks-guide)                                                           |
| Un deuxième référentiel a besoin de la même configuration                                                      | Empaquetez-la en tant que [plugin](/docs/fr/plugins/overview)                                     |

Les mêmes déclencheurs vous indiquent quand mettre à jour ce que vous avez déjà. Une erreur répétée ou un commentaire d'examen récurrent est une modification de CLAUDE.md, pas une correction ponctuelle dans le chat. Un flux de travail que vous continuez à ajuster manuellement est une skill qui a besoin d'une autre révision.

<h3 id="compare-similar-features">
  Comparer les fonctionnalités similaires
</h3>

Certaines fonctionnalités peuvent sembler similaires. Pour une explication plus approfondie du choix entre elles, consultez [Steering Claude Code : when to use CLAUDE.md, skills, hooks, and subagents](https://claude.com/blog/steering-claude-code-skills-hooks-rules-subagents-and-more) sur le blog. Voici comment les distinguer.

<Tabs>
  <Tab title="Skill vs Subagent">
    Les skills et les subagents résolvent des problèmes différents :

    * **Skills** sont du contenu réutilisable que vous pouvez charger dans n'importe quel contexte
    * **Subagents** sont des travailleurs isolés qui s'exécutent séparément de votre conversation principale

    | Aspect                                                     | Skill                                                        | Subagent                                                                            |
    | ---------------------------------------------------------- | ------------------------------------------------------------ | ----------------------------------------------------------------------------------- |
    | **Ce que c'est**                                           | Instructions, connaissances ou flux de travail réutilisables | Travailleur isolé avec son propre contexte                                          |
    | **Avantage clé**                                           | Partagez le contenu entre les contextes                      | Isolation du contexte. Le travail se fait séparément, seul le résumé revient        |
    | **Impact de la [fenêtre de contexte](/docs/fr/context-window)** | S'ajoute à votre fenêtre principale                          | Utilise une fenêtre séparée avec ses propres tokens d'entrée et de sortie           |
    | **Meilleur pour**                                          | Matériel de référence, flux de travail invocables            | Tâches qui lisent de nombreux fichiers, travail parallèle, travailleurs spécialisés |

    **Les skills peuvent être de référence ou d'action.** Les skills de référence fournissent des connaissances que Claude utilise tout au long de votre session (comme votre guide de style API). Les skills d'action disent à Claude de faire quelque chose de spécifique (comme `/deploy` qui exécute votre flux de travail de déploiement).

    **Utilisez un subagent** quand vous avez besoin d'isolation du contexte ou quand votre fenêtre de contexte se remplit. Le subagent pourrait lire des dizaines de fichiers ou exécuter des recherches étendues, mais votre conversation principale ne reçoit qu'un résumé. Puisque le travail du subagent ne consomme pas votre contexte principal, c'est aussi utile quand vous n'avez pas besoin que le travail intermédiaire reste visible. Les subagents personnalisés peuvent avoir leurs propres instructions et peuvent précharger des skills.

    **Ils peuvent se combiner.** Un subagent peut précharger des skills spécifiques (champ `skills:`). Une skill peut s'exécuter dans un contexte isolé en utilisant `context: fork`. Consultez [Skills](/docs/fr/skills) pour plus de détails.
  </Tab>

  <Tab title="CLAUDE.md vs Skill">
    Les deux stockent des instructions, mais elles se chargent différemment et servent des objectifs différents.

    | Aspect                                  | CLAUDE.md                          | Skill                                             |
    | --------------------------------------- | ---------------------------------- | ------------------------------------------------- |
    | **Se charge**                           | À chaque session, automatiquement  | À la demande                                      |
    | **Peut inclure des fichiers**           | Oui, avec les importations `@path` | Oui, avec les importations `@path`                |
    | **Peut déclencher des flux de travail** | Non                                | Oui, avec `/<name>`                               |
    | **Meilleur pour**                       | Règles « toujours faire X »        | Matériel de référence, flux de travail invocables |

    **Mettez-le dans CLAUDE.md** si Claude devrait toujours le savoir : conventions de codage, commandes de construction, structure du projet, règles « ne jamais faire X ».

    **Mettez-le dans une skill** si c'est du matériel de référence dont Claude a besoin parfois (documentation API, guides de style) ou un flux de travail que vous déclenchez avec `/<name>` (déployer, examiner, publier).

    **Règle générale :** Gardez CLAUDE.md sous 200 lignes. S'il grandit, déplacez le contenu de référence vers des skills ou divisez-le en fichiers [`.claude/rules/`](/docs/fr/memory#organize-rules-with-claude/rules/).
  </Tab>

  <Tab title="CLAUDE.md vs Output style">
    Les deux donnent à Claude des instructions permanentes. CLAUDE.md porte ce que Claude devrait savoir, et un style de sortie définit comment Claude répond.

    | Aspect            | CLAUDE.md                                                            | Style de sortie                                                                                               |
    | ----------------- | -------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
    | **Contient**      | Faits et règles sur votre projet                                     | Un rôle, un ton et un format de réponse                                                                       |
    | **Changement**    | Toujours chargé                                                      | Un actif à la fois ; [changez les styles](/docs/fr/output-styles#change-your-output-style) quand vous le souhaitez |
    | **Meilleur pour** | Commandes de construction, conventions, règles « ne jamais faire X » | Réponses plus courtes, explications aux côtés du code, un rôle non-ingénierie                                 |

    **Mettez-le dans CLAUDE.md** si c'est vrai du projet quel que soit le style dans lequel vous êtes : conventions de codage, commandes de construction, structure du projet.

    **Utilisez un style de sortie** si c'est sur la réponse elle-même et que vous pourriez vouloir l'éteindre à nouveau : longueur, format, combien Claude explique, ou un rôle différent comme un assistant en écriture. Claude Code inclut [des styles intégrés](/docs/fr/output-styles#built-in-output-styles), et vous pouvez écrire les vôtres.

    **Ils se combinent.** CLAUDE.md reste chargé quel que soit le style que vous choisissez. Claude suit les deux comme instructions, donc aucun n'est appliqué. Pour tout ce qui doit se produire à chaque fois, utilisez un [hook](/docs/fr/hooks-guide).
  </Tab>

  <Tab title="CLAUDE.md vs Rules vs Skills">
    Les trois stockent des instructions, mais elles se chargent différemment :

    | Aspect            | CLAUDE.md                                            | `.claude/rules/`                                                    | Skill                                             |
    | ----------------- | ---------------------------------------------------- | ------------------------------------------------------------------- | ------------------------------------------------- |
    | **Se charge**     | À chaque session                                     | À chaque session, ou quand les fichiers correspondants sont ouverts | À la demande, quand invoqué ou pertinent          |
    | **Portée**        | Projet entier                                        | Peut être limité aux chemins de fichiers                            | Spécifique à la tâche                             |
    | **Meilleur pour** | Conventions principales et commandes de construction | Directives spécifiques au langage ou au répertoire                  | Matériel de référence, flux de travail répétables |

    **Utilisez CLAUDE.md** pour les instructions que chaque session a besoin : commandes de construction, conventions de test, architecture du projet.

    **Utilisez les règles** pour garder CLAUDE.md concentré. Les règles avec [frontmatter `paths`](/docs/fr/memory#path-specific-rules) ne se chargent que quand Claude travaille avec des fichiers correspondants, économisant du contexte.

    **Utilisez les skills** pour le contenu dont Claude n'a besoin que parfois, comme la documentation API ou une liste de contrôle de déploiement que vous déclenchez avec `/<name>`.
  </Tab>

  <Tab title="Subagent vs Flux de travail dynamique">
    Les deux font du travail en dehors de votre conversation principale. Avec les subagents, Claude décide tour après tour ce qui s'exécute ensuite. Dans un flux de travail, le script décide :

    * **Subagents** sont des travailleurs que Claude génère, chacun retournant un résumé à la conversation qui l'a généré
    * **[Flux de travail dynamique](/docs/fr/workflows)** sont des scripts que Claude écrit qui exécutent de nombreux subagents en arrière-plan et retournent un résultat

    **Utilisez un subagent** quand vous avez besoin d'un travailleur rapide et ciblé : rechercher une question, vérifier une affirmation, examiner un fichier. Le subagent fait le travail et retourne un résumé, afin que votre conversation principale reste propre. Les subagents que Claude a nommés quand il les a générés peuvent aussi [se messaguer les uns les autres](/docs/fr/sub-agents#what-loads-at-startup).

    **Utilisez un flux de travail dynamique** quand un travail [dépasse une poignée de subagents](/docs/fr/workflows#when-to-use-a-workflow), ou quand vous voulez que les conclusions soient vérifiées en croisant avant que vous les voyiez, comme un audit à l'échelle du codebase, une grande migration ou un plan rédigé sous plusieurs angles. Pour en démarrer un, [demandez un flux de travail dans votre invite](/docs/fr/workflows#ask-for-a-workflow-in-your-prompt).

    **Pour transmettre une conclusion d'une de vos sessions à une autre**, demandez au Claude de la première session de l'envoyer. Claude la livre avec [messagerie inter-sessions](/docs/fr/cross-session-messaging). [Exécutez les agents en parallèle](/docs/fr/agents) compare les autres façons d'exécuter plus d'un Claude à la fois, y compris les sessions que vous transmettez et vérifiez plus tard.
  </Tab>

  <Tab title="MCP vs Skill">
    MCP connecte Claude à des services externes. Les skills étendent ce que Claude connaît, y compris comment utiliser efficacement ces services.

    | Aspect           | MCP                                                                    | Skill                                                                                      |
    | ---------------- | ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------ |
    | **Ce que c'est** | Protocole de connexion à des services externes                         | Connaissances, flux de travail et matériel de référence                                    |
    | **Fournit**      | Accès aux outils et aux données                                        | Connaissances, flux de travail, matériel de référence                                      |
    | **Exemples**     | Intégration Slack, requêtes de base de données, contrôle de navigateur | Liste de contrôle de relecture de code, flux de travail de déploiement, guide de style API |

    Ces solutions résolvent des problèmes différents et fonctionnent bien ensemble :

    **MCP** donne à Claude des outils spécialisés pour un système externe, avec la connexion et l'authentification gérées par le serveur.

    **Skills** donnent à Claude des connaissances sur la façon d'utiliser efficacement ces outils, plus des flux de travail que vous pouvez déclencher avec `/<name>`. Une skill pourrait inclure le schéma de votre base de données et les modèles de requête, ou un flux de travail `/post-to-slack` avec les règles de formatage des messages de votre équipe.
  </Tab>

  <Tab title="Hook vs Skill">
    Claude Code exécute un hook à un événement du cycle de vie ; il charge une skill dans le contexte pour que Claude l'applique.

    | Aspect               | Hook                                                                                              | Skill                                                                                       |
    | -------------------- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
    | **S'exécute**        | Une commande shell, une requête HTTP, un appel d'outil MCP, une invite LLM ou un subagent         | Instructions que Claude lit et suit                                                         |
    | **Déclenché par**    | [Événements du cycle de vie](/docs/fr/hooks#hook-events) tels que `PostToolUse` ou `SessionStart`      | Vous tapant `/<name>`, ou Claude correspondant à la description de votre tâche              |
    | **Déterminisme**     | Se déclenche toujours sur son événement ; le déclencheur est garanti                              | Claude interprète les instructions ; le résultat peut varier                                |
    | **Coût du contexte** | Zéro sauf si le hook retourne une sortie                                                          | La description se charge à chaque session ; le contenu complet se charge quand utilisé      |
    | **Meilleur pour**    | Linting après les modifications, blocage des commandes dangereuses, journalisation, notifications | Flux de travail qui nécessitent du raisonnement, matériel de référence, tâches multi-étapes |

    **Utilisez un hook** quand l'action doit se produire de la même manière à chaque fois et n'a pas besoin que Claude réfléchisse. Par exemple : formater à la sauvegarde, rejeter `rm -rf /`, publier un message Slack quand une session se termine.

    **Utilisez une skill** quand Claude devrait décider comment appliquer les étapes, ou quand le contenu est une connaissance plutôt qu'un script. Par exemple : une liste de contrôle `/release`, votre guide de style API, un playbook de débogage.

    **Mettez les garde-fous dans les hooks.** Une instruction comme « ne jamais modifier `.env` » dans CLAUDE.md ou une skill est une demande, pas une garantie. Un hook `PreToolUse` qui bloque la modification est une application. Si une règle doit tenir à chaque fois, faites-en un hook plutôt qu'une instruction d'invite.

    **La sortie du hook atterrit dans le contexte.** Un hook `PostToolUse` qui exécute votre linter alimente les résultats en tant que texte que Claude lit ; une skill `/fix-lint` dit à Claude comment les résoudre.
  </Tab>
</Tabs>

<h3 id="understand-how-features-layer">
  Comprendre comment les fonctionnalités se superposent
</h3>

Les fonctionnalités peuvent être définies à plusieurs niveaux : à l'échelle de l'utilisateur, par projet, via des plugins ou via des politiques gérées. Vous pouvez également imbriquer des fichiers CLAUDE.md dans des sous-répertoires ou placer des skills dans des packages spécifiques d'un monorepo. Quand la même fonctionnalité existe à plusieurs niveaux, voici comment elles se superposent :

* **Les fichiers CLAUDE.md** sont additifs : tous les niveaux contribuent du contenu au contexte de Claude simultanément. Les fichiers de votre répertoire de travail et au-dessus se chargent au lancement ; les sous-répertoires se chargent au fur et à mesure que vous y travaillez. Quand les instructions entrent en conflit, Claude utilise son jugement pour les réconcilier. Consultez [comment les fichiers CLAUDE.md se chargent](/docs/fr/memory#how-claude-md-files-load).
* **Les skills et subagents** se remplacent par nom : quand le même nom existe à plusieurs niveaux, une définition gagne en fonction de la priorité (géré > utilisateur > projet pour les skills ; géré > drapeau CLI > projet > utilisateur > plugin pour les subagents). Les skills de plugin sont [espacées de noms](/docs/fr/plugins/components#skills) pour éviter les conflits. Consultez [découverte de skills](/docs/fr/skills#resolve-skills-that-share-a-name) et [portée du subagent](/docs/fr/sub-agents#choose-the-subagent-scope).
* **Les serveurs MCP** se remplacent par nom : local > projet > utilisateur. Consultez [portée MCP](/docs/fr/mcp#scope-hierarchy-and-precedence).
* **Les hooks** fusionnent : tous les hooks enregistrés se déclenchent pour leurs événements correspondants indépendamment de la source. Consultez [hooks](/docs/fr/hooks-guide).

<h3 id="combine-features">
  Combiner les fonctionnalités
</h3>

Chaque extension résout un problème différent : CLAUDE.md gère le contexte toujours actif, les skills gèrent les connaissances et les flux de travail à la demande, MCP gère les connexions externes, les subagents gèrent l'isolation et les hooks gèrent l'automatisation. Les configurations réelles les combinent en fonction de votre flux de travail.

Par exemple, vous pourriez utiliser CLAUDE.md pour les conventions de projet, une skill pour votre flux de travail de déploiement, MCP pour vous connecter à votre base de données et un hook pour exécuter le linting après chaque modification. Chaque fonctionnalité gère ce pour quoi elle est la meilleure.

| Modèle                 | Comment ça fonctionne                                                                                                | Exemple                                                                                                              |
| ---------------------- | -------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| **Skill + MCP**        | MCP fournit la connexion ; une skill enseigne à Claude comment l'utiliser bien                                       | MCP se connecte à votre base de données, une skill documente votre schéma et les modèles de requête                  |
| **Skill + Subagent**   | Une skill génère des subagents pour le travail parallèle                                                             | La skill `/audit` lance des subagents de sécurité, de performance et de style qui travaillent dans un contexte isolé |
| **CLAUDE.md + Skills** | CLAUDE.md contient les règles toujours actives ; les skills contiennent le matériel de référence chargé à la demande | CLAUDE.md dit « suivez nos conventions API », une skill contient le guide de style API complet                       |
| **Hook + MCP**         | Un hook déclenche des actions externes via MCP                                                                       | Le hook post-édition envoie une notification Slack quand Claude modifie des fichiers critiques                       |

<h2 id="understand-context-costs">
  Comprendre les coûts du contexte
</h2>

Chaque fonctionnalité que vous ajoutez consomme une partie du contexte de Claude. Trop peut remplir votre fenêtre de contexte, mais cela peut aussi ajouter du bruit qui rend Claude moins efficace ; les skills peuvent ne pas se déclencher correctement, ou Claude peut perdre de vue vos conventions. Comprendre ces compromis vous aide à construire une configuration efficace. Pour une vue interactive de la façon dont ces fonctionnalités se combinent dans une session en cours d'exécution, consultez [Explorez la fenêtre de contexte](/docs/fr/context-window).

<h3 id="context-cost-by-feature">
  Coût du contexte par fonctionnalité
</h3>

Chaque fonctionnalité a une stratégie de chargement et un coût de contexte différents :

| Fonctionnalité           | Quand elle se charge                                        | Ce qui se charge                                                                                                                  | Coût du contexte                                          |
| ------------------------ | ----------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| **CLAUDE.md**            | Début de session                                            | Contenu complet                                                                                                                   | À chaque requête                                          |
| **Styles de sortie**     | Début de session, et à nouveau quand vous changez de styles | Les instructions complètes du style actif ; rien pour le style Par défaut                                                         | À chaque requête                                          |
| **Skills**               | Début de session + quand utilisé                            | Descriptions au démarrage, contenu complet quand utilisé                                                                          | Faible (descriptions à chaque requête)\*                  |
| **Serveurs MCP**         | Début de session                                            | Noms d'outils ; schémas complets à la demande                                                                                     | Faible jusqu'à ce qu'un outil soit utilisé                |
| **Intelligence du code** | Après les modifications de fichiers et à la demande         | Diagnostics après les modifications ; emplacements des symboles à la recherche                                                    | Faible ; réduit les lectures de fichiers ailleurs         |
| **Subagents**            | Quand généré                                                | Contexte frais avec les skills spécifiées, ou la conversation parent pour un [fork](/docs/fr/sub-agents#fork-the-current-conversation) | Isolé de la session principale                            |
| **Hooks**                | Au déclenchement                                            | Rien (s'exécute en externe)                                                                                                       | Zéro, sauf si le hook retourne du contexte supplémentaire |

\*Par défaut, les descriptions de skills se chargent au début de la session afin que Claude puisse décider quand les utiliser. Définissez `disable-model-invocation: true` dans le frontmatter d'une skill pour la masquer complètement à Claude jusqu'à ce que vous l'invoquiez manuellement. Pour une skill que vous n'avez pas écrite, définissez [`skillOverrides`](/docs/fr/skills#override-skill-visibility-from-settings) dans les paramètres pour faire la même chose sans modifier son fichier.

<h3 id="understand-how-features-load">
  Comprendre comment les fonctionnalités se chargent
</h3>

Chaque fonctionnalité se charge à différents points de votre session. Les onglets ci-dessous expliquent quand chacune se charge et ce qui entre dans le contexte.

<img src="https://mintcdn.com/claude-code/ikqp3_70mqIahteV/images/context-loading.svg?fit=max&auto=format&n=ikqp3_70mqIahteV&q=85&s=aab139e750494a237ae2e0c8f9139b0a" className="dark:hidden" alt="Chargement du contexte : CLAUDE.md se charge au début de la session et reste dans chaque requête. Les noms d'outils MCP se chargent au démarrage avec les schémas complets reportés jusqu'à utilisation. Les skills chargent les descriptions au démarrage, le contenu complet à l'invocation. Les subagents obtiennent un contexte isolé. Les hooks s'exécutent en externe." width="720" height="382" data-path="images/context-loading.svg" />

<img src="https://mintcdn.com/claude-code/_xqph1dUOslCOwsj/images/context-loading-dark.svg?fit=max&auto=format&n=_xqph1dUOslCOwsj&q=85&s=b274089ef9612d9c760bca9838557626" className="hidden dark:block" alt="Chargement du contexte : CLAUDE.md se charge au début de la session et reste dans chaque requête. Les noms d'outils MCP se chargent au démarrage avec les schémas complets reportés jusqu'à utilisation. Les skills chargent les descriptions au démarrage, le contenu complet à l'invocation. Les subagents obtiennent un contexte isolé. Les hooks s'exécutent en externe." width="720" height="382" data-path="images/context-loading-dark.svg" />

<Tabs>
  <Tab title="CLAUDE.md">
    **Quand :** Début de session

    **Ce qui se charge :** Contenu complet de tous les fichiers CLAUDE.md (niveaux géré, utilisateur et projet).

    **Héritage :** Claude lit les fichiers CLAUDE.md de votre répertoire de travail jusqu'à la racine et découvre les fichiers imbriqués dans les sous-répertoires au fur et à mesure qu'il accède à ces fichiers. Consultez [Comment les fichiers CLAUDE.md se chargent](/docs/fr/memory#how-claude-md-files-load) pour plus de détails.

    <Tip>Gardez CLAUDE.md sous 200 lignes. Déplacez le matériel de référence vers les skills, qui se chargent à la demande. Pour obtenir des [propositions de réduction pour un CLAUDE.md enregistré](/docs/fr/memory#my-claude-md-is-too-large), exécutez `/doctor`.</Tip>
  </Tab>

  <Tab title="Skills">
    Les skills sont des capacités supplémentaires dans la boîte à outils de Claude. Elles peuvent être du matériel de référence (comme un guide de style API) ou des flux de travail invocables que vous déclenchez avec `/<name>` (comme `/deploy`). Claude Code inclut des [skills groupées](/docs/fr/commands) comme `/code-review`, `/batch` et `/debug` qui fonctionnent directement. Vous pouvez également créer les vôtres.

    **Quand :** Dépend de la configuration de la skill. Par défaut, les descriptions se chargent au début de la session et le contenu complet se charge quand utilisé. Pour les skills utilisateur uniquement (`disable-model-invocation: true`), rien ne se charge jusqu'à ce que vous les invoquiez.

    **Ce qui se charge :** Pour les skills invocables par modèle, Claude voit les noms et descriptions dans chaque requête. Quand vous invoquez une skill avec `/<name>` ou que Claude la charge automatiquement, le contenu complet se charge dans votre conversation.

    **Comment Claude choisit les skills :** Claude associe votre tâche aux descriptions de skills pour décider lesquelles sont pertinentes. Si les descriptions sont vagues ou se chevauchent, Claude peut charger la mauvaise skill ou en manquer une qui aiderait. Pour dire à Claude d'utiliser une skill spécifique, invoquez-la avec `/<name>`. Les skills avec `disable-model-invocation: true` sont invisibles à Claude jusqu'à ce que vous les invoquiez.

    **Coût du contexte :** Faible jusqu'à utilisation. Les skills utilisateur uniquement ont un coût zéro jusqu'à invocation.

    **Dans les subagents :** Les skills fonctionnent différemment dans les subagents. Au lieu du chargement à la demande, les skills listées dans le champ `skills` du subagent sont entièrement préchargées dans son contexte au lancement. Les subagents peuvent toujours découvrir et invoquer les skills de projet, utilisateur et plugin non listées via l'outil Skill.

    <Tip>Utilisez `disable-model-invocation: true` pour les skills avec des effets secondaires. Cela économise du contexte et garantit que seul vous les déclenchez.</Tip>
  </Tab>

  <Tab title="Serveurs MCP">
    **Quand :** Début de session.

    **Ce qui se charge :** Noms d'outils et instructions du serveur à partir des serveurs connectés. Les schémas JSON complets restent reportés jusqu'à ce que Claude ait besoin d'un outil spécifique.

    **Coût du contexte :** [Recherche d'outils](/docs/fr/mcp#scale-with-mcp-tool-search) est activée par défaut, donc les outils MCP inactifs consomment un contexte minimal.

    <Tip>Exécutez `/mcp` pour voir l'état de la connexion de chaque serveur. Exécutez `/context all` pour voir combien de tokens chaque outil MCP chargé utilise. Claude Code [se reconnecte automatiquement aux serveurs distants](/docs/fr/mcp#automatic-reconnection) s'ils se déconnectent, et vous pouvez déconnecter les serveurs que vous n'utilisez pas activement.</Tip>
  </Tab>

  <Tab title="Intelligence du code">
    **Quand :** Après les modifications de fichiers, et à la demande quand Claude navigue dans le code.

    **Ce qui se charge :** Erreurs de type et avertissements après chaque modification de fichier. Informations de définition, de référence et de type quand Claude recherche un symbole.

    **Coût du contexte :** Faible. Les recherches de symboles remplacent souvent les lectures de fichiers larges, donc l'utilisation nette du contexte peut diminuer.

    <Tip>L'outil LSP est inactif jusqu'à ce que vous installiez un [plugin d'intelligence du code](/docs/fr/plugins/code-intelligence) pour votre langage.</Tip>
  </Tab>

  <Tab title="Subagents">
    **Quand :** À la demande, quand vous ou Claude en générez un pour une tâche.

    **Ce qui se charge :** Contexte frais et isolé contenant :

    * L'invite système de l'agent, pas l'invite système de Claude Code
    * Contenu complet des skills listées dans le champ `skills:` de l'agent
    * CLAUDE.md et statut git, sauf les agents Explore et Plan intégrés [qui omettent les deux](/docs/fr/sub-agents#what-loads-at-startup), et un agent dont la définition définit [`omitClaudeMd`](/docs/fr/sub-agents#supported-frontmatter-fields) ignore les fichiers CLAUDE.md utilisateur, projet et local
    * Quel que soit le contexte que l'agent principal transmet dans l'invite

    Pour un [fork](/docs/fr/sub-agents#fork-the-current-conversation), Claude Code charge la conversation parent jusqu'à présent, l'invite système et les outils à la place.

    **Coût du contexte :** Isolé de la session principale.

    <Tip>Utilisez les subagents pour le travail qui n'a pas besoin de votre contexte de conversation complet. Leur isolation empêche de gonfler votre session principale.</Tip>
  </Tab>

  <Tab title="Hooks">
    **Quand :** Au déclenchement. Claude Code exécute les hooks à des événements de cycle de vie spécifiques comme l'exécution d'outils, les limites de session, la soumission d'invite, les demandes de permission et la compaction. Consultez [Hooks](/docs/fr/hooks) pour la liste complète.

    **Ce qui se charge :** Rien par défaut. Les hooks s'exécutent en dehors de la conversation principale.

    **Coût du contexte :** Zéro, sauf si le hook retourne une sortie qui est ajoutée en tant que messages à votre conversation.

    <Tip>Les hooks sont idéaux pour les effets secondaires (linting, journalisation) qui n'ont pas besoin d'affecter le contexte de Claude.</Tip>
  </Tab>
</Tabs>

<h2 id="learn-more">
  En savoir plus
</h2>

Chaque fonctionnalité a son propre guide avec des instructions de configuration, des exemples et des options de configuration.

<CardGroup cols={2}>
  <Card title="CLAUDE.md" icon="file-lines" href="/docs/fr/memory">
    Stockez le contexte du projet, les conventions et les instructions
  </Card>

  <Card title="Skills" icon="brain" href="/docs/fr/skills">
    Donnez à Claude une expertise de domaine et des flux de travail réutilisables
  </Card>

  <Card title="Subagents" icon="users" href="/docs/fr/sub-agents">
    Déléguez le travail à un contexte isolé
  </Card>

  <Card title="Dynamic workflows" icon="network" href="/docs/fr/workflows">
    Exécutez de nombreux subagents à partir d'un seul script
  </Card>

  <Card title="Cross-session messaging" icon="terminal" href="/docs/fr/cross-session-messaging">
    Laissez Claude envoyer des messages à vos autres sessions
  </Card>

  <Card title="MCP" icon="plug" href="/docs/fr/mcp">
    Connectez Claude à des services externes
  </Card>

  <Card title="Hooks" icon="bolt" href="/docs/fr/hooks-guide">
    Automatisez les actions avec des hooks
  </Card>

  <Card title="Plugins" icon="puzzle-piece" href="/docs/fr/plugins/overview">
    Empaquetez et partagez des ensembles de fonctionnalités
  </Card>

  <Card title="Marketplaces" icon="store" href="/docs/fr/plugins/create-marketplace">
    Hébergez et distribuez des collections de plugins
  </Card>
</CardGroup>
