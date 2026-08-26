> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Suivez, rembobinez et résumez les modifications et la conversation de Claude pour gérer l'état de la session.

Claude Code suit automatiquement les modifications de fichiers effectuées par Claude au fur et à mesure que vous travaillez, ce qui vous permet d'annuler rapidement les modifications et de revenir à des états antérieurs si quelque chose s'écarte de la trajectoire.

<h2 id="how-checkpoints-work">
  Comment fonctionne le checkpointing
</h2>

Au fur et à mesure que vous travaillez avec Claude, le checkpointing capture automatiquement l'état de votre code avant chaque invite utilisateur. Ce filet de sécurité vous permet de poursuivre des tâches ambitieuses et à grande échelle en sachant que vous pouvez toujours revenir à un état de code antérieur.

<h3 id="automatic-tracking">
  Suivi automatique
</h3>

Claude Code suit toutes les modifications apportées par ses outils d'édition de fichiers :

* Chaque invite utilisateur crée un nouveau checkpoint
* Claude Code conserve des snapshots de fichiers pour les 100 checkpoints les plus récents dans une session. L'abandon d'un checkpoint plus ancien supprime les fichiers snapshot que nul autre checkpoint ne référence, sauf le premier snapshot de chaque fichier, que l'extension VS Code utilise comme référence pour les diffs de session. Avant v2.1.208, ces fichiers snapshot remplacés restaient sur le disque jusqu'au nettoyage de la session.
* Les checkpoints sont enregistrés avec la conversation, donc une session reprise peut toujours `/rewind` vers eux
* Nettoyés automatiquement avec les sessions après 30 jours (configurable)

<h3 id="rewind-and-summarize">
  Rembobiner et résumer
</h3>

Exécutez `/rewind`, ou appuyez sur `Esc` deux fois lorsque le champ de saisie d'invite est vide, pour ouvrir le menu de rembobinage.

<Note>
  Si le champ de saisie d'invite contient du texte, double `Esc` l'efface à la place d'ouvrir le menu. Le texte effacé est enregistré dans votre historique de saisie, appuyez donc sur `Haut` pour le rappeler après avoir terminé dans le menu de rembobinage.
</Note>

Le menu de rembobinage répertorie chaque invite que vous avez envoyée pendant la session. Sélectionnez le point sur lequel vous souhaitez agir, puis choisissez une action :

* **Restaurer le code et la conversation** : revenir au code et à la conversation à ce moment
* **Restaurer la conversation** : rembobiner jusqu'à ce message tout en conservant le code actuel
* **Restaurer le code** : annuler les modifications de fichiers tout en conservant la conversation
* **Résumer à partir d'ici** : compresser la conversation à partir de ce moment en avant dans un résumé, libérant de l'espace de context window
* **Résumer jusqu'à ici** : compresser la conversation avant ce moment dans un résumé, en conservant les messages ultérieurs intacts
* **Annuler** : revenir à la liste des messages sans apporter de modifications

Après la restauration de la conversation ou le choix de Résumer à partir d'ici, l'invite originale du message sélectionné est restaurée dans le champ de saisie afin que vous puissiez la renvoyer ou la modifier.

Le choix de Résumer jusqu'à ici vous laisse à la fin de la conversation avec le champ de saisie vide.

<h4 id="rewind-past-a-cleared-conversation">
  Rembobiner au-delà d'une conversation effacée
</h4>

Si vous avez exécuté `/clear` plus tôt dans le même processus Claude Code, le menu de rembobinage affiche une entrée supplémentaire en haut de la liste intitulée `/resume <session-id> (previous session)`. Sélectionnez-la pour reprendre la conversation qui était active avant l'exécution de `/clear`. L'entrée est disponible jusqu'à ce que vous quittiez Claude Code ou repreniez une session différente, et nécessite Claude Code v2.1.191 ou version ultérieure. Sur les versions antérieures, exécutez `/resume` et choisissez la session précédente dans la liste à la place.

