> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi izin

> Kontrol apa yang dapat diakses Claude Code dan lakukan dengan aturan izin terperinci, mode, dan kebijakan terkelola.

Claude Code mendukung izin terperinci sehingga Anda dapat menentukan dengan tepat apa yang diizinkan dilakukan oleh agen dan apa yang tidak. Pengaturan izin dapat diperiksa ke dalam kontrol versi dan didistribusikan ke semua pengembang di organisasi Anda, serta disesuaikan oleh pengembang individual.

<h2 id="permission-system">
  Sistem izin
</h2>

Claude Code menggunakan sistem izin berjenjang untuk menyeimbangkan kekuatan dan keamanan. Tabel menunjukkan, untuk setiap jenis alat, apakah mode Manual meminta persetujuan sebelum tindakan dijalankan. [Mode izin](#permission-modes) lainnya mengubah mana dari ini yang meminta Anda; dalam mode otomatis pengklasifikasi meninjau tindakan alih-alih Anda, dan [bagaimana pengklasifikasi mengevaluasi tindakan](/docs/id/permission-modes#how-the-classifier-evaluates-actions) mencantumkan mana yang dilihatnya.

| Jenis alat      | Contoh               | Persetujuan diperlukan                                                                                                   | Perilaku "Ya, dan jangan tanya lagi"        |
| :-------------- | :------------------- | :----------------------------------------------------------------------------------------------------------------------- | :------------------------------------------ |
| Hanya baca      | Pembacaan file, Grep | Tidak, dalam [direktori kerja dan direktori tambahan](#working-directories)                                              | T/A                                         |
| Perintah Bash   | Eksekusi shell       | Ya, kecuali serangkaian [perintah hanya baca](#read-only-commands) yang tertanam                                         | Secara permanen per repositori dan perintah |
| Modifikasi file | Edit/tulis file      | Ya                                                                                                                       | Hingga akhir sesi                           |
| Pengambilan web | WebFetch             | Ya, kecuali serangkaian [domain dokumentasi yang telah disetujui sebelumnya](/docs/id/tools-reference#webfetch-tool-behavior) | Secara permanen per repositori dan domain   |
| Pencarian web   | WebSearch            | Ya                                                                                                                       | Secara permanen per repositori              |

Ketika Anda memilih "Ya, dan jangan tanya lagi" dan persetujuan disimpan secara permanen, seperti untuk perintah Bash atau domain WebFetch, Claude Code menyimpan aturan ke `.claude/settings.local.json` di akar repositori git, diselesaikan melalui [worktrees](/docs/id/worktrees) ke checkout utama. Aturan berlaku untuk sesi masa depan di mana pun dalam repositori itu, termasuk sesi yang dimulai di subdirektori dan di worktrees. Persetujuan modifikasi file tidak disimpan ke file: seperti yang ditunjukkan tabel, itu berlangsung hingga sesi berakhir. Dalam beberapa kasus, seperti di luar repositori git atau di Windows, Claude Code tidak menggunakan akar repositori; [Tempat Claude Code mencari setiap file](/docs/id/settings#where-claude-code-looks-for-each-file) mencantumkan kasus-kasus itu dan tempat itu menyimpan aturan sebagai gantinya.

Sebelum v2.1.211, Claude Code selalu menyimpan aturan di direktori awal, jadi persetujuan yang diberikan di worktree atau subdirektori tidak berlaku untuk sisa repositori. Aturan yang disimpan versi sebelumnya di subdirektori atau worktree masih berlaku untuk sesi yang dimulai di sana.

Kadang-kadang prompt izin hanya menawarkan persetujuan satu kali, tanpa opsi "jangan tanya lagi" dan tanpa opsi untuk mengizinkan tindakan untuk sisa sesi. Claude Code menawarkan opsi-opsi itu hanya ketika prompt dapat menunjukkan kepada Anda semua yang akan mereka izinkan, jadi aturan yang Anda simpan dari prompt mencakup hanya apa yang dinamai opsinya. Ketika prompt hanya menawarkan persetujuan satu kali, setujui tindakan sekali, atau tambahkan aturan sendiri di [`/permissions`](#manage-permissions).

<h3 id="add-a-comment-when-you-answer-a-permission-prompt">
  Tambahkan komentar ketika Anda menjawab prompt izin
</h3>

Anda dapat melampirkan catatan kepada Claude ketika Anda menyetujui atau menolak satu tindakan. Pada sebagian besar prompt izin, termasuk Bash, PowerShell, file, dan prompt alat MCP, pindah ke **Ya** atau **Tidak** dan tekan `Tab` untuk membuka bidang komentar pada opsi itu. Prompt WebFetch dan browser tidak menawarkan bidang. Opsi yang mengizinkan tindakan untuk sisa sesi atau menyimpan aturan juga tidak mengambil satu.

Dengan bidang terbuka, ketik komentar dan kemudian tekan salah satu kunci ini:

* `Enter`: mengirimkan jawaban Anda dengan komentar terlampir. Jika Anda membiarkan bidang kosong, Claude Code mengirimkan jawaban tanpa komentar.
* `Tab`: menutup bidang tanpa menjawab. Claude Code menyimpan teks yang Anda ketik dan masih mengirimkannya jika Anda menjawab dengan opsi itu.
* `Shift+Tab`: pada prompt file, seperti prompt Edit atau Write, menutup bidang sama seperti `Tab`. Sebelum v2.1.235, menekan `Shift+Tab` di dalam bidang malah memilih opsi yang mengizinkan tindakan untuk sisa sesi, jadi Claude Code menyetujui tindakan untuk sisa sesi dan membuang komentar.

Claude Code mengirimkan komentar secara berbeda tergantung pada cara Anda menjawab:

* **Ya**: Claude Code menjalankan tindakan, kemudian mengirimkan komentar Anda ke Claude setelah hasilnya.
* **Tidak**: Claude Code mengirimkan komentar Anda ke Claude sebagai alasan penolakan, dan Claude terus bekerja. Jika Anda memilih **Tidak** tanpa komentar pada prompt dari percakapan utama, Claude Code menghentikan giliran.

<h2 id="manage-permissions">
  Kelola izin
</h2>

Anda dapat melihat dan mengelola izin alat Claude Code dengan `/permissions`. Dialog ini mencantumkan semua aturan izin dan file `settings.json` tempat setiap aturan berasal. Anda dapat membuka dialog saat Claude sedang bekerja: ketika Anda menambah atau menghapus aturan, Claude Code menerapkan perubahan mulai dari panggilan alat Claude berikutnya dalam giliran yang sama. Sebelum v2.1.234, Claude Code mengantrikan perintah hingga giliran selesai.

* Aturan **Allow** memungkinkan Claude Code menggunakan alat yang ditentukan tanpa persetujuan manual.
* Aturan **Ask** meminta konfirmasi setiap kali Claude Code mencoba menggunakan alat yang ditentukan.
* Aturan **Deny** mencegah Claude Code menggunakan alat yang ditentukan.

Aturan dievaluasi secara berurutan: deny, kemudian ask, kemudian allow. Kecocokan pertama dalam urutan tersebut menentukan hasilnya, dan spesifisitas aturan tidak mengubah urutan.

Aturan deny yang luas seperti `Bash(aws *)` memblokir setiap panggilan yang cocok, termasuk panggilan yang juga cocok dengan aturan allow yang lebih sempit seperti `Bash(aws s3 ls)`. Aturan allow tidak dapat membuat pengecualian dari aturan deny. Prioritas yang sama berlaku antara ask dan allow: aturan ask yang cocok meminta konfirmasi bahkan ketika aturan allow yang lebih spesifik juga cocok dengan panggilan yang sama.

Aturan deny berperilaku berbeda tergantung pada apakah mereka menamai alat atau membatasi pola dalam satu alat. Nama alat biasa seperti `Bash` menghapus alat dari konteks Claude sepenuhnya, sehingga Claude tidak pernah melihatnya. Jika Anda menambahkan aturan seperti itu di tengah sesi, Claude tidak dapat memanggil alat dari panggilan alat berikutnya; [Menolak seluruh alat](/docs/id/prompt-caching#denying-an-entire-tool) mencakup apa yang terjadi pada definisi yang telah Claude lihat. Aturan yang dibatasi seperti `Bash(rm *)` membiarkan alat tersedia dan memblokir panggilan yang cocok ketika Claude mencoba menggunakannya.

Penghapusan nama biasa berlaku untuk setiap alat kecuali [`EndConversation`](/docs/id/tools-reference#endconversation-tool-behavior): aturan deny tidak dapat menghapusnya saat alat lain tetap ada, dan aturan ask tidak pernah memintanya.

<Note>
  Aturan izin ditegakkan oleh Claude Code, bukan oleh model. Instruksi dalam prompt Anda atau `CLAUDE.md` membentuk apa yang Claude coba lakukan, tetapi mereka tidak mengubah apa yang Claude Code izinkan. Untuk memberikan atau mencabut akses, gunakan `/permissions`, aturan yang dijelaskan di sini, [mode izin](/docs/id/permission-modes), atau [hook PreToolUse](#extend-permissions-with-hooks).
</Note>

Ketika [mode auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) tersedia untuk sesi Anda, dialog juga mencakup [aturan pengklasifikasi mode auto](/docs/id/auto-mode-config#edit-rules-from-permissions). Pilih tab **Auto mode** untuk melihatnya.

<h2 id="permission-modes">
  Mode izin
</h2>

Claude Code mendukung beberapa mode izin yang mengontrol bagaimana alat disetujui. Lihat [Permission modes](/docs/id/permission-modes) untuk mengetahui kapan menggunakan masing-masing. Untuk mengubah mode yang dimulai sesi, atur `defaultMode` dalam [file pengaturan](/docs/id/settings#where-settings-live) Anda. [Mode mana yang dimulai sesi](/docs/id/permission-modes#which-mode-a-session-starts-in) mencakup default bawaan untuk setiap paket dan apa yang dibaca ekstensi VS Code.

| Mode                | Deskripsi                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| :------------------ | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `default`           | Meminta izin pada penggunaan pertama setiap alat. Berlabel Manual di CLI, ekstensi VS Code dan JetBrains, dan aplikasi desktop, dan Claude Code menerima `manual` sebagai alias. Label dan alias memerlukan Claude Code v2.1.200 atau lebih baru. Label aplikasi desktop tidak bergantung pada versi CLI Anda                                                                                                                                                                                                                                                                                                                   |
| `acceptEdits`       | Secara otomatis menerima edit file dan perintah sistem file umum seperti `mkdir`, `touch`, `mv`, dan `cp` untuk jalur di direktori kerja atau `additionalDirectories`                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `plan`              | Claude membaca file dan menjalankan perintah shell hanya-baca untuk menjelajahi tetapi tidak mengedit file sumber Anda; dengan [mode auto](/docs/id/permission-modes#eliminate-prompts-with-auto-mode) tersedia, perintah yang disetujui classifier juga berjalan. Berlabel Plan di CLI dan ekstensi VS Code                                                                                                                                                                                                                                                                                                                         |
| `auto`              | Secara otomatis menyetujui panggilan alat dengan pemeriksaan keamanan latar belakang yang memverifikasi tindakan selaras dengan permintaan Anda                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `dontAsk`           | Secara otomatis menolak setiap panggilan yang sebaliknya akan meminta; pembacaan file di direktori kerja Anda dan tindakan lain yang tidak memerlukan persetujuan masih berjalan, begitu juga dengan alat yang telah disetujui sebelumnya melalui `/permissions` atau aturan `permissions.allow`. `AskUserQuestion`, alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool), dan alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dalam sesi di mana pengaturan itu mencapai Claude Code ditolak bahkan jika Anda telah mengizinkannya |
| `bypassPermissions` | Melewati prompt izin, kecuali untuk [tindakan yang tidak ada mode auto-approve](/docs/id/permission-modes#actions-no-mode-auto-approves)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |

<Warning>
  Dalam mode `bypassPermissions`, Claude Code melewati prompt izin, termasuk untuk penulisan ke [jalur terlindungi](/docs/id/permission-modes#protected-paths) seperti `.git` dan `.claude`. [Penjaga pesan lintas-sesi](/docs/id/permission-modes#skip-all-checks-with-bypasspermissions-mode) masih berlaku. Hanya gunakan mode ini di lingkungan terisolasi seperti kontainer atau VM tempat Claude Code tidak dapat menyebabkan kerusakan.
</Warning>

Untuk mencegah mode `bypassPermissions` atau `auto` digunakan, atur `permissions.disableBypassPermissionsMode` atau `permissions.disableAutoMode` ke `"disable"` dalam [file pengaturan](/docs/id/settings#where-settings-live) apa pun. Ini paling berguna dalam [pengaturan terkelola](#managed-settings) di mana mereka tidak dapat ditimpa.

<h2 id="permission-rule-syntax">
  Sintaks aturan izin
</h2>

Aturan izin mengikuti format `Tool` atau `Tool(specifier)`. Tanda kurung di dalam specifier bersifat literal, jadi perintah atau jalur yang mengandungnya tidak memerlukan escaping.

<h3 id="match-all-uses-of-a-tool">
  Cocokkan semua penggunaan alat
</h3>

Untuk mencocokkan semua penggunaan alat, gunakan hanya nama alat tanpa tanda kurung:

| Aturan     | Efek                                         |
| :--------- | :------------------------------------------- |
| `Bash`     | Mencocokkan semua perintah Bash              |
| `WebFetch` | Mencocokkan semua permintaan pengambilan web |
| `Read`     | Mencocokkan semua pembacaan file             |

`Bash(*)` setara dengan `Bash` dan mencocokkan semua perintah Bash. Sebagai aturan penolakan, kedua bentuk menghapus alat dari konteks Claude.

<h3 id="use-specifiers-for-fine-grained-control">
  Gunakan specifier untuk kontrol terperinci
</h3>

Tambahkan specifier dalam tanda kurung untuk mencocokkan penggunaan alat tertentu:

| Aturan                         | Efek                                                    |
| :----------------------------- | :------------------------------------------------------ |
| `Bash(npm run build)`          | Mencocokkan perintah yang tepat `npm run build`         |
| `Read(./.env)`                 | Mencocokkan pembacaan file `.env` di direktori saat ini |
| `WebFetch(domain:example.com)` | Mencocokkan permintaan pengambilan ke example.com       |

<h3 id="match-by-input-parameter">
  Cocokkan berdasarkan parameter input
</h3>

Aturan penolakan dan tanya dapat mencocokkan parameter input tingkat atas pada alat apa pun yang dibangun dengan `Tool(param:value)`.

Untuk mencocokkan parameter pada alat MCP, berikan aturan penolakan dengan [`--disallowedTools`](/docs/id/cli-reference#cli-flags). Ketika Claude Code memuat file pengaturan, aturan `mcp__` apa pun yang memiliki tanda kurung akan dilewati. Claude Code mencantumkan aturan yang dilewati dalam dialog pengaturan tidak valid ketika sesi interaktif dimulai, dan dalam output [`claude doctor`](/docs/id/debug-your-config#check-resolved-settings).

Aturan parameter cocok ketika Claude memanggil alat dengan parameter tersebut diatur ke nilai yang tepat. Aturan izin untuk satu nilai parameter tidak akan menetapkan bahwa panggilan aman secara keseluruhan, jadi aturan izin terus menggunakan sintaks specifier masing-masing alat. Ini berfungsi untuk parameter skalar apa pun yang diterima alat:

| Aturan                         | Cocok                                           |
| :----------------------------- | :---------------------------------------------- |
| `Agent(model:opus)`            | Panggilan Agent yang meminta tingkat model Opus |
| `Agent(isolation:worktree)`    | Panggilan Agent yang meminta git worktree       |
| `Bash(run_in_background:true)` | Panggilan Bash yang berjalan di latar belakang  |

Pencocokan parameter mengikuti aturan ini:

* Nama parameter harus berupa bidang langsung dari input alat, seperti `model` pada alat Agent. Bidang yang bersarang di dalam objek atau array tidak dapat dicocokkan
* Setiap aturan menamai satu parameter. Untuk membatasi pada `model` dan `isolation`, tulis dua aturan, `Agent(model:opus)` dan `Agent(isolation:worktree)`, daripada menggabungkannya dalam satu aturan
* Nilai mendukung `*` sebagai wildcard yang mencocokkan urutan karakter apa pun, jadi `Agent(isolation:*)` mencocokkan nilai isolasi eksplisit apa pun. Tanpa `*` pencocokan bersifat tepat
* Parameter yang dihilangkan model tidak pernah dicocokkan, jadi `Agent(model:*)` tidak mencocokkan panggilan yang membiarkan `model` tidak diatur
* Nilai dibandingkan dengan input literal yang dikirim Claude, sebelum normalisasi apa pun. `Agent(model:opus)` mencocokkan alias `opus` tetapi bukan ID model lengkap. Jalankan dengan [`--verbose`](/docs/id/cli-reference) untuk melihat nama dan nilai parameter yang tepat dalam setiap panggilan alat
* Spasi di sekitar titik dua diabaikan

Anda tidak dapat mencocokkan bidang konten utama alat dengan cara ini: `command` untuk Bash dan PowerShell, `file_path` untuk Read, Edit, dan Write, `path` untuk Grep dan Glob, `notebook_path` untuk NotebookEdit, dan `url` untuk WebFetch. Aturan seperti `Bash(command:rm *)` dapat dilewati oleh perintah gabungan, jadi Claude Code mengabaikannya dan mengeluarkan peringatan startup. Gunakan `Bash(rm *)`, `Read(./path)`, atau `WebFetch(domain:host)` sebagai gantinya.

<h3 id="wildcard-patterns">
  Pola wildcard
</h3>

`*` dalam aturan Bash mencocokkan teks apa pun, termasuk spasi, jadi satu aturan mencakup keluarga perintah. Aturan tanpa `*` mencocokkan satu perintah yang tepat.

<Warning>
  Letakkan `*` setelah subperintah. Dalam `git log --oneline main`, `git` adalah program dan `log` adalah subperintah, kata yang menentukan apa yang dilakukan program. Claude Code mencocokkan semua yang sebelum `*` pertama seperti yang ditulis, jadi kata-kata tersebut adalah yang membatasi aturan: `Bash(git log *)` hanya memungkinkan perintah `git log`, dan `Bash(git *)` memungkinkan setiap perintah git. Claude Code [memperingatkan saat startup](/docs/id/errors#has-a-wildcard-before-the-rest-of-the-command) tentang aturan izin dengan `*` sebelum subperintah, seperti `Bash(git * main)`.
</Warning>

Tulis perintah yang ingin Anda jalankan Claude tanpa bertanya, dan ganti bagian yang bervariasi dengan `*`. Dengan konfigurasi ini, Claude Code menjalankan skrip npm dan git commit tanpa bertanya dan menolak perintah yang dimulai dengan `git push`. Push yang ditulis dengan cara lain, seperti `git -C . push`, tidak cocok; lihat [apa yang tidak cocok dengan aturan Bash](#bash-rule-limits).

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

`*` dapat berada di mana saja dalam aturan: di awal, di tengah, atau di akhir. Setiap baris menunjukkan aturan, perintah yang cocok, dan perintah terdekat yang tidak cocok:

| Anda tulis             | Cocok                                                                                | Tidak cocok                            |
| :--------------------- | :----------------------------------------------------------------------------------- | :------------------------------------- |
| `Bash(npm run build)`  | `npm run build`                                                                      | `npm run build --watch`                |
| `Bash(npm run *)`      | `npm run build`, `npm run test --watch`, `npm run`                                   | `npm install`                          |
| `Bash(git log * main)` | `git log --oneline main`, `git log -5 main`, `git log --output=<file> main`          | `git log main`, `git push origin main` |
| `Bash(git * main)`     | `git merge main`, `git push origin main`, `git -c core.fsmonitor=<script> diff main` | `git log`                              |
| `Bash(* --version)`    | `node --version`, `bash -c 'echo hi' --version`                                      | `node -v`                              |
| `Bash(ls *)`           | `ls -la`, `ls`                                                                       | `lsof`                                 |
| `Bash(ls*)`            | `ls -la`, `lsof`                                                                     |                                        |
| `Bash(* --help *)`     | `npm --help x`                                                                       | `npm --help`                           |

Tiga aturan pencocokan menghasilkan baris-baris tersebut:

* **`*` mewakili teks apa pun yang ada di tempatnya.** Dalam `Bash(git * main)`, itu mewakili subperintah, jadi Claude Code mencocokkan setiap subperintah git dan setiap opsi sebelumnya. Itu termasuk `-c`, yang membuat git menjalankan program yang Anda beri nama. Dalam `Bash(* --version)`, `*` mewakili program, jadi program apa pun cocok.
* **`*` di akhir, dengan spasi sebelumnya, juga mencocokkan perintah telanjang.** `Bash(ls *)` mencocokkan `ls`, dan `Bash(git log *)` mencocokkan `git log`. Itu hanya berlaku ketika `*` trailing adalah satu-satunya wildcard aturan: `Bash(* --help *)` mencocokkan `npm --help x` tetapi bukan `npm --help`.
* **Spasi sebelum `*` trailing adalah bagian dari aturan.** `Bash(ls *)` memerlukan spasi setelah `ls`, jadi `lsof` tidak cocok. `Bash(ls*)` tidak memiliki spasi, jadi itu juga mencocokkan `lsof`.

Akhiran `:*` adalah cara setara untuk menulis wildcard trailing, jadi `Bash(ls:*)` mencocokkan perintah yang sama dengan `Bash(ls *)`.

Dialog izin menulis bentuk yang dipisahkan spasi ketika Anda memilih "Ya, jangan tanya lagi" untuk awalan perintah. Bentuk `:*` hanya dikenali di akhir pola. Dalam pola seperti `Bash(git:* push)`, titik dua diperlakukan sebagai karakter literal dan tidak akan mencocokkan perintah git.

<h3 id="tool-name-wildcards">
  Wildcard nama alat
</h3>

Aturan penolakan dan tanya juga menerima pola glob dalam posisi nama alat. Pola harus cocok dengan nama alat lengkap: `"*"` cocok dengan setiap alat, dan `"mcp__*"` cocok dengan setiap alat MCP di semua server. Alat yang cocok dengan aturan penolakan nama telanjang dihapus dari konteks Claude, sama seperti nama alat telanjang, termasuk pengecualian [`EndConversation`](/docs/id/tools-reference#endconversation-tool-behavior): penolakan glob tidak dapat menghapusnya sementara alat lain tetap ada, dan tanya glob tidak pernah memintanya. Konfigurasi ini menolak setiap alat MCP:

```json theme={null}
{
  "permissions": {
    "deny": [
      "mcp__*"
    ]
  }
}
```

Aturan izin menerima glob nama alat hanya setelah awalan literal `mcp__<server>__`. Segmen server harus bebas glob sehingga aturan menamai server spesifik yang Anda konfigurasi. `mcp__puppeteer__*` cocok dengan setiap alat dari server `puppeteer`, dan `mcp__github__get_*` cocok dengan alat `get_` miliknya. Glob izin yang tidak berlabuh seperti `"*"`, `"B*"`, atau `"mcp__*"` dilewati dengan peringatan dan tidak secara otomatis menyetujui apa pun.

Aturan penolakan atau tanya yang nama alatnya tidak cocok dengan alat yang dikenal menghasilkan peringatan startup untuk menangkap kesalahan ketik. Nama alat yang berisi `_` atau `*` dikecualikan dari pemeriksaan, dan begitu juga nama alat yang telah dihapus Claude Code, seperti `TaskOutput`.

Label yang ditampilkan untuk alat dalam transkrip dan dialog izin dapat berbeda dari nama kanoniknya. Misalnya, alat yang diberi label `Stop Task` dalam transkrip memiliki nama kanonik `TaskStop`. Aturan izin dan [pencocokan hook](/docs/id/hooks) tidak cocok dengan label, jadi aturan yang ditulis sebagai `Stop Task` tidak cocok. Untuk aturan penolakan dan tanya, peringatan startup di atas menangkap ketidaksesuaian. Gunakan nama kanonik yang tercantum dalam [referensi alat](/docs/id/tools-reference).

<h2 id="tool-specific-permission-rules">
  Aturan izin khusus alat
</h2>

<h3 id="bash">
  Bash
</h3>

Aturan Bash cocok dengan seluruh teks perintah, dengan `*` mewakili teks apa pun. [Pola wildcard](#wildcard-patterns) menunjukkan perintah mana yang cocok dengan setiap bentuk aturan dan di mana menempatkan `*`. Bagian lainnya mencakup cara Claude Code mencocokkan perintah gabungan dan pembungkus, apa yang tidak cocok dengan aturan, perintah baca-saja, dan pengalihan.

<h4 id="compound-commands">
  Perintah gabungan
</h4>

<Tip>
  Claude Code menyadari operator shell, jadi aturan seperti `Bash(safe-cmd *)` tidak akan memberinya izin untuk menjalankan perintah `safe-cmd && other-cmd`. Pemisah perintah yang dikenali adalah `&&`, `||`, `;`, `|`, `|&`, `&`, dan baris baru. Aturan harus cocok dengan setiap subperintah secara independen.
</Tip>

Aturan tolak dan tanya berlaku ketika subperintah apa pun cocok dengan mereka, termasuk perintah yang bersarang di dalam subshell, substitusi perintah, atau badan alur kontrol seperti loop `for`. Aturan tanya seperti `Bash(git clean *)` masih meminta Anda untuk `cd /tmp && git clean -f` atau `echo "$(git clean -f)"`, bahkan dalam [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode).

Ketika `&&` atau `||` tidak memiliki apa pun setelahnya, seperti dalam `npm test &&`, Claude Code memperlakukan perintah sebagai tidak dapat diurai dan tidak membaginya menjadi subperintah untuk pencocokan aturan izin, jadi aturan seperti `Bash(npm *)` tidak menyetujuinya.

Ketika Anda menyetujui perintah gabungan dengan "Ya, dan jangan tanya lagi", Claude Code menyimpan aturan terpisah untuk setiap subperintah yang memerlukan persetujuan, bukan satu aturan untuk string gabungan lengkap. Misalnya, menyetujui `git status && npm test` menyimpan aturan untuk `npm test`, jadi invokasi `npm test` di masa depan dikenali terlepas dari apa yang mendahului `&&`. Subperintah seperti `cd` ke direktori di luar direktori kerja Anda menghasilkan aturan Read mereka sendiri untuk jalur itu. Hingga 5 aturan dapat disimpan untuk satu perintah gabungan.

<h4 id="process-wrappers">
  Pembungkus
</h4>

Sebelum mencocokkan aturan Bash, Claude Code menghilangkan serangkaian pembungkus tetap, jadi aturan seperti `Bash(npm test *)` juga cocok dengan `timeout 30 npm test`. Pembungkus yang dihilangkan adalah `timeout`, `time`, `nice`, `nohup`, dan `stdbuf`, ditambah shell builtin `command` dan `builtin`, dan `noglob` zsh. Masing-masing menjalankan argumennya sebagai perintah aktual. Dua bentuk terkait tidak dihilangkan: bentuk kueri `command -v`, yang mencari perintah daripada menjalankannya, dan `nocorrect` zsh.

Claude Code juga menghilangkan penugasan terdepan dari variabel lingkungan tertentu yang dikenal aman, jadi `Bash(npm test *)` cocok dengan `NODE_ENV=test npm test`. Aturan izin tidak akan cocok melampaui penugasan variabel apa pun yang lain. Aturan tolak atau tanya cocok melampaui penugasan terdepan apa pun, jadi `Bash(rm *)` dalam tolak masih cocok dengan `FOO=bar rm -rf tmp/`.

`xargs` telanjang juga dihilangkan, jadi `Bash(grep *)` cocok dengan `xargs grep pattern`. Penghilangan hanya berlaku ketika `xargs` tidak memiliki flag: invokasi seperti `xargs -n1 grep pattern` dicocokkan sebagai perintah `xargs`, jadi aturan yang ditulis untuk perintah dalam tidak mencakupnya.

Daftar pembungkus ini bawaan dan tidak dapat dikonfigurasi. Pelari lingkungan pengembangan seperti `direnv exec`, `devbox run`, `mise exec`, `npx`, dan `docker exec` tidak ada dalam daftar. Karena alat-alat ini menjalankan argumen mereka sebagai perintah, aturan seperti `Bash(devbox run *)` cocok dengan apa pun yang datang setelah `run`, termasuk `devbox run rm -rf .`. Untuk menyetujui pekerjaan di dalam pelari lingkungan, tulis aturan spesifik yang mencakup baik pelari maupun perintah dalam, seperti `Bash(devbox run npm test)`. Tambahkan satu aturan per perintah dalam yang ingin Anda izinkan.

Pembungkus Exec seperti `watch`, `setsid`, `ionice`, dan `flock` tidak dapat disetujui otomatis oleh aturan awalan seperti `Bash(watch *)`, jadi dalam mode Manual mereka selalu meminta. Hal yang sama berlaku untuk `find` dengan `-exec` atau `-delete`: aturan `Bash(find *)` tidak mencakup bentuk-bentuk ini. Untuk menyetujui invokasi spesifik, tulis aturan pencocokan tepat untuk string perintah lengkap.

<h4 id="bash-rule-limits">
  Apa yang tidak cocok dengan aturan Bash
</h4>

Aturan Bash cocok dengan teks perintah yang ditulis Claude, setelah Claude Code membagi [perintah gabungan](#compound-commands) dan menghilangkan [pembungkus](#process-wrappers). Aturan ini tidak cocok dengan program yang sama yang dipanggil dalam bentuk berbeda, jadi aturan tolak atau tanya mencakup invokasi yang biasanya dihasilkan Claude dan bukan batas keamanan di sekitar program. Aturan-aturan ini dalam `deny` atau `ask` menghentikan bentuk pertama dan bukan yang lain:

| Aturan             | Menghentikan               | Tidak menghentikan                                                                                    |
| :----------------- | :------------------------- | :---------------------------------------------------------------------------------------------------- |
| `Bash(curl *)`     | `curl https://example.com` | `/usr/bin/curl https://example.com`, `sh -c 'curl https://example.com'`                               |
| `Bash(rm *)`       | `rm -rf build/`            | `/bin/rm -rf build/`, `bash -c 'rm -rf build/'`                                                       |
| `Bash(git push *)` | `git push origin main`     | `git -C . push origin main`, `git -c push.default=current push origin main`, `git 'push' origin main` |

Aturan lain Anda dan mode izin memutuskan perintah di kolom terakhir.

Untuk penegakan sistem file dan jaringan yang tidak bergantung pada teks perintah, gunakan [sandboxing](/docs/id/sandboxing). Untuk memeriksa teks perintah lengkap dengan logika Anda sendiri sebelum dijalankan, gunakan hook [PreToolUse](#extend-permissions-with-hooks).

<h4 id="read-only-commands">
  Perintah baca-saja
</h4>

Claude Code mengenali serangkaian perintah Bash bawaan sebagai baca-saja dan menjalankannya tanpa prompt izin dalam setiap mode, kecuali untuk jalur yang [`permissions.blockReadsOutsideWorkingDirectories`](/docs/id/settings-reference#permissions-blockreadsoutsideworkingdirectories) lindungi. Himpunan ini mencakup `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd`, dan bentuk baca-saja dari `git`. Himpunan ini tidak dapat dikonfigurasi; untuk memerlukan prompt untuk salah satu perintah ini, tambahkan aturan `ask` atau `deny` untuk itu. Dalam mode otomatis, perintah-perintah ini juga dapat menunggu tinjauan pengklasifikasi; lihat [bagaimana pengklasifikasi mengevaluasi tindakan](/docs/id/permission-modes#how-the-classifier-evaluates-actions).

Pengalihan seperti `ls > out.txt` menambahkan pemeriksaan pada target. Lihat [Pengalihan](#redirections).

Pola glob yang tidak dikutip diizinkan untuk perintah yang setiap flagnya baca-saja, jadi `ls *.ts` dan `wc -l src/*.py` berjalan tanpa prompt.

Dalam mode Manual, perintah dari himpunan ini masih meminta dalam kasus-kasus ini:

* **Glob yang tidak dikutip untuk perintah dengan flag yang mampu menulis**: perintah dengan flag yang mampu menulis atau mampu exec, seperti `find`, `sort`, `sed`, dan `git`, meminta ketika glob yang tidak dikutip ada, karena glob dapat berkembang menjadi flag seperti `-delete`.
* **`docker` yang menunjuk ke daemon lain**: bentuk baca-saja dari `docker` meminta ketika perintah membawa flag yang memilih daemon berbeda, seperti `-H`, `--context`, atau `--url` dan `--connection` Podman.
* **`file` dengan flag pembuka jalur**: `file` meminta ketika melewatkan `-m`/`--magic-file` atau `-f`/`--files-from`, karena flag-flag ini membuat `file` membuka jalur yang dinamai dalam nilai flag.
* **Jalur jaringan di Windows**: perintah yang argumennya mencakup jalur jaringan (UNC), seperti `\\server\share\file`, meminta karena mengakses jalur jaringan dapat mengirim kredensial Windows Anda ke host yang dinamainya. Pemeriksaan yang sama berlaku untuk perintah [alat PowerShell](/docs/id/tools-reference#powershell-tool).
* **Perintah yang analisisnya tidak dapat diurai**: ketika Claude Code tidak dapat sepenuhnya mengurai perintah, itu meminta persetujuan daripada memperlakukan perintah sebagai baca-saja. Perintah yang lebih panjang dari 10.000 karakter selalu meminta karena melebihi apa yang dianalisis.

`cd` ke jalur di dalam direktori kerja Anda atau [direktori tambahan](#working-directories) juga baca-saja, dan perintah gabungan seperti `cd packages/api && ls` berjalan tanpa prompt ketika setiap bagian memenuhi syarat sendiri. Kombinasi-kombinasi ini meminta bahkan ketika setiap bagian baca-saja:

* **`cd` dengan `git`**: meminta ketika `cd` berubah ke direktori berbeda, karena menjalankan `git` di direktori baru dapat menjalankan hook direktori itu. `cd` yang targetnya diselesaikan ke direktori kerja saat ini adalah no-op dan tidak memicu prompt.
* **`cd` dengan pengalihan**: meminta ketika Claude Code tidak dapat menentukan direktori mana target pengalihan diselesaikan terhadap setelah `cd` berjalan. Perintah yang satu-satunya target pengalihannya adalah `/dev/null`, seperti `cd app; grep -r pattern . 2>/dev/null`, tidak meminta, karena `/dev/null` tidak bergantung pada direktori kerja.

<Warning>
  Pola izin Bash yang mencoba membatasi argumen perintah rapuh. Misalnya, `Bash(curl http://github.com/ *)` dimaksudkan untuk membatasi curl ke URL GitHub, tetapi tidak akan cocok dengan variasi seperti:

  * Opsi sebelum URL: `curl -X GET http://github.com/...`
  * Protokol berbeda: `curl https://github.com/...`
  * Pengalihan: `curl -L http://short.example.com/xyz`, yang dialihkan ke GitHub
  * Variabel: `URL=http://github.com && curl $URL`

  Untuk penyaringan URL yang lebih andal, pertimbangkan:

  * **Batasi alat jaringan Bash**: gunakan aturan tolak untuk menghentikan `curl`, `wget`, dan perintah serupa, kemudian gunakan alat WebFetch dengan izin `WebFetch(domain:github.com)` untuk domain yang diizinkan. Aturan tolak tidak cocok dengan program yang sama berdasarkan jalur atau di dalam `sh -c`, jadi pasangkan dengan [daftar allowlist jaringan sandbox](/docs/id/sandboxing#network-isolation) ketika pembatasan harus berlaku; lihat [apa yang tidak cocok dengan aturan Bash](#bash-rule-limits)
  * **Gunakan hook PreToolUse**: implementasikan hook yang memvalidasi URL dalam perintah Bash dan memblokir domain yang tidak diizinkan
  * **Tambahkan panduan CLAUDE.md**: jelaskan pola curl yang diizinkan Anda dalam `CLAUDE.md`. Ini membentuk apa yang Claude coba tetapi tidak memberlakukan batas, jadi pasangkan dengan salah satu opsi di atas

  Perhatikan bahwa menggunakan WebFetch saja tidak mencegah akses jaringan. Jika Bash diizinkan, Claude masih dapat menggunakan `curl`, `wget`, atau alat lain untuk menjangkau URL apa pun.
</Warning>

<h4 id="redirections">
  Pengalihan
</h4>

Ketika perintah mengalihkan output atau input, Claude Code memeriksa target pengalihan terhadap aturan file Anda seolah-olah Claude menulis atau membaca file itu secara langsung:

* **Pengalihan output**: untuk `> file`, `>> file`, atau `2> file`, pemeriksaan mencakup aturan izin `Edit` Anda, [jalur terlindungi](/docs/id/permission-modes#protected-paths), dan [direktori kerja](#working-directories). Aturan seperti `Bash(git commit *)` mengizinkan perintah, bukan target. Target yang dimulai dengan `~` atau berisi karakter glob memerlukan persetujuan Anda.
* **Pengalihan input**: untuk `< file`, pemeriksaan mencakup aturan izin `Read` Anda dan direktori kerja. Target di luar direktori kerja memerlukan persetujuan Anda kecuali aturan izin mencakupnya. Target yang berisi pola glob, atau jalur relatif yang mengikuti `cd` dalam perintah yang sama, memerlukan persetujuan Anda bahkan ketika aturan izin mencakupnya. Claude Code memeriksa target input dalam v2.1.257 dan yang lebih baru.

Target tanpa file di belakangnya tidak diperiksa: `/dev/null`, bentuk file-descriptor seperti `2>&1` dan `<&3`, dan here-docs dan here-strings.

Claude Code juga memeriksa file yang ditulis perintah `tee`, termasuk dalam pipeline seperti `make | tee build.log`. Pemeriksaan mencakup aturan izin `Edit` Anda, [jalur terlindungi](/docs/id/permission-modes#protected-paths), dan [direktori kerja](#working-directories). Aturan izin seperti `Bash(tee *)` tidak mencakup tujuan di luar direktori kerja. Claude Code memeriksa target `tee` dalam v2.1.269 dan yang lebih baru.

<h3 id="powershell">
  PowerShell
</h3>

Aturan izin PowerShell menggunakan bentuk yang sama dengan aturan Bash. Wildcard dengan `*` cocok di posisi apa pun, akhiran `:*` setara dengan ` *` di belakang, dan `PowerShell` atau `PowerShell(*)` telanjang cocok dengan setiap perintah. Konfigurasi ini mengizinkan perintah `Get-ChildItem` dan `git commit` sambil memblokir `Remove-Item`:

```json theme={null}
{
  "permissions": {
    "allow": [
      "PowerShell(Get-ChildItem *)",
      "PowerShell(git commit *)"
    ],
    "deny": [
      "PowerShell(Remove-Item *)"
    ]
  }
}
```

Alias umum dikanonisasi sebelum pencocokan. Aturan yang ditulis untuk nama cmdlet juga cocok dengan aliasnya, jadi `PowerShell(Get-ChildItem *)` cocok dengan `gci`, `ls`, dan `dir` juga. Pencocokan tidak peka huruf besar-kecil.

Claude Code mengurai AST PowerShell dan memeriksa setiap perintah dalam perintah gabungan secara independen. Operator pipeline `|`, pemisah pernyataan `;`, dan pada PowerShell 7+ operator rantai `&&` dan `||` membagi perintah gabungan menjadi subperintah. Aturan harus cocok dengan setiap subperintah agar perintah gabungan diizinkan.

<h3 id="read-and-edit">
  Read dan Edit
</h3>

Untuk memblokir alat file Claude dari membaca file atau direktori, tambahkan aturan tolak `Read` untuk jalurnya, seperti `Read(./.env)` atau `Read(./secrets/**)`; [Exclude sensitive files](/docs/id/settings-reference#exclude-sensitive-files) memiliki contoh siap tempel.

Aturan `Edit` berlaku untuk semua alat bawaan yang mengedit file. Claude membuat upaya terbaik untuk menerapkan aturan `Read` ke semua alat bawaan yang membaca file seperti Grep dan Glob, ke penyebutan `@file` dalam prompt Anda, dan ke konteks file pilihan dan file terbuka yang [IDE](/docs/id/vs-code#the-built-in-ide-mcp-server) yang terhubung bagikan dengan Claude.

Aturan tolak `Read` juga memblokir alat [Edit dan Write](/docs/id/errors#file-is-covered-by-a-read-deny-rule) pada jalur yang sama, termasuk membuat file baru di sana. NotebookEdit tidak tercakup, jadi tambahkan aturan tolak `Edit` untuk jalur yang tidak boleh diubah alat apa pun. Pemeriksaan memerlukan Claude Code v2.1.208 atau lebih baru pada edit, dan v2.1.228 atau lebih baru pada write.

Claude Code memeriksa izin file hanya terhadap aturan `Edit(path)` dan `Read(path)`. Jika Anda menulis aturan jalur untuk `Write`, `NotebookEdit`, `Glob`, atau alat `MultiEdit` warisan sebagai gantinya, Claude Code menerima aturan tetapi tidak pernah berkonsultasi dengannya, dan [memperingatkan saat startup](/docs/id/errors#is-not-matched-by-file-permission-checks), kecuali untuk aturan `Glob` yang dilewatkan dalam `--allowedTools`. Gunakan `Edit(docs/**)` sebagai pengganti `Write(docs/**)`, `NotebookEdit(docs/**)`, atau `MultiEdit(docs/**)`, dan `Read(docs/**)` sebagai pengganti `Glob(docs/**)`. Claude Code tidak memperingatkan tentang aturan nama alat tanpa jalur, seperti aturan tolak untuk `Write`; aturan itu cocok di mana-mana di tingkat alat. Memerlukan Claude Code v2.1.210 atau lebih baru.

<Warning>
  Aturan tolak Read dan Edit berlaku untuk alat file bawaan Claude, ke perintah file yang Claude Code kenali dalam Bash, seperti `cat`, `head`, `tail`, `sed`, dan `tee`, dan ke target pengalihan Bash [redirections](#redirections) seperti `> file` dan `< file`. Aturan-aturan ini tidak berlaku untuk perintah yang membaca file tanpa menamakannya, seperti `grep -r pattern .` dijalankan dari direktori yang menyimpan file, atau ke subproses arbitrer yang membaca atau menulis file secara tidak langsung, seperti skrip Python atau Node yang membuka file sendiri. Untuk penegakan tingkat OS yang memblokir semua proses dari mengakses jalur, [aktifkan sandbox](/docs/id/sandboxing).
</Warning>

Aturan Read dan Edit keduanya menggunakan sintaks pola [gitignore](https://git-scm.com/docs/gitignore) dengan empat jenis pola yang berbeda; untuk pola direktori segmen tunggal, kedalaman pencocokan juga bergantung pada jenis aturan, dijelaskan nanti di bagian ini:

| Pola                 | Arti                                      | Contoh                           | Cocok                                                             |
| -------------------- | ----------------------------------------- | -------------------------------- | ----------------------------------------------------------------- |
| `//path`             | Jalur absolut dari akar sistem file       | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                                         |
| `~/path`             | Jalur dari direktori rumah                | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                                    |
| `/path`              | Jalur relatif terhadap sumber pengaturan  | `Edit(/src/**/*.ts)`             | `<primary working directory>/src/**/*.ts` dalam pengaturan proyek |
| `path` atau `./path` | Jalur relatif terhadap direktori saat ini | `Read(*.env)`                    | `<cwd>/*.env`                                                     |

<Warning>
  Pola seperti `/Users/alice/file` bukan jalur absolut. Garis miring tunggal terdepan berlabuh di sumber pengaturan, bukan akar sistem file. Gunakan `//Users/alice/file` untuk jalur absolut.
</Warning>

Pola `/path` berlabuh di direktori yang terkait dengan sumber pengaturan yang mendefinisikannya, jadi aturan yang sama cocok dengan lokasi berbeda tergantung di mana Anda menempatkannya:

| Aturan didefinisikan dalam                        | `/path` diselesaikan ke            |
| :------------------------------------------------ | :--------------------------------- |
| Pengaturan proyek di `.claude/settings.json`      | `<primary working directory>/path` |
| Pengaturan lokal di `.claude/settings.local.json` | `<primary working directory>/path` |
| Pengaturan pengguna di `~/.claude/settings.json`  | `~/.claude/path`                   |
| File yang dilewatkan dengan `--settings <file>`   | `<directory of file>/path`         |
| Flag CLI atau aturan sesi                         | `<primary working directory>/path` |

Aturan yang Anda tambahkan melalui `/permissions` mengikuti baris untuk file pengaturan yang Anda simpan.

Aturan pengaturan lokal berlabuh di [direktori kerja utama](#working-directories) sesi, bukan di akar repositori tempat Claude Code [menyimpan file](#permission-system) dalam v2.1.211 dan yang lebih baru. Dalam sesi yang dimulai di akar repositori, dua direktori sama; dalam sesi [worktree](/docs/id/worktrees), aturan bersama seperti `Edit(/src/**)` cocok dengan direktori `src/` worktree itu sendiri.

Aturan tolak seperti `Read(/secrets/**)` dalam pengaturan pengguna memblokir `~/.claude/secrets/**`, bukan direktori `secrets` dalam proyek Anda. Untuk menulis aturan dalam pengaturan pengguna yang berlaku di dalam setiap proyek, gunakan jalur absolut `//` atau jalur relatif rumah `~/` sebagai gantinya.

Di Windows, jalur dinormalisasi ke bentuk POSIX sebelum pencocokan. `C:\Users\alice` menjadi `/c/Users/alice`, jadi gunakan `//c/**/.env` untuk mencocokkan file `.env` di mana pun di drive itu. Untuk mencocokkan di semua drive, gunakan `//**/.env`.

Contoh:

* `Edit(/docs/**)`: edit dalam `<primary working directory>/docs/`, bukan `/docs/` atau `<primary working directory>/.claude/docs/`
* `Read(~/.zshrc)`: membaca `.zshrc` direktori rumah Anda
* `Edit(//tmp/scratch.txt)`: edit jalur absolut `/tmp/scratch.txt`
* `Read(src/**)`: sebagai aturan izin, membaca dari `<current-directory>/src/` saja; sebagai aturan tolak atau tanya, cocok dengan direktori `src` di kedalaman apa pun di bawah direktori saat ini

Aturan hanya cocok dengan file di bawah jangkarnya; dalam batas itu, kedalaman pencocokan bergantung pada bentuk pola dan, untuk pola direktori segmen tunggal, jenis aturan, dijelaskan di bawah. Nama file telanjang mengikuti semantik gitignore dan cocok di kedalaman apa pun, jadi `Read(.env)` dan `Read(**/.env)` setara:

| Aturan tolak                      | Memblokir                                          | Tidak memblokir                                |
| --------------------------------- | -------------------------------------------------- | ---------------------------------------------- |
| `Read(.env)` atau `Read(**/.env)` | `.env` apa pun di atau di bawah direktori saat ini | `.env` dalam direktori induk atau proyek lain  |
| `Read(//**/.env)`                 | `.env` apa pun di mana pun di sistem file          | tidak ada; aturan berlabuh di akar sistem file |

Pola relatif dengan segmen direktori tunggal, seperti `src/**`, cocok di kedalaman berbeda tergantung pada jenis aturan:

* **Aturan izin**: `Edit(src/**)` cocok hanya dengan `<cwd>/src` dan file di bawahnya. Untuk mengizinkan nama direktori di kedalaman apa pun, tulis `Edit(**/src/**)`.
* **Aturan tolak dan tanya**: `Read(secrets/**)` cocok dengan direktori bernama `secrets` di kedalaman apa pun di bawah direktori saat ini, jadi aturan juga berlaku untuk salinan bersarang.

Setiap bentuk pola lainnya cocok di kedalaman yang sama dalam setiap jenis aturan: `Edit(/src/**)` dan `Edit(src/components/**)` cocok hanya di lokasi berlabuh mereka, sementara `Edit(**/src/**)` cocok di kedalaman apa pun.

Contoh berikut menunjukkan setiap bentuk pola terhadap proyek dengan direktori `src/` tingkat atas dan salinan bersarang di bawah `vendor/`:

```text theme={null}
<current-directory>/
├── src/
│   └── app.ts
└── vendor/
    └── pkg/
        └── src/
            └── lib.js
```

| Aturan                                         | Cocok dengan `src/app.ts` | Cocok dengan `vendor/pkg/src/lib.js` |
| :--------------------------------------------- | :------------------------ | :----------------------------------- |
| `Edit(src/**)` sebagai aturan izin             | Ya                        | Tidak                                |
| `Edit(src/**)` sebagai aturan tolak atau tanya | Ya                        | Ya                                   |
| `Edit(/src/**)` dalam jenis aturan apa pun     | Ya                        | Tidak                                |
| `Edit(**/src/**)` dalam jenis aturan apa pun   | Ya                        | Ya                                   |

<Note>
  Dalam pola gitignore, `*` cocok dalam segmen jalur tunggal dan dapat muncul di posisi apa pun dalam pola, sementara `**` cocok di seluruh direktori.
</Note>

Ketika Anda menyetujui jalur file dengan "Ya, dan jangan tanya lagi", Claude Code menghindari karakter pola gitignore dalam jalur itu, seperti `[`, `]`, dan `*`, jadi aturan yang dihasilkan cocok hanya dengan jalur literal yang Anda setujui. Aturan yang Anda tulis sendiri tidak dihindari. Sebelum v2.1.202, Claude Code menyimpan jalur tanpa dihindari, jadi aturan yang dihasilkan untuk direktori bernama `[2024-06] Reports` dapat gagal mencocokkan jalurnya sendiri atau mencocokkan direktori saudara yang tidak dimaksudkan.

Anda tidak perlu menghindari tanda kurung dalam jalur, jadi `Edit(./Finance (2024)/**)` cocok dengan folder `Finance (2024)` seperti yang dieja.

Aturan tolak atau tanya yang jalurnya tidak dapat digunakan sebagai pola gitignore masih menjaga jalur itu yang tepat. Aturan izin dengan pola yang tidak dapat digunakan tidak menyetujui apa pun.

Pola tolak atau tanya yang dimulai dengan `!` adalah negasi gitignore. Aturan ini mengukir jalur yang cocok dari aturan `path` atau `./path` yang tercantum sebelumnya. Dalam daftar `deny` satu file pengaturan, `Read(*.env)` diikuti oleh `Read(!sample.env)` memblokir setiap file yang namanya berakhir dengan `.env` di kedalaman apa pun, kecuali file bernama `sample.env`. Aturan `!` yang tercantum pertama tidak mengukir apa pun.

Pengukiran mencapai hanya aturan dari sumber yang sama. `Read(!.env)` dalam pengaturan proyek atau dalam `--disallowedTools` tidak membatalkan tolak `Read(./.env)` dari pengaturan terkelola atau file pengaturan lain apa pun.

Dua batas mempersempit apa yang dapat diukir pola `!`:

* Claude Code membaca pola `!` relatif terhadap direktori saat ini bahkan ketika `/`, `~/`, atau `//` mengikuti `!`, jadi pola tidak dapat menjangkau aturan yang berlabuh dengan salah satu awalan itu. `Read(!~/notes/public/**)` tidak mengukir apa pun dari `Read(~/notes/**)`.
* Pengukiran tidak dapat membuka kembali file di dalam direktori yang aturan blokir secara keseluruhan. Dengan `Read(secrets/**)` dan `Read(!secrets/public/**)`, Claude Code masih memblokir `secrets/public` bersama dengan sisa `secrets`.

Ketika Claude mengakses symlink, aturan izin memeriksa dua jalur: symlink itu sendiri dan file yang diselesaikannya. Aturan izin dan tolak memperlakukan pasangan itu berbeda: aturan izin kembali ke meminta Anda, sementara aturan tolak memblokir langsung.

* **Aturan izin**: berlaku hanya ketika jalur symlink dan targetnya cocok. Symlink di dalam direktori yang diizinkan yang menunjuk ke luar masih meminta Anda.
* **Aturan tolak**: berlaku ketika jalur symlink atau targetnya cocok. Symlink yang menunjuk ke file yang ditolak ditolak sendiri. Misalnya, dengan `Read(./project/**)` diizinkan dan `Read(~/.ssh/**)` ditolak, symlink di `./project/key` yang menunjuk ke `~/.ssh/id_rsa` diblokir: target gagal aturan izin dan cocok dengan aturan tolak.

Di macOS dan Linux, aturan tolak atau tanya yang ditulis melalui direktori yang disymlink dengan pola `//`, `~/`, atau `/` juga berlaku di lokasi nyata direktori. Misalnya, di macOS, di mana `/etc` diselesaikan ke `/private/etc`, `Read(//etc/**)` juga memblokir `/private/etc/hosts`. Sebelum v2.1.268, aturan tolak atau tanya yang ditulis melalui direktori yang disymlink tidak berlaku pada jalur yang diberikan oleh lokasi nyatanya.

Ketika alat membuka file yang disetujui, Claude Code [mengkonfirmasi jalur masih diselesaikan ke lokasi yang pemeriksaan izin setujui](/docs/id/errors#refusing-after-a-symlink-changed).

Grep dan Glob mencari direktori yang argumen `path` diselesaikan. Claude Code menerapkan aturan tolak `Read` ke direktori itu.

<h3 id="webfetch">
  WebFetch
</h3>

Aturan WebFetch menggunakan awalan `domain:` dan cocok terhadap nama host URL yang diminta. Pencocokan tidak peka huruf besar-kecil, mendukung wildcard `*`, dan menghilangkan titik di belakang dari baik aturan maupun nama host sehingga `example.com.` dan `example.com` diperlakukan sama.

* `WebFetch(domain:example.com)` cocok dengan permintaan ke `example.com`
* `WebFetch(domain:*.example.com)` cocok dengan subdomain apa pun di kedalaman apa pun, seperti `api.example.com` atau `a.b.example.com`, tetapi bukan `example.com` itu sendiri
* `WebFetch(domain:*)` cocok dengan setiap domain. Aturan ini tidak sama dengan aturan `WebFetch` telanjang; lihat [Allow or deny every fetch](#allow-or-deny-every-fetch)

Di posisi apa pun selain `*.` terdepan atau `*` telanjang, wildcard cocok hanya dengan teks antara dua titik. `WebFetch(domain:example.*)` cocok dengan `example.org`, di mana `*` menjadi `org`, tetapi bukan `example.evil.com`, di mana `*` harus menjadi `evil.com` dan menyeberangi titik. Ini menjaga wildcard di belakang dari mencocokkan domain yang dapat didaftarkan penyerang.

Wildcard dalam aturan `WebFetch` memerlukan Claude Code v2.1.172 atau lebih baru untuk mencocokkan fetch.

<h4 id="allow-or-deny-every-fetch">
  Izinkan atau tolak setiap fetch
</h4>

Aturan `WebFetch` telanjang adalah nama alat tanpa bagian `domain:`, seperti `"deny": ["WebFetch"]`. Baik itu maupun `WebFetch(domain:*)` mencakup setiap URL, tetapi Claude Code menerapkannya berbeda, dan hanya bentuk `domain:` yang juga menambahkan domainnya ke [daftar domain yang diizinkan atau ditolak](/docs/id/sandboxing#network-isolation) sandbox. Bagian itu mencantumkan bentuk wildcard yang dihormati sandbox dan versi yang menambahkan `*` telanjang.

Setiap baris menunjukkan apa yang dilakukan aturan dalam daftar `allow` dan dalam daftar `deny`:

| Aturan               | Dalam `allow`                                                                                      | Dalam `deny`                                                                                                                                        |
| :------------------- | :------------------------------------------------------------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------- |
| `WebFetch`           | Claude fetch tanpa meminta Anda. Tidak mengubah host mana yang dapat dijangkau perintah sandboxed. | Claude Code menghapus alat `WebFetch`, jadi Claude tidak dapat fetch sama sekali. Tidak mengubah host mana yang dapat dijangkau perintah sandboxed. |
| `WebFetch(domain:*)` | Claude fetch tanpa meminta Anda, dan perintah sandboxed dapat menjangkau host apa pun.             | Claude Code menyimpan alat dan menolak setiap fetch, dan perintah sandboxed tidak dapat menjangkau host apa pun.                                    |

Dua bentuk juga berbeda pada pembacaan [artifact](/docs/id/artifacts), halaman yang alat Artifact terbitkan di claude.ai. Aturan tolak atau tanya `WebFetch` telanjang tidak berlaku pada pembacaan itu. Aturan `domain:` yang mencakup `claude.ai` atau host konten `*.claudeusercontent.com`, seperti `WebFetch(domain:claude.ai)` atau `WebFetch(domain:*)`, menolak setiap pembacaan atau meminta sebelumnya. Aturan [`Artifact`](/docs/id/artifacts#disable-artifacts) melakukan hal yang sama.

Ketika aturan memblokir pembacaan, penolakan menamai aturan. Sebelum v2.1.268, aturan tolak `WebFetch` telanjang memblokir setiap pembacaan artifact, dan aturan tanya telanjang meminta sebelum setiap pembacaan.

Untuk membiarkan Claude fetch dengan bebas sambil menjaga daftar allowlist sandbox seperti apa adanya, gunakan bentuk telanjang. `settings.json` ini melakukan itu:

```json theme={null}
{
  "permissions": {
    "allow": ["WebFetch"]
  }
}
```

Ketika Anda meminta Claude untuk fetch halaman, itu fetch tanpa prompt. Ketika Anda meminta itu menjalankan `curl` [sandboxed](/docs/id/sandboxing) terhadap host di luar daftar allowlist sandbox, Claude Code masih meminta Anda untuk host itu, karena aturan telanjang tidak menambahkan host ke daftar allowlist.

Dalam [mode otomatis](/docs/id/permission-modes#eliminate-prompts-with-auto-mode), Claude sebagai gantinya menamai host dalam [domain yang diizinkan per perintah](/docs/id/sandboxing#per-command-allowed-domains-in-auto-mode) perintah untuk pengklasifikasi ditinjau.

<h3 id="mcp">
  MCP
</h3>

Aturan MCP menggunakan nama server seperti yang dikonfigurasi dalam Claude Code, secara opsional diikuti oleh nama alat dari server itu.

* `mcp__puppeteer` cocok dengan alat apa pun yang disediakan oleh server `puppeteer`
* `mcp__puppeteer__*` menggunakan sintaks wildcard dan juga cocok dengan semua alat dari server `puppeteer`
* `mcp__puppeteer__puppeteer_navigate` cocok dengan alat `puppeteer_navigate` yang disediakan oleh server `puppeteer`

Jika organisasi Anda telah menetapkan alat [claude.ai connector](/docs/id/mcp#organization-controls-on-connector-tools) ke `ask` dan pengaturan itu mencapai Claude Code dalam sesi Anda, aturan izin untuk alat itu tidak berlaku: Claude Code meminta pada setiap panggilan, bahkan dalam mode `auto` dan `bypassPermissions`. Dalam mode `dontAsk`, yang tidak pernah meminta, Claude Code menolak panggilan sebagai gantinya. Alat dari konektor yang Claude Code ambil sendiri muncul sebagai `mcp__claude_ai_<server>__<tool>`.

Dalam sesi [Cowork](https://claude.com/docs/cowork/overview) di aplikasi Claude Desktop, Claude menjalankan perintah shell melalui alat `mcp__workspace__bash` Cowork daripada alat `Bash` bawaan, dan Cowork juga menyediakan `mcp__workspace__web_fetch` untuk web fetch. Claude Code juga menerapkan aturan tolak yang menamai seluruh alat `Bash` atau `WebFetch` ke alat Cowork ini, jadi aturan tolak `Bash` terkelola menghentikan Claude dari menjalankan perintah shell dalam Cowork. Ketika Claude Code memblokir panggilan seperti itu, pesan menamai alat Cowork: `Permission to use mcp__workspace__bash has been denied.` Aturan izin tidak terbawa: Claude Code tidak pernah menerapkan aturan izin `Bash` ke `mcp__workspace__bash`.

<h3 id="agent-subagents">
  Agent (subagents)
</h3>

Gunakan aturan `Agent(AgentName)` untuk mengontrol [subagent](/docs/id/sub-agents) mana yang dapat digunakan Claude:

* `Agent(Explore)` cocok dengan subagent Explore
* `Agent(Plan)` cocok dengan subagent Plan
* `Agent(my-custom-agent)` cocok dengan subagent kustom bernama `my-custom-agent`

Tambahkan aturan-aturan ini ke array `deny` dalam pengaturan Anda atau gunakan flag CLI `--disallowedTools` untuk menonaktifkan agen spesifik. Untuk menonaktifkan agen Explore:

```json theme={null}
{
  "permissions": {
    "deny": ["Agent(Explore)"]
  }
}
```

<h3 id="cd">
  Cd
</h3>

Aturan `Cd` mengontrol direktori mana yang dapat dipindahkan perintah [`/cd`](/docs/id/commands) ke sesi. `Cd` bukan alat yang dapat dipanggil model: Claude tidak dapat memanggilnya, dan aturan berlaku hanya ketika Anda menjalankan `/cd` sendiri.

Aturan tolak `Cd` telanjang menonaktifkan `/cd` sepenuhnya. Aturan tolak `Cd(<path-pattern>)` memblokir target yang cocok. Aturan tolak memeriksa setiap ejaan target, termasuk setiap lompatan symlink yang diselesaikannya, jadi aturan yang ditulis untuk satu jalur juga memblokir target yang diselesaikan ke itu.

Menambahkan aturan izin `Cd` apa pun beralih `/cd` ke mode daftar allowlist: direktori target yang diselesaikan harus cocok dengan salah satu aturan izin Anda, atau `/cd` menolak. Tanpa aturan `Cd` yang dikonfigurasi, `/cd` menyimpan perilaku defaultnya dan meminta Anda untuk mempercayai direktori yang tidak dikenal.

Pola jalur berbagi jangkar `//`, `~/`, dan `/` dari [aturan Read dan Edit](#read-and-edit), tetapi pencocokan berlabuh pada seluruh jalur direktori daripada gaya gitignore. `*` cocok dengan tepat satu segmen jalur dan `**` cocok di seluruh segmen. `/**` di belakang juga cocok dengan akar yang dinamainya.

| Aturan                | Cocok                                                                             | Tidak cocok                |
| --------------------- | --------------------------------------------------------------------------------- | -------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                                                                      | `~/code/app/src`, `~/code` |
| `Cd(~/code/**)`       | `~/code` dan direktori apa pun di bawahnya                                        | direktori di luar `~/code` |
| `Cd(**/node_modules)` | direktori `node_modules` apa pun di kedalaman apa pun di bawah direktori saat ini | `node_modules/pkg`         |

<h2 id="extend-permissions-with-hooks">
  Perluas izin dengan hook
</h2>

[Hook Claude Code](/docs/id/hooks-guide) memungkinkan Anda mendaftarkan perintah shell kustom yang mengevaluasi izin saat runtime. Ketika Claude Code membuat panggilan alat, hook PreToolUse berjalan sebelum prompt izin, untuk setiap alat kecuali [`EndConversation`](/docs/id/tools-reference#endconversation-tool-behavior). Output hook dapat menolak panggilan alat, memaksa prompt, atau melewati prompt untuk membiarkan panggilan berlanjut.

Keputusan hook tidak melewati aturan izin. Claude Code mengevaluasi aturan deny dan ask terlepas dari apa yang dikembalikan hook PreToolUse: aturan deny yang cocok memblokir panggilan, dan aturan ask masih meminta bahkan ketika hook mengembalikan `"allow"` atau `"ask"`. Ini mempertahankan prioritas deny-first yang dijelaskan dalam [Kelola izin](#manage-permissions), termasuk aturan deny yang ditetapkan dalam pengaturan terkelola.

Alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) juga masih meminta ketika hook mengembalikan `"allow"`, begitu juga alat connector [yang organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dalam sesi di mana pengaturan tersebut mencapai Claude Code.

Hook pemblokiran juga memiliki prioritas atas aturan allow. Hook yang keluar dengan kode 2 menghentikan panggilan alat sebelum aturan izin dievaluasi, jadi blokir berlaku bahkan ketika aturan allow akan membiarkan panggilan berlanjut. Untuk menjalankan semua perintah Bash tanpa prompt kecuali untuk beberapa yang ingin Anda blokir, tambahkan `"Bash"` ke daftar allow Anda dan daftarkan hook PreToolUse yang menolak perintah tertentu itu. Lihat [Block edits to protected files](/docs/id/hooks-guide#block-edits-to-protected-files) untuk skrip hook yang dapat Anda sesuaikan.

<h2 id="working-directories">
  Direktori kerja
</h2>

Secara default, Claude memiliki akses ke file di direktori tempat Anda meluncurkannya. Direktori tersebut adalah direktori kerja utama sesi sampai Anda [memindahkan sesi dengan `/cd`](#move-the-session-to-another-directory). Anda dapat memperluas akses ini:

* **Saat startup**: gunakan argumen CLI `--add-dir <path>`
* **Selama sesi**: gunakan perintah `/add-dir`
* **Konfigurasi persisten**: tambahkan ke `additionalDirectories` dalam [file pengaturan](/docs/id/settings#where-settings-live)

File di direktori tambahan mengikuti aturan izin yang sama dengan direktori kerja asli: mereka menjadi dapat dibaca tanpa prompt, dan izin edit file mengikuti mode izin saat ini.

Anda tidak dapat menambahkan sebagian besar [jalur jaringan](/docs/id/errors#working-directory-is-a-network-path), seperti berbagi UNC `\\server\share`, sebagai direktori kerja, karena pencarian dapat menghubungi host yang dinamainya. Di Windows, petakan berbagi ke huruf drive sebagai gantinya dan teruskan drive dengan `--add-dir` saat peluncuran.

Atur [`permissions.blockReadsOutsideWorkingDirectories`](/docs/id/settings-reference#permissions-blockreadsoutsideworkingdirectories) untuk membuat alat file menolak jalur yang dibatasi dalam setiap mode izin. Dalam mode otomatis, Claude Code menawarkan untuk mengaktifkannya pertama kali Claude [membaca di luar direktori kerja](/docs/id/permission-modes#first-read-outside-the-working-directories).

Dalam sesi latar belakang di macOS, host sesi meminta akses ke folder yang dilindungi seperti `~/Desktop`, `~/Documents`, dan `~/Downloads` secara terpisah dari terminal Anda ketika Claude perlu membaca atau menulis file di sana; jika pembacaan di sana gagal dengan `Operation not permitted`, lihat [cara memberikan akses folder ke sesi latar belakang](/docs/id/agent-view#background-sessions-can%E2%80%99t-read-desktop-documents-or-downloads-on-macos).

<h3 id="move-the-session-to-another-directory">
  Memindahkan sesi ke direktori lain
</h3>

Untuk memindahkan sesi ke direktori kerja utama yang berbeda, daripada [menambahkan direktori](#working-directories) bersama yang saat ini, jalankan `/cd <path>`. Claude Code menyimpan percakapan, memuat `CLAUDE.md` direktori baru, dan meminta Anda untuk [mempercayai workspace](#project-allow-rules-and-workspace-trust) jika Anda belum pernah bekerja di dalamnya sebelumnya. Setelahnya, Claude Code [menemukan sesi yang dipindahkan](/docs/id/sessions#resume-a-session) ketika Anda menjalankan `--resume` dari direktori baru.

Segera setelah Anda memindahkan, Claude Code menerapkan konfigurasi proyek direktori baru:

* Pengaturan proyeknya, termasuk aturan izin mereka dan [hooks](/docs/id/hooks)
* Server [`.mcp.json`](/docs/id/mcp#project-scope) miliknya, tunduk pada [persetujuan server](/docs/id/mcp#project-server-approvals-and-workspace-trust) yang sama seperti saat startup, dan server MCP [local-scope](/docs/id/mcp#local-scope) yang Anda daftarkan di dalamnya
* [plugins](/docs/id/plugins/overview) yang diaktifkan pengaturannya, [skills](/docs/id/skills#discovery-from-parent-and-nested-directories) miliknya, dan [subagents](/docs/id/sub-agents) miliknya
* Nilai [`env`](/docs/id/settings-reference#env) miliknya, diterapkan di atas variabel lingkungan dari pengaturan direktori sebelumnya, yang tetap berlaku

Claude Code juga memutuskan sambungan proyek direktori sebelumnya dan server MCP [local-scope](/docs/id/mcp#local-scope), dan server dari [plugins](/docs/id/mcp#plugin-provided-mcp-servers) yang tidak lagi diaktifkan setelah perpindahan. Ini mengambil [direktori tambahan](#working-directories) dari pengaturan direktori baru alih-alih yang sebelumnya, dan menyimpan direktori yang Anda tambahkan dengan `--add-dir` atau `/add-dir`. Hooks yang diaktifkan perpindahan masih menerima [`${CLAUDE_PROJECT_DIR}`](/docs/id/hooks#reference-scripts-by-path) diatur ke akar proyek tempat sesi dimulai.

Ketika direktori baru belum dipercaya, Claude Code mencantumkan dalam prompt kepercayaan aturan izin, direktori tambahan, hooks, dan perintah pembantu yang akan diaktifkan pengaturan direktori, sehingga Anda dapat meninjau sebelum menerima. Jika Anda menolak, sesi tetap berada di tempat asalnya. Sebelum v2.1.246, `/cd` tidak menerapkan pengaturan direktori baru, hooks, server MCP, atau skills sampai Anda melanjutkan sesi, dan prompt kepercayaannya tidak mencantumkan apa yang akan diaktifkan pengaturan direktori.

Batasi atau nonaktifkan target `/cd` dengan aturan izin [`Cd`](#cd).

<h3 id="additional-directories-grant-file-access-not-configuration">
  Direktori tambahan memberikan akses file, bukan konfigurasi
</h3>

Menambahkan direktori memperluas tempat Claude dapat membaca dan mengedit file. Ini tidak membuat direktori itu akar konfigurasi penuh: sebagian besar konfigurasi `.claude/` tidak ditemukan dari direktori tambahan, meskipun beberapa jenis dimuat sebagai pengecualian.

Pengecualian ini hanya berlaku untuk direktori yang ditambahkan dengan flag `--add-dir` atau perintah `/add-dir`, termasuk direktori yang ditambahkan Agent SDK melalui flag. Direktori yang tercantum dalam `permissions.additionalDirectories` dalam file pengaturan memberikan akses file saja dan tidak memuat konfigurasi apa pun di bawah ini.

[`additionalDirectories`](/docs/id/agent-sdk/typescript#options) opsi Agent SDK dalam TypeScript dan [`add_dirs`](/docs/id/agent-sdk/python#claudeagentoptions) opsi dalam Python menerima pengecualian juga, meskipun opsi TypeScript berbagi namanya dengan kunci pengaturan. SDK melewatkan setiap entri ke Claude Code sebagai `--add-dir`, sehingga direktori tersebut berperilaku seperti direktori yang ditambahkan flag. Skills, perintah, dan subagents dari direktori yang ditambahkan flag apa pun dimuat melalui sumber pengaturan [`project`](/docs/id/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources), sehingga mereka tidak dimuat ketika Anda mengecualikan sumber itu dengan [`--setting-sources`](/docs/id/cli-reference) pada CLI atau `settingSources` dalam SDK, dan [bare mode](/docs/id/headless#start-faster-with-bare-mode) melewati perintah dan subagents di antara mereka.

Jenis konfigurasi berikut dimuat dari direktori `--add-dir`:

| Konfigurasi                                                                           | Dimuat dari `--add-dir`                                                                                                                                                       |
| :------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Skills](/docs/id/skills) di `.claude/skills/`                                             | Ya, dengan live reload                                                                                                                                                        |
| [File perintah](/docs/id/skills#where-skills-live) di `.claude/commands/`                  | Ya, tanpa live reload. Ketika direktori yang ditambahkan dan proyek Anda keduanya mendefinisikan perintah dengan nama yang sama, Claude Code menjalankan perintah proyek Anda |
| [Subagents](/docs/id/sub-agents) di `.claude/agents/`                                      | Ya, tanpa live reload                                                                                                                                                         |
| [Settings](/docs/id/settings) di `.claude/settings.json` dan `.claude/settings.local.json` | Kunci `enabledPlugins` dan [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces) saja                                                                     |
| File [CLAUDE.md](/docs/id/memory), `.claude/rules/`, dan `CLAUDE.local.md`                 | Hanya ketika `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` diatur. `CLAUDE.local.md` juga memerlukan sumber pengaturan `local`, yang diaktifkan secara default             |

Untuk memuat skills, perintah, dan subagents dari subdirektori [direktori kerja utama](#working-directories) Anda di tengah sesi, jalankan `/add-dir` dengan jalur subdirektori tersebut. Claude Code memuat mereka untuk sisa sesi tanpa meminta Anda atau menambahkan direktori kerja, karena subdirektori sudah dapat dibaca. Ini memerlukan Claude Code v2.1.257 atau lebih baru.

Claude Code menemukan gaya output dari direktori kerja saat ini dan induknya, direktori pengguna Anda di `~/.claude/`, dan pengaturan terkelola. Hooks dan kunci `.claude/settings.json` lainnya dimuat dari folder `.claude/` direktori kerja saat ini tanpa fallback direktori induk, bersama dengan `~/.claude/settings.json` pengguna Anda dan pengaturan terkelola. `.claude/settings.local.json` dimuat dari akar repositori git sebagai gantinya, bahkan ketika Anda memulai Claude Code dalam subdirektori, kecuali dalam kasus di mana Claude Code [tidak menggunakan akar repositori](/docs/id/settings#where-claude-code-looks-for-each-file), seperti di Windows; sebelum v2.1.211, itu juga dimuat hanya dari direktori kerja saat ini. Sesi [Agent SDK](/docs/id/agent-sdk/claude-code-features#control-filesystem-settings-with-settingsources) memuat dari direktori kerja di semua versi.

Untuk berbagi konfigurasi itu di seluruh proyek, gunakan salah satu pendekatan ini:

* **Konfigurasi tingkat pengguna**: tempatkan file di `~/.claude/agents/`, `~/.claude/output-styles/`, atau `~/.claude/settings.json` untuk membuatnya tersedia di setiap proyek
* **Plugins**: paket dan distribusikan konfigurasi sebagai [plugin](/docs/id/plugins/overview) yang dapat diinstal tim
* **Luncurkan dari direktori konfigurasi**: jalankan Claude Code dari direktori yang berisi konfigurasi `.claude/` yang ingin Anda gunakan

<h2 id="how-permissions-interact-with-sandboxing">
  Bagaimana izin berinteraksi dengan sandboxing
</h2>

Izin dan [sandboxing](/docs/id/sandboxing) adalah lapisan keamanan pelengkap:

* **Izin** mengontrol alat mana yang dapat digunakan Claude Code dan file atau domain mana yang dapat diaksesnya. Mereka berlaku untuk Bash, Read, Edit, WebFetch, MCP, dan setiap alat lainnya, kecuali bahwa aturan deny atau ask tidak dapat memblokir [`EndConversation`](/docs/id/tools-reference#endconversation-tool-behavior) sementara alat lain tetap ada.
* **Sandboxing** menyediakan penegakan tingkat OS yang membatasi akses sistem file dan jaringan perintah shell. Ini hanya berlaku untuk perintah Bash, PowerShell, dan [Monitor](/docs/id/tools-reference#monitor-tool) serta proses anak mereka.

Gunakan keduanya untuk pertahanan berlapis, karena pembatasan sandbox masih berlaku bahkan jika injeksi prompt melewati pengambilan keputusan Claude. Jalur dan domain dari pengaturan sandbox dan aturan izin [digabungkan ke dalam konfigurasi sandbox akhir](/docs/id/sandboxing#permission-rules).

Ketika Anda mengaktifkan sandboxing dan membiarkan `autoAllowBashIfSandboxed` pada default `true`, perintah Bash yang di-sandbox berjalan tanpa meminta bahkan jika izin Anda mencakup aturan ask `Bash` biasa, atau [bentuk setara `Bash(*)`](#match-all-uses-of-a-tool): batas sandbox menggantikan prompt seluruh alat tersebut.

Dalam [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode), Claude Code melewati substitusi ini. Tanpa aturan ask, [perintah baca-saja bawaan](#read-only-commands) masih berjalan tanpa meminta, dan perintah shell lainnya melalui alur izin reguler sementara Anda masih merencanakan; lihat [plan mode](/docs/id/permission-modes#analyze-before-you-edit-with-plan-mode) untuk bagaimana Claude Code membatasi perintah di sana. Dengan aturan ask `Bash` biasa, setiap perintah Bash meminta, termasuk perintah baca-saja yang di-sandbox, sama seperti di luar sandboxing. Sebelum v2.1.212, substitusi diterapkan dalam plan mode juga.

Pemeriksaan ini masih berlaku:

* Aturan ask yang dibatasi konten seperti `Bash(git push *)` masih memaksa prompt
* Aturan deny eksplisit masih berlaku
* Perintah `rm` atau `rmdir` yang menargetkan [jalur kritis](/docs/id/permission-modes#critical-paths) masih melalui alur izin reguler

Perintah yang tidak akan berjalan di-sandbox, seperti perintah yang dikecualikan, menghormati aturan ask `Bash` biasa seperti biasanya. Lihat [sandbox modes](/docs/id/sandboxing#sandbox-modes) untuk mengubah perilaku ini.

<span id="managed-only-settings" />

<h2 id="managed-settings">
  Pengaturan terkelola
</h2>

Untuk organisasi yang memerlukan kontrol terpusat, administrator menerapkan pengaturan terkelola yang tidak dapat ditimpa oleh pengaturan pengguna dan proyek, kecuali untuk beberapa [kunci yang sensitif terhadap keamanan](/docs/id/settings#exceptions-to-managed-settings-precedence). [Menerapkan pengaturan terkelola](/docs/id/managed-settings) mencakup mekanisme pengiriman, prioritas dalam tingkat terkelola, dan [kunci yang hanya dapat diatur oleh pengaturan terkelola](/docs/id/managed-settings#managed-only-settings).

Salah satu kunci tersebut, [`allowManagedPermissionRulesOnly`](/docs/id/settings-reference#allowmanagedpermissionrulesonly), membuat pengaturan terkelola menjadi satu-satunya sumber pengaturan untuk aturan izin. Entrinya mencantumkan setiap sumber yang kemudian diabaikan oleh Claude Code.

`disableBypassPermissionsMode` biasanya ditempatkan dalam pengaturan terkelola untuk memberlakukan kebijakan organisasi, tetapi berfungsi dari cakupan apa pun. Pengguna dapat mengaturnya dalam pengaturan mereka sendiri untuk mengunci diri mereka sendiri dari mode bypass.

<h2 id="settings-precedence">
  Prioritas pengaturan
</h2>

Aturan izin mengikuti [prioritas pengaturan](/docs/id/settings#settings-precedence) yang sama dengan semua pengaturan Claude Code lainnya, dengan pengaturan terkelola tertinggi: tidak ada tingkat lain, termasuk argumen baris perintah, yang dapat menimpa aturan izin terkelola.

Jika alat ditolak di tingkat mana pun, tidak ada tingkat lain yang dapat mengizinkannya. Misalnya, penolakan pengaturan terkelola tidak dapat ditimpa oleh `--allowedTools`, dan `--disallowedTools` dapat menambahkan pembatasan di luar apa yang ditentukan pengaturan terkelola.

Hal yang sama berlaku di seluruh cakupan pengaturan: jika pengaturan pengguna mengizinkan izin dan pengaturan proyek menolaknya, aturan penolakan memblokir izin tersebut. Kebalikannya juga benar: penolakan tingkat pengguna memblokir izin tingkat proyek, karena aturan penolakan dari cakupan apa pun dievaluasi sebelum aturan izin.

Host penyematan dapat menyediakan kebijakan terkelola tambahan melalui opsi SDK `managedSettings`, termasuk aturan izin allow kecuali admin menetapkan kunci `allowManaged*Only`; [Deliver policy to Claude Desktop sessions](/docs/id/claude-apps-gateway#deliver-policy-to-claude-desktop-sessions) mencakup kapan kebijakan embedder berlaku sepenuhnya.

<h2 id="project-allow-rules-and-workspace-trust">
  Aturan izin proyek dan kepercayaan ruang kerja
</h2>

Aturan `permissions.allow` dan entri `permissions.additionalDirectories` dalam `.claude/settings.json` proyek memberikan kemampuan, jadi Claude Code menerapkannya hanya setelah Anda menerima [dialog kepercayaan ruang kerja](/docs/id/security#additional-safeguards) untuk folder tersebut. Dialog mencantumkan aturan dan direktori yang akan diberikan folder sehingga Anda dapat meninjau terlebih dahulu. Aturan `deny` dan `ask` tidak terpengaruh, karena hanya membatasi.

Claude Code menyimpan kepercayaan yang Anda terima sesuai dengan tempat Anda memulainya:

* Dalam repositori, Claude Code mengunci kepercayaan pada akar repositori git, jadi kepercayaan mencakup seluruh repositori kecuali repositori git apa pun yang bersarang di dalamnya, seperti submodul. Dalam [worktree](/docs/id/worktrees), ia menggunakan akar checkout utama, seperti yang dilakukannya untuk [aturan yang disimpan](#permission-system).
* Di luar repositori, Claude Code mengunci kepercayaan pada direktori tempat Anda memulainya, dan kepercayaan mencakup subdirektori apa pun dari direktori tersebut kecuali repositori git yang bersarang di dalamnya, seperti klon. Setiap subdirektori yang tercakup kemudian dihitung sebagai folder yang induknya Anda percayai.
* Ketika Anda memulai di direktori home Anda, Claude Code menyimpan kepercayaan hanya untuk sesi saat ini dan tidak menulisnya ke disk; lihat catatan [safeguard tambahan](/docs/id/security#additional-safeguards).

Claude Code menampilkan dialog kepercayaan hanya dalam sesi interaktif. Jalankan `claude -p` atau sesi SDK tidak pernah menampilkannya, dan mempercayai folder induk tidak dihitung untuk aturan ini, jadi [Apa yang berjalan sebelum Anda mempercayai folder](#what-runs-before-you-trust-a-folder) mengatakan konten repositori mana yang masih digunakan Claude Code dalam masing-masing dari dua situasi tersebut.

<h3 id="when-your-local-settings-file-needs-trust">
  Ketika file pengaturan lokal Anda memerlukan kepercayaan
</h3>

`.claude/settings.local.json` biasanya adalah file Anda sendiri, jadi Claude Code menerapkan aturan izin dan direktori tambahannya tanpa langkah kepercayaan. Ketika file dilacak dalam git, atau `.claude` adalah symlink, Claude Code memperlakukannya sebagai disediakan repositori sebagai gantinya dan menahan aturannya sampai Anda mempercayai folder.

Claude Code menjalankan git untuk membedakan keduanya, dan hanya menjalankan git setelah Anda telah mempercayai folder: Anda menerima dialog kepercayaan untuk itu atau untuk direktori induk yang kepercayaannya meluas ke itu, atau Anda berada dalam sesi `-p` atau SDK, yang dihitung sebagai diterima. Sampai saat itu, tempat Anda memulai Claude Code menentukan apa yang terjadi pada aturan file:

* **Di home konfigurasi Anda:** Claude Code menerapkan `.claude/settings.local.json` folder tersebut segera tanpa menjalankan git. Home konfigurasi Anda adalah direktori home Anda, atau direktori yang subdirektori `.claude` Anda telah atur sebagai [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars#variables). Jika direktori `CLAUDE_CONFIG_DIR` tersebut berada di dalam repositori git dan Claude Code [menyimpan pengaturan lokal Anda di akar repositori](/docs/id/settings#where-claude-code-looks-for-each-file) sebagai gantinya, ia menahan aturannya seperti di tempat lain.
* **Di tempat lain:** Claude Code menahan aturan file seperti pengaturan proyek. Setelah pemeriksaan telah berjalan, Claude Code menerapkan aturan file yang tidak dilacak, atau file dalam direktori di luar repositori git apa pun, meskipun Anda belum mempercayai folder yang tepat itu.

<Note>
  Pengecualian home konfigurasi melewati hanya langkah kepercayaan. `~/.claude/settings.local.json` masih [cakupan lokal](/docs/id/settings#compare-the-scope-of-each-settings-file), jadi Claude Code membacanya hanya dalam sesi yang Anda mulai di direktori home Anda sendiri, bukan di setiap proyek. Untuk menerapkan aturan izin di semua proyek Anda, tambahkan itu ke pengaturan pengguna Anda sebagai gantinya: `~/.claude/settings.json`, atau `$CLAUDE_CONFIG_DIR/settings.json` ketika `CLAUDE_CONFIG_DIR` diatur.
</Note>

Pada versi 2.1.196 hingga 2.1.199, Claude Code menahan aturan file di home konfigurasi Anda dan di luar repositori git juga, dan mencetak peringatan [`this workspace has not been trusted`](/docs/id/errors#workspace-has-not-been-trusted) di sana. Sebelum v2.1.207, Claude Code menerapkan aturan file yang tidak dilacak sebelum Anda menerima dialog.

<h3 id="what-runs-before-you-trust-a-folder">
  Apa yang berjalan sebelum Anda mempercayai folder
</h3>

Setiap baris adalah satu jenis konten yang dapat disediakan repositori. Kolom adalah dua situasi di mana Anda belum mempercayai folder itu sendiri: Anda hanya mempercayai folder induk, atau Anda menjalankan `claude -p` atau SDK di sana, yang tidak pernah menampilkan dialog kepercayaan. Kolom folder induk tidak berlaku di dalam [repositori bersarang](#project-allow-rules-and-workspace-trust): dalam sesi interaktif Claude Code menampilkan dialog kepercayaan untuk itu, dan jalankan `claude -p` atau SDK di sana mengikuti kolom `claude -p`.

| Apa yang disediakan repositori                                                                                                                                                                                                                                                                                  | Anda hanya mempercayai folder induk                                                                                                                                                              | `claude -p` atau SDK, folder tidak pernah dipercaya                                                                                                                                                                    |
| :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Hooks](/docs/id/hooks) dalam file pengaturan, blok [`env`](/docs/id/settings-reference#env) dan perintah pembantu seperti [`apiKeyHelper`](/docs/id/settings-reference#apikeyhelper), dan [hooks](/docs/id/hooks#hooks-in-skills-and-agents) skill proyek dan [`allowed-tools`](/docs/id/skills#pre-approve-tools-for-a-skill)          | Digunakan                                                                                                                                                                                        | Digunakan. Kepercayaan ruang kerja tidak pernah membatasi `allowed-tools` skill dalam sesi apa pun                                                                                                                     |
| Aturan `permissions.allow` dan `additionalDirectories` dalam `.claude/settings.json`                                                                                                                                                                                                                            | Tidak digunakan sampai Anda menerima dialog kepercayaan, yang muncul lagi mencantumkannya                                                                                                        | Tidak digunakan. Claude Code mencetak peringatan [`this workspace has not been trusted`](/docs/id/errors#workspace-has-not-been-trusted) ke stderr                                                                          |
| Hooks frontmatter dalam [subagent](/docs/id/sub-agents#hooks-in-subagent-frontmatter) proyek, plugin [`@skills-dir`](/docs/id/plugins/loading#plugins-shared-through-a-repository) proyek, dan entri [`extraKnownMarketplaces`](/docs/id/settings-reference#extraknownmarketplaces) dari repositori atau direktori `--add-dir` | Tidak digunakan, dan tidak ada dialog yang ditawarkan                                                                                                                                            | Tidak digunakan                                                                                                                                                                                                        |
| [`mcpServers`](/docs/id/sub-agents#scope-mcp-servers-to-a-subagent) inline dalam frontmatter subagent dari repositori atau direktori `--add-dir`. Sebelum v2.1.238, Claude Code memuat server ini dalam kedua situasi                                                                                                | Tidak digunakan, dan tidak ada dialog yang ditawarkan                                                                                                                                            | Tidak digunakan                                                                                                                                                                                                        |
| Server dalam `.mcp.json`, termasuk yang [disetujui repositori dalam pengaturannya sendiri](/docs/id/mcp#project-server-approvals-and-workspace-trust)                                                                                                                                                                | Claude Code menanyakan Anda sebelum menghubungkannya. Persetujuan repositori itu sendiri tidak dihitung                                                                                          | Terhubung tanpa bertanya, disetujui atau tidak. SDK memuat mereka hanya ketika `settingSources` mencakup pengaturan proyek. `claude mcp list` di folder yang sama masih melaporkan server seperti itu sebagai tertunda |
| [`headersHelper`](/docs/id/mcp#trust-a-folder-before-its-headershelper-runs) pada server dalam `.mcp.json`. Sebelum v2.1.238, Claude Code menjalankan pembantu dalam kedua situasi                                                                                                                                   | Tidak dijalankan sampai Anda menerima dialog kepercayaan, yang muncul lagi menamai tempat pembantu dideklarasikan. Claude Code menghubungkan server dengan `headers` statis saja sampai saat itu | Tidak dijalankan. Claude Code menghubungkan server dengan `headers` statis saja dan mencetak baris [`headersHelper not run`](/docs/id/errors#headershelper-not-run) per server ke stderr                                    |

Untuk baris yang memerlukan folder yang tepat ini dipercaya, percayai dengan tangan: atur `projects["<path>"].hasTrustDialogAccepted` ke `true` dalam `~/.claude.json`, di mana `<path>` adalah akar repositori, atau folder itu sendiri di luar repositori. Claude Code mencetak kunci yang tepat dalam baris log debug untuk hook subagent yang dilewati atau server MCP inline, dalam peringatan stderr untuk aturan izin yang dilewati, dan dalam baris `headersHelper not run` untuk pembantu yang dilewati.

Sebelum Anda menjalankan `claude -p` dalam repositori yang tidak Anda tulis, putuskan apa yang mungkin dijalankannya di mesin Anda:

* Berikan `--setting-sources user`, atau atur `settingSources` SDK tanpa pengaturan proyek, jadi Claude Code tidak membaca file pengaturan proyek atau `.mcp.json` nya
* Mulai dengan [`--bare`](/docs/id/headless#start-faster-with-bare-mode) jadi Claude Code tidak membaca hooks, skills, perintah khusus, subagents, plugins, atau server `.mcp.json` dari proyek. Blok `env` proyek dan pembantu seperti `awsAuthRefresh` dalam file pengaturannya masih berlaku, dan Claude Code membaca `apiKeyHelper` hanya dari `--settings`
* Berikan `--settings '{"disableAllHooks": true}'` untuk [mematikan hooks](/docs/id/hooks#disable-or-remove-hooks) untuk jalankan itu. Menetapkannya dalam pengaturan pengguna Anda saja tidak cukup, karena pengaturan proyek repositori mengambil alih milik Anda dan dapat menetapkannya kembali ke `false`
* Tambahkan entri [`disabledMcpjsonServers`](/docs/id/settings-reference#disabledmcpjsonservers) untuk menolak server `.mcp.json` berdasarkan nama dalam setiap jenis sesi

<h2 id="example-configurations">
  Contoh konfigurasi
</h2>

[Repositori](https://github.com/anthropics/claude-code/tree/main/examples/settings) ini mencakup konfigurasi pengaturan pemula untuk skenario penerapan umum. Gunakan ini sebagai titik awal dan sesuaikan dengan kebutuhan Anda.

<h2 id="see-also">
  Lihat juga
</h2>

* [Semua pengaturan](/docs/id/settings-reference#permission-settings): setiap kunci pengaturan, termasuk kunci izin
* [Konfigurasi mode auto](/docs/id/auto-mode-config): beri tahu pengklasifikasi mode auto infrastruktur mana yang dipercaya organisasi Anda
* [Sandboxing](/docs/id/sandboxing): isolasi sistem file dan jaringan tingkat OS untuk perintah Bash
* [Authentication](/docs/id/authentication): atur akses pengguna ke Claude Code
* [Security](/docs/id/security): perlindungan keamanan dan praktik terbaik
* [Hooks](/docs/id/hooks-guide): otomatisasi alur kerja dan perluas evaluasi izin
