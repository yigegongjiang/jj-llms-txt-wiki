> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Kit de communication

> Annonces de lancement, messages de campagne progressive et réponses FAQ pour déployer Claude Code dans votre organisation d'ingénierie.

Cette page est destinée aux administrateurs et aux responsables d'ingénierie qui déploient Claude Code auprès d'une équipe. Elle fournit des annonces prêtes à être envoyées, une campagne progressive de conseils et astuces, et des réponses FAQ d'une ligne pour les questions les plus fréquemment posées.

<Note>
  Traitez tout ce qui se trouve ici comme un brouillon, pas comme une copie définitive. Réécrivez chaque message dans la voix de votre organisation, remplacez les tâches d'exemple par de vrais bugs et modules de votre propre base de code, et remplacez les `[espaces réservés entre crochets]` avant d'envoyer. Les annonces qui stimulent l'adoption sont celles qui semblent avoir été écrites par quelqu'un de votre entreprise.
</Note>

<h2 id="launch-communications">
  Communications de lancement
</h2>

Une annonce en deux formats, plus deux variantes optionnelles. Choisissez celle qui correspond à votre déploiement et réécrivez-la à partir de là.

<h3 id="before-you-send">
  Avant d'envoyer
</h3>

Parcourez cette liste de contrôle avant que l'annonce ne soit envoyée. Chaque élément comble une lacune qui se transformerait autrement en fil de support le jour du lancement.

