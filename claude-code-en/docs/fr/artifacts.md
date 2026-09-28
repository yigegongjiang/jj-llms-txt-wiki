> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Partager la sortie de session en tant qu'artefacts

> Les artefacts transforment le travail de Claude Code en pages en direct et interactives sur claude.ai que vous pouvez garder privées, partager avec votre organisation ou publier via un lien public.

<Note>
  Les artefacts sont disponibles sur les plans Pro, Max, Team et Enterprise et nécessitent une session connectée avec [`/login`](/docs/fr/setup#authenticate). Consultez [Disponibilité](#availability) pour l'ensemble complet des exigences.
</Note>

Un [artefact](https://claude.com/features/artifacts) est une page web en direct et interactive que Claude Code publie depuis votre session vers une URL privée sur claude.ai. Vous l'ouvrez dans un navigateur, et elle se met à jour sur place à mesure que la session continue. Partagez-la depuis l'en-tête de la page quand vous voulez que quelqu'un d'autre la voie aussi.

<Frame>
  <img src="https://mintcdn.com/claude-code/kaHIYYMIYMYPxQg9/images/artifacts-viewer.png?fit=max&auto=format&n=kaHIYYMIYMYPxQg9&q=85&s=dbfd671cdb0d15f49f808b9e89778fe1" alt="Un artefact ouvert dans un navigateur à claude.ai/code/artifact. L'en-tête du visualiseur affiche le titre de l'artefact acme-funnel-fix, un bouton Partager et l'avatar de l'auteur. Le menu Partager est ouvert avec le bouton bascule Toujours partager la dernière version, un sélecteur de version indiquant Partage de la version 2, un sélecteur d'audience Tous chez Acme et un bouton Copier le lien. Sous l'en-tête, la page d'artefact affiche deux maquettes mobiles côte à côte, un graphique en entonnoir et une rangée de cartes de métriques." width="2511" height="1890" data-path="images/artifacts-viewer.png" />
</Frame>

<h2 id="when-to-use-an-artifact">
  Quand utiliser un artifact
</h2>

Utilisez un artifact quand le texte du terminal n'est pas le bon médium pour ce que Claude a produit : une sortie qui est plus facile à consulter et avec laquelle interagir que de lire ligne par ligne. Claude construit la page à partir de tout ce que votre session peut atteindre, y compris votre base de code et les données qu'il récupère via vos [outils connectés](/docs/fr/mcp), de sorte que la page peut afficher des choses qui prendraient des paragraphes à décrire. Par exemple, demandez à Claude de :

* Guider un relecteur à travers une demande de tirage avec des diffs annotés
* Afficher un tableau de bord à partir des données que la session a déjà récupérées
* Présenter plusieurs options de conception ou d'implémentation côte à côte
* Maintenir une chronologie d'investigation qui se remplit pendant qu'une tâche longue s'exécute
* Envoyer à un coéquipier un lien au lieu de coller la sortie dans Slack
* Publier un tableau de statut qui [récupère les données actualisées via les connecteurs MCP](#pull-live-data-with-mcp-connectors) chaque fois que quelqu'un l'ouvre

Consultez [Ce que vous pouvez créer](#what-you-can-build) pour les invites qui correspondent à celles-ci, et [Récupérer les données en direct avec les connecteurs MCP](#pull-live-data-with-mcp-connectors) pour l'invite du tableau alimenté par connecteur.

<h3 id="what-an-artifact-is-not">
  Ce qu'un artifact n'est pas
</h3>

Un artifact est une capture de travail : une page autonome sans backend, elle ne peut donc pas servir plusieurs routes. Pour un outil interne hébergé avec un backend, déployez-le plutôt sur votre propre infrastructure. Consultez [Contraintes de page](#page-constraints) pour l'ensemble complet des limites.

<h2 id="create-an-artifact">
  Créer un artifact
</h2>

Claude peut publier un artifact de sa propre initiative lorsque le résultat convient à une page, ou vous pouvez en demander un directement. Pour en demander un, nommez la fonctionnalité ou décrivez le résultat visuel que vous souhaitez en langage naturel. Un bon candidat est tout ce qui est plus facile à voir qu'à lire sous forme de texte, comme un diff annoté, un graphique ou un ensemble d'options à comparer. Les invites ci-dessous sont deux exemples ; consultez [Ce que vous pouvez créer](#what-you-can-build) pour plus de modèles.

```text wrap theme={null}
Make an artifact that walks through this PR with the diff annotated inline.
```

```text wrap theme={null}
Build a dashboard artifact of last week's deploy failures by service and keep it updated as you investigate.
```

Sauf si vous nommez un emplacement, Claude écrit la page dans un fichier HTML ou Markdown dans un répertoire temporaire en dehors de votre projet, puis la publie. La publication d'un nouvel artifact passe par le [mode de permission](/docs/fr/permission-modes) de votre session :

* **Mode Auto** : le classificateur examine la publication au lieu de vous demander, donc Claude peut publier une page sans que vous voyiez une invite. Le mode dans lequel vos sessions commencent dépend de votre plan ; consultez [le mode de permission initial](/docs/fr/permission-modes#eliminate-prompts-with-auto-mode).
* **Modes Manuel et Accepter les modifications** : Claude Code demande une permission ; il pourrait dire quelque chose comme `Claude wants to publish deploy-failures.html, uploading it to claude.ai (Anthropic's servers) to host as the page "Deploy failures by service", private to you until you share it`. Sélectionnez **Oui** pour publier.

Après avoir approuvé un artifact une fois, Claude Code le republiera sans demander, et demandera à nouveau dans certains cas, notamment lorsque :

* Claude déclare une capacité d'exécution pour la page, comme les [appels de connecteur](#pull-live-data-with-mcp-connectors) ou les [téléchargements de fichiers](#offer-a-file-download)
* Vous l'avez depuis [partagé publiquement](#share-an-artifact)
* Vous l'avez depuis partagé avec des personnes spécifiques ou votre organisation avec la dernière version choisie comme version que les spectateurs voient

Après la première publication, Claude imprime l'URL, et votre navigateur s'ouvre sur la nouvelle page. Si vous avez envoyé l'invite via [Contrôle à distance](/docs/fr/remote-control) depuis claude.ai, Claude Desktop ou l'application mobile Claude, aucun onglet ne s'ouvre sur la machine exécutant la session. Le navigateur s'ouvre là la prochaine fois que Claude publie l'artifact à partir d'une invite que vous tapez au terminal. Appuyez sur `Ctrl+]` à tout moment pour rouvrir l'artifact le plus récent de la session.

Claude choisit le titre de l'artifact et un emoji, et les deux apparaissent dans votre [galerie d'artifacts](#share-an-artifact) sur claude.ai et dans les liens partagés. Claude peut également choisir une icône d'onglet de navigateur qui correspond à ce que la page est, comme un graphique ou un calendrier. Demandez à Claude un titre, un emoji ou une icône d'onglet spécifique si vous en voulez un.

Pour empêcher le navigateur de s'ouvrir automatiquement lors de la publication d'un nouvel artifact, définissez `CLAUDE_CODE_ARTIFACT_AUTO_OPEN=0` dans votre environnement.

Si Claude répond qu'il ne peut pas publier, ou écrit un fichier HTML local sans lien, l'outil n'est pas activé pour votre session. Vérifiez les exigences de [Disponibilité](#availability).

<h2 id="update-an-artifact">
  Mettre à jour un artifact
</h2>

Demandez à Claude de réviser la page, ou laissez une tâche longue durée republier au fur et à mesure de sa progression. Claude modifie le fichier sous-jacent et republish à la même URL.

```text wrap theme={null}
Ajouter une ventilation par région sous le graphique de synthèse et republish.
```

Toute personne ayant la page ouverte voit la mise à jour en place. Chaque publication devient une version, et à partir du contrôle **Share** dans l'en-tête de la page, vous pouvez choisir quelle version les lecteurs voient.

Pour mettre à jour un artifact à partir d'une session différente, donnez à Claude son URL, ou attachez-le avec [`/artifacts`](#find-an-artifact-again). Sans l'un ou l'autre, une nouvelle session crée un nouvel artifact au lieu de mettre à jour un existant.

```text wrap theme={null}
Update https://claude.ai/code/artifact/5fbea6f3-... with today's numbers.
```

<h2 id="find-an-artifact-again">
  Retrouver un artifact
</h2>

Exécutez `/artifacts` dans Claude Code pour lister chaque artifact que vous possédez et chaque artifact partagé avec vous. Sélectionnez-en un et appuyez sur `o` pour l'ouvrir dans votre navigateur ou `c` pour copier son lien. Appuyez sur `Entrée` pour l'attacher à la session actuelle ; avant la v2.1.216, `Entrée` l'ouvrait dans votre navigateur. Claude Code lit la liste à partir de votre compte claude.ai, donc cela fonctionne dans une nouvelle session et après `/clear`, quand le lien a disparu du terminal. Nécessite Claude Code v2.1.208 ou version ultérieure.

<h2 id="share-an-artifact">
  Partager un artifact
</h2>

Un nouvel artifact n'est visible que pour vous. Pour le partager, ouvrez l'artifact dans votre navigateur et utilisez le contrôle **Partager** dans l'en-tête de la page. L'en-tête contient également un lien vers votre galerie sur [claude.ai/code/artifacts](https://claude.ai/code/artifacts), qui répertorie tous les artifacts que vous avez créés.

Les spectateurs de votre organisation peuvent voir qui a publié la page : sur un artifact partagé au sein de votre organisation, votre nom figure dans le menu de titre, et sur un artifact public, il figure dans l'en-tête de la page pour les spectateurs connectés de votre organisation. Un spectateur qui ouvre un lien public sans se connecter, ou depuis l'extérieur de votre organisation, voit l'étiquette « Le contenu est généré par l'utilisateur et non vérifié. » à la place de votre nom.

Les personnes avec lesquelles vous pouvez partager dépendent de votre plan :

* **Au sein de votre organisation** : sur les plans Team et Enterprise, accordez l'accès à des personnes spécifiques de votre organisation, ou à tous les membres. Les spectateurs se connectent à claude.ai en tant que membres de votre organisation pour voir la page.
* **Publiquement** : partagez un lien que n'importe qui sur Internet peut ouvrir, sans connexion à claude.ai requise. Sur les plans Pro et Max, un lien public est le seul moyen de partager un artifact. Sur les plans Team et Enterprise, le partage public est désactivé jusqu'à ce qu'un propriétaire [l'active pour l'organisation](#control-public-sharing).

<h3 id="let-someone-edit-with-you">
  Laisser quelqu'un modifier avec vous
</h3>

Les personnes avec lesquelles vous partagez sont des spectateurs par défaut : elles voient chaque version que vous publiez mais ne peuvent pas modifier la page. Sur les plans Team et Enterprise, vous pouvez également faire de quelqu'un un éditeur. Dans la boîte de dialogue de partage, ajoutez une personne et changez son rôle de **spectateur** à **éditeur**.

Un éditeur publie de nouvelles versions de la même manière que vous [mettez à jour l'artifact à partir d'une autre session](#update-an-artifact) : il donne à Claude l'URL de l'artifact, ou l'attache à partir de [`/artifacts`](#find-an-artifact-again), et Claude récupère le contenu actuel et le republish avec ses modifications. Tous les utilisateurs ayant la page ouverte voient chaque mise à jour en direct.

<h2 id="read-an-artifact-shared-with-you">
  Lire un artifact partagé avec vous
</h2>

Quand quelqu'un partage un artifact avec vous, vous pouvez demander à Claude de le lire : donnez à Claude son URL, ou attachez-le depuis [`/artifacts`](#find-an-artifact-again).

Claude lit une page que quelqu'un d'autre a écrite de la même manière qu'il lit une page web avec [WebFetch](/docs/fr/tools-reference#webfetch-tool-behavior) : il obtient un résumé de ce qu'il a demandé plutôt que la page brute, et le résumé rapporte les instructions écrites dans la page au lieu de les relayer. Claude Code sauvegarde également le code source complet de la page dans un fichier local, que Claude peut ouvrir quand il a besoin du contenu exact, par exemple pour republier l'artifact en tant qu'[éditeur](#let-someone-edit-with-you).

<h2 id="collect-comments-on-an-artifact">
  Collecter les commentaires sur un artifact
</h2>

Lorsque vous partagez un artifact au sein de votre organisation, les personnes avec lesquelles vous le partagez peuvent laisser des commentaires sur la page, et vous pouvez demander à Claude de lire ces commentaires et d'y répondre. Vous avez besoin de Claude Code v2.1.221 ou version ultérieure et d'un plan Team ou Enterprise, car seul un artifact que vous [partagez au sein de votre organisation](#share-an-artifact) accepte les commentaires. Claude lit les commentaires dans deux cas :

* **Vous demandez à Claude de les lire** : donnez à Claude l'URL de l'artifact et demandez les commentaires. Claude énumère chaque fil de discussion et marque les commentaires envoyés par quelqu'un qui peut modifier l'artifact.
* **Quelqu'un qui peut modifier l'artifact envoie un commentaire à Claude** : dans un fil de discussion sur la page, il envoie un commentaire avec **Send to Claude**, ou mentionne `@claude` dans celui-ci. De l'une ou l'autre façon, il active le fil de discussion.

Claude ne peut répondre à ou résoudre que les fils de discussion activés. Les autres fils restent ouverts jusqu'à ce qu'une personne les résolve sur la page. Les lecteurs voient chaque réponse attribuée à Claude, via vous.

Si vous partagez un artifact publiquement, les lecteurs ne peuvent pas commenter : la page affiche `Comments aren't available while this Artifact is shared publicly.` Pour basculer un artifact qui a déjà des fils de commentaires vers un lien public, supprimez d'abord les fils.

Pour demander les commentaires vous-même, donnez à Claude l'URL :

```text wrap theme={null}
Read the comments on https://claude.ai/code/artifact/5fbea6f3-... and make the changes the commenters ask for.
```

Si Claude vous dit qu'il ne peut pas lire les commentaires, confirmez votre version, votre session et votre paramètre de drapeau de fonctionnalité :

* Vous exécutez Claude Code v2.1.221 ou version ultérieure.
* Vous n'êtes pas dans votre première session depuis que vous avez installé Claude Code ou mis à niveau à partir d'une version antérieure à v2.1.221. Dans cette [première session après une installation ou une mise à niveau](/docs/fr/env-vars#first-session-after-an-install-or-upgrade), Claude pourrait ne pas être en mesure de lire les commentaires ; démarrez une nouvelle session et demandez à nouveau.
* Vous n'avez pas désactivé la récupération des drapeaux de fonctionnalités.

<h3 id="let-claude-reply-to-comments-on-its-own">
  Laisser Claude répondre aux commentaires automatiquement
</h3>

Après que votre session publie un artifact, Claude Code surveille cet artifact pour les commentaires aussi longtemps que la session s'exécute. Lorsque quelqu'un qui peut modifier l'artifact envoie un commentaire à Claude, il atteint votre session immédiatement, et Claude peut lire le fil de discussion et répondre sans que vous le demandiez.

Vous avez besoin de Claude Code v2.1.228 ou version ultérieure. Si vous avez désactivé la [récupération des drapeaux de fonctionnalités](/docs/fr/env-vars#features-that-need-feature-flag-fetching), Claude Code ne surveille pas les commentaires.

Votre [mode de permission](/docs/fr/permission-modes) décide ce que Claude fait lorsqu'un commentaire envoyé arrive :

* **Claude répond automatiquement** : lorsque votre mode de permission permet à Claude de publier la réponse sans vous demander, Claude lit le fil de discussion et répond, et modifie l'artifact lorsque le commentaire demande une modification. Vous voyez `Auto-replied to comment thread on Artifact: <name>` ou `Auto-edited Artifact: <name> in response to a comment thread`.
* **Claude vous attend** : en dehors du plan mode, lorsque la publication de la réponse nécessiterait votre approbation, vous voyez `Comments are waiting on Artifact: <name>`. Claude vous demande alors l'approbation pour lire le fil de discussion, et à nouveau pour publier la réponse.
* **Claude s'arrête en plan mode** : vous voyez `Comments are waiting on Artifact: <name>`, et Claude ne répond pas jusqu'à ce que vous quittiez le plan mode et lui demandiez de lire et répondre.

Claude arrête également de répondre automatiquement à un artifact après avoir traité 60 commentaires envoyés ou activations de fil de discussion sur cet artifact en une heure. Vous voyez `Comments are waiting on Artifact: <name>` une fois, et Claude reprend à mesure que les commentaires de cette heure vieillissent.

Exécutez `/tasks` pour voir chaque artifact que votre session surveille, listé comme une tâche de mises à jour en direct. Vous pouvez empêcher Claude de répondre automatiquement de l'une de ces façons :

* **Appuyez sur Ctrl+C une fois à une invite inactive** : Claude arrête de répondre sur chaque artifact que votre session surveille. Les réponses recommencent après que vous envoyiez votre prochain message.
* **Arrêtez la tâche dans `/tasks`** : Claude arrête de répondre sur cet artifact jusqu'à ce que vous lui demandiez de reprendre les réponses là-bas. La publication de l'artifact à nouveau ne redémarre pas les réponses, et l'arrêt s'applique toujours lorsque vous reprenez la session plus tard.
* **Appuyez sur `Ctrl+X Ctrl+K` deux fois en 3 secondes** : l'accord qui [arrête chaque sous-agent d'arrière-plan en cours d'exécution](/docs/fr/interactive-mode#general-controls) arrête également Claude de répondre sur chaque artifact pour le reste de la session. Demander à Claude de reprendre les réponses n'annule pas cet arrêt.

Si le service qui livre les commentaires devient indisponible ou cesse de répondre, Claude Code continue d'essayer de se reconnecter pendant un certain temps, puis arrête de surveiller chaque artifact que votre session surveillait.

<h2 id="pull-live-data-with-mcp-connectors">
  Récupérer des données en direct avec les connecteurs MCP
</h2>

Un artifact peut appeler des [connecteurs MCP](/docs/fr/mcp#use-mcp-servers-from-claude-ai) chaque fois que quelqu'un le consulte, de sorte que la page affiche des données actuelles plutôt qu'une capture instantanée de la session qui l'a créé. Les appels de connecteur à partir d'artifacts sont disponibles sur les plans Pro, Max, Team et Enterprise et nécessitent Claude Code v2.1.209 ou version ultérieure. Sur les versions antérieures, Claude publie la page avec les données que la session a rassemblées lors de sa création.

Pour créer une page alimentée par un connecteur, nommez le connecteur et les données que vous souhaitez dans votre prompt :

```text wrap theme={null}
Build a dashboard artifact of our open pull requests that pulls the live list through my GitHub connector when the page loads.
```

Claude déclare quels connecteurs la page peut appeler lors de la publication, et la page ne peut pas appeler de connecteurs en dehors de cette déclaration. Seuls les connecteurs de votre compte claude.ai sont admissibles : Claude les nomme dans la déclaration, et lorsque quelqu'un consulte la page, chaque appel [s'exécute via la propre connexion du compte qui consulte](#how-connector-calls-work-for-viewers) à ce connecteur. Les serveurs MCP locaux que vous configurez dans Claude Code, tels que les serveurs de `.mcp.json`, peuvent fournir des données pendant que Claude crée la page, mais la page publiée ne peut pas les appeler.

La page récupère les données au chargement et peut s'actualiser à un intervalle ou lorsqu'un lecteur utilise un contrôle d'actualisation sur la page. Les réponses sont mises en cache dans le navigateur du lecteur, de sorte qu'une page rouverte s'affiche à partir des réponses mises en cache immédiatement, puis se met à jour avec les résultats actualisés.

<h3 id="how-connector-calls-work-for-viewers">
  Comment les appels de connecteur fonctionnent pour les lecteurs
</h3>

Lorsqu'une page publiée appelle un connecteur, l'appel utilise le compte de la personne qui consulte la page, et non le compte de la personne qui l'a publiée :

* **Chaque lecteur utilise ses propres connecteurs** : les appels passent par les outils connectés du compte qui consulte, de sorte que deux personnes ouvrant le même tableau de bord peuvent voir des données différentes selon ce que leurs comptes peuvent accéder. La page ne voit jamais les identifiants de personne ; claude.ai effectue les appels au nom de la page.
* **Les lecteurs approuvent d'abord l'accès** : claude.ai demande à chaque lecteur la permission avant le premier appel de connecteur de la page. Un lecteur qui refuse, ou qui n'a pas connecté un connecteur que la page utilise, voit toujours la page sans ses sections en direct.
* **Les actions utilisent également le compte du lecteur** : une page peut offrir des contrôles qui invoquent des outils de connecteur avec des effets secondaires, tels que publier un message ou mettre à jour un problème. L'action s'exécute via le compte de celui qui sélectionne le contrôle.

Lorsque vous prévoyez de partager une page alimentée par un connecteur, demandez à Claude d'inclure un message de secours dans chaque section en direct qui nomme le connecteur dont elle a besoin. Un lecteur qui manque la connexion voit alors ce qu'il faut connecter au lieu d'une section vide.

Un artifact qui appelle des connecteurs ne peut pas être partagé via un lien public sur aucun plan. Sur les plans Team et Enterprise, vous pouvez le garder privé ou [le partager au sein de votre organisation](#share-an-artifact). Sur les plans Pro et Max, où un lien public est le seul moyen de partager, un artifact alimenté par un connecteur reste privé pour vous.

<h3 id="the-page-shows-no-live-data-for-a-viewer">
  La page n'affiche aucune donnée en direct pour un lecteur
</h3>

Lorsqu'une page alimentée par un connecteur s'affiche mais que ses sections en direct restent vides pour quelqu'un avec qui vous l'avez partagée, travaillez à travers ces causes :

* **Le lecteur n'a pas connecté le connecteur** : les connecteurs sont par compte, de sorte que chaque lecteur a besoin de sa propre connexion à chaque connecteur que la page appelle. Il peut en ajouter un sous **Paramètres > Connecteurs** sur claude.ai, puis recharger la page.
* **Le lecteur a refusé la demande de permission** : un refus dure pour le reste de ce chargement de page. Recharger la page ramène la demande de permission.
* **Les appels de connecteur sont désactivés pour l'organisation** : un propriétaire contrôle le [bouton bascule **Activer les connecteurs d'artifact**](#control-connector-calls-from-artifacts) dans les paramètres d'administration.
* **La page appelle des noms d'outils que le connecteur n'expose pas** : les sections affectées restent vides pour tout le monde, y compris vous. Cela peut se produire lorsqu'une page nomme les outils individuels derrière un connecteur de style passerelle qui n'expose que quelques-uns de ses propres outils. Demandez à Claude de corriger les noms d'outils que la page appelle et de la publier à nouveau.

  Lorsque Claude publie la page et que les outils de ce connecteur sont disponibles dans votre session, Claude Code vérifie les noms d'outils que la page déclare par rapport à eux, avertit Claude des noms qui ne correspondent pas, et refuse la publication lorsqu'aucun ne correspond. Avant la v2.1.265, il publiait la page sans les vérifier.

<h2 id="offer-a-file-download">
  Proposer un téléchargement de fichier
</h2>

Un artifact peut proposer aux lecteurs un fichier généré par la page, comme une exportation CSV d'un tableau ou une image PNG d'un graphique. Le lecteur l'enregistre via un contrôle de téléchargement sur la page, comme un bouton. Les téléchargements de fichiers sont une capacité d'exécution que claude.ai active par compte, donc Claude vérifie si votre compte la possède avant de construire le contrôle.

Les lecteurs ne peuvent pas enregistrer un fichier à partir d'un lien de téléchargement ordinaire ou d'un script sur la page, car la visionneuse d'artifacts sur claude.ai bloque tout téléchargement que la page démarre elle-même, y compris les liens vers les URL `data:` ou `blob:`. Si une page a des boutons de téléchargement construits de cette façon, demandez à Claude de les reconstruire avec la capacité de téléchargements.

Pour proposer un fichier, demandez le contrôle et le format de fichier dans votre prompt :

```text wrap theme={null}
Add a button that downloads this table as a CSV file.
```

Claude déclare la capacité de téléchargements dans le cadre de la publication, de la même manière qu'il [déclare les connecteurs](#pull-live-data-with-mcp-connectors).

<h2 id="what-you-can-build">
  Ce que vous pouvez créer
</h2>

Un artefact est une seule page HTML, donc tout ce que vous pouvez exprimer en HTML, CSS et JavaScript en ligne est dans le champ d'application. Les modèles ci-dessous reviennent le plus souvent.

<h3 id="walk-through-a-change">
  Parcourir une modification
</h3>

Demandez une page qui affiche un diff ou une modification de conception avec des annotations à côté des lignes pertinentes, afin que les relecteurs puissent lire votre raisonnement à côté du code au lieu de le reconstruire à partir d'une description.

```text wrap theme={null}
Make an artifact that walks through this PR. Render the diff with margin annotations and color-code findings by severity.
```

<h3 id="compare-alternatives">
  Comparer les alternatives
</h3>

Demandez plusieurs variantes sur une page afin de pouvoir les évaluer les unes par rapport aux autres. Cela fonctionne pour les mises en page, le texte, les formes d'API ou les plans d'implémentation.

```text wrap theme={null}
Make an artifact with four distinctly different layouts for the settings panel. Vary density and grouping, and lay them out as a grid with a one-line tradeoff under each.
```

<h3 id="tune-with-interactive-controls">
  Affiner avec des contrôles interactifs
</h3>

Demandez des curseurs, des bascules ou des champs d'entrée liés à ce que vous ajustez, afin de pouvoir explorer les valeurs directement au lieu de les décrire.

```text wrap theme={null}
Build an artifact with sliders for the easing curve, duration, and delay so I can try values on this transition. Show the animation live as I move them.
```

<h3 id="bring-the-result-back-to-your-session">
  Ramener le résultat à votre session
</h3>

Un artefact peut servir d'éditeur léger pour une décision que vous remettez ensuite à Claude. Demandez un contrôle d'exportation qui produit du texte que vous pouvez coller dans le terminal, afin que le résultat de l'interaction avec la page revienne à la session au lieu de rester sur la page.

```text wrap theme={null}
Make a triage board artifact with each open issue as a draggable card across Now, Next, Later, and Cut columns. Add a "Copy as prompt" button that gives me the final ordering to paste back here.
```

<h3 id="track-work-in-progress">
  Suivre le travail en cours
</h3>

Demandez à Claude de maintenir un artefact à jour pendant qu'une tâche longue s'exécute, afin que quiconque dispose du lien puisse suivre sans lire le terminal.

```text wrap theme={null}
Turn this migration plan into a checklist artifact. Check items off as you complete them and add a note for anything you skip.
```

<h2 id="improve-the-visual-design">
  Améliorer la conception visuelle
</h2>

Claude applique une compétence de conception intégrée lorsqu'il crée un artifact, de sorte que les pages obtiennent une palette, une typographie et une mise en page délibérées sans invite supplémentaire. Cette compétence recherche également un système de conception existant dans votre projet avant de choisir le sien. Les design tokens sont les valeurs nommées de couleur, de typographie et d'espacement que votre système de conception réutilise. Pour maintenir les artifacts cohérents avec la marque de votre produit, enregistrez-les où Claude peut les trouver, par exemple dans le [CLAUDE.md](/docs/fr/memory) du projet ou dans un fichier de thème de votre référentiel :

```markdown theme={null}
## Design system

- Colors: primary #1a4d8f, accent #f59e0b, surface #f8fafc
- Typography: Inter for body, JetBrains Mono for code
- Spacing: 8px scale, 6px border radius
```

Claude traite votre système de conception comme ayant une priorité plus élevée que ses propres choix, et votre invite comme ayant une priorité plus élevée que les deux. Le titre et le format ci-dessus sont un exemple ; toute liste claire de couleurs, de polices et d'espacement fonctionne.

Pour la typographie, Claude peut charger une police à partir de Google Fonts, la seule source de police externe qu'une page artifact peut charger. Claude intègre toute autre police en tant que `@font-face` data URI et donne à chaque police une pile de secours, de sorte que la page s'affiche toujours si une police ne se charge pas. Pour utiliser une police spécifique, nommez-la dans votre invite ou dans votre système de conception.

<h2 id="draft-a-design-canvas">
  Créer un canevas de conception
</h2>

Pour créer une maquette d'une interface utilisateur, un flux d'écran, une page d'accueil ou une affiche plutôt que de construire une page, exécutez `/design` avec un résumé. Claude crée la conception sous forme d'artboards sur un canevas et publie le canevas en tant qu'artefact Design. Le résumé nomme ce que vous voulez dessiner :

```text wrap theme={null}
/design a settings screen for a mobile banking app
```

Ouvrez l'artefact publié dans un navigateur de bureau pour examiner les artboards. Sélectionnez un élément sur un artboard et modifiez-le, et vos modifications s'enregistrent automatiquement. Vous pouvez exporter chaque artboard en PNG ou PDF.

`/design` nécessite une session où [les artefacts sont disponibles](#availability) et Claude Code v2.1.265 ou version ultérieure.

<h2 id="page-constraints">
  Contraintes de page
</h2>

Chaque artifact est une page autonome. Claude Code enveloppe le fichier que vous publiez dans une coquille de document HTML et le sert selon une politique de sécurité du contenu (CSP) stricte, qui façonne ce que la page peut faire.

| Contrainte               | Effet                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| :----------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Requêtes externes        | La page peut charger des polices de caractères à partir de Google Fonts et des scripts à partir de [cinq hôtes CDN publics](#allowlist-the-viewer-domain) : cdnjs, unpkg, les CDN Tailwind et jQuery, et les chemins sélectionnés sur jsDelivr tels que `/npm/`. La CSP bloque chaque image externe et tous les autres scripts, feuilles de style et polices externes, et permet aux appels `fetch`, XHR et WebSocket d'atteindre uniquement l'origine de la page et les hôtes Google Fonts. Claude charge donc toute bibliothèque dont la page a besoin à partir de l'un de ces CDN, intègre tous les autres CSS et JavaScript, et incorpore les images en tant qu'URI de données. [Les appels Connector](#pull-live-data-with-mcp-connectors) passent par claude.ai, qui effectue lui-même l'appel réseau. |
| Pas de backend           | Un artifact est une page statique. Il ne peut pas authentifier les visiteurs lui-même.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Téléchargements          | La page ne peut pas lancer un téléchargement elle-même. Pour permettre aux visiteurs d'enregistrer un fichier généré par la page, Claude déclare la capacité de téléchargements. Voir [Proposer un téléchargement de fichier](#offer-a-file-download).                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| Page unique              | Les liens relatifs ne se résolvent pas, car rien n'est déployé aux côtés de la page. Pour le contenu multi-sections, Claude utilise des ancres dans la page plutôt que des fichiers séparés.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| Types de fichiers source | Le fichier publié doit être `.html`, `.htm` ou `.md`, et doit être décodable en UTF-8, ou en UTF-16 little-endian par sa marque d'ordre des octets. Les fichiers Markdown s'affichent en tant que pages de document stylisées avec du code en surbrillance syntaxique. Un fichier qui ne se décode pas, ou qui contient le caractère de remplacement `U+FFFD`, est [refusé avec la ligne et la colonne à corriger](/docs/fr/errors#the-source-file-is-not-valid-utf-8-text).                                                                                                                                                                                                                                                                                                                                      |
| Taille rendue            | La page rendue doit faire 16 Mio ou moins. Les grandes images incorporées sont la cause habituelle lorsqu'une publication échoue pour des raisons de taille.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |

Générer un artifact utilise des tokens de sortie comme toute autre réponse, et une page stylisée est plus gourmande en tokens que le même contenu sous forme de texte terminal. Les CSS intégrés, le JavaScript pour les contrôles interactifs, et surtout les images incorporées en tant qu'URI de données sont les principaux contributeurs. Pour réduire le coût en tokens d'un artifact :

* Préférez SVG, ou HTML et CSS, pour les diagrammes plutôt que les images raster incorporées
* Omettez l'interactivité dont vous n'avez pas besoin
* Faites en sorte que la page résume les grands ensembles de données plutôt que de les intégrer en totalité

<h2 id="availability">
  Disponibilité
</h2>

Les artefacts nécessitent chaque condition ci-dessous. Lorsque l'une d'elles n'est pas remplie, Claude écrit un fichier HTML local ou dit qu'il ne peut pas publier à la place.

| Exigence                    | Disponible quand                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| :-------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plan                        | Pro, Max, Team ou Enterprise. Sur les plans Pro et Max, les artefacts sont privés pour vous jusqu'à ce que vous les partagiez, et aucune gestion d'administrateur ne s'applique. Sur les plans Team, les artefacts sont activés par défaut. Sur les plans Enterprise, un propriétaire [les active](#manage-artifacts-for-your-organization) dans les paramètres d'administration de claude.ai.                                                                       |
| Authentification            | La session est sauvegardée par un compte claude.ai : connectez-vous avec `/login` dans la CLI ou l'application de bureau. Les sessions Claude Tag sont connectées via l'identité de l'agent, donc aucune étape n'est nécessaire. Les sessions utilisant une clé API, un [jeton de passerelle](/docs/fr/llm-gateway) ou une identifiant de fournisseur cloud ne peuvent pas publier.                                                                                       |
| Fournisseur de modèle       | API Anthropic. Non disponible sur [Amazon Bedrock](/docs/fr/amazon-bedrock), [Google Cloud's Agent Platform](/docs/fr/google-vertex-ai) ou [Microsoft Foundry](/docs/fr/microsoft-foundry).                                                                                                                                                                                                                                                                                         |
| Politique organisationnelle | Les clés de chiffrement gérées par le client (CMEK), HIPAA et [Zéro rétention de données](/docs/fr/zero-data-retention) ne sont pas activées pour l'organisation.                                                                                                                                                                                                                                                                                                         |
| Surface                     | Claude Code CLI, ou l'application de bureau Claude version 1.13576.0 ou ultérieure. Les sessions [Claude Tag](https://claude.com/docs/claude-tag/overview) peuvent également publier des artefacts lorsque Claude Tag et les artefacts sont activés pour l'organisation. Désactivé par défaut dans les contextes [Agent SDK](/docs/fr/agent-sdk/overview), GitHub Action et MCP-server, et lorsque [`CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`](/docs/fr/env-vars) est défini. |

La disponibilité des artefacts pour votre organisation provient de la politique de votre organisation, que Claude Code charge depuis `api.anthropic.com`. Lorsque Claude Code ne peut pas charger la politique, les artefacts ne sont pas disponibles. Lorsque vous en demandez un, Claude explique pourquoi.

Si un proxy, un VPN ou un filtre web est impliqué, demandez à votre administrateur informatique de laisser passer `api.anthropic.com`. Claude Code continue à réessayer en arrière-plan, et les artefacts deviennent disponibles une fois que la politique est chargée et les autorise.

<h2 id="disable-artifacts">
  Désactiver les artefacts
</h2>

Pour désactiver les artefacts pour vos propres sessions indépendamment du paramètre de votre organisation, utilisez l'une des options suivantes :

| Méthode                                  | Paramètre                                                                                                                                          |
| :--------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`/config`](/docs/fr/commands)                | Désactivez la ligne **Artifacts**, ce qui écrit [`"enableArtifact": false`](/docs/fr/settings-reference#enableartifact) dans vos paramètres utilisateur |
| [Fichier de paramètres](/docs/fr/settings)    | Définissez `"enableArtifact": false`. Le paramètre déprécié `"disableArtifact": true` désactive également les artefacts                            |
| [Variable d'environnement](/docs/fr/env-vars) | Définissez `CLAUDE_CODE_DISABLE_ARTIFACT=1`                                                                                                        |
| [Règle de permission](/docs/fr/permissions)   | Ajoutez `Artifact` à `permissions.deny`                                                                                                            |

Une fois que vous désactivez les artefacts dans un fichier [`--settings`](/docs/fr/cli-reference#cli-flags) ou avec `CLAUDE_CODE_DISABLE_ARTIFACT`, ou que votre administrateur les désactive dans les [paramètres gérés](/docs/fr/server-managed-settings), aucun fichier de paramètres ne peut les réactiver. Avant la v2.1.242, un fichier plus haut dans la [pile de précédence](/docs/fr/settings#settings-precedence) pouvait réactiver les artefacts même lorsqu'un fichier de précédence inférieure définissait `"enableArtifact": false`.

Vous pouvez également définir `"enableArtifact": false` dans le fichier `.claude/settings.json` ou `.claude/settings.local.json` d'un projet pour désactiver les artefacts pour les sessions de ce projet. Un `"enableArtifact": true` dans l'un ou l'autre fichier ne les réactive pas. Le respect de la clé dans les paramètres du projet et locaux nécessite Claude Code v2.1.242 ou une version ultérieure.

Si vous ajoutez une règle de refus ou de demande `WebFetch` sans partie `domain:`, elle ne désactive pas les artefacts ni ne bloque les lectures d'artefacts. Une [règle `WebFetch(domain:claude.ai)` dans `deny` ou `ask` s'applique aux lectures d'artefacts](/docs/fr/permissions#allow-or-deny-every-fetch).

<h2 id="manage-artifacts-for-your-organization">
  Gérer les artefacts pour votre organisation
</h2>

Les propriétaires sur les plans Team et Enterprise contrôlent les artefacts à partir des [paramètres d'administration de claude.ai](https://claude.ai/admin-settings/claude-code). Le contenu des artefacts est stocké sur l'infrastructure exploitée par Anthropic et n'est visible que pour les membres authentifiés de l'organisation de publication, sauf si l'artefact est [partagé publiquement](#control-public-sharing).

<h3 id="enable-or-disable-artifacts">
  Activer ou désactiver les artefacts
</h3>

Pour activer ou désactiver les artefacts pour l'ensemble de l'organisation, allez à [**Paramètres > Claude Code > Capacités**](https://claude.ai/admin-settings/claude-code) et utilisez le bouton bascule **Artefacts**. Sur les plans Enterprise avec contrôle d'accès basé sur les rôles, vous pouvez également limiter les artefacts à des rôles spécifiques : allez à [**Paramètres > Rôles**](https://claude.ai/admin-settings/roles), modifiez un rôle et définissez la permission **Artefacts** sous le groupe **Claude Code**.

<h3 id="control-connector-calls-from-artifacts">
  Contrôler les appels de connecteur à partir des artefacts
</h3>

Les [appels de connecteur à partir des artefacts](#pull-live-data-with-mcp-connectors) disposent de leur propre bouton bascule, distinct du bouton bascule **Artefacts** qui active ou désactive les artefacts. Allez à [**Paramètres > Capacités**](https://claude.ai/admin-settings/capabilities) et utilisez le bouton bascule **Activer les connecteurs d'artefacts**. Le même bouton bascule régit les appels de connecteur à partir des artefacts créés dans les conversations de claude.ai, c'est pourquoi il se trouve sous **Paramètres > Capacités** plutôt que sous **Paramètres > Claude Code**.

<h3 id="control-public-sharing">
  Contrôler le partage public
</h3>

Le partage public est désactivé par défaut sur les plans Team et Enterprise, de sorte que les membres ne peuvent partager les artefacts que dans l'organisation jusqu'à ce qu'un propriétaire l'active. Pour permettre aux membres de publier des artefacts vers des liens publics que n'importe qui peut consulter sans se connecter, allez à **Paramètres > Claude Code > Capacités** et activez **Partage externe** sous le bouton bascule **Artefacts**. Le désactiver à nouveau bloque l'accès via les liens publics existants sans modifier l'audience de chaque artefact ; l'accès reprend si vous le réactivez.

<h3 id="set-a-retention-policy">
  Définir une politique de rétention
</h3>

Pour définir la durée pendant laquelle les artefacts sont conservés avant suppression automatique, allez à [**Paramètres > Contrôles de données et de confidentialité**](https://claude.ai/admin-settings/data-privacy-controls). Vous pouvez définir des périodes de rétention distinctes pour les artefacts qui sont encore privés pour leur auteur et les artefacts qui ont été partagés.

<h3 id="review-the-audit-log">
  Examiner le journal d'audit
</h3>

La publication, le partage et la suppression d'un artefact apparaissent chacun dans le journal d'audit de votre organisation sous les types d'événements `claude_artifact_*`, la même famille utilisée pour les artefacts créés dans les conversations de claude.ai.

<h3 id="allowlist-the-viewer-domain">
  Ajouter le domaine du visualiseur à la liste blanche
</h3>

Le visualiseur sur claude.ai charge chaque artefact à partir d'une origine `*.claudeusercontent.com` en bac à sable. Si votre organisation restreint l'accès réseau sortant, ajoutez ce domaine à votre liste blanche à côté de `claude.ai`. Consultez [Exigences d'accès réseau](/docs/fr/network-config#network-access-requirements) pour la liste complète.

Un artefact qui charge une police de caractères à partir de [Google Fonts](#improve-the-visual-design) demande également `fonts.googleapis.com` et `fonts.gstatic.com`. Les deux hôtes sont facultatifs. Si vous les bloquez, les artefacts s'affichent dans les polices de secours. Bloquez avec un rejet rapide plutôt qu'une suppression silencieuse afin que la demande de police échoue immédiatement au lieu de retarder le premier rendu de la page.

Les artefacts peuvent également charger des bibliothèques JavaScript, telles que React ou un package de graphiques, à partir de `cdnjs.cloudflare.com`, `cdn.jsdelivr.net`, `cdn.tailwindcss.com`, `code.jquery.com` et `unpkg.com`, et d'aucun autre hôte externe. Si vous bloquez ces hôtes, les parties d'un artefact qui dépendent d'une bibliothèque ne fonctionnent pas, et contrairement à une police bloquée, une bibliothèque bloquée n'a pas de secours. Bloquez avec un rejet rapide ici aussi, afin qu'une demande de bibliothèque bloquée échoue immédiatement plutôt que de rester en attente jusqu'à l'expiration du délai.

<h3 id="list-and-delete-artifacts-with-the-compliance-api">
  Lister et supprimer les artefacts avec l'API de conformité
</h3>

L'[API de conformité](https://docs.claude.com/en/api/compliance) fournit des points de terminaison pour lister les artefacts d'une organisation, récupérer le contenu d'une version spécifique et supprimer un artefact :

| Méthode  | Point de terminaison                                                |
| :------- | :------------------------------------------------------------------ |
| `GET`    | `/v1/compliance/code/artifacts`                                     |
| `GET`    | `/v1/compliance/code/artifacts/{artifact_id}/versions/{version_id}` |
| `DELETE` | `/v1/compliance/code/artifacts/{artifact_id}`                       |

Pour les schémas de demande et de réponse, consultez la [référence de l'API de conformité](https://docs.claude.com/en/api/compliance/code/artifacts).

<h2 id="related-resources">
  Ressources connexes
</h2>

* Parcourez les [modèles d'invite et flux de travail](/docs/fr/prompt-library) qui s'associent aux artefacts
* Transformez une invite d'artefact que vous réutilisez en [compétence](/docs/fr/skills) afin de pouvoir l'invoquer en tant que commande
* [Connectez les serveurs MCP](/docs/fr/mcp) afin que Claude puisse extraire les données en direct dans un artefact pendant qu'il crée la page
