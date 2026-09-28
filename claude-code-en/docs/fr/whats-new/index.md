> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Quoi de neuf

> Un digest hebdomadaire des fonctionnalités notables de Claude Code, avec des extraits de code, des démos et du contexte sur leur importance.

Le digest hebdomadaire pour développeurs met en évidence les fonctionnalités les plus susceptibles de changer votre façon de travailler. Chaque entrée inclut du code exécutable, une courte démo et un lien vers la documentation complète. Pour chaque correction de bug et amélioration mineure, consultez le [changelog](/docs/en/changelog).

<Update label="Week 37" description="7–11 septembre 2026" tags={["v2.1.263–v2.1.269"]}>
  **`claude plugin eval`** : exécutez votre plugin par rapport à une suite de cas de test, notez les résultats et comparez par rapport à une ligne de base sans plugin. `claude plugin eval init` rédige les cas et les évaluateurs pour vous.

  Également cette semaine : extrayez n'importe quel **volet Claude Code Desktop** dans sa propre fenêtre et redockez-le plus tard ; le paramètre **`maxEffortLevel`** limite le niveau d'effort sur chaque fournisseur ; et une page que **WebFetch** n'a pas terminé de télécharger dans les cinq minutes échoue au lieu de rester bloquée.

  [Lire le digest de la Week 37 →](/docs/fr/whats-new/2026-w37)
</Update>

<Update label="Week 36" description="31 août – 4 septembre 2026" tags={["v2.1.251–v2.1.261"]}>
  **Claude Fable 5.1** : disponible dans Claude Code avec une fenêtre de contexte de 1M de tokens.

  Également cette semaine : sur les plans Pro et Max, **l'utilisation d'ordinateur dans l'application Desktop** fonctionne en arrière-plan sur macOS pendant que vous continuez à travailler ; dans le rendu en plein écran, **`/diff`** ouvre un panneau en direct à côté de la conversation qui se rafraîchit au fur et à mesure que Claude effectue des modifications ; et **`/skill-doctor`** affiche ce que chacune de vos compétences coûte en contexte et à quelle fréquence elle est utilisée.

  [Lire le digest de la Week 36 →](/docs/fr/whats-new/2026-w36)
</Update>

<Update label="Week 35" description="24–28 août 2026" tags={["v2.1.240–v2.1.250"]}>
  **Reprendre les sessions de terminal dans l'application Desktop** : tapez `/resume` dans la boîte d'invite Claude Code Desktop pour reprendre n'importe quelle session que vous avez démarrée à partir de la CLI, avec la conversation complète et le contexte intacts.

  Également cette semaine : **les commentaires rédigés par Claude** font que Claude rédige un rapport de commentaires quand quelque chose se passe mal dans une session, que vous examinez et envoyez depuis `/feedback` ; **`--restricted`** démarre une session sans les outils d'exécution de commandes ou vos paramètres utilisateur et projet, pour les harnais d'évaluation sur les machines partagées ; et le paramètre **`modelPicker`** contrôle les modèles que le sélecteur `/model` répertorie.

  [Lire le digest de la Week 35 →](/docs/fr/whats-new/2026-w35)
</Update>

<Update label="Week 34" description="17–21 août 2026" tags={["v2.1.234–v2.1.239"]}>
  **`/design`** : un aperçu de recherche qui apporte le flux de travail de l'artboard de Claude Design dans la CLI et Claude Code Desktop, construit sur des artifacts, afin que Claude rédige des artboards modifiables pour votre interface utilisateur et implémente celui que vous choisissez.

  Également cette semaine : le **style de sortie Concise** intégré fait que Claude commence par le résultat et saute le préambule ; toute machine exécutant `claude remote-control` s'affiche sous la forme d'une **carte d'appareil** sur votre téléphone afin que vous puissiez démarrer une session sur celle-ci à partir de l'onglet Code ; et **`ANTHROPIC_DEFAULT_MODEL`** définit le modèle sur lequel les nouvelles sessions commencent.

  [Lire le digest de la Week 34 →](/docs/fr/whats-new/2026-w34)
