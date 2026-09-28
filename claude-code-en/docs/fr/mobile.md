> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Claude Code sur mobile

> Démarrez, surveillez et pilotez les tâches Claude Code depuis votre téléphone avec l'application Claude pour iOS et Android.

L'application Claude pour [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) et [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude) est un client pour les sessions Claude Code plutôt qu'un endroit où le code s'exécute. Depuis votre téléphone, vous accédez à des [sessions cloud](#start-and-monitor-cloud-sessions) et des [projets](/docs/fr/claude-projects) dans le cloud, une session s'exécutant sur votre propre machine via [Remote Control](#continue-a-local-session-with-remote-control), ou l'application Desktop via [Dispatch](/docs/fr/desktop#sessions-from-dispatch).

<Note>
  Claude Code n'a pas d'application mobile distincte : les sessions cloud et Remote Control se trouvent tous deux dans l'onglet **Code** de l'application Claude, et Dispatch est une tâche à laquelle vous envoyez des messages dans l'application.
</Note>

<h2 id="get-the-app">
  Obtenir l'application
</h2>

<Steps>
  <Step title="Télécharger l'application Claude">
    Installez l'application Claude pour [iOS](https://apps.apple.com/us/app/claude-by-anthropic/id6473753684) ou [Android](https://play.google.com/store/apps/details?id=com.anthropic.claude). Sur un iPad, installez la même application iOS.

    <Tip>
      Exécutez `/mobile` dans une session Claude Code pour afficher un code QR pour [claude.ai/mobile](https://claude.ai/mobile), qui ouvre le bon app store pour votre téléphone. `/ios` et `/android` font la même chose.
    </Tip>
  </Step>

  <Step title="Se connecter">
    Connectez-vous avec le même compte claude.ai et la même organisation que vous utilisez pour Claude Code. Les sessions cloud et Remote Control nécessitent un compte claude.ai, ils ne sont donc pas accessibles avec une clé API Anthropic Console ou auprès d'un fournisseur tiers tel qu'Amazon Bedrock.
  </Step>

  <Step title="Ouvrir l'onglet Code">
    Appuyez sur **Code** dans la navigation de l'application pour accéder à vos sessions, ou ouvrez [claude.ai/code/new](https://claude.ai/code/new) sur votre téléphone pour démarrer une nouvelle session Code dans l'application. Si vous ne voyez pas l'onglet Code, votre plan ou votre organisation peut ne pas inclure ces fonctionnalités ; consultez [disponibilité par plan d'abonnement](/docs/fr/feature-availability#availability-by-subscription-plan).
  </Step>
</Steps>

<h2 id="work-from-your-phone">
  Travailler depuis votre téléphone
</h2>

Depuis l'application, vous pouvez démarrer des sessions cloud, ouvrir un projet, piloter une session Claude Code s'exécutant sur votre ordinateur, ou envoyer une tâche à Dispatch. L'application est la même pour chacun ; ils diffèrent par l'endroit où le travail se fait.

| Fonctionnalité                                 | Ce à quoi vous vous connectez                                                          | Quand l'utiliser                                                                                                                                                                                |
| :--------------------------------------------- | :------------------------------------------------------------------------------------- | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Cloud sessions](/docs/fr/claude-code-on-the-web)   | Une session sur l'infrastructure cloud, gérée par Anthropic par défaut                 | Votre référentiel est sur GitHub et la tâche doit continuer à s'exécuter après avoir rangé votre téléphone. Consultez le [guide de démarrage rapide cloud](/docs/fr/web-quickstart) pour configurer. |
| [Projects](/docs/fr/claude-projects)                | Une conversation où Claude coordonne des sessions cloud parallèles en tant que threads | Vous avez un flux de travail connexe plutôt qu'une seule tâche et vous voulez voir quels threads se sont terminés ou ont besoin de vous.                                                        |
| [Remote Control](/docs/fr/remote-control)           | Une session Claude Code s'exécutant sur votre ordinateur                               | Le travail a besoin de votre système de fichiers local, d'outils ou de serveurs MCP.                                                                                                            |
| [Dispatch](/docs/fr/desktop#sessions-from-dispatch) | L'application Desktop sur votre ordinateur                                             | Vous voulez envoyer une tâche et laisser Dispatch décider comment l'exécuter. Nécessite un plan Pro ou Max.                                                                                     |

Si votre ordinateur sera éteint, utilisez les sessions cloud ou un projet, qui s'exécutent dans le cloud et continuent avec votre ordinateur portable fermé. Remote Control et Dispatch pilotent votre propre machine, elle doit donc rester allumée avec Claude Code ou l'application Desktop en cours d'exécution. Si votre machine se met en veille pendant une session Remote Control, Claude Code se reconnecte quand la machine revient en ligne.

Pour une comparaison plus complète, consultez [travailler quand vous êtes loin de votre terminal](/docs/fr/platforms#work-when-you-are-away-from-your-terminal).

Les sessions cloud et Remote Control s'exécutent à partir de l'onglet **Code**. Pour Dispatch, que vous envoyez en tant que tâche dans l'application, consultez [sessions de Dispatch](/docs/fr/desktop#sessions-from-dispatch).

<h3 id="start-and-monitor-cloud-sessions">
  Démarrer et surveiller les sessions cloud
</h3>

Les sessions cloud exécutent les tâches sur l'infrastructure cloud, gérée par Anthropic par défaut, donc une session continue après avoir rangé votre téléphone. À partir de l'onglet Code, sélectionnez un référentiel et une branche, décrivez la tâche et soumettez-la. Les sessions persistent sur les appareils : une tâche que vous démarrez sur votre ordinateur portable est prête à être examinée depuis votre téléphone, et une que vous démarrez depuis votre téléphone vous attend quand vous êtes de retour à votre bureau.

Ouvrez une session dans l'application pour vérifier la progression, répondre aux questions de Claude ou la diriger dans une nouvelle direction. Vous pouvez également dire à Claude de [surveiller une demande de tirage](/docs/fr/claude-code-on-the-web#auto-fix-pull-requests) et corriger les défaillances CI ou les commentaires d'examen au fur et à mesure qu'ils arrivent. Pour connecter GitHub et configurer votre environnement, suivez le [guide de démarrage rapide cloud](/docs/fr/web-quickstart), et consultez [Utiliser Claude Code dans le cloud](/docs/fr/claude-code-on-the-web) pour tout ce que les sessions cloud peuvent faire.

<h3 id="continue-a-local-session-with-remote-control">
  Continuer une session locale avec Remote Control
</h3>

Remote Control connecte l'application Claude à une session Claude Code s'exécutant sur votre machine, de sorte que l'exécution du code et l'accès au système de fichiers restent locaux tandis que vous pilotez la session depuis votre téléphone. Démarrez la session sur votre ordinateur avec `claude remote-control`, ou exécutez `/remote-control` dans une session déjà ouverte. Ensuite, scannez le code QR que le terminal peut afficher, ou ouvrez l'application Claude, appuyez sur **Code**, et choisissez la session dans la liste. Consultez [se connecter depuis un autre appareil](/docs/fr/remote-control#connect-from-another-device) pour chaque option.

Quand vous ajoutez une pièce jointe dans l'application Claude, elle atteint également la session locale :

* **Photos** : Claude voit les photos jointes directement comme faisant partie de votre message. Claude Code enregistre également chaque photo sous `~/.claude/uploads/` et indique à Claude le chemin du fichier enregistré, de sorte que Claude peut copier l'image dans les fichiers qu'il crée.
* **Autres fichiers** : Claude Code les télécharge sur votre machine et les transmet à Claude en tant que références de fichier `@`.

Pour les exigences, les modes d'invocation et la résolution des problèmes, consultez l'[aperçu de Remote Control](/docs/fr/remote-control).

<h3 id="get-push-notifications">
  Obtenir des notifications push
</h3>

Quand Remote Control est actif, Claude peut envoyer des notifications push à votre téléphone, généralement quand une tâche longue se termine ou quand il a besoin d'une décision de votre part. Vous pouvez également en demander une dans votre invite, par exemple `notify me when the tests finish`. Consultez [notifications push mobiles](/docs/fr/remote-control#mobile-push-notifications) pour les deux bascules `/config` et la résolution des problèmes de livraison.

Dispatch envoie sa propre notification quand une session Code qu'il a créée se termine ou a besoin de votre approbation, décrite dans [sessions de Dispatch](/docs/fr/desktop#sessions-from-dispatch).

<h2 id="limitations">
  Limitations
</h2>

Le client mobile couvre la plupart de ce dont une session a besoin, avec quelques limitations :

* **Commandes locales uniquement** : les commandes qui s'exécutent uniquement dans l'interface du terminal, telles que `/plugin` et `/resume`, ne fonctionnent pas depuis l'application. Les [limitations de Remote Control](/docs/fr/remote-control#limitations) listent les commandes qui fonctionnent depuis mobile et comment leur comportement diffère.
* **Modes de permission** : les sessions cloud offrent Accept edits, Plan et Auto dans le menu déroulant du mode, et les sessions Remote Control offrent Manual, Accept edits et Plan. Vous ne pouvez pas sélectionner Bypass permissions depuis l'application dans les deux cas, et vous ne pouvez pas sélectionner Auto pour une session Remote Control. Consultez [changer les modes de permission](/docs/fr/permission-modes#switch-permission-modes).
* **Plans Dispatch** : Dispatch nécessite un plan Pro ou Max et n'est pas disponible sur Team ou Enterprise.

<h2 id="related-resources">
  Ressources connexes
</h2>

* [Plateformes et intégrations](/docs/fr/platforms) : comparez chaque surface sur laquelle Claude Code s'exécute
* [Claude Code sur le web](/docs/fr/claude-code-on-the-web) : comment les sessions cloud s'exécutent et comment déplacer le travail vers et depuis votre terminal
* [Configurer les environnements cloud](/docs/fr/cloud-environments) : niveaux d'accès réseau, variables d'environnement et scripts de configuration pour les sessions cloud
* [Remote Control](/docs/fr/remote-control) : continuer une session locale depuis n'importe quel appareil
* [Sessions de Dispatch](/docs/fr/desktop#sessions-from-dispatch) : comment les tâches Dispatch deviennent des sessions Code dans l'application Desktop
* [Channels](/docs/fr/channels) : posez une question à Claude depuis votre téléphone via Telegram, Discord ou iMessage tandis que le travail s'exécute sur votre machine
* [Claude Code dans Slack](/docs/fr/slack) : déléguez les tâches de codage depuis votre espace de travail Slack en mentionnant `@Claude`
