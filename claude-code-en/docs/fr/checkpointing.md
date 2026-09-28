> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Checkpointing

> Suivez, rembobinez et résumez les modifications et la conversation de Claude pour gérer l'état de la session.

Claude Code suit automatiquement les modifications de fichiers effectuées par Claude au fur et à mesure que vous travaillez, ce qui vous permet d'annuler rapidement les modifications et de revenir à des états antérieurs si quelque chose s'écarte de la trajectoire.

<h2 id="how-checkpoints-work">
  Comment fonctionne le checkpointing
</h2>

Au fur et à mesure que vous travaillez avec Claude, le checkpointing capture automatiquement l'état de votre code avant chaque invite que vous envoyez et qui démarre un tour.

<h3 id="automatic-tracking">
  Suivi automatique
</h3>

Claude Code suit toutes les modifications apportées par ses outils d'édition de fichiers :

* Chaque invite que vous envoyez et qui démarre un tour crée un nouveau checkpoint
* Claude Code conserve des snapshots de fichiers pour les 100 checkpoints les plus récents dans une session. L'abandon d'un checkpoint plus ancien supprime les fichiers snapshot que nul autre checkpoint ne référence, sauf le premier snapshot de chaque fichier, que l'extension VS Code utilise comme référence pour les diffs de session.
* Claude Code enregistre les checkpoints avec la conversation, donc vous pouvez toujours exécuter `/rewind` après avoir repris une session
* Claude Code supprime les snapshots de fichiers d'une session lors du [nettoyage de rétention](/docs/fr/claude-directory#cleaned-up-automatically), par défaut environ 30 jours après le dernier enregistrement de la session. Le rembobinage vers un checkpoint dont les snapshots ont disparu peut échouer avec [`No files were restored`](/docs/fr/errors#no-files-were-restored). Pour conserver les snapshots plus longtemps, définissez [`cleanupPeriodDays`](/docs/fr/settings-reference#cleanupperioddays).

<h3 id="rewind-and-summarize">
  Rembobiner et résumer
</h3>

Exécutez `/rewind`, ou appuyez sur `Esc` deux fois lorsque le champ de saisie d'invite est vide, pour ouvrir le menu de rembobinage.

<Note>
  Si le champ de saisie d'invite contient du texte, double `Esc` l'efface à la place d'ouvrir le menu. Le texte effacé est enregistré dans votre historique de saisie, appuyez donc sur `Haut` pour le rappeler après avoir terminé dans le menu de rembobinage.
</Note>