</Update>

<Update label="Week 33" description="10–14 août 2026" tags={["v2.1.225–v2.1.233"]}>
  **Auto-continue après une limite d'utilisation sur Desktop** : quand vous atteignez votre limite de session dans Claude Code Desktop, cochez **Auto-continue when limits reset** sur la carte de limite et l'application réessaie le tour interrompu une fois que la limite est réinitialisée.

  Également cette semaine : **fork mode** est activé par défaut dans les sessions interactives, afin que Claude puisse confier une tâche secondaire à un sous-agent qui hérite de la conversation complète ; les URL de demande de fusion **GitLab** fonctionnent avec `--worktree` et la vue `claude agents`, et les places de marché clonent les URL `gitlab.com` nues ; et taper **`@`** dans l'invite mentionne une autre session Claude par nom.

  [Lire le digest de la Week 33 →](/docs/fr/whats-new/2026-w33)
</Update>

<Update label="Week 32" description="3–7 août 2026" tags={["v2.1.220–v2.1.224"]}>
  **Messagerie entre sessions** : sur macOS et Linux, vos sessions Claude Code peuvent maintenant se envoyer des messages les unes aux autres, afin que Claude transmette une découverte ou une décision d'une session à une autre au lieu que vous la réexpliquiez.

  Également cette semaine : les **environnements auto-hébergés** exécutent les sessions cloud de Claude Code sur l'infrastructure que votre organisation exploite, en bêta public sur les plans Team et Enterprise ; **auto mode** devient le mode de permission par défaut pour les nouvelles sessions sur les plans Pro, Max et Team à partir du 14 août ; et l'**extension VS Code** obtient la vue Focus.

  [Lire le digest de la Week 32 →](/docs/fr/whats-new/2026-w32)
</Update>

<Update label="Week 30" description="20–24 juillet 2026" tags={["v2.1.214–v2.1.219"]}>
  **Claude Opus 5** : le nouveau modèle Opus par défaut dans Claude Code, avec une fenêtre de contexte de 1M de tokens et le mode rapide à 10 $/50 $ par MTok.

  Également cette semaine : **Claude Code Desktop** ouvre un volet iOS Simulator en bêta public afin que Claude puisse exécuter votre application et la parcourir en cliquant pendant que vous regardez ; le **plugin Claude Security** exécute une analyse de vulnérabilité multi-agents de votre base de code et transforme les résultats que vous choisissez en correctifs que vous appliquez vous-même ; et **`/code-review`** s'exécute en tant que sous-agent d'arrière-plan.

  [Lire le digest de la Week 30 →](/docs/fr/whats-new/2026-w30)
</Update>

<Update label="Week 29" description="13–17 juillet 2026" tags={["v2.1.207–v2.1.212"]}>
  **Les artifacts appellent vos connecteurs MCP** : un artifact publié peut extraire des données en direct et prendre des mesures via les connecteurs MCP propres de chaque visualiseur quand ils ouvrent la page, et cette semaine ajoute également des liens de partage public, des rôles d'éditeur sur Team et Enterprise, et des artifacts créés à partir de sessions Claude Tag.

  Également cette semaine : le **mode lecteur d'écran** remplace l'interface de terminal visuelle par du texte simple et linéaire pour les lecteurs d'écran tels que VoiceOver et NVDA ; **`/fork`** copie votre conversation dans une nouvelle session d'arrière-plan pendant que vous continuez à travailler ; et **auto mode** n'a plus besoin d'une variable d'opt-in sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry.

  [Lire le digest de la Week 29 →](/docs/fr/whats-new/2026-w29)
</Update>

<Update label="Week 28" description="6–10 juillet 2026" tags={["v2.1.202–v2.1.206"]}>
  **Navigateur intégré sur Desktop** : Claude Code sur desktop obtient un navigateur intégré, afin que Claude puisse afficher des docs, des designs ou n'importe quel autre site et interagir avec les pages de la même manière qu'il le fait avec vos aperçus de serveur de développement local.

  Également cette semaine : **`/doctor`** est une vérification complète de configuration qui diagnostique les problèmes et peut les corriger, avec `/checkup` comme alias ; **auto mode** bloque la falsification de transcription et demande une confirmation avant `rm -rf` sur les variables non résolues ; et les **lignes de vue agent** affichent un mot d'état coloré et un titre écrit par un classificateur.

  [Lire le digest de la Week 28 →](/docs/fr/whats-new/2026-w28)
