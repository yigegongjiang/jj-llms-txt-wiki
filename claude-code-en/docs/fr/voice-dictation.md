> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Dictée vocale

> Parlez vos invites dans l'interface de ligne de commande Claude Code avec la dictée vocale en maintenant ou en appuyant.

Parlez vos invites au lieu de les taper dans l'interface de ligne de commande Claude Code. Votre parole est transcrite en direct dans l'entrée d'invite, vous pouvez donc mélanger la voix et la saisie dans le même message. Activez la dictée avec `/voice`, puis maintenez une touche enfoncée pendant que vous parlez ou appuyez une fois pour commencer et à nouveau pour envoyer.

<Note>
  Le mode appui nécessite Claude Code v2.1.116 ou version ultérieure. Vérifiez votre version avec `claude --version`.
</Note>

La dictée fonctionne également dans la [vue agent](/docs/fr/agent-view#peek-and-reply). Maintenez ou appuyez sur votre touche push-to-talk pendant que l'entrée de dispatch ou une réponse du panneau de peek est active pour dicter à une session en arrière-plan.

<h2 id="requirements">
  Conditions requises
</h2>

La dictée vocale envoie votre audio enregistré aux serveurs d'Anthropic pour la transcription. L'audio n'est pas traité localement. Elle nécessite tous les éléments suivants :

* **Un compte Claude.ai** : le service de reconnaissance vocale n'est disponible que lorsque vous vous authentifiez avec un compte, et n'est pas disponible lorsque Claude Code est configuré pour utiliser directement une clé API Anthropic, Amazon Bedrock, Google Cloud's Agent Platform ou Microsoft Foundry.
* **Une organisation sans conformité HIPAA activée** : `/voice` affiche `Voice mode is disabled by your organization's policy` lorsque cette restriction s'applique.
* **Un microphone local** : la dictée vocale ne fonctionne pas dans les environnements distants tels que [Claude Code sur le web](/docs/fr/claude-code-on-the-web) ou les sessions SSH.
* **WSLg, si vous exécutez Claude Code dans WSL** : WSLg est inclus avec WSL2 lorsqu'il est installé à partir du Microsoft Store sur Windows 10 ou 11. Si WSLg n'est pas disponible, par exemple sur WSL1, exécutez Claude Code en Windows natif à la place.

La transcription ne consomme pas de messages Claude ni de jetons et ne compte pas vers les limites affichées dans `/usage`. Consultez [utilisation des données](/docs/fr/data-usage) pour savoir comment Anthropic traite vos données.

L'enregistrement audio utilise un module natif intégré sur macOS, Linux et Windows. Sur Linux, si le module natif ne peut pas se charger, Claude Code revient à `arecord` d'ALSA utils ou `rec` de SoX. Si aucun n'est disponible, `/voice` affiche une commande d'installation pour votre gestionnaire de paquets.

L'[extension VS Code](/docs/fr/vs-code) de Claude Code prend également en charge la dictée vocale avec la même exigence de compte Claude.ai. Elle n'est pas disponible dans les sessions VS Code Remote, y compris SSH, Dev Containers et Codespaces, car le microphone se trouve sur votre machine locale et l'extension s'exécute sur l'hôte distant.

<h2 id="enable-voice-dictation">
  Activer la dictée vocale
</h2>

Exécutez `/voice` pour activer la dictée. La première fois que vous l'activez, Claude Code exécute une vérification du microphone. Sur macOS, cela déclenche l'invite de permission du microphone système pour votre terminal s'il n'a jamais été accordé.

```
/voice
Voice mode enabled (hold). Hold space to record. Dictation language: en (/config to change).
```

`/voice` accepte un argument de mode optionnel :

| Commande      | Effet                                                  |
| :------------ | :----------------------------------------------------- |
| `/voice`      | Basculer activé ou désactivé, conserver le mode actuel |
| `/voice hold` | Activer en [mode maintien](#hold-to-record)            |
| `/voice tap`  | Activer en [mode appui](#tap-to-record-and-send)       |
| `/voice off`  | Désactiver                                             |

La dictée vocale persiste entre les sessions. Définissez-la directement dans votre [fichier de paramètres utilisateur](/docs/fr/settings) au lieu d'exécuter `/voice` :

```json theme={null}
{
  "voice": {
    "enabled": true,
    "mode": "tap"
  }
}
```

Lorsque la dictée vocale est activée, le pied de page d'entrée affiche un indice `hold space to speak` lorsque l'invite est vide. L'indice reflète votre liaison `voice:pushToTalk` actuelle et se met à jour si vous [reliez la clé de dictée](#rebind-the-dictation-key). Le texte d'indice est le même dans les deux modes, et il n'apparaît pas si vous avez une [ligne d'état personnalisée](/docs/fr/statusline) configurée.

La transcription est optimisée pour le vocabulaire de codage dans les deux modes. Les termes de développement courants comme `regex`, `OAuth`, `JSON` et `localhost` sont reconnus correctement, et le nom de votre projet actuel et le nom de la branche git sont ajoutés automatiquement comme indices de reconnaissance.

<h2 id="hold-to-record">
  Mode maintien
</h2>

Le mode maintien est un système push-to-talk : l'enregistrement s'exécute pendant que vous maintenez la touche enfoncée et s'arrête lorsque vous la relâchez. C'est le mode par défaut.

Maintenez `Space` enfoncée pour commencer l'enregistrement. Claude Code détecte une touche maintenue en observant les événements de répétition rapide des touches de votre terminal, il y a donc une brève période de préchauffage avant le début de l'enregistrement. Le pied de page affiche `keep holding…` pendant le préchauffage, puis bascule vers une forme d'onde en direct une fois l'enregistrement actif.

Les premiers caractères de répétition des touches tapent dans l'entrée pendant le préchauffage et sont supprimés automatiquement lorsque l'enregistrement s'active. Un simple appui sur `Space` tape toujours un espace, car la détection de maintien ne se déclenche que sur la répétition rapide.

<Tip>
  Pour ignorer le préchauffage, basculez vers [mode appui](#tap-to-record-and-send) avec `/voice tap`, ou [reliez à une combinaison de modificateurs](#rebind-the-dictation-key) comme `meta+k`. Les combinaisons de modificateurs commencent l'enregistrement au premier appui sur une touche.
</Tip>

Votre parole apparaît dans l'invite au fur et à mesure que vous parlez, estompée jusqu'à ce que la transcription soit finalisée. Relâchez `Space` pour arrêter l'enregistrement et finaliser le texte. La transcription est insérée à la position de votre curseur et le curseur reste à la fin du texte inséré, vous pouvez donc mélanger la saisie et la dictée dans n'importe quel ordre. Maintenez `Space` à nouveau pour ajouter un autre enregistrement, ou déplacez d'abord le curseur pour insérer la parole ailleurs dans l'invite :

```
> refactor the auth middleware to ▮
  # hold Space, speak "use the new token validation helper"
> refactor the auth middleware to use the new token validation helper▮
```

Par défaut, relâcher la touche insère la transcription et attend que vous appuyiez sur `Enter`. Définissez `"autoSubmit": true` dans l'objet de paramètres `voice` pour envoyer l'invite automatiquement lorsque vous relâchez la touche, tant que la transcription contient au moins trois mots.

<h2 id="tap-to-record-and-send">
  Mode appui pour enregistrer et envoyer
</h2>

Le mode appui bascule l'enregistrement avec un seul appui sur une touche : appuyez une fois pour commencer, parlez, puis appuyez à nouveau pour envoyer l'invite. Il n'y a pas de préchauffage, et vous n'avez pas besoin de maintenir la touche enfoncée.

Activez le mode appui avec `/voice tap`. Avec l'entrée d'invite vide, appuyez sur `Space` pour commencer l'enregistrement. Le pied de page affiche une forme d'onde en direct pendant l'enregistrement. Appuyez à nouveau sur `Space` pour arrêter.

Claude Code insère la transcription et soumet l'invite automatiquement lorsque la transcription contient au moins trois mots. Les transcriptions plus courtes sont insérées mais non soumises, donc un appui accidentel n'envoie pas un mot isolé.

Le seuil de trois mots compte les mots pour les langues écrites sans espaces. À partir de la v2.1.195, les transcriptions en japonais, chinois et thaï comptent les mots individuels, ils se soumettent donc automatiquement en mode appui et en mode maintien avec `autoSubmit`. Les versions antérieures comptaient une transcription sans espaces comme un seul mot et ne la soumettaient jamais automatiquement.

Le premier appui ne commence l'enregistrement que lorsque l'entrée d'invite est vide, vous pouvez donc toujours taper des espaces normalement en composant un message. Le deuxième appui arrête l'enregistrement quel que soit le contenu de l'entrée. L'enregistrement s'arrête également automatiquement après 15 secondes de silence ou deux minutes au total.

<h2 id="change-the-dictation-language">
  Modifier la langue de dictée
</h2>

La dictée vocale utilise le même [paramètre `language`](/docs/fr/settings) qui contrôle la langue de réponse de Claude. Si ce paramètre est vide, la dictée utilise par défaut l'anglais. Dans l'extension VS Code, si `language` est vide, la dictée utilise le paramètre `accessibility.voice.speechLanguage` de VS Code avant d'utiliser par défaut l'anglais.

<Accordion title="Langues de dictée prises en charge">
  | Langue      | Code |
  | :---------- | :--- |
  | Tchèque     | `cs` |
  | Danois      | `da` |
  | Néerlandais | `nl` |
  | Anglais     | `en` |
  | Français    | `fr` |
  | Allemand    | `de` |
  | Grec        | `el` |
  | Hindi       | `hi` |
  | Indonésien  | `id` |
  | Italien     | `it` |
  | Japonais    | `ja` |
  | Coréen      | `ko` |
  | Norvégien   | `no` |
  | Polonais    | `pl` |
  | Portugais   | `pt` |
  | Russe       | `ru` |
  | Espagnol    | `es` |
  | Suédois     | `sv` |
  | Turc        | `tr` |
  | Ukrainien   | `uk` |
</Accordion>

Définissez la langue dans `/config` ou directement dans les paramètres. Vous pouvez utiliser soit le [code de langue BCP 47](https://en.wikipedia.org/wiki/IETF_language_tag) soit le nom de la langue :

```json theme={null}
{
  "language": "japanese"
}
```

Si votre paramètre `language` ne figure pas dans la liste prise en charge, `/voice` vous avertit lors de l'activation et revient à l'anglais pour la dictée. Les réponses textuelles de Claude ne sont pas affectées par ce retour.

<h2 id="rebind-the-dictation-key">
  Relier à nouveau la touche de dictée
</h2>

La touche de dictée est liée à `voice:pushToTalk` dans le contexte `Chat` et utilise par défaut `Space`. La même liaison contrôle les modes maintien et appui. Reliez-la dans [`~/.claude/keybindings.json`](/docs/fr/keybindings) :

```json theme={null}
{
  "bindings": [
    {
      "context": "Chat",
      "bindings": {
        "meta+k": "voice:pushToTalk",
        "space": null
      }
    }
  ]
}
```

L'action `voice:pushToTalk` utilise une touche à la fois. Lorsque vous liez une touche personnalisée, elle remplace la liaison `Space` par défaut plutôt que d'ajouter un deuxième déclencheur, donc la ligne `"space": null` dans cet exemple est pour plus de clarté et peut être omise sans modifier le comportement.

En mode maintien, évitez de lier une touche de lettre simple comme `v` car la détection de maintien dépend de la répétition des touches et la lettre tape dans l'invite pendant le préchauffage. Utilisez `Space`, ou utilisez une combinaison de modificateurs comme `meta+k` pour commencer l'enregistrement au premier appui sur une touche sans préchauffage. Le mode appui n'a pas de préchauffage, donc la plupart des touches fonctionnent.

Certaines touches ne sont pas livrées aux applications de terminal et ne peuvent pas être liées du tout. Par exemple, `Caps Lock` affiche une erreur si vous essayez de la lier. Consultez [personnaliser les raccourcis clavier](/docs/fr/keybindings) pour la syntaxe complète des liaisons de touches et la liste des raccourcis réservés.

<h2 id="troubleshooting">
  Dépannage
</h2>

Problèmes courants lorsque la dictée vocale ne s'active pas ou n'enregistre pas :

* **`Voice mode requires a Claude.ai account`** : vous êtes authentifié avec une clé API ou un fournisseur tiers. Exécutez `/login` pour vous connecter avec un compte Claude.ai.
* **`Voice mode is disabled by your organization's policy`** : la configuration de conformité de votre organisation désactive la dictée vocale, comme décrit dans [Exigences](#requirements). Contactez l'administrateur de votre organisation pour confirmer si la dictée vocale est disponible pour votre organisation.
* **`Microphone access is denied`** : accordez la permission du microphone à votre terminal dans les paramètres système. Sur macOS, allez à Paramètres système → Confidentialité et sécurité → Microphone et activez votre application terminal, puis exécutez `/voice` à nouveau. Sur Windows, allez à Paramètres → Confidentialité et sécurité → Microphone et activez l'accès au microphone pour les applications de bureau, puis exécutez `/voice` à nouveau. Si votre terminal n'est pas répertorié dans les paramètres macOS, consultez [Terminal non répertorié dans les paramètres du microphone macOS](#terminal-not-listed-in-macos-microphone-settings).
* **`No audio recording tool found` sur Linux** : le module audio natif n'a pas pu se charger et aucun retour n'est installé. Installez SoX avec la commande affichée dans le message d'erreur, par exemple `sudo apt-get install sox`.
* **`Voice mode requires a microphone, but SoX could not open an audio capture device`** : SoX est installé, mais l'hôte n'a pas de périphérique de capture audio, par exemple un serveur sans interface graphique ou un conteneur. Exécutez Claude Code sur une machine avec un microphone. À partir de la v2.1.195, Claude Code sur Linux signale ce message dans cette situation ; les versions antérieures vous demandaient d'installer SoX même s'il était déjà installé.
* **`Voice mode could not find a working audio recorder in WSL`** : WSLg achemine l'audio via PulseAudio plutôt qu'un périphérique ALSA, donc SoX a besoin que son backend PulseAudio soit installé explicitement. Exécutez `sudo apt install sox libsox-fmt-pulse`. L'installation de `sox` seul installe le backend ALSA, qui ne peut pas enregistrer sur WSL car il n'y a pas de périphérique `/dev/snd`.
* **`Voice input is failing repeatedly and has been paused`** : la dictée vocale a rencontré plusieurs échecs de capture d'affilée et a cessé de tenter de nouvelles sessions jusqu'à ce qu'une réussisse. Un échec compte que le microphone ne démarre pas ou que l'enregistreur démarre puis s'arrête sans produire d'audio. Cela signifie généralement que le microphone ou la pile audio sur cet hôte ne peut pas capturer l'audio, par exemple un serveur sans interface graphique, un shell distant sans passage audio, ou une permission de microphone refusée. Confirmez un périphérique d'entrée fonctionnant, corrigez la cause sous-jacente à partir des entrées ci-dessus, puis déclenchez à nouveau la voix. Avant la v2.1.202, seuls les échecs de démarrage comptaient vers la pause.
* **Rien ne se passe en maintenant `Space` en mode maintien** : regardez l'entrée d'invite pendant que vous maintenez. Si les espaces continuent à s'accumuler, la dictée vocale est probablement désactivée ; exécutez `/voice hold` pour l'activer. Si un ou deux espaces seulement apparaissent puis rien, la dictée vocale est activée mais la détection de maintien ne se déclenche pas. La détection de maintien nécessite que votre terminal envoie des événements de répétition des touches, elle ne peut donc pas détecter une touche maintenue si la répétition des touches est désactivée au niveau du système d'exploitation. Basculez vers le mode appui avec `/voice tap` pour éviter l'exigence de répétition des touches.
* **Appuyer sur `Space` tape un espace au lieu d'enregistrer en mode appui** : le premier appui ne commence l'enregistrement que lorsque l'entrée d'invite est vide. Effacez d'abord l'entrée, ou vérifiez que vous êtes en mode appui en exécutant `/voice tap`.
* **`No audio detected from microphone`** : l'enregistrement a commencé mais a capturé le silence. Confirmez que le périphérique d'entrée correct est défini comme valeur par défaut du système et que son niveau d'entrée n'est pas coupé ou proche de zéro. Sur Windows, ouvrez Paramètres → Système → Son → Entrée et sélectionnez votre microphone. Sur macOS, ouvrez Paramètres système → Son → Entrée.
* **`Voice connection failed`** : votre enregistrement n'a jamais atteint le service de transcription car la connexion a échoué. Vérifiez votre réseau et réessayez. Un enregistrement qui ne capture aucun audio signale `No audio detected from microphone` au lieu de ce message. Avant la v2.1.200, un microphone silencieux pouvait signaler un échec de connexion, ce qui suggérait un problème réseau alors que le problème réel était le périphérique d'entrée.
* **`No speech detected`** : l'audio a atteint le service de transcription mais aucun mot n'a été reconnu. Parlez plus près du microphone, réduisez le bruit de fond et confirmez que votre [langue de dictée](#change-the-dictation-language) correspond à la langue que vous parlez.
* **La transcription est brouillée ou dans la mauvaise langue** : la dictée utilise par défaut l'anglais. Si vous dictez dans une autre langue, définissez-la d'abord dans `/config`. Consultez [Modifier la langue de dictée](#change-the-dictation-language).

<h3 id="terminal-not-listed-in-macos-microphone-settings">
  Terminal non répertorié dans les paramètres du microphone macOS
</h3>

Si votre application terminal n'apparaît pas sous Paramètres système → Confidentialité et sécurité → Microphone, il n'y a pas de bascule que vous pouvez activer. Réinitialisez l'état de permission pour votre terminal afin que la prochaine exécution de `/voice` déclenche une nouvelle invite de permission macOS.

<Steps>
  <Step title="Réinitialiser la permission du microphone pour votre terminal">
    Exécutez `tccutil reset Microphone <bundle-id>`, en remplaçant `<bundle-id>` par l'identifiant de votre terminal : `com.apple.Terminal` pour le Terminal intégré, ou `com.googlecode.iterm2` pour iTerm2. Pour les autres terminaux, recherchez l'identifiant avec `osascript -e 'id of app "AppName"'`.

    <Warning>
      Vous pouvez exécuter `tccutil reset Microphone` sans ID de bundle, mais cela révoque l'accès au microphone de chaque application sur votre Mac, y compris les applications comme Zoom ou Slack. Chaque application devra demander à nouveau l'accès à la prochaine utilisation, donc ne l'exécutez pas pendant un appel actif.
    </Warning>
  </Step>

  <Step title="Quitter et relancer votre terminal">
    macOS ne re-demandera pas un processus qui est déjà en cours d'exécution. Quittez l'application terminal avec Cmd+Q, pas seulement fermez ses fenêtres, puis ouvrez-la à nouveau.
  </Step>

  <Step title="Déclencher une nouvelle invite">
    Démarrez Claude Code et exécutez `/voice`. macOS demande l'accès au microphone ; autorisez-le.
  </Step>
</Steps>

<h2 id="see-also">
  Voir aussi
</h2>

* [Personnaliser les raccourcis clavier](/docs/fr/keybindings) : relier `voice:pushToTalk` et d'autres actions de clavier CLI
* [Configurer les paramètres](/docs/fr/settings) : référence complète pour les clés de paramètres `voice`, `language` et autres
* [Mode interactif](/docs/fr/interactive-mode) : raccourcis clavier, modes d'entrée et contrôles de session
* [Commandes](/docs/fr/commands) : référence pour `/voice`, `/config` et toutes les autres commandes
