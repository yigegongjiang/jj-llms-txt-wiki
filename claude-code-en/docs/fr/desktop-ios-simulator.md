> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Tester les applications iOS dans le simulateur

> Claude Code Desktop ouvre votre application dans le volet Simulateur iOS lorsque Claude la crée, l'exécute ou la vérifie, avec un simulateur distinct pour chaque session.

<Note>
  Le volet Simulateur iOS est en bêta publique dans Claude Code Desktop sur macOS. Il est disponible sur les plans Pro, Max, Team et Enterprise, sauf dans les organisations Enterprise qui ont une configuration HIPAA activée.
</Note>

Le volet Simulateur iOS affiche votre application en cours d'exécution dans le Simulateur iOS d'Apple à côté de votre conversation dans Claude Code Desktop. Lorsque Claude crée, installe, lance ou vérifie votre application dans un simulateur, le volet s'ouvre automatiquement et diffuse l'écran de l'appareil en direct. Utilisez-le pour regarder Claude exécuter et tester votre application, ou pour parcourir l'application vous-même en appuyant sur les éléments tandis que Claude continue à travailler.

Le volet du simulateur pilote le simulateur directement, il n'a donc pas besoin de [l'utilisation de l'ordinateur](/docs/fr/desktop#let-claude-use-your-computer) et ne prend jamais le contrôle de votre écran ni ne masque vos autres fenêtres. Depuis la CLI, Claude accède au Simulateur iOS via [l'utilisation de l'ordinateur](/docs/fr/computer-use#test-a-simulator-flow), qui contrôle le simulateur sur votre écran de la même manière que vous le feriez avec une souris.

<h2 id="requirements">
  Conditions requises
</h2>

Le volet du simulateur utilise les outils de simulateur d'Apple, que l'application de bureau n'inclut pas. Avant de commencer une session, assurez-vous que vous disposez de :