</Update>

<Update label="Week 27" description="29 juin – 3 juillet 2026" tags={["v2.1.195–v2.1.201"]}>
  **Claude Sonnet 5** : le nouveau modèle par défaut pour les sièges d'abonnement Pro, Team Standard et Enterprise, avec un codage de premier ordre et une utilisation d'outils au prix de Sonnet, une fenêtre de contexte native de 1M de tokens, et la réflexion adaptative activée par défaut.

  Également cette semaine : **Claude dans Chrome** est généralement disponible sur tous les plans Anthropic directs ; **les sous-agents s'exécutent en arrière-plan par défaut** afin que Claude continue de travailler pendant qu'ils s'exécutent ; **Claude Desktop sur Linux** arrive en bêta sur Ubuntu et Debian ; et **`/radio`** se connecte à Claude FM lo-fi radio.

  [Lire le digest de la Week 27 →](/docs/fr/whats-new/2026-w27)
</Update>

<Update label="Week 26" description="22–26 juin 2026" tags={["v2.1.185–v2.1.193"]}>
  **`claude mcp login`** : authentifiez un serveur MCP configuré depuis votre shell au lieu du menu interactif `/mcp`, et effacez ses identifiants stockés plus tard avec `claude mcp logout`.

  Également cette semaine : **shell mode répond à la sortie de commande** (`! npm test` obtient une explication sans une deuxième invite) ; **`/rewind`** peut reprendre une conversation d'avant que `/clear` ne soit exécuté ; et les **sous-agents d'arrière-plan** font maintenant surface les invites de permission dans la session principale au lieu de les refuser automatiquement.

  [Lire le digest de la Week 26 →](/docs/fr/whats-new/2026-w26)
</Update>

<Update label="Week 25" description="15–19 juin 2026" tags={["v2.1.178–v2.1.183"]}>
  **Artifacts** : transformez la sortie d'une session en une page en direct et partageable sur claude.ai qui se met à jour sur place au fur et à mesure que la session progresse, maintenant en bêta sur les plans Team et Enterprise.

  Également cette semaine : **les règles de refus et de demande correspondent aux paramètres d'outil** avec `Tool(param:value)`, par exemple `Agent(model:opus)` ; **`/config key=value`** définit n'importe quel paramètre depuis l'invite, en mode `-p`, et depuis Remote Control ; et **auto mode bloque les commandes git destructrices** quand vous n'avez pas demandé à abandonner le travail local.

  [Lire le digest de la Week 25 →](/docs/fr/whats-new/2026-w25)
</Update>