<h4 id="restore-vs-summarize">
  Restaurer vs. résumer
</h4>

Les options de restauration annulent l'état : elles annulent les modifications de code, l'historique de conversation, ou les deux. Les options de résumé compriment une partie de la conversation dans un résumé généré par l'IA sans modifier les fichiers sur le disque :

* **Résumer à partir d'ici** : les messages avant le message sélectionné restent intacts. Le message sélectionné et tout ce qui suit sont remplacés par un résumé. Utilisez ceci pour abandonner une discussion secondaire tout en conservant le contexte initial en détail complet.
* **Résumer jusqu'à ici** : les messages avant le message sélectionné sont remplacés par un résumé. Le message sélectionné et tout ce qui suit restent intacts, et vous restez à la fin de la conversation. Utilisez ceci pour compresser la discussion de configuration initiale tout en conservant le travail récent en détail complet.

Dans les deux cas, les messages originaux sont conservés dans la transcription de session, afin que Claude puisse référencer les détails si nécessaire. Vous pouvez taper des instructions optionnelles pour guider sur quoi le résumé se concentre. C'est similaire à `/compact`, mais ciblé : au lieu de résumer l'ensemble de la conversation, vous choisissez quel côté du message sélectionné compresser.

<Note>
  Résumer vous garde dans la même session et compresse le contexte. Si vous souhaitez vous brancher et essayer une approche différente tout en préservant la session originale intacte, utilisez plutôt [fork](/docs/fr/sessions#branch-a-session) (`claude --continue --fork-session`).
</Note>

<h2 id="common-use-cases">
  Cas d'usage courants
</h2>

Les checkpoints sont particulièrement utiles quand :

* **Explorer les alternatives** : essayez différentes approches d'implémentation sans perdre votre point de départ
* **Récupérer des erreurs** : annulez rapidement les modifications qui ont introduit des bugs ou cassé des fonctionnalités
* **Itérer sur les fonctionnalités** : expérimentez des variations en sachant que vous pouvez revenir à des états fonctionnels
* **Libérer de l'espace de contexte** : résumez une session de débogage verbeuse à partir du point médian en avant, en conservant vos instructions initiales intactes

<h2 id="limitations">
  Limitations
</h2>

<h3 id="bash-command-changes-not-tracked">
  Les modifications de commandes Bash ne sont pas suivies
</h3>

Le checkpointing ne suit pas les fichiers modifiés par les commandes bash. Par exemple, si Claude Code exécute :

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

Ces modifications de fichiers ne peuvent pas être annulées via le rembobinage. Seules les modifications de fichiers directs effectuées via les outils d'édition de fichiers de Claude sont suivies.

<h3 id="external-changes-not-tracked">
  Les modifications externes ne sont pas suivies
</h3>

Le checkpointing suit uniquement les fichiers qui ont été modifiés au cours de la session actuelle. Les modifications manuelles que vous apportez aux fichiers en dehors de Claude Code et les modifications d'autres sessions concurrentes ne sont normalement pas capturées, sauf si elles modifient par hasard les mêmes fichiers que la session actuelle.

<h3 id="not-a-replacement-for-version-control">
  Pas un remplacement du contrôle de version
</h3>

Les checkpoints sont conçus pour une récupération rapide au niveau de la session. Pour un historique de version permanent et la collaboration :

* Continuez à utiliser le contrôle de version (ex. Git) pour les commits, les branches et l'historique à long terme
* Les checkpoints complètent mais ne remplacent pas le contrôle de version approprié
* Pensez aux checkpoints comme « annulation locale » et à Git comme « historique permanent »

<h2 id="see-also">
  Voir aussi
</h2>

* [Mode interactif](/docs/fr/interactive-mode) - Raccourcis clavier et contrôles de session
* [Commandes](/docs/fr/commands) - Accès aux checkpoints en utilisant `/rewind`
* [Référence CLI](/docs/fr/cli-reference) - Options de ligne de commande