Le menu de rembobinage répertorie chaque invite que vous avez envoyée pendant la session, sauf les [messages qui ont rejoint un tour en cours](#messages-sent-mid-turn-not-checkpointed). Sélectionnez le point sur lequel vous souhaitez agir, puis choisissez une action :

* **Restaurer le code et la conversation** : revenir au code et à la conversation à ce moment
* **Restaurer la conversation** : rembobiner jusqu'à ce message tout en conservant le code actuel
* **Restaurer le code** : annuler les modifications de fichiers tout en conservant la conversation
* **Résumer à partir d'ici** : compresser la conversation à partir de ce moment en avant dans un résumé, libérant de l'espace de context window
* **Résumer jusqu'à ici** : compresser la conversation avant ce moment dans un résumé, en conservant les messages ultérieurs intacts
* **Annuler** : revenir à la liste des messages sans apporter de modifications

Les deux options de restauration de code n'apparaissent que lorsque le checkpoint sélectionné a des modifications de fichiers suivies à annuler. Si aucune modification de fichier n'a été capturée après ce point, le menu propose uniquement **Restaurer la conversation**, les options de résumé et **Annuler**.

Après la restauration de la conversation ou le choix de Résumer à partir d'ici, l'invite originale du message sélectionné est restaurée dans le champ de saisie afin que vous puissiez la renvoyer ou la modifier.

Le choix de Résumer jusqu'à ici vous laisse à la fin de la conversation avec le champ de saisie vide. Avec l'une ou l'autre option de résumé, un marqueur **Summarized conversation** apparaît dans la conversation où les messages compressés se trouvaient.

<h4 id="rewind-past-a-cleared-conversation">
  Rembobiner au-delà d'une conversation effacée
</h4>

Si vous avez exécuté `/clear` plus tôt dans le même processus Claude Code, le menu de rembobinage affiche une entrée supplémentaire en haut de la liste intitulée `/resume <session-id> (previous session)`. Sélectionnez-la pour reprendre la conversation qui était active avant l'exécution de `/clear`. L'entrée est disponible jusqu'à ce que vous quittiez Claude Code ou repreniez une session différente.

<h4 id="guide-a-summary">
  Guider un résumé
</h4>

Le résumé ne modifie pas les fichiers sur le disque, et les messages originaux restent dans la transcription de session, donc Claude peut toujours référencer les détails. Pour guider sur quoi le résumé se concentre, mettez en surbrillance une option **Résumer** avec les touches fléchées et tapez des instructions où la ligne indique **add context (optional)**, puis appuyez sur `Entrée`. La sélection de l'option avec sa touche numérique résume immédiatement sans instructions.

<Note>
  Résumer vous garde dans la même session et compresse le contexte, comme un `/compact` ciblé. Pour vous brancher et essayer une approche différente tout en préservant la session originale intacte, utilisez plutôt [`/branch`](/docs/fr/sessions#branch-a-session) ou `claude --continue --fork-session`.
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

Le checkpointing ne suit pas les fichiers modifiés par les commandes Bash. Par exemple, si Claude Code exécute :

```bash theme={null}
rm file.txt
mv old.txt new.txt
cp source.txt dest.txt
```

Ces modifications de fichiers ne peuvent pas être annulées via le rembobinage. Seules les modifications de fichiers directs effectuées via les outils d'édition de fichiers de Claude sont suivies.

<h3 id="subagent-edits-not-restored">
  Les modifications des subagents ne sont pas restaurées
</h3>

Un [subagent](/docs/fr/sub-agents) effectue des modifications avec les outils d'édition de fichiers de Claude, mais Claude Code ne capture généralement pas ces modifications dans les checkpoints de votre session. Le fait que le rembobinage les restaure dépend de la façon dont le subagent s'exécute :

* **Skill forké en avant-plan** : un [skill avec `context: fork`](/docs/fr/skills#run-skills-in-a-subagent) qui s'exécute en avant-plan modifie votre arborescence de travail pendant votre propre tour, donc le rembobinage restaure ses modifications comme d'habitude. Définissez `background: false` pour exécuter un fork en avant-plan ; quelques situations, [listées sur la page des skills](/docs/fr/skills#run-skills-in-a-subagent), l'exécutent là-bas indépendamment du paramètre.
* **Tout autre subagent** : le rembobinage ne restaure pas les modifications. Utilisez git pour les annuler. Cela inclut un skill forké qui s'exécute en arrière-plan, le paramètre par défaut, et une exécution [`/code-review --fix`](/docs/fr/code-review) en arrière-plan.

<h3 id="external-changes-not-tracked">
  Les modifications externes ne sont pas suivies
</h3>

Le checkpointing suit uniquement les fichiers qui ont été modifiés au cours de la session actuelle. Les modifications manuelles que vous apportez aux fichiers en dehors de Claude Code et les modifications d'autres sessions concurrentes ne sont normalement pas capturées, sauf si elles modifient par hasard les mêmes fichiers que la session actuelle.

<h3 id="messages-sent-mid-turn-not-checkpointed">
  Les messages envoyés en cours de tour ne sont pas checkpointés
</h3>

Lorsqu'un message que vous [mettez en file d'attente pendant que Claude travaille](/docs/fr/interactive-mode#queue-messages-while-claude-works) atteint Claude au cours du tour en cours, il rejoint ce tour au lieu de commencer un nouveau. Le message apparaît dans la conversation, mais Claude Code ne crée pas de checkpoint pour lui, et le menu de rembobinage ne le répertorie pas. Un message en file d'attente que Claude Code envoie comme son propre tour reçoit un checkpoint comme d'habitude.

Pour supprimer un tel message, ou annuler les modifications que Claude a apportées après son arrivée, rembobinez jusqu'à l'invite qui a démarré le tour. Cela rembobine le tour entier, y compris le travail que Claude a effectué avant l'arrivée de votre message.

<h3 id="symlinked-and-hard-linked-paths-not-restored">
  Les chemins symlinked et hard-linked ne sont pas restaurés
</h3>

Le checkpointing ne rembobine pas les fichiers symlinked ou hard-linked. Lorsque vous sélectionnez **Restore code** ou **Restore code and conversation** dans le menu `/rewind`, Claude Code ignore tout chemin suivi qui est un symlink ou un hard link et affiche un avertissement `Restored the code, but skipped N files`. Les fichiers ignorés conservent leur contenu actuel. Pour annuler les modifications de la session sur l'un d'eux, demandez à Claude d'inverser la modification ou modifiez le fichier vous-même. Les fichiers de configuration qu'un gestionnaire de dotfiles symlinke dans votre projet et les fichiers que pnpm hard-linke en place entrent tous deux dans cette catégorie.

Pour voir quels chemins une restauration ignore, activez la journalisation de débogage avec `/debug` avant de restaurer : le journal de débogage à `~/.claude/debug/<session-id>.txt` nomme chaque chemin ignoré. Pour chaque raison d'ignorance et les étapes de récupération, consultez [l'entrée skipped-files dans la référence des erreurs](/docs/fr/errors#restored-the-code-but-skipped-files).

<h3 id="not-a-replacement-for-version-control">
  Pas un remplacement du contrôle de version
</h3>

Les checkpoints sont conçus pour une récupération rapide au niveau de la session. Pour un historique de version permanent et la collaboration, continuez à utiliser le contrôle de version, tel que Git, pour les commits, les branches et l'historique à long terme.

<h2 id="see-also">
  Voir aussi
</h2>

* [Mode interactif](/docs/fr/interactive-mode) - Raccourcis clavier et contrôles de session
* [Commandes](/docs/fr/commands) - Accès aux checkpoints en utilisant `/rewind`
* [Référence CLI](/docs/fr/cli-reference) - Options de ligne de commande