<Update label="Week 24" description="8–12 juin 2026" tags={["v2.1.166–v2.1.176"]}>
  **`/cd`** : déplacez la session actuelle vers un nouveau répertoire de travail en milieu de conversation sans reconstruire le cache de prompt.

  Également cette semaine : **les sous-agents peuvent générer leurs propres sous-agents** (les chaînes d'arrière-plan sont limitées à cinq niveaux de profondeur) ; **`--safe-mode`** démarre Claude Code avec toutes les personnalisations désactivées pour le dépannage ; et **`fallbackModel`** configure jusqu'à trois modèles de secours essayés dans l'ordre.

  [Lire le digest de la Week 24 →](/docs/fr/whats-new/2026-w24)
</Update>

<Update label="Week 23" description="1er–5 juin 2026" tags={["v2.1.158–v2.1.165"]}>
  **Auto mode sur Amazon Bedrock, Google Cloud's Agent Platform et Microsoft Foundry** : auto mode est maintenant disponible sur les fournisseurs tiers pour Opus 4.7 et Opus 4.8, remplaçant les invites de permission par des vérifications de sécurité en arrière-plan.

  Également cette semaine : **les modifications automatiques plus sûres** demandent une confirmation avant d'écrire des fichiers qui peuvent exécuter du code en mode `acceptEdits` ; **`/plugin list`** affiche vos plugins installés en ligne ; et les **exigences de version** permettent aux déploiements gérés d'exiger une plage de version Claude Code approuvée.

  [Lire le digest de la Week 23 →](/docs/fr/whats-new/2026-w23)
</Update>

<Update label="Week 22" description="25–29 mai 2026" tags={["v2.1.150–v2.1.157"]}>
  **Claude Opus 4.8** : le nouveau modèle par défaut pour Max, Team Premium, Enterprise pay-as-you-go, et les comptes Anthropic API, avec un effort élevé par défaut et `/effort xhigh` pour les tâches les plus difficiles.

  Également cette semaine : **dynamic workflows** orchestrent des dizaines à des centaines de sous-agents à partir d'un script que Claude écrit ; le **security-guidance plugin** examine les modifications de Claude pour les vulnérabilités pendant qu'il travaille ; et **fast mode** s'exécute sur Opus 4.8 à 10 $/50 $ par MTok.

  [Lire le digest de la Week 22 →](/docs/fr/whats-new/2026-w22)
</Update>

<Update label="Week 21" description="18–22 mai 2026" tags={["v2.1.143–v2.1.149"]}>
  **Auto mode sur le plan Pro** : auto mode s'exécute maintenant sur les comptes Pro et supporte Sonnet 4.6 aux côtés d'Opus, remplaçant les invites de permission par des vérifications de sécurité en arrière-plan.

  Également cette semaine : **`/usage`** détaille ce qui pilote vos limites de plan par skill, sous-agent, plugin et serveur MCP ; la nouvelle commande **`/code-review`** signale les bugs de correction ; et les **background sessions** apparaissent dans `/resume` et restent actives quand elles sont épinglées.

  [Lire le digest de la Week 21 →](/docs/fr/whats-new/2026-w21)
</Update>

<Update label="Week 20" description="11–15 mai 2026" tags={["v2.1.139–v2.1.142"]}>
  **Agent view** : `claude agents` ouvre un écran pour chaque session Claude Code, montrant ce qui s'exécute, ce qui vous attend, et ce qui est terminé.

  Également cette semaine : **`/goal`** garde Claude au travail sur plusieurs tours jusqu'à ce qu'une condition d'achèvement soit satisfaite ; **fast mode** s'exécute maintenant sur Opus 4.7 par défaut ; et le **menu Rewind** peut compresser le contexte antérieur avec « Summarize up to here ».

  [Lire le digest de la Week 20 →](/docs/fr/whats-new/2026-w20)
</Update>

<Update label="Week 19" description="4–8 mai 2026" tags={["v2.1.128–v2.1.136"]}>
  **Les plugins se chargent à partir d'archives `.zip` et d'URL** : `--plugin-dir` accepte maintenant les fichiers `.zip`, et `--plugin-url` récupère une archive de plugin pour la session actuelle.

  Également cette semaine : **`worktree.baseRef`** choisit si les nouveaux worktrees se branchent à partir de la valeur par défaut distante ou du `HEAD` local ; **les règles de refus dur en mode auto** bloquent les actions sans condition indépendamment des exceptions d'autorisation ; et **les hooks voient le niveau d'effort actif** via `effort.level` et `$CLAUDE_EFFORT`.

  [Lire le digest de la Week 19 →](/docs/fr/whats-new/2026-w19)
</Update>

<Update label="Week 18" description="27 avril – 1er mai 2026" tags={["v2.1.120–v2.1.126"]}>
  **Windows sans Git Bash** : Git pour Windows n'est plus requis, et Claude Code utilise PowerShell comme outil shell en l'absence de Bash.

  Également cette semaine : **`claude ultrareview`** apporte l'examen de code cloud à CI et aux scripts ; **`claude project purge`** nettoie l'état local d'un projet ; et coller une **URL de PR dans `/resume`** trouve la session qui l'a créée.

  [Lire le digest de la Week 18 →](/docs/fr/whats-new/2026-w18)
</Update>

<Update label="Week 17" description="20–24 avril 2026" tags={["v2.1.114–v2.1.119"]}>
  **`/ultrareview`** s'ouvre en tant qu'aperçu de recherche public : une flotte d'agents chasseurs de bugs s'exécute dans le cloud et les résultats reviennent automatiquement dans votre CLI ou Desktop.

  Également cette semaine : **session recap** vous montre ce qui s'est passé pendant qu'un terminal était en arrière-plan ; **custom themes** vous permet de créer et de déployer des palettes de couleurs depuis `/theme` ou un plugin ; et **Claude Code sur le web** bénéficie d'une refonte avec une nouvelle barre latérale de sessions et une disposition glisser-déposer.

  [Lire le digest de la Week 17 →](/docs/fr/whats-new/2026-w17)
</Update>

<Update label="Week 16" description="13–17 avril 2026" tags={["v2.1.105–v2.1.113"]}>
  **Claude Opus 4.7** arrive en tant que nouveau défaut sur Max et Team Premium, avec un nouveau niveau d'effort `xhigh` qui est le paramètre recommandé pour la plupart des travaux de codage et un curseur `/effort` interactif pour l'ajuster.

  Également cette semaine : **Routines** sur Claude Code sur le web déclenche des agents cloud modélisés à partir d'un calendrier, d'un événement GitHub ou d'un appel API ; **notifications push mobiles** vous pingent sur votre téléphone quand une tâche longue se termine ou que Claude a besoin de vous ; `/usage` affiche ce qui pilote vos limites ; et la CLI passe à des binaires natifs.

  [Lire le digest de la Week 16 →](/docs/fr/whats-new/2026-w16)
</Update>

<Update label="Week 15" description="6–10 avril 2026" tags={["v2.1.92–v2.1.101"]}>
  **Ultraplan** entre en aperçu précoce : rédigez un plan dans le cloud depuis votre CLI, examinez-le et commentez-le dans un éditeur web, puis exécutez-le à distance ou récupérez-le localement. La première exécution crée maintenant automatiquement un environnement cloud pour vous.

  Également cette semaine : l'outil **Monitor** diffuse les événements d'arrière-plan dans la conversation afin que Claude puisse suivre les logs et réagir en direct, `/loop` s'auto-règle lorsque vous omettez l'intervalle, `/team-onboarding` empaquette votre configuration dans un guide rejouable, et `/autofix-pr` active la correction automatique des PR depuis votre terminal.

  [Lire le digest de la Week 15 →](/docs/fr/whats-new/2026-w15)
</Update>

<Update label="Week 14" description="30 mars – 3 avril 2026" tags={["v2.1.86–v2.1.91"]}>
  **Computer use** arrive à la CLI en aperçu de recherche : Claude peut ouvrir des applications natives, cliquer dans l'interface utilisateur et vérifier les modifications depuis votre terminal. Idéal pour fermer la boucle sur les choses que seule une interface graphique peut vérifier.

  Également cette semaine : leçons interactives `/powerup`, rendu alt-screen sans scintillement, une limite de taille de résultat MCP par outil jusqu'à 500K, et exécutables de plugin sur le `PATH` de l'outil Bash.

  [Lire le digest de la Week 14 →](/docs/fr/whats-new/2026-w14)
</Update>

<Update label="Week 13" description="23–27 mars 2026" tags={["v2.1.83–v2.1.85"]}>
  **Auto mode** arrive en aperçu de recherche : un classificateur gère vos invites de permission afin que les actions sûres s'exécutent sans interruption et que les actions risquées soient bloquées. Le juste milieu entre approuver tout et `--dangerously-skip-permissions`.

  Également cette semaine : computer use dans l'application Desktop, correction automatique des PR sur Web, recherche de transcription avec `/`, un outil PowerShell natif pour Windows, et hooks `if` conditionnels.

  [Lire le digest de la Week 13 →](/docs/fr/whats-new/2026-w13)
</Update>