| Élément                                                                                                             | Pourquoi c'est important                                                                                                                                                    |
| ------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Canal `#claude-code` créé et lié dans le message                                                                    | Donne aux questions un seul endroit où atterrir                                                                                                                             |
| Commande d'installation testée sur au moins une machine de votre environnement                                      | Détecte les problèmes de proxy ou de pare-feu avant que tout le monde les rencontre en même temps                                                                           |
| Lien de sécurité et de gestion des données prêt ([Utilisation des données](/docs/fr/data-usage) ou l'équivalent interne) | « Où va mon code ? » sera la première réponse                                                                                                                               |
| Une tâche concrète choisie, un vrai bug ou fichier de votre base de code                                            | Les exemples génériques ne convertissent pas ; « corriger le test instable dans `auth_test.go` » le fait                                                                    |
| Un propriétaire nommé pour le canal pendant les 48 premières heures                                                 | Les questions sans réponse le jour du lancement tuent l'élan                                                                                                                |
| Un sponsor de la direction générale prêt à envoyer ou à cosigner l'annonce                                          | Les lancements envoyés par un cadre voient systématiquement un taux d'adoption plus élevé la première semaine que ceux envoyés par un administrateur ou une équipe d'outils |

<h3 id="the-announcement">
  L'annonce
</h3>

Utilisez ceci comme votre message de déploiement standard à l'échelle de l'organisation. Il explique ce qu'est Claude Code, fournit un chemin d'installation de deux minutes, donne aux lecteurs une tâche concrète à essayer, et répond à « où va mon code ? » avant que quiconque ait besoin de poser la question.

<Tabs>
  <Tab title="Email">
    ```text theme={null}
    Objet : Claude Code est en direct pour [Ingénierie / votre équipe]

    Équipe,

    À partir d'aujourd'hui, vous avez accès à Claude Code, un agent de codage IA qui s'exécute dans
    votre terminal, lit votre base de code réelle et travaille sur des tâches réelles du début à la fin :
    débogage, refactorisations, tests, PR. Ce n'est pas de l'autocomplétion et ce n'est pas une fenêtre de chat.
    Il édite des fichiers, exécute vos commandes et demande la permission avant tout ce qui est risqué.

    Commencez en deux minutes :

        curl -fsSL https://claude.ai/install.sh | bash
        cd <your-repo>
        claude

    Puis exécutez /init une fois. Claude lit votre projet et écrit un CLAUDE.md avec
    vos commandes de construction et conventions, afin que vous arrêtiez de réexpliquer les bases.

    Ensuite, essayez l'une de celles-ci sur le référentiel dans lequel vous êtes déjà :

      - ' Le test dans [fichier] est instable. Découvrez pourquoi et corrigez-le '
      - ' Expliquez-moi comment [module] gère [X] '
      - ' Regardez mon diff de travail et dites-moi ce qui est risqué avant que je pousse '

    Où va votre code : Claude Code s'exécute dans votre terminal et communique directement
    avec l'API d'Anthropic, sans serveurs tiers dans la boucle. Il demande avant
    d'éditer des fichiers ou d'exécuter des commandes. En vertu de notre accord Entreprise, Anthropic
    n'utilise pas votre code ou vos invites pour entraîner ses modèles.
    Détails : https://code.claude.com/docs/fr/data-usage
             https://code.claude.com/docs/fr/security

    Où aller avec des questions : #claude-code. [Nom du propriétaire] le surveille
    cette semaine.

    - [Nom]

    P.S. Vous préférez votre éditeur ? Il y a une extension VS Code et un plugin
    JetBrains. Le même agent, pas de terminal requis.
    ```
  </Tab>

  <Tab title="Slack ou Teams">
    ```markdown theme={null}
    🚀 *Claude Code est en direct pour [équipe]*

    Agent de codage IA, s'exécute dans votre terminal, lit votre référentiel, fait du vrai travail :
    bugs, refactorisations, tests, PR. Demande avant de toucher à quoi que ce soit.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd your-repo` → `claude`

    *Première chose à essayer* → exécutez `/init`, puis : « le test dans [fichier] est instable,
    découvrez pourquoi et corrigez-le. »

    🔒 S'exécute dans votre terminal, communique uniquement avec l'API d'Anthropic. En vertu de notre
    plan Entreprise, votre code et vos invites ne sont pas utilisés pour entraîner les modèles.
    Utilisation des données → https://code.claude.com/docs/fr/data-usage

    📚 Démarrage rapide · VS Code · Cours gratuit d'1 heure
       https://code.claude.com/docs/fr/quickstart
       https://code.claude.com/docs/fr/vs-code
       https://anthropic.skilljar.com/claude-code-in-action

    Questions → ce fil. [Propriétaire] est en première ligne.
    ```
  </Tab>
</Tabs>

<h3 id="executive-sponsor-variant">
  Variante de sponsor exécutif
</h3>

Envoyez ceci de la part de votre cadre sponsor, comme le CTO, le CIO ou le SVP Ingénierie, sous son nom et depuis son compte. Les lancements envoyés sous le nom d'un cadre voient systématiquement des taux d'ouverture plus élevés et une activation plus rapide la première semaine que le même message d'un administrateur ou d'une équipe d'outils. Cela signale une priorité de l'entreprise plutôt qu'une expérience optionnelle.

Cette version est délibérément réduite à une seule demande : installez-la et exécutez-la sur une tâche réelle. Le travail du cadre est de faire atterrir la demande ; l'annonce standard et `#claude-code` gèrent le comment.

<Tabs>
  <Tab title="Email">
    ```text theme={null}
    Objet : Une chose que j'aimerais que chaque ingénieur essaie cette semaine

    Équipe,

    Nous avons activé Claude Code pour toute l'ingénierie. C'est un agent IA
    qui fonctionne directement dans votre terminal, sur votre base de code réelle, et les
    résultats précoces des équipes qui l'utilisent déjà sont assez solides pour que je veuille
    que tout le monde l'utilise cette semaine.

    Je demande dix minutes :

        curl -fsSL https://claude.ai/install.sh | bash
        cd <your-repo>
        claude

    Ensuite, donnez-lui une tâche réelle : le bug que vous avez remis à plus tard, ou ' expliquez-moi
    comment [module] fonctionne. '

    C'est toute la demande. [Nom du propriétaire] et l'équipe sont dans #claude-code pour
    tout ce que vous rencontrez en chemin.

    - [Nom du cadre]
      [Titre]
    ```
  </Tab>

  <Tab title="Slack ou Teams">
    ```markdown theme={null}
    📣 *De [Nom du cadre] : une chose à essayer cette semaine*

    Nous avons activé *Claude Code* pour toute l'ingénierie. Les résultats précoces sont
    assez solides pour que je demande à tout le monde de lui donner dix minutes sur du vrai
    travail cette semaine.

    `curl -fsSL https://claude.ai/install.sh | bash` → `cd your-repo` →
    `claude` → donnez-lui une tâche réelle.

    C'est tout. Questions → #claude-code.
    ```
  </Tab>
</Tabs>

<h3 id="pilot-group-variant">
  Variante du groupe pilote
</h3>

À utiliser pour un déploiement par phases. Envoyez uniquement à la cohorte pilote.

```text theme={null}
Objet : Vous êtes dans le pilote Claude Code

[Nom / équipe],

Vous êtes dans la première vague de Claude Code chez [entreprise]. Nous avons choisi ce groupe
parce que vous le mettrez sur des problèmes réels et nous direz la vérité à ce sujet.

La demande : utilisez-le sur au moins une tâche réelle cette semaine, puis laissez une note dans
#claude-code-pilot couvrant ce qui a fonctionné, ce qui était ennuyeux et ce qui
vous a surpris. Ce retour d'information décide comment nous le déploierons à tout le monde d'autre.

[Continuez avec « Commencez en deux minutes » de l'annonce standard]

Une chose supplémentaire pour les pilotes : lors de votre premier changement multi-fichier, appuyez sur Maj+Tab
jusqu'à ce que vous voyiez « plan ». Claude exposera exactement ce qu'il a l'intention de faire
avant de toucher à un fichier. C'est le moyen le plus rapide d'étalonner combien le faire confiance.
```

<h3 id="champion-recruitment-dm">
  DM de recrutement de champion
</h3>

Après le lancement, envoyez un DM aux deux ou trois personnes les plus actives dans `#claude-code`.

```text theme={null}
Salut [nom], tes messages #claude-code font plus pour l'adoption que mon
annonce. Quelques personnes m'ont dit que ton [fil / capture d'écran]
était la raison pour laquelle elles l'ont vraiment essayé.

Tu veux le rendre semi-officiel ? Peu d'effort : continue surtout à poster ce que
tu postes, plus un premier accès aux nouvelles fonctionnalités et une ligne directe vers l'équipe
Anthropic. Je peux partager un petit guide si tu es partant.
```

<h2 id="tips-and-tricks-campaign">
  Campagne de conseils et astuces
</h2>

Messages Slack ou Teams prêts à coller conçus pour stimuler l'activation des fonctionnalités après le lancement. Chacun suit le même modèle : un crochet, le gain, une invite « essayez maintenant » et un lien de documentation. Versez-les un ou deux par semaine dans `#claude-code`, ou choisissez la poignée qui correspond aux lacunes de votre équipe. Ils se suffisent à eux-mêmes sans ordre requis.

Copiez le corps du message de chaque bloc directement dans Slack ou Teams. Remplacez les `[espaces réservés entre crochets]` avant d'envoyer.

<h3 id="get-started">
  Commencer
</h3>

**Choisir le bon modèle**

```markdown theme={null}
🎯 *Conseil : Adaptez le modèle au moment*

Utiliser Opus pour corriger une faute de frappe gaspille du calcul. Utiliser Haiku pour une
refactorisation de 12 fichiers, c'est demander une refonte.

Claude Code s'exécute sur les mêmes modèles que l'application Claude, et vous pouvez basculer
en milieu de session. *Sonnet* est le workhorse par défaut pour le travail de fonctionnalité quotidienne,
les bugs, les tests et les révisions. Utilisez *Opus* pour les grandes refactorisations, le débogage complexe,
ou tout ce qui est à enjeux élevés. Passez à *Haiku* pour les questions rapides,
le formatage et les modifications mécaniques où la vitesse gagne. *Fable 5* est le modèle le plus
capable pour vos tâches les plus difficiles et les plus longues ; ce n'est pas le
défaut, donc sélectionnez-le avec `/model fable`, et notez que le contenu de cybersécurité et
de biologie revient automatiquement à Opus.

*Essayez maintenant :* tapez `/model` et choisissez Sonnet si vous ne l'avez pas déjà fait. C'est
le bon défaut pour la plupart des tâches.

📖 Configuration du modèle → https://code.claude.com/docs/fr/model-config
```

| Modèle  | Meilleur pour                                                                                                                                                                                                       |
| ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Fable 5 | Les tâches les plus difficiles et les plus longues. Opt-in uniquement : sélectionnez-le avec `/model fable`. Le contenu de cybersécurité ou de biologie [revient à Opus](/docs/fr/model-config#automatic-model-fallback) |
| Opus    | Refactorisations à grande échelle, débogage complexe, décisions architecturales, changements à enjeux élevés                                                                                                        |
| Sonnet  | Travail de fonctionnalité quotidienne, corrections de bugs, tests, documentation, révision de code. Défaut recommandé.                                                                                              |
| Haiku   | Questions rapides, formatage, modifications mécaniques, itération rapide                                                                                                                                            |

**Victoires rapides à essayer en premier**

```markdown theme={null}
🚀 *Conseil : Trois choses à essayer dans vos 10 premières minutes*

Vous avez installé Claude Code mais vous ne savez pas vraiment quoi lui demander ? Commencez par
les choses qui vous ennuient toute la semaine.

  - Corriger quelque chose d'ennuyeux : « le test dans [fichier] est instable, découvrez pourquoi »
  - S'orienter dans le code que vous n'avez pas écrit : « expliquez-moi comment [module] fonctionne »
  - Vérifier avant de pousser : « regardez mon diff de travail et dites-moi ce qui
    semble risqué »

Aucune de ces choses n'a besoin de configuration. Juste `cd` dans votre référentiel et exécutez `claude`.

*Essayez maintenant :* choisissez le bug que vous avez évité et collez le message d'erreur.

📖 Démarrage rapide → https://code.claude.com/docs/fr/quickstart
```

<h3 id="project-memory">
  Mémoire du projet
</h3>

**`/init` et CLAUDE.md**

```markdown theme={null}
📁 *Conseil : Arrêtez de réexpliquer votre référentiel à chaque session*

Vous dites à Claude « nous utilisons pnpm, pas npm » pour la cinquième fois ? Il y a
une correction unique.

Exécutez `/init` une fois par référentiel. Claude lit la structure de votre projet et écrit un
fichier CLAUDE.md avec vos commandes de construction, architecture et conventions.
Chaque session future dans ce référentiel commence automatiquement à partir de ce fichier. Gardez-le
sous deux écrans. C'est une feuille de triche, pas de la documentation.

*Essayez maintenant :* ouvrez votre référentiel principal, exécutez `claude`, tapez `/init`. Trente
secondes, rentabilisé à chaque session après.

📖 CLAUDE.md et mémoire du projet → https://code.claude.com/docs/fr/memory
```

**Références @**

```markdown theme={null}
📎 *Conseil : Arrêtez de coller le contenu des fichiers dans le chat*

Vous copiez 200 lignes d'un composant dans votre invite pour que Claude puisse le « voir » ?
Vous n'êtes pas obligé.

Tapez `@` puis un chemin de fichier. Claude tire le fichier directement dans le contexte.
Fonctionne aussi pour les répertoires entiers.

> les styles dans @src/components/Button.tsx semblent décalés, vérifiez par rapport à
> @docs/design-system.md

*Essayez maintenant :* tapez `@` puis Tab. L'autocomplétion vous montre chaque fichier à portée.

📖 Référencement des fichiers → https://code.claude.com/docs/fr/common-workflows
```

<h3 id="control-and-safety">
  Contrôle et sécurité
</h3>

**Modes de permission**

```markdown theme={null}
🛡️ *Conseil : Une frappe entre « regarder mais ne pas toucher » et « fais-le juste »*

Parfois, vous voulez que Claude demande avant chaque modification. Parfois, vous voulez juste
qu'il expédie. Vous ne devriez pas avoir à choisir une fois pour toutes.

*Maj+Tab* parcourt la longueur de la laisse que Claude obtient : *Manual* (la
valeur de paramètre `default`) demande avant chaque action, *acceptEdits* laisse les modifications de fichiers et les commandes de système de fichiers courantes
passer tout en vérifiant avant les autres commandes shell, et *plan*
propose des modifications pour votre approbation avant que quoi que ce soit ne soit touché. Le mode plan est
le constructeur de confiance, alors commencez par là pour tout ce qui touche plusieurs fichiers.

*Essayez maintenant :* sur votre prochaine refactorisation, appuyez sur Maj+Tab jusqu'à ce que vous voyiez « plan »,
puis décrivez le changement. Vous obtiendrez une proposition complète avant qu'un seul fichier ne bouge.

📖 Modes de permission → https://code.claude.com/docs/fr/permissions
```

**Checkpointing et `/rewind`**

```markdown theme={null}
⏪ *Conseil : Il y a un bouton d'annulation pour toute la conversation*

Claude a emprunté le mauvais chemin il y a trois tours et maintenant vous le démêlez ?
Vous n'avez pas à corriger vers l'avant.

`/rewind` revient à un point antérieur de la conversation, y compris les
modifications de fichiers que Claude a apportées en chemin. Le checkpointing est automatique ; vous
ne configurez rien.

*Essayez maintenant :* appuyez sur *Échap* deux fois pour ouvrir le menu de rembobinage, ou tapez `/rewind`.
Choisissez le point avant que les choses ne deviennent de travers.

📖 Checkpointing → https://code.claude.com/docs/fr/checkpointing
```

<h3 id="connect-your-tools">
  Connectez vos outils
</h3>

**Connecteurs MCP**

```markdown theme={null}
🔌 *Conseil : Laissez Claude lire votre suivi de problèmes pour que vous n'ayez pas à coller les tickets*

Coller des tickets Jira dans le terminal semble être un pas en arrière.
C'est le cas.

Un fichier de configuration (`.mcp.json` à la racine de votre projet) connecte Claude à GitHub,
Jira, Linear, ou quel que soit le suivi que vous utilisez. Ensuite, « quel est le problème
de priorité supérieure qui m'est assigné ? » et « vas-y et corrige-le » se produisent dans la même
conversation.

*Essayez maintenant :* demandez à Claude « configure un connecteur MCP pour [GitHub/Jira/Linear]
dans ce référentiel ». Il écrira la configuration pour vous.

📖 Connecteurs MCP → https://code.claude.com/docs/fr/mcp
```

<h3 id="automate-your-workflows">
  Automatisez vos flux de travail
</h3>

**Skills**

```markdown theme={null}
⚡ *Conseil : Transformez cette invite que vous continuez à retaper en commande*

Vous avez tapé « résumez ce sur quoi j'ai travaillé aujourd'hui à partir du git log, formatez-le pour le standup »
trois fois cette semaine ? C'est une commande slash qui attend de se produire.

Un fichier SKILL.md dans `.claude/skills/<name>/` devient une invite réutilisable ; tapez
`/name` pour l'exécuter. Faites-en un la deuxième fois que vous tapez une invite multi-étapes
que vous avez tapée avant. Chemin le plus facile : demandez à Claude de le faire pour vous.

*Essayez maintenant :* tapez « fais-moi un skill /standup qui résume ce sur quoi j'ai travaillé
aujourd'hui à partir du git log », puis exécutez `/standup` demain matin.

📖 Skills → https://code.claude.com/docs/fr/skills
```

**Hooks**

```markdown theme={null}
🔔 *Conseil : Soyez averti quand votre refactorisation se termine*

Vous êtes assis à votre bureau en regardant Claude travailler sur une tâche longue ? Vous avez
de meilleures choses à faire pendant ces huit minutes.

Les hooks sont des commandes shell qui se déclenchent sur les événements Claude Code. Un hook Stop qui
envoie une notification de bureau signifie que vous pouvez lancer une longue refactorisation, vous éloigner,
et être averti au moment où c'est fait.

*Essayez maintenant :* demandez à Claude « ajoute un hook Stop qui envoie une notification de bureau
quand tu termines ». Il écrira le script et le connectera.

📖 Guide des hooks → https://code.claude.com/docs/fr/hooks-guide
```

<h3 id="day-to-day-development">
  Développement au jour le jour
</h3>

**Captures d'écran et images**

```markdown theme={null}
📸 *Conseil : Arrêtez de décrire la boîte de dialogue d'erreur. Montrez-la juste.*

Vous tapez « il y a une boîte rouge qui dit quelque chose à propos d'une référence nulle
et elle pointe vers la ligne 47 environ » ? Prenez une capture d'écran.

Faites glisser une capture d'écran directement dans le terminal et Claude la voit : boîtes de dialogue d'erreur,
maquettes d'interface utilisateur, photos de tableau blanc, exports Figma. *Ctrl+V* colle depuis
le presse-papiers (utilisez aussi Ctrl+V sur macOS, pas Cmd+V).

*Essayez maintenant :* la prochaine fois que quelque chose de visuel se casse, prenez une capture d'écran et collez-la
directement dans l'invite. Ensuite, tapez juste « qu'est-ce qui ne va pas ici ? »

📖 Travailler avec des images → https://code.claude.com/docs/fr/common-workflows
```

**Flux de travail Git**

```markdown theme={null}
🌿 *Conseil : Confiez toute la cérémonie git*

La correction a pris 5 minutes. Le message de commit, la branche et la description de PR
ont pris 15. Ce ratio est faux.

Claude gère le flux git complet : commits avec des messages conventionnels,
branches, PR avec des résumés appropriés. Une demande : « corrige le décalage d'un, committe
avec un message de commit conventionnel et ouvre une PR. » Vous révisez le travail de quelqu'un d'autre ?
Collez l'URL de la PR et demandez à Claude de vous expliquer le diff.

*Essayez maintenant :* après votre prochaine correction, au lieu de basculer vers votre client git,
tapez juste « committe ceci avec un bon message et ouvre une PR ».

📖 Création de pull requests → https://code.claude.com/docs/fr/common-workflows
```

<h3 id="share-and-scale">
  Partager et mettre à l'échelle
</h3>

**Plugins**

```markdown theme={null}
📦 *Conseil : Quelqu'un a probablement déjà construit ce skill*

Vous êtes sur le point de passer une heure à construire une commande `/deploy` ? Vérifiez si elle
existe déjà.

Les skills sont regroupés et partagés en tant que plugins. `/plugin` parcourt ce qui est
disponible et s'installe en une seule étape. Cinq minutes de navigation peuvent économiser une heure de construction.

*Essayez maintenant :* tapez `/plugin` et faites défiler. Vous trouverez au moins une
chose que vous ne saviez pas que vous vouliez.

📖 Plugins → https://code.claude.com/docs/fr/plugins
```

<h3 id="security-and-admin">
  Sécurité et administration
</h3>

**Architecture de sécurité**

```markdown theme={null}
🔐 *Conseil : La réponse à « est-ce sûr ? » pour la prochaine fois qu'on vous le demande*

Quelqu'un de votre équipe va demander « attends, où va mon code ? »
Voici la version courte que vous pouvez coller.

Permission-first par conception. Chaque modification de fichier, commande shell et appel externe
est contrôlé par votre approbation. Le CLI s'exécute dans votre terminal et communique
directement avec l'API d'Anthropic, sans serveurs tiers, et supporte le sandboxing optionnel au niveau du système d'exploitation
pour les commandes shell. En vertu de notre plan Entreprise,
Anthropic n'utilise pas votre code ou vos invites pour entraîner ses modèles.

*Essayez maintenant :* enregistrez ces deux liens pour la prochaine fois que la question se pose.
Ils répondent à la plupart des questions d'examen de sécurité.

📖 https://code.claude.com/docs/fr/security
📖 https://code.claude.com/docs/fr/data-usage
```

**Meilleures pratiques**

```markdown theme={null}
✅ *Conseil : Les 4 habitudes qui séparent « l'ai essayé une fois » de « l'utilise quotidiennement »*

La plupart des gens qui rebondissent sur Claude Code en ont sauté une. La plupart des gens
qui restent les ont tous faits la première semaine.

  - Commencez en mode plan pour tout ce qui touche plusieurs fichiers
  - Exécutez /init tôt ; le contexte se compose
  - Examinez les diffs avant de committer ; Claude peut se tromper avec confiance
  - Vérifiez les modifications qui touchent les chemins critiques ; traitez-le comme un junior
    pointu, pas un oracle

*Essayez maintenant :* si vous n'en avez fait qu'un ou deux, choisissez celui qui vous manque
et faites-le sur votre prochaine tâche. Postez ce qui a changé dans #claude-code.

📖 Meilleures pratiques → https://code.claude.com/docs/fr/best-practices
```

<h2 id="quick-reference">
  Référence rapide
</h2>

<h3 id="faq-responses">
  Réponses FAQ
</h3>

Réponses d'une ligne pour les questions les plus fréquemment posées.

| Question                                          | Réponse                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| « Fonctionne-t-il dans VS Code ? »                | Oui. Il y a une extension VS Code et un plugin JetBrains avec les mêmes fonctionnalités, intégrés dans votre éditeur. [VS Code →](/docs/fr/vs-code)                                                                                                                                                                                                                                                                                                                                                                           |
| « Dois-je configurer quelque chose en premier ? » | Non. Installez, puis exécutez `claude` dans n'importe quel référentiel. Exécutez `/init` une fois et vous êtes prêt. [Démarrage rapide →](/docs/fr/quickstart)                                                                                                                                                                                                                                                                                                                                                                |
| « Où va mon code ? »                              | Le CLI s'exécute dans votre terminal et envoie le contexte à l'API d'Anthropic pour l'inférence, sans serveurs tiers. En vertu de votre plan Entreprise, votre code et vos invites ne sont pas utilisés pour entraîner les modèles. [Utilisation des données →](/docs/fr/data-usage)                                                                                                                                                                                                                                          |
| « Peut-il voir tout mon référentiel ? »           | Il lit ce à quoi vous lui donnez accès. Les lectures de fichiers dans votre répertoire de travail ne demandent pas ; les invites de permission contrôlent les modifications, les commandes shell non-lecture seule et les lectures d'outils de fichiers en dehors de ce répertoire. Un ensemble intégré de commandes shell en lecture seule telles que `ls` et `cat` s'exécute sans demander ; limitez-le avec les [règles `denyRead` du sandbox](/docs/fr/sandboxing#filesystem-isolation). [Permissions →](/docs/fr/permissions) |
| « En quoi c'est différent de Copilot ? »          | Copilot complète les lignes. Claude Code est un agent qui lit les fichiers, exécute les commandes et effectue des modifications multi-fichiers. [Aperçu →](/docs/fr/overview)                                                                                                                                                                                                                                                                                                                                                 |
| « Que devrais-je essayer en premier ? »           | Un bug que vous avez remis à plus tard parce que c'est fastidieux. « Le test dans \[fichier] est instable, découvrez pourquoi. » [Démarrage rapide →](/docs/fr/quickstart)                                                                                                                                                                                                                                                                                                                                                    |

<h3 id="prompt-templates">
  Modèles d'invite
</h3>

Partagez ces invites de démarrage avec les ingénieurs qui ont installé mais ne savent pas quoi demander. Chacune est formulée comme elle serait tapée dans une session réelle ; remplacez les pièces entre crochets par des fichiers de votre propre référentiel.

| Tâche                       | Invite                                                                                                 |
| --------------------------- | ------------------------------------------------------------------------------------------------------ |
| Corriger un bug             | « les tests dans \[fichier] échouent, découvrez pourquoi et corrigez-le »                              |
| Comprendre le code          | « expliquez-moi comment \[module] fonctionne, puis dites-moi où se trouve le point d'entrée »          |
| Refactorisation sûre        | « refactorisez \[module] pour \[objectif], utilisez le mode plan pour que je puisse d'abord examiner » |
| Écrire des tests            | « écrivez des tests pour \[fichier] qui couvrent les cas limites autour de \[scénario] »               |
| Examiner avant de committer | « regardez mon diff de travail et dites-moi ce qui semble risqué »                                     |
| Ouvrir une PR               | « corrigez \[problème], écrivez un commit conventionnel et ouvrez une PR avec un résumé »              |
| Créer un skill              | « fais-moi un skill /ship qui exécute les tests et lint avant commit »                                 |
| Déboguer une trace de pile  | « voici la trace de pile, trouvez la cause racine, ne la recouvrez pas juste »                         |

<Tip>
  Claude Code est livré fréquemment. Vérifiez les détails spécifiques à la version par rapport à la [page d'accueil de la documentation](/docs/fr/overview) avant de distribuer en interne.
</Tip>