* Claude Desktop v1.24012.0 ou version ultérieure
* Un Mac, car le Simulateur iOS d'Apple ne fonctionne que sur macOS
* [Xcode](https://developer.apple.com/xcode/) avec la plateforme iOS installée, qui fournit les appareils simulateurs. Si Xcode ne liste pas encore de simulateurs, consultez [Le volet du simulateur indique qu'aucun simulateur n'a été trouvé](#the-simulator-pane-says-no-simulators-were-found)
  * Utilisez Xcode 26.x. Le volet ne fonctionne pas encore avec Xcode 27, qui remplace l'application Simulator par Device Hub. Si `xcode-select` pointe vers Xcode 27 sur votre Mac, consultez [Le volet du simulateur échoue avec Xcode 27](#the-simulator-pane-fails-with-xcode-27)

<Note>
  Sur cette page, « appareil » fait référence à un iPhone ou iPad simulé, l'un des mêmes appareils simulateurs que vous gérez dans Xcode sous **Window → Devices and Simulators**, et non à du matériel physique.
</Note>

Le volet du simulateur est disponible dans les sessions locales uniquement. Dans les sessions [cloud](/docs/fr/desktop#run-long-running-tasks-in-the-cloud) et [SSH](/docs/fr/desktop#ssh-sessions), Claude s'exécute sur une machine qui ne peut pas accéder aux simulateurs sur votre Mac.

<h2 id="run-your-app-in-the-simulator">
  Exécuter votre application dans le simulateur
</h2>

Vous n'avez pas besoin d'une commande ou d'un paramètre pour ouvrir le volet du simulateur. Claude l'ouvre lorsqu'il exécute votre application dans un simulateur.

<Steps>
  <Step title="Ouvrir votre projet iOS">
    Dans Claude Code Desktop, ouvrez l'onglet **Code** et démarrez une session avec le dossier de projet de votre application comme [dossier de projet](/docs/fr/desktop#start-a-session). Tout projet qui crée une application pour le Simulateur iOS fonctionne.
  </Step>

  <Step title="Demander à Claude d'exécuter ou de tester l'application">
    Formulez la tâche autour de l'exécution ou de la vérification de l'application. Par exemple :

    ```text theme={null}
    Build the app and run it in the simulator to check the onboarding flow.
    ```
  </Step>

  <Step title="Regarder l'application dans le volet du simulateur">
    Lorsque l'application se lance dans un simulateur, le volet Simulateur iOS s'ouvre à côté de la conversation. La première fois que Claude utilise un appareil, l'application de bureau vous demande de l'autoriser ; consultez [Accorder à Claude l'accès à un appareil](#grant-claude-access-to-a-device). Claude installe l'application, la parcourt en appuyant sur les éléments, et lit l'écran pour vérifier ses propres modifications pendant que vous regardez.
  </Step>
</Steps>

Le volet du simulateur s'ouvre chaque fois que Claude lance l'application dans un simulateur, à tout moment de la session. Lorsque votre demande concerne la visualisation de l'application, par exemple « le nouvel écran a-t-il l'air correct ? », Claude démarre un simulateur avant de commencer le travail. Après que Claude ait corrigé un bogue ou modifié un écran, demandez-lui de vérifier la modification : relancer l'application rouvre le volet s'il n'est pas ouvert.

Le volet du simulateur affiche l'appareil dans lequel l'application a réellement été lancée. Pour tester sur un appareil spécifique, nommez-le dans votre demande, par exemple « exécutez-le sur le simulateur iPhone SE », et Claude cible cet appareil lorsqu'il crée et lance l'application.

Un appareil que Claude démarre apparaît également dans l'application Simulator d'Apple, et Claude peut installer l'application sur un appareil que vous avez déjà démarré.

Vous pouvez également ouvrir le volet du simulateur vous-même. Une fois que la session a un simulateur attaché ou a modifié des fichiers Swift, le menu **Views** dans la barre d'outils de la session affiche une entrée **iOS Simulator**. Si le volet n'affiche pas encore d'appareil, cliquez sur **Attach simulator**, ou choisissez un appareil spécifique dans le menu des appareils à côté ; choisir un appareil arrêté le démarre. Si Xcode ou ses simulateurs manquent, le volet affiche les étapes de configuration à la place et les coche au fur et à mesure que vous les complétez.

<h2 id="control-the-simulator-yourself">
  Contrôler le simulateur vous-même
</h2>

Le volet du simulateur est interactif, pas seulement un visualiseur. Pendant que Claude travaille, ou entre les tâches, vous pouvez :

* Appuyer et faire glisser en cliquant et en faisant glisser sur l'écran de l'appareil
* Appuyer sur les boutons matériels avec les mêmes raccourcis que l'application Simulator d'Apple : **Cmd+Shift+H** pour Accueil, **Cmd+L** pour verrouiller, **Cmd+Flèche vers le haut** et **Cmd+Flèche vers le bas** pour le volume
* Faire pivoter l'appareil d'un quart de tour dans le sens des aiguilles d'une montre avec le bouton de rotation ou **Cmd+Flèche vers la droite**
* Changer l'appareil que le volet affiche à partir du menu des appareils, qui liste la version du système d'exploitation de chaque simulateur et s'il est démarré
* Enregistrer une capture d'écran avec **Cmd+S** ou un enregistrement d'écran avec **Cmd+R**, en utilisant les boutons de capture du volet ou les raccourcis ; les fichiers sont enregistrés sur votre Bureau
* Arrêter la diffusion en continu d'un appareil sans l'arrêter en cliquant sur **Detach simulator**, ce qui ramène le volet à son état **Attach simulator**

La ligne sous le nom de l'appareil règle le flux vidéo du simulateur. Réduisez la **Frame rate** ou la **Resolution** si le volet surcharge votre Mac, basculez **Encoding** entre H.264 et JPEG, ou cochez **FPS** pour afficher la fréquence d'images que le volet reçoit. Ces paramètres modifient la façon dont le volet affiche l'appareil, pas la façon dont l'application s'exécute.

Vous et Claude pilotez le même appareil, donc vos appuis modifient l'état de l'application que Claude voit. Pour que Claude vérifie un écran spécifique, accédez-y en appuyant, puis demandez. Pendant que Claude pilote l'appareil, le volet affiche un badge **Claude is using this device** au-dessus de l'écran ; attendez avant d'appuyer jusqu'à ce que le badge disparaisse, afin que le résultat reflète l'application plutôt que votre entrée.

<h2 id="how-sessions-manage-devices">
  Comment les sessions gèrent les appareils
</h2>

Chaque appareil appartient à la session qui l'a lancé, donc les [sessions parallèles](/docs/fr/desktop#work-in-parallel-with-sessions) ne partagent pas un appareil : ce que vous voyez dans le volet d'une session reflète le travail de cette session, pas celui d'une autre. Changer de session dans la barre latérale change la vue du simulateur avec la conversation, et revenir reprend le même appareil où il s'était arrêté. Si Claude travaille avec plus d'un appareil, chacun ouvre son propre volet, jusqu'à 4 par session.

Claude Code Desktop arrête les simulateurs qu'il a démarrés une fois qu'ils ne sont plus utilisés : lorsque vous quittez l'application, lorsque vous archivez la session, ou 10 minutes après avoir détaché un appareil de son volet. Les appareils que vous démarrez vous-même, que ce soit à partir du volet ou dans l'application Simulator d'Apple, ne sont jamais arrêtés automatiquement. Pour arrêter l'appareil attaché immédiatement, utilisez le bouton d'arrêt dans le volet.

<h2 id="grant-claude-access-to-a-device">
  Accorder à Claude l'accès à un appareil
</h2>

Claude demande votre consentement avant de contrôler un appareil, tandis que la création de l'application ou l'ouverture d'une URL sur celui-ci suit le mode de permission de votre session. Vous ou votre organisation pouvez également désactiver complètement l'accès de Claude.

<h3 id="allow-a-device-the-first-time">
  Autoriser un appareil la première fois
</h3>

La première fois que Claude utilise un simulateur, l'application de bureau vous demande de l'autoriser. Le consentement couvre le contrôle de cet appareil et la prise de captures d'écran de celui-ci, et vous le donnez une fois par appareil plutôt qu'une fois par session. Les captures d'écran de Claude de l'appareil sont envoyées à Anthropic et conservées selon vos paramètres normaux de rétention des conversations, donc ne vous connectez pas à des comptes réels sur un appareil que Claude utilise.

Après avoir autorisé un appareil, les actions de Claude sur celui-ci, telles que l'appui, la saisie, le lancement de l'application et la prise de captures d'écran, s'exécutent sans autres invites. Elles ont la même confiance que vous cliquant dans le volet, et elles ne touchent que l'appareil simulé, donc le volet n'a pas besoin des permissions macOS Accessibility et Screen Recording que l'utilisation de l'ordinateur nécessite.

Si vous refusez, l'appareil démarre toujours et le volet fonctionne toujours pour vos propres appuis ; seul l'accès de Claude reste désactivé. Pour changer d'avis plus tard, cliquez sur **Let Claude use it** dans le volet.

<h3 id="actions-that-follow-your-permission-mode">
  Actions qui suivent votre mode de permission
</h3>

Deux actions suivent le [mode de permission](/docs/fr/permissions#permission-modes) de votre session au lieu du consentement unique :

* Ouvrir une URL sur l'appareil, par exemple pour tester un lien profond ou charger une page dans Safari de l'appareil, car une URL peut transporter des données hors de l'appareil.
* Créer l'application, car `xcodebuild` exécute les scripts de construction de votre projet sur votre Mac. Vérifier une construction déjà en cours ne demande pas.

<h3 id="turn-off-simulator-access">
  Désactiver l'accès au simulateur
</h3>

Vous pouvez désactiver l'accès au simulateur de Claude dans les paramètres de l'application de bureau. Les organisations ont deux façons de le désactiver pour tout le monde :

* Le paramètre géré `disableMobileSimulatorTools` [managed setting](/docs/fr/desktop#managed-settings) bloque les outils de simulateur de Claude. Le volet du simulateur reste utilisable pour vos propres appuis, et le paramètre ne peut pas être remplacé depuis l'application.
* La clé de politique `requireCoworkFullVmSandbox`, qui exécute les outils de Claude à l'intérieur d'une machine virtuelle isolée au lieu de sur votre Mac, désactive le volet du simulateur et les outils de simulateur de Claude entièrement, donc le volet ne peut pas attacher un appareil pendant qu'il est défini.

Claude vous indique quand l'un ou l'autre s'applique.

<h2 id="limitations">
  Limitations
</h2>

Claude pilote uniquement les appareils simulés et ne peut pas contrôler un iPhone ou iPad physique. Pour tester sur un, exécutez l'application sur celui-ci à partir de Xcode vous-même, puis décrivez ce que vous voyez ou joignez une capture d'écran à la conversation pour que Claude travaille à partir de celle-ci.

<h2 id="troubleshooting">
  Dépannage
</h2>

<h3 id="the-simulator-pane-doesn’t-open-when-claude-runs-the-app">
  Le volet du simulateur ne s'ouvre pas lorsque Claude exécute l'application
</h3>

Claude n'a peut-être pas reconnu que vous vouliez exécuter ou tester l'application, ou les outils de simulateur peuvent être manquants. Vérifiez les points suivants :

* Énoncez l'objectif explicitement, par exemple « exécutez l'application dans le Simulateur iOS et parcourez le flux d'inscription ».
* Confirmez que Xcode et les simulateurs iOS sont installés et que votre version de Xcode répond aux [conditions requises](#requirements).
* Si votre organisation gère Claude Code, les [outils de simulateur peuvent être désactivés par la politique](#turn-off-simulator-access).
* Si vous êtes dans une organisation Enterprise qui a une configuration HIPAA activée, le volet du simulateur n'est pas disponible pour vous.
* Le volet du simulateur nécessite Claude Desktop v1.24012.0 ou version ultérieure. Ouvrez **Claude → Check for Updates**, puis redémarrez l'application.

<h3 id="the-simulator-pane-says-no-simulators-were-found">
  Le volet du simulateur indique qu'aucun simulateur n'a été trouvé
</h3>

Si `xcode-select` pointe vers Xcode 27, le volet peut signaler qu'aucun simulateur n'a été trouvé même si des appareils existent ; consultez [Le volet du simulateur échoue avec Xcode 27](#the-simulator-pane-fails-with-xcode-27). Sinon, Xcode est installé mais n'a pas de simulateurs iOS à lister. Le volet du simulateur affiche les étapes de configuration à suivre et les coche au fur et à mesure que chacune se termine. Pour installer la pièce manquante manuellement, téléchargez le runtime du simulateur iOS à partir des paramètres de Xcode, ou exécutez `xcodebuild -downloadPlatform iOS`.

<h3 id="the-simulator-pane-fails-with-xcode-27">
  Le volet du simulateur échoue avec Xcode 27
</h3>

Le volet ne fonctionne pas encore avec Xcode 27, qui remplace l'application Simulator par Device Hub. Avec Xcode 27 sélectionné, l'attachement d'un appareil échoue, ou le volet signale qu'aucun simulateur n'a été trouvé même si des appareils existent.

Le volet utilise le Xcode vers lequel `xcode-select` pointe. Si Xcode 27 est votre seule installation, installez d'abord Xcode 26.x à côté. Ensuite, sélectionnez l'installation 26.x par son chemin. Par exemple, s'il est installé en tant que `/Applications/Xcode-26.4.app` :

```bash theme={null}
sudo xcode-select -s /Applications/Xcode-26.4.app
```

Exécutez `xcode-select -p` pour vérifier quelle installation est sélectionnée.

<h2 id="see-also">
  Voir aussi
</h2>

* [Utilisation de l'ordinateur dans Desktop](/docs/fr/desktop#let-claude-use-your-computer) : contrôle d'écran pour les applications sans volet dédié
* [Utilisation de l'ordinateur depuis la CLI](/docs/fr/computer-use) : comment la CLI accède au Simulateur iOS
* [Travailler en parallèle avec les sessions](/docs/fr/desktop#work-in-parallel-with-sessions) : comment les sessions isolent les modifications
* [Commencer avec Claude Code Desktop](/docs/fr/desktop-quickstart)
