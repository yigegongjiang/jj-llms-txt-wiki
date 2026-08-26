> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Konfigurasi izin

> Kontrol apa yang dapat diakses Claude Code dan lakukan dengan aturan izin terperinci, mode, dan kebijakan terkelola.

Claude Code mendukung izin terperinci sehingga Anda dapat menentukan dengan tepat apa yang diizinkan dilakukan oleh agen dan apa yang tidak. Pengaturan izin dapat diperiksa ke dalam kontrol versi dan didistribusikan ke semua pengembang di organisasi Anda, serta disesuaikan oleh pengembang individual.

<h2 id="permission-system">
  Sistem izin
</h2>

Claude Code menggunakan sistem izin berjenjang untuk menyeimbangkan kekuatan dan keamanan:

| Jenis alat      | Contoh               | Persetujuan diperlukan                                                           | Perilaku "Ya, jangan tanya lagi"                  |
| :-------------- | :------------------- | :------------------------------------------------------------------------------- | :------------------------------------------------ |
| Hanya baca      | Pembacaan file, Grep | Tidak, dalam [direktori kerja dan direktori tambahan](#working-directories)      | T/A                                               |
| Perintah Bash   | Eksekusi shell       | Ya, kecuali serangkaian [perintah hanya baca](#read-only-commands) yang tertanam | Secara permanen per direktori proyek dan perintah |
| Modifikasi file | Edit/tulis file      | Ya                                                                               | Hingga akhir sesi                                 |

Pada prompt izin Bash atau PowerShell, tekan `Ctrl+E` untuk menampilkan penjelasan perintah: apa yang dilakukannya, mengapa Claude menjalankannya, dan apa yang bisa salah, diberi label **Risiko rendah**, **Risiko sedang**, atau **Risiko tinggi**. Claude Code mengirimkan perintah dan deskripsi Claude sendiri tentang panggilan tersebut ke model untuk menghasilkan penjelasan hanya ketika Anda menekan `Ctrl+E`, bukan pada setiap prompt. Menampilkan penjelasan tidak menjalankan perintah; tekan `Ctrl+E` lagi untuk menyembunyikannya.

Untuk mematikan pintasan, atur [`permissionExplainerEnabled`](/docs/id/settings#global-config-settings) ke `false` dalam `~/.claude.json`.

<h2 id="manage-permissions">
  Kelola izin
</h2>

Anda dapat melihat dan mengelola izin alat Claude Code dengan `/permissions`. UI ini mencantumkan semua aturan izin dan file `settings.json` tempat mereka bersumber.

* Aturan **Allow** memungkinkan Claude Code menggunakan alat yang ditentukan tanpa persetujuan manual.
* Aturan **Ask** meminta konfirmasi setiap kali Claude Code mencoba menggunakan alat yang ditentukan.
* Aturan **Deny** mencegah Claude Code menggunakan alat yang ditentukan.

Aturan dievaluasi secara berurutan: deny, kemudian ask, kemudian allow. Kecocokan pertama dalam urutan tersebut menentukan hasilnya, dan spesifisitas aturan tidak mengubah urutan.

Aturan deny yang luas seperti `Bash(aws *)` memblokir setiap panggilan yang cocok, termasuk panggilan yang juga cocok dengan aturan allow yang lebih sempit seperti `Bash(aws s3 ls)`, jadi aturan deny tidak dapat membawa pengecualian daftar izin. Prioritas yang sama berlaku antara ask dan allow: aturan ask yang cocok meminta konfirmasi bahkan ketika aturan allow yang lebih spesifik juga cocok dengan panggilan yang sama.

Aturan deny berperilaku berbeda tergantung pada apakah mereka menamai alat atau membatasi pola dalam satu alat. Nama alat biasa seperti `Bash` menghapus alat dari konteks Claude sepenuhnya, jadi Claude tidak pernah melihatnya. Aturan yang dibatasi seperti `Bash(rm *)` membiarkan alat tersedia dan memblokir panggilan yang cocok ketika Claude mencoba menggunakannya.

<Note>
  Aturan izin ditegakkan oleh Claude Code, bukan oleh model. Instruksi dalam prompt Anda atau `CLAUDE.md` membentuk apa yang Claude coba lakukan, tetapi mereka tidak mengubah apa yang Claude Code izinkan. Untuk memberikan atau mencabut akses, gunakan `/permissions`, aturan yang dijelaskan di sini, [mode izin](/docs/id/permission-modes), atau [hook PreToolUse](#extend-permissions-with-hooks).
</Note>

<h2 id="permission-modes">
  Mode izin
</h2>

Claude Code mendukung beberapa mode izin yang mengontrol bagaimana alat disetujui. Lihat [Permission modes](/docs/id/permission-modes) untuk mengetahui kapan menggunakan masing-masing. Atur `defaultMode` dalam [file pengaturan](/docs/id/settings#settings-files) Anda:

| Mode                | Deskripsi                                                                                                                                                                                                                                                                                                                                                                         |
| :------------------ | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `default`           | Perilaku standar: meminta izin pada penggunaan pertama setiap alat. Berlabel Manual di CLI, ekstensi VS Code dan JetBrains, dan aplikasi desktop, dan Claude Code menerima `manual` sebagai alias. Label dan alias memerlukan Claude Code v2.1.200 atau lebih baru. Label aplikasi desktop tidak bergantung pada versi CLI Anda                                                   |
| `acceptEdits`       | Secara otomatis menerima edit file dan perintah sistem file umum seperti `mkdir`, `touch`, `mv`, dan `cp` untuk jalur di direktori kerja atau `additionalDirectories`                                                                                                                                                                                                             |
| `plan`              | Claude membaca file dan menjalankan perintah shell hanya-baca untuk menjelajahi tetapi tidak mengedit file sumber Anda. Berlabel Plan di CLI dan ekstensi VS Code                                                                                                                                                                                                                 |
| `auto`              | Secara otomatis menyetujui panggilan alat dengan pemeriksaan keamanan latar belakang yang memverifikasi tindakan selaras dengan permintaan Anda                                                                                                                                                                                                                                   |
| `dontAsk`           | Secara otomatis menolak alat kecuali pra-disetujui melalui `/permissions` atau aturan `permissions.allow`. `AskUserQuestion`, alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) ditolak bahkan jika Anda telah mengizinkannya |
| `bypassPermissions` | Melewati prompt izin, kecuali yang dipaksa oleh aturan `ask` eksplisit, alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool). Penghapusan direktori root dan home seperti `rm -rf /` juga masih meminta sebagai circuit breaker  |

<Warning>
  Mode `bypassPermissions` melewati prompt izin, termasuk untuk penulisan ke `.git`, `.config/git`, `.claude`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`, `.yarn`, dan `.mvn`. Hanya gunakan mode ini di lingkungan terisolasi seperti kontainer atau VM tempat Claude Code tidak dapat menyebabkan kerusakan.

  Beberapa prompt masih aktif dalam mode ini. Aturan `ask` eksplisit, alat konektor [organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools), dan alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) masih meminta. Penghapusan yang menargetkan akar sistem file atau direktori home, seperti `rm -rf /` dan `rm -rf ~`, juga meminta sebagai circuit breaker terhadap kesalahan model, termasuk ketika perintah berisi substitusi perintah dengan `$(...)` atau backtick, atau substitusi proses dengan `<(...)`. Sebelum v2.1.208, hanya bentuk biasa, seperti `rm -rf ~` yang diketik sebagai perintahnya sendiri, yang meminta; perintah yang mencapai penghapusan melalui substitusi tidak.
</Warning>

Untuk mencegah mode `bypassPermissions` atau `auto` digunakan, atur `permissions.disableBypassPermissionsMode` atau `permissions.disableAutoMode` ke `"disable"` dalam [file pengaturan](/docs/id/settings#settings-files) apa pun. Ini paling berguna dalam [pengaturan terkelola](#managed-settings) di mana mereka tidak dapat ditimpa.

<h2 id="permission-rule-syntax">
  Sintaks aturan izin
</h2>

Aturan izin mengikuti format `Tool` atau `Tool(specifier)`.

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

Aturan penolakan dan tanya dapat mencocokkan parameter input tingkat atas pada alat apa pun dengan `Tool(param:value)`. Aturan cocok ketika Claude memanggil alat dengan parameter tersebut diatur ke nilai yang tepat. Aturan izin untuk satu nilai parameter tidak akan menetapkan bahwa panggilan aman secara keseluruhan, jadi aturan izin terus menggunakan sintaks specifier masing-masing alat. Ini berfungsi untuk parameter skalar apa pun yang diterima alat:

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

Bidang yang sudah dicocokkan alat dengan aturan canonicalizing miliknya sendiri tidak dapat dicocokkan dengan cara ini: `command` untuk Bash dan PowerShell, `file_path` untuk Read, Edit, dan Write, `path` untuk Grep dan Glob, `notebook_path` untuk NotebookEdit, dan `url` untuk WebFetch. Aturan seperti `Bash(command:rm *)` dapat dilewati oleh perintah gabungan, jadi Claude Code mengabaikannya dan mengeluarkan peringatan startup. Gunakan `Bash(rm *)`, `Read(./path)`, atau `WebFetch(domain:host)` sebagai gantinya.

<h3 id="wildcard-patterns">
  Pola wildcard
</h3>

Aturan Bash mendukung pola glob dengan `*`. Wildcard dapat muncul di posisi mana pun dalam perintah. Konfigurasi ini memungkinkan perintah npm dan git commit sambil memblokir git push:

```json theme={null}
{
  "permissions": {
    "allow": [
      "Bash(npm run *)",
      "Bash(git commit *)",
      "Bash(git * main)",
      "Bash(* --version)",
      "Bash(* --help *)"
    ],
    "deny": [
      "Bash(git push *)"
    ]
  }
}
```

Spasi sebelum `*` penting: `Bash(ls *)` mencocokkan `ls -la` tetapi bukan `lsof`, sementara `Bash(ls*)` mencocokkan keduanya. Akhiran `:*` adalah cara setara untuk menulis wildcard trailing, jadi `Bash(ls:*)` mencocokkan perintah yang sama dengan `Bash(ls *)`.

Dialog izin menulis bentuk yang dipisahkan spasi ketika Anda memilih "Ya, jangan tanya lagi" untuk awalan perintah. Bentuk `:*` hanya dikenali di akhir pola. Dalam pola seperti `Bash(git:* push)`, titik dua diperlakukan sebagai karakter literal dan tidak akan mencocokkan perintah git.

<h3 id="tool-name-wildcards">
  Wildcard nama alat
</h3>

Aturan penolakan dan tanya juga menerima pola glob dalam posisi nama alat. Pola harus cocok dengan nama alat lengkap: `"*"` cocok dengan setiap alat, dan `"mcp__*"` cocok dengan setiap alat MCP di semua server. Alat yang cocok dengan aturan penolakan nama telanjang dihapus dari konteks Claude, sama seperti nama alat telanjang. Konfigurasi ini menolak setiap alat MCP:

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

Aturan penolakan atau tanya yang nama alatnya tidak cocok dengan alat yang dikenal menghasilkan peringatan startup untuk menangkap kesalahan ketik. Nama alat yang berisi `_` atau `*` dikecualikan dari pemeriksaan.

Label yang ditampilkan untuk alat dalam transkrip dan dialog izin dapat berbeda dari nama kanoniknya. Misalnya, alat yang diberi label `Stop Task` dalam transkrip memiliki nama kanonik `TaskStop`. Aturan izin dan [pencocokan hook](/docs/id/hooks) hanya cocok dengan nama kanonik, jadi aturan yang ditulis sebagai `Stop Task` tidak cocok. Untuk aturan penolakan dan tanya, peringatan startup di atas menangkap ketidaksesuaian. Gunakan nama kanonik yang tercantum dalam [referensi alat](/docs/id/tools-reference).

<h2 id="tool-specific-permission-rules">
  Aturan izin khusus alat
</h2>

<h3 id="bash">
  Bash
</h3>

Aturan izin Bash mendukung pencocokan wildcard dengan `*`. Wildcard dapat muncul di posisi mana pun dalam perintah, termasuk di awal, tengah, atau akhir:

* `Bash(npm run build)` mencocokkan perintah Bash yang tepat `npm run build`
* `Bash(npm run test *)` mencocokkan perintah Bash yang dimulai dengan `npm run test`
* `Bash(npm *)` mencocokkan perintah apa pun yang dimulai dengan `npm `
* `Bash(* install)` mencocokkan perintah apa pun yang berakhir dengan ` install`
* `Bash(git * main)` mencocokkan perintah seperti `git checkout main` dan `git log --oneline main`

Satu `*` mencocokkan urutan karakter apa pun termasuk spasi, jadi satu wildcard dapat mencakup beberapa argumen. `Bash(git *)` mencocokkan `git log --oneline --all`, dan `Bash(git * main)` mencocokkan `git push origin main` serta `git merge main`.

Ketika `*` muncul di akhir dengan spasi sebelumnya (seperti `Bash(ls *)`), ini memberlakukan batas kata, memerlukan awalan diikuti oleh spasi atau akhir string. Misalnya, `Bash(ls *)` mencocokkan `ls -la` tetapi bukan `lsof`. Sebaliknya, `Bash(ls*)` tanpa spasi mencocokkan `ls -la` dan `lsof` karena tidak ada batasan batas kata.

<h4 id="compound-commands">
  Perintah gabungan
</h4>

<Tip>
  Claude Code menyadari operator shell, jadi aturan seperti `Bash(safe-cmd *)` tidak akan memberinya izin untuk menjalankan perintah `safe-cmd && other-cmd`. Pemisah perintah yang dikenali adalah `&&`, `||`, `;`, `|`, `|&`, `&`, dan baris baru. Aturan harus mencocokkan setiap subperintah secara independen.
</Tip>

Ketika Anda menyetujui perintah gabungan dengan "Ya, jangan tanya lagi", Claude Code menyimpan aturan terpisah untuk setiap subperintah yang memerlukan persetujuan, bukan satu aturan untuk string gabungan lengkap. Misalnya, menyetujui `git status && npm test` menyimpan aturan untuk `npm test`, jadi invokasi `npm test` di masa depan dikenali terlepas dari apa yang mendahului `&&`. Subperintah seperti `cd` ke subdirektori menghasilkan aturan Read mereka sendiri untuk jalur itu. Hingga 5 aturan dapat disimpan untuk satu perintah gabungan.

<h4 id="process-wrappers">
  Pembungkus proses
</h4>

Sebelum mencocokkan aturan Bash, Claude Code menghilangkan serangkaian pembungkus proses tetap sehingga aturan seperti `Bash(npm test *)` juga mencocokkan `timeout 30 npm test`. Pembungkus yang dikenali adalah `timeout`, `time`, `nice`, `nohup`, dan `stdbuf`.

`xargs` telanjang juga dihilangkan, jadi `Bash(grep *)` mencocokkan `xargs grep pattern`. Penghilangan hanya berlaku ketika `xargs` tidak memiliki flag: invokasi seperti `xargs -n1 grep pattern` dicocokkan sebagai perintah `xargs`, jadi aturan yang ditulis untuk perintah inner tidak mencakupnya.

Daftar pembungkus ini bawaan dan tidak dapat dikonfigurasi. Pelari lingkungan pengembangan seperti `direnv exec`, `devbox run`, `mise exec`, `npx`, dan `docker exec` tidak ada dalam daftar. Karena alat ini menjalankan argumen mereka sebagai perintah, aturan seperti `Bash(devbox run *)` mencocokkan apa pun yang datang setelah `run`, termasuk `devbox run rm -rf .`. Untuk menyetujui pekerjaan di dalam pelari lingkungan, tulis aturan spesifik yang mencakup baik pelari maupun perintah inner, seperti `Bash(devbox run npm test)`. Tambahkan satu aturan per perintah inner yang ingin Anda izinkan.

Pembungkus exec seperti `watch`, `setsid`, `ionice`, dan `flock` selalu meminta dan tidak dapat disetujui otomatis oleh aturan awalan seperti `Bash(watch *)`. Hal yang sama berlaku untuk `find` dengan `-exec` atau `-delete`: aturan `Bash(find *)` tidak mencakup bentuk ini. Untuk menyetujui invokasi spesifik, tulis aturan pencocokan tepat untuk string perintah lengkap.

<h4 id="read-only-commands">
  Perintah hanya baca
</h4>

Claude Code mengenali serangkaian perintah Bash bawaan sebagai hanya baca dan menjalankannya tanpa prompt izin di setiap mode. Ini termasuk `ls`, `cat`, `echo`, `pwd`, `head`, `tail`, `grep`, `find`, `wc`, `which`, `diff`, `stat`, `du`, `cd`, dan bentuk hanya baca dari `git`. Serangkaian ini tidak dapat dikonfigurasi; untuk memerlukan prompt untuk salah satu perintah ini, tambahkan aturan `ask` atau `deny` untuk itu.

Pola glob yang tidak dikutip diizinkan untuk perintah yang setiap flagnya hanya baca, jadi `ls *.ts` dan `wc -l src/*.py` berjalan tanpa prompt. Perintah dengan flag yang mampu menulis atau exec, seperti `find`, `sort`, `sed`, dan `git`, masih meminta ketika glob yang tidak dikutip ada karena glob dapat berkembang menjadi flag seperti `-delete`.

`cd` ke jalur di dalam direktori kerja Anda atau [direktori tambahan](#working-directories) juga hanya baca. Perintah gabungan seperti `cd packages/api && ls` berjalan tanpa prompt ketika setiap bagian memenuhi syarat sendiri. Menggabungkan `cd` dengan `git` dalam satu perintah gabungan meminta ketika `cd` berubah ke direktori yang berbeda, karena menjalankan `git` di direktori baru dapat menjalankan hook direktori itu. `cd` yang targetnya diselesaikan ke direktori kerja saat ini adalah no-op dan tidak memicu prompt ini.

Menggabungkan `cd` dengan pengalihan output dalam satu perintah gabungan juga meminta ketika Claude Code tidak dapat menentukan direktori mana target pengalihan diselesaikan setelah `cd` berjalan. Perintah yang satu-satunya target pengalihan adalah `/dev/null`, seperti `cd app; grep -r pattern . 2>/dev/null`, tidak memicu prompt ini, karena `/dev/null` tidak bergantung pada direktori kerja.

<Warning>
  Pola izin Bash yang mencoba membatasi argumen perintah rapuh. Misalnya, `Bash(curl http://github.com/ *)` dimaksudkan untuk membatasi curl ke URL GitHub, tetapi tidak akan mencocokkan variasi seperti:

  * Opsi sebelum URL: `curl -X GET http://github.com/...`
  * Protokol berbeda: `curl https://github.com/...`
  * Pengalihan: `curl -L http://bit.ly/xyz`, yang mengalihkan ke GitHub
  * Variabel: `URL=http://github.com && curl $URL`
  * Spasi ekstra: `curl  http://github.com`

  Untuk penyaringan URL yang lebih andal, pertimbangkan:

  * **Batasi alat jaringan Bash**: gunakan aturan deny untuk memblokir `curl`, `wget`, dan perintah serupa, kemudian gunakan alat WebFetch dengan izin `WebFetch(domain:github.com)` untuk domain yang diizinkan
  * **Gunakan hook PreToolUse**: implementasikan hook yang memvalidasi URL dalam perintah Bash dan memblokir domain yang tidak diizinkan
  * **Tambahkan panduan CLAUDE.md**: jelaskan pola curl yang diizinkan Anda di `CLAUDE.md`. Ini membentuk apa yang Claude coba tetapi tidak memberlakukan batas, jadi pasangkan dengan salah satu opsi di atas

  Perhatikan bahwa menggunakan WebFetch saja tidak mencegah akses jaringan. Jika Bash diizinkan, Claude masih dapat menggunakan `curl`, `wget`, atau alat lain untuk menjangkau URL apa pun.
</Warning>

<h3 id="powershell">
  PowerShell
</h3>

Aturan izin PowerShell menggunakan bentuk yang sama dengan aturan Bash. Wildcard dengan `*` cocok di posisi mana pun, akhiran `:*` setara dengan trailing ` *`, dan PowerShell telanjang atau `PowerShell(*)` cocok dengan setiap perintah. Konfigurasi ini memungkinkan perintah `Get-ChildItem` dan `git commit` sambil memblokir `Remove-Item`:

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

Alias umum dikanonikalisasi sebelum pencocokan. Aturan yang ditulis untuk nama cmdlet juga cocok dengan aliasnya, jadi `PowerShell(Get-ChildItem *)` cocok dengan `gci`, `ls`, dan `dir` juga. Pencocokan tidak peka huruf besar-kecil.

Claude Code mengurai AST PowerShell dan memeriksa setiap perintah dalam perintah gabungan secara independen. Operator pipeline `|`, pemisah pernyataan `;`, dan pada PowerShell 7+ operator rantai `&&` dan `||` membagi perintah gabungan menjadi subperintah. Aturan harus cocok dengan setiap subperintah agar perintah gabungan diizinkan.

<h3 id="read-and-edit">
  Read dan Edit
</h3>

Aturan `Edit` berlaku untuk semua alat bawaan yang mengedit file. Claude membuat upaya terbaik untuk menerapkan aturan `Read` ke semua alat bawaan yang membaca file seperti Grep dan Glob, ke penyebutan `@file` dalam prompt Anda, dan ke seleksi dan konteks file terbuka yang [IDE](/docs/id/vs-code#the-built-in-ide-mcp-server) yang terhubung bagikan dengan Claude.

Aturan deny `Read` juga memblokir [alat Edit](/docs/id/errors#file-is-covered-by-a-read-deny-rule) pada jalur yang sama, termasuk membuat file baru di sana. Write dan NotebookEdit tidak tercakup, jadi tambahkan aturan deny `Edit` untuk jalur yang tidak boleh diubah alat apa pun. Memerlukan Claude Code v2.1.208 atau lebih baru.

<Warning>
  Aturan deny Read dan Edit berlaku untuk alat file bawaan Claude dan untuk perintah file yang Claude Code kenali di Bash, seperti `cat`, `head`, `tail`, dan `sed`. Mereka tidak berlaku untuk subproses arbitrer yang membaca atau menulis file secara tidak langsung, seperti skrip Python atau Node yang membuka file itu sendiri. Untuk penegakan tingkat OS yang memblokir semua proses dari mengakses jalur, [aktifkan sandbox](/docs/id/sandboxing).
</Warning>

Aturan Read dan Edit keduanya mengikuti spesifikasi [gitignore](https://git-scm.com/docs/gitignore) dengan empat jenis pola yang berbeda:

| Pola                 | Arti                                      | Contoh                           | Cocok                                                |
| -------------------- | ----------------------------------------- | -------------------------------- | ---------------------------------------------------- |
| `//path`             | Jalur absolut dari akar sistem file       | `Read(//Users/alice/secrets/**)` | `/Users/alice/secrets/**`                            |
| `~/path`             | Jalur dari direktori home                 | `Read(~/Documents/*.pdf)`        | `/Users/alice/Documents/*.pdf`                       |
| `/path`              | Jalur relatif terhadap sumber pengaturan  | `Edit(/src/**/*.ts)`             | `<project root>/src/**/*.ts` dalam pengaturan proyek |
| `path` atau `./path` | Jalur relatif terhadap direktori saat ini | `Read(*.env)`                    | `<cwd>/*.env`                                        |

<Warning>
  Pola seperti `/Users/alice/file` bukan jalur absolut. Garis miring tunggal terkemuka berlabuh di sumber pengaturan, bukan akar sistem file. Gunakan `//Users/alice/file` untuk jalur absolut.
</Warning>

Pola `/path` berlabuh di direktori yang terkait dengan file pengaturan yang mendefinisikannya, jadi aturan yang sama cocok dengan lokasi berbeda tergantung di mana Anda menempatkannya:

| Aturan didefinisikan dalam                                    | `/path` diselesaikan ke    |
| :------------------------------------------------------------ | :------------------------- |
| Pengaturan proyek atau lokal, seperti `.claude/settings.json` | `<project root>/path`      |
| Pengaturan pengguna di `~/.claude/settings.json`              | `~/.claude/path`           |
| File yang dilewatkan dengan `--settings <file>`               | `<directory of file>/path` |
| Flag CLI, `/permissions`, atau aturan sesi                    | `<original cwd>/path`      |

Aturan deny seperti `Read(/secrets/**)` dalam pengaturan pengguna memblokir `~/.claude/secrets/**`, bukan direktori `secrets` dalam proyek Anda. Untuk menulis aturan dalam pengaturan pengguna yang berlaku di dalam setiap proyek, gunakan jalur absolut `//` atau jalur relatif home `~/` sebagai gantinya.

Di Windows, jalur dinormalisasi ke bentuk POSIX sebelum pencocokan. `C:\Users\alice` menjadi `/c/Users/alice`, jadi gunakan `//c/**/.env` untuk mencocokkan file `.env` di mana pun di drive itu. Untuk mencocokkan di semua drive, gunakan `//**/.env`.

Contoh:

* `Edit(/docs/**)`: edit di `<project>/docs/`, bukan `/docs/` atau `<project>/.claude/docs/`
* `Read(~/.zshrc)`: membaca `.zshrc` direktori home Anda
* `Edit(//tmp/scratch.txt)`: edit jalur absolut `/tmp/scratch.txt`
* `Read(src/**)`: membaca dari `<current-directory>/src/`

Aturan hanya cocok dengan file di bawah jangkarannya, jadi jangkar menentukan seberapa jauh aturan deny mencapai. Nama file telanjang mengikuti semantik gitignore dan cocok di kedalaman apa pun, jadi `Read(.env)` dan `Read(**/.env)` setara:

| Aturan deny                       | Memblokir                                          | Tidak memblokir                                    |
| --------------------------------- | -------------------------------------------------- | -------------------------------------------------- |
| `Read(.env)` atau `Read(**/.env)` | `.env` apa pun di atau di bawah direktori saat ini | `.env` di direktori induk atau proyek lain         |
| `Read(//**/.env)`                 | `.env` apa pun di mana pun di sistem file          | tidak ada; aturan ini berlabuh di akar sistem file |

<Note>
  Dalam pola gitignore, `*` mencocokkan dalam satu segmen jalur dan dapat muncul di posisi mana pun dalam pola, sementara `**` mencocokkan di seluruh direktori. Untuk memungkinkan semua akses file, gunakan hanya nama alat tanpa tanda kurung: `Read`, `Edit`, atau `Write`.
</Note>

Ketika Anda menyetujui jalur file dengan "Ya, jangan tanya lagi", Claude Code menghindari karakter pola gitignore dalam jalur itu, seperti `[`, `]`, dan `*`, jadi aturan yang dihasilkan hanya cocok dengan jalur literal yang Anda setujui. Aturan yang Anda tulis sendiri tidak dihindari.

Ketika Claude mengakses symlink, aturan izin memeriksa dua jalur: symlink itu sendiri dan file yang diselesaikannya. Aturan allow dan deny memperlakukan pasangan itu secara berbeda: aturan allow kembali ke meminta Anda, sementara aturan deny memblokir sepenuhnya.

* **Aturan allow**: berlaku hanya ketika jalur symlink dan targetnya cocok. Symlink di dalam direktori yang diizinkan yang menunjuk ke luar masih meminta Anda.
* **Aturan deny**: berlaku ketika jalur symlink atau targetnya cocok. Symlink yang menunjuk ke file yang ditolak itu sendiri ditolak.

Misalnya, dengan `Read(./project/**)` diizinkan dan `Read(~/.ssh/**)` ditolak, symlink di `./project/key` menunjuk ke `~/.ssh/id_rsa` diblokir: target gagal aturan allow dan cocok dengan aturan deny.

<h3 id="webfetch">
  WebFetch
</h3>

Aturan WebFetch menggunakan awalan `domain:` dan cocok dengan nama host dari URL yang diminta. Pencocokan tidak peka huruf besar-kecil, mendukung wildcard `*`, dan menghilangkan titik trailing dari aturan dan nama host sehingga `example.com.` dan `example.com` diperlakukan sama.

* `WebFetch(domain:example.com)` mencocokkan permintaan ke `example.com`
* `WebFetch(domain:*.example.com)` mencocokkan subdomain apa pun di kedalaman apa pun, seperti `api.example.com` atau `a.b.example.com`, tetapi bukan `example.com` itu sendiri
* `WebFetch(domain:*)` mencocokkan setiap domain dan setara dengan aturan WebFetch telanjang

Di posisi mana pun selain `*.` terkemuka atau `*` telanjang, wildcard mencocokkan hanya teks antara dua titik. `WebFetch(domain:example.*)` mencocokkan `example.org`, di mana `*` menjadi `org`, tetapi bukan `example.evil.com`, di mana `*` harus menjadi `evil.com` dan melintasi titik. Ini mencegah wildcard trailing dari mencocokkan domain yang dapat didaftarkan penyerang.

<h3 id="mcp">
  MCP
</h3>

Aturan MCP menggunakan nama server seperti yang dikonfigurasi di Claude Code, secara opsional diikuti oleh nama alat dari server itu.

* `mcp__puppeteer` mencocokkan alat apa pun yang disediakan oleh server `puppeteer`
* `mcp__puppeteer__*` menggunakan sintaks wildcard dan juga mencocokkan semua alat dari server `puppeteer`
* `mcp__puppeteer__puppeteer_navigate` mencocokkan alat `puppeteer_navigate` yang disediakan oleh server `puppeteer`

Jika organisasi Anda telah menetapkan alat [konektor claude.ai](/docs/id/mcp#organization-controls-on-connector-tools) ke `ask`, aturan allow untuk alat itu tidak berlaku: Claude Code meminta pada setiap panggilan, bahkan dalam mode `auto` dan `bypassPermissions`. Dalam mode `dontAsk`, yang tidak pernah meminta, Claude Code menolak panggilan sebagai gantinya. Alat konektor muncul sebagai `mcp__claude_ai_<server>__<tool>`.

<h3 id="agent-subagents">
  Agent (subagents)
</h3>

Gunakan aturan `Agent(AgentName)` untuk mengontrol [subagents](/docs/id/sub-agents) mana yang dapat digunakan Claude:

* `Agent(Explore)` mencocokkan subagent Explore
* `Agent(Plan)` mencocokkan subagent Plan
* `Agent(my-custom-agent)` mencocokkan subagent kustom bernama `my-custom-agent`

Tambahkan aturan ini ke array `deny` dalam pengaturan Anda atau gunakan flag CLI `--disallowedTools` untuk menonaktifkan agen tertentu. Untuk menonaktifkan agen Explore:

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

Aturan `Cd` mengontrol direktori mana yang dapat dipindahkan oleh perintah [`/cd`](/docs/id/commands) ke sesi. `Cd` bukan alat yang dapat dipanggil model: Claude tidak dapat memanggilnya, dan aturan hanya berlaku ketika Anda menjalankan `/cd` sendiri.

Aturan deny `Cd` telanjang menonaktifkan `/cd` sepenuhnya. Aturan deny `Cd(<path-pattern>)` memblokir target yang cocok. Aturan deny memeriksa setiap ejaan target, termasuk setiap lompatan symlink yang diselesaikannya, jadi aturan yang ditulis untuk satu jalur juga memblokir target yang diselesaikan ke itu.

Menambahkan aturan allow `Cd` apa pun beralih `/cd` ke mode allowlist: direktori target yang diselesaikan harus cocok dengan salah satu aturan allow Anda, atau `/cd` menolak. Tanpa aturan `Cd` yang dikonfigurasi, `/cd` mempertahankan perilaku defaultnya dan meminta Anda untuk mempercayai direktori yang tidak dikenal.

Pola jalur berbagi jangkar `//`, `~/`, dan `/` dari [aturan Read dan Edit](#read-and-edit), tetapi pencocokan berlabuh ke seluruh jalur direktori daripada gaya gitignore. `*` mencocokkan tepat satu segmen jalur dan `**` mencocokkan di seluruh segmen. Trailing `/**` juga mencocokkan akar yang dinamainya.

| Aturan                | Cocok                                                 | Tidak cocok                |
| --------------------- | ----------------------------------------------------- | -------------------------- |
| `Cd(~/code/*)`        | `~/code/app`                                          | `~/code/app/src`, `~/code` |
| `Cd(~/code/**)`       | `~/code` dan direktori apa pun di bawahnya            | direktori di luar `~/code` |
| `Cd(**/node_modules)` | direktori `node_modules` apa pun di kedalaman apa pun | `node_modules/pkg`         |

<h2 id="extend-permissions-with-hooks">
  Perluas izin dengan hook
</h2>

[Hook Claude Code](/docs/id/hooks-guide) menyediakan cara untuk mendaftarkan perintah shell kustom guna melakukan evaluasi izin saat runtime. Ketika Claude Code membuat panggilan alat, hook PreToolUse berjalan sebelum prompt izin. Output hook dapat menolak panggilan alat, memaksa prompt, atau melewati prompt untuk membiarkan panggilan berlanjut.

Keputusan hook tidak melewati aturan izin. Claude Code mengevaluasi aturan deny dan ask terlepas dari apa yang dikembalikan hook PreToolUse: aturan deny yang cocok memblokir panggilan, dan aturan ask masih meminta bahkan ketika hook mengembalikan `"allow"` atau `"ask"`. Ini mempertahankan prioritas deny-first yang dijelaskan dalam [Kelola izin](#manage-permissions), termasuk aturan deny yang ditetapkan dalam pengaturan terkelola.

Alat Connector [yang organisasi Anda atur ke `ask`](/docs/id/mcp#organization-controls-on-connector-tools) dan alat MCP yang ditandai [`requiresUserInteraction`](/docs/id/mcp#require-approval-for-a-specific-tool) juga masih meminta ketika hook mengembalikan `"allow"`.

Hook pemblokiran juga memiliki prioritas atas aturan allow. Hook yang keluar dengan kode 2 menghentikan panggilan alat sebelum aturan izin dievaluasi, jadi blokir berlaku bahkan ketika aturan allow akan membiarkan panggilan berlanjut. Untuk menjalankan semua perintah Bash tanpa prompt kecuali untuk beberapa yang ingin Anda blokir, tambahkan `"Bash"` ke daftar allow Anda dan daftarkan hook PreToolUse yang menolak perintah tertentu itu. Lihat [Block edits to protected files](/docs/id/hooks-guide#block-edits-to-protected-files) untuk skrip hook yang dapat Anda sesuaikan.

<h2 id="working-directories">
  Direktori kerja
</h2>

Secara default, Claude memiliki akses ke file di direktori tempat diluncurkan. Anda dapat memperluas akses ini:

* **Saat startup**: gunakan argumen CLI `--add-dir <path>`
* **Selama sesi**: gunakan perintah `/add-dir`
* **Konfigurasi persisten**: tambahkan ke `additionalDirectories` dalam [file pengaturan](/docs/id/settings#settings-files)

File di direktori tambahan mengikuti aturan izin yang sama dengan direktori kerja asli: mereka menjadi dapat dibaca tanpa prompt, dan izin edit file mengikuti mode izin saat ini.

Dalam sesi latar belakang di macOS, host sesi meminta akses ke folder yang dilindungi seperti `~/Desktop`, `~/Documents`, dan `~/Downloads` secara terpisah dari terminal Anda ketika Claude perlu membaca atau menulis file di sana; jika pembacaan di sana gagal dengan `Operation not permitted`, lihat [cara memberikan akses folder ke sesi latar belakang](/docs/id/agent-view#background-sessions-can't-read-desktop-documents-or-downloads-on-macos).

Untuk mengubah direktori kerja utama sesi alih-alih menambahkan direktori lain, gunakan [`/cd`](/docs/id/commands). Perintah `/cd` memerlukan Claude Code v2.1.169 atau lebih baru. Tidak seperti `/add-dir`, perintah ini memindahkan sesi: `CLAUDE.md` direktori baru dimuat dan `--resume` menemukan sesi dari sana.

<h3 id="additional-directories-grant-file-access-not-configuration">
  Direktori tambahan memberikan akses file, bukan konfigurasi
</h3>

Menambahkan direktori memperluas tempat Claude dapat membaca dan mengedit file. Ini tidak membuat direktori itu akar konfigurasi penuh: sebagian besar konfigurasi `.claude/` tidak ditemukan dari direktori tambahan, meskipun beberapa jenis dimuat sebagai pengecualian.

Pengecualian ini hanya berlaku untuk direktori yang ditambahkan dengan flag `--add-dir` atau perintah `/add-dir`. Direktori yang tercantum dalam `permissions.additionalDirectories` dalam file pengaturan memberikan akses file saja dan tidak memuat konfigurasi apa pun di bawah ini.

Jenis konfigurasi berikut dimuat dari direktori `--add-dir`:

| Konfigurasi                                                                           | Dimuat dari `--add-dir`                                                                                                                                           |
| :------------------------------------------------------------------------------------ | :---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [Skills](/docs/id/skills) di `.claude/skills/`                                             | Ya, dengan live reload                                                                                                                                            |
| [Subagents](/docs/id/sub-agents) di `.claude/agents/`                                      | Ya                                                                                                                                                                |
| [Settings](/docs/id/settings) di `.claude/settings.json` dan `.claude/settings.local.json` | Kunci `enabledPlugins` dan `extraKnownMarketplaces` saja                                                                                                          |
| File [CLAUDE.md](/docs/id/memory), `.claude/rules/`, dan `CLAUDE.local.md`                 | Hanya ketika `CLAUDE_CODE_ADDITIONAL_DIRECTORIES_CLAUDE_MD=1` diatur. `CLAUDE.local.md` juga memerlukan sumber pengaturan `local`, yang diaktifkan secara default |

Perintah dan output styles ditemukan dari direktori kerja saat ini dan induknya, direktori pengguna Anda di `~/.claude/`, dan pengaturan terkelola. Hooks dan kunci `settings.json` lainnya dimuat dari folder `.claude/` direktori kerja saat ini tanpa fallback direktori induk, bersama dengan `~/.claude/settings.json` pengguna Anda dan pengaturan terkelola. Untuk berbagi konfigurasi itu di seluruh proyek, gunakan salah satu pendekatan ini:

* **Konfigurasi tingkat pengguna**: tempatkan file di `~/.claude/agents/`, `~/.claude/output-styles/`, atau `~/.claude/settings.json` untuk membuatnya tersedia di setiap proyek
* **Plugins**: paket dan distribusikan konfigurasi sebagai [plugin](/docs/id/plugins) yang dapat diinstal tim
* **Luncurkan dari direktori konfigurasi**: jalankan Claude Code dari direktori yang berisi konfigurasi `.claude/` yang ingin Anda gunakan

<h2 id="how-permissions-interact-with-sandboxing">
  Bagaimana izin berinteraksi dengan sandboxing
</h2>

Izin dan [sandboxing](/docs/id/sandboxing) adalah lapisan keamanan pelengkap:

* **Izin** mengontrol alat mana yang dapat digunakan Claude Code dan file atau domain mana yang dapat diaksesnya. Mereka berlaku untuk semua alat, termasuk Bash, Read, Edit, WebFetch, dan MCP.
* **Sandboxing** menyediakan penegakan tingkat OS yang membatasi akses sistem file dan jaringan alat Bash. Ini hanya berlaku untuk perintah Bash dan proses anak mereka.

Gunakan keduanya untuk pertahanan berlapis:

* Aturan deny izin memblokir Claude dari bahkan mencoba mengakses sumber daya terbatas
* Pembatasan sandbox mencegah perintah Bash menjangkau sumber daya di luar batas yang ditentukan, bahkan jika injeksi prompt melewati pengambilan keputusan Claude
* Pembatasan sistem file di sandbox menggabungkan pengaturan [`sandbox.filesystem`](/docs/id/sandboxing) dengan aturan deny Read dan Edit; keduanya digabungkan ke dalam batas sandbox akhir
* Pembatasan jaringan menggabungkan aturan izin WebFetch dengan daftar `allowedDomains` dan `deniedDomains` sandbox

Ketika sandboxing diaktifkan dengan `autoAllowBashIfSandboxed: true`, yang merupakan default, perintah Bash yang di-sandbox berjalan tanpa meminta bahkan jika izin Anda mencakup aturan ask `Bash` biasa, atau [bentuk setara `Bash(*)`](#match-all-uses-of-a-tool): batas sandbox menggantikan prompt seluruh alat tersebut. Aturan ask yang dibatasi konten seperti `Bash(git push *)` masih memaksa prompt, aturan deny eksplisit masih berlaku, dan perintah `rm` atau `rmdir` yang menargetkan `/`, direktori home Anda, atau jalur sistem kritis lainnya masih memicu prompt. Perintah yang tidak akan berjalan di sandbox, seperti perintah yang dikecualikan, menghormati aturan ask `Bash` biasa seperti biasanya. Lihat [sandbox modes](/docs/id/sandboxing#sandbox-modes) untuk mengubah perilaku ini.

<h2 id="managed-settings">
  Pengaturan terkelola
</h2>

Untuk organisasi yang memerlukan kontrol terpusat atas konfigurasi Claude Code, administrator dapat menerapkan pengaturan terkelola yang tidak dapat ditimpa oleh pengaturan pengguna atau proyek. Pengaturan kebijakan ini mengikuti format yang sama dengan file pengaturan reguler dan dapat dikirimkan melalui kebijakan MDM/tingkat OS, file pengaturan terkelola, [pengaturan yang dikelola server](/docs/id/server-managed-settings), atau [gateway aplikasi Claude](/docs/id/claude-apps-gateway) yang di-host sendiri. Lihat [file pengaturan](/docs/id/settings#settings-files) untuk mekanisme pengiriman dan lokasi file.

<h3 id="managed-only-settings">
  Pengaturan khusus terkelola
</h3>

Pengaturan berikut hanya dibaca dari pengaturan terkelola. Menempatkan mereka dalam file pengaturan pengguna atau proyek tidak memiliki efek.

| Pengaturan                                     | Deskripsi                                                                                                                                                                                                                                                                                                                                          |
| :--------------------------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `allowAllClaudeAiMcps`                         | Ketika `true`, konektor claude.ai dimuat bersama `managed-mcp.json` yang diterapkan alih-alih ditekan oleh kontrol eksklusifnya. Lihat [Managed MCP configuration](/docs/id/managed-mcp)                                                                                                                                                                |
| `allowedChannelPlugins`                        | Daftar izin plugin saluran yang dapat mendorong pesan. Menggantikan daftar izin Anthropic default saat diatur. Memerlukan `channelsEnabled: true`. Lihat [Restrict which channel plugins can run](/docs/id/channels#restrict-which-channel-plugins-can-run)                                                                                             |
| `allowManagedHooksOnly`                        | Ketika `true`, hanya hook terkelola, hook SDK, dan hook dari plugin yang dipaksa-aktifkan dalam pengaturan terkelola `enabledPlugins` yang dimuat. Hook pengguna, proyek, dan semua plugin lainnya diblokir                                                                                                                                        |
| `allowManagedMcpServersOnly`                   | Ketika `true`, hanya `allowedMcpServers` dari pengaturan terkelola yang dihormati. `deniedMcpServers` masih digabung dari semua sumber. Lihat [Managed MCP configuration](/docs/id/managed-mcp)                                                                                                                                                         |
| `allowManagedPermissionRulesOnly`              | Ketika `true`, mencegah pengaturan pengguna dan proyek dari mendefinisikan aturan izin `allow`, `ask`, atau `deny`. Hanya aturan dalam pengaturan terkelola yang berlaku. Tidak mempengaruhi daftar izin server MCP; untuk itu, atur `allowManagedMcpServersOnly`                                                                                  |
| `blockedMarketplaces`                          | Daftar blokir sumber marketplace. Sumber yang diblokir diperiksa sebelum mengunduh, jadi mereka tidak pernah menyentuh sistem file. Lihat [managed marketplace restrictions](/docs/id/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                             |
| `channelsEnabled`                              | Izinkan [channels](/docs/id/channels) untuk organisasi. Lihat [enterprise controls](/docs/id/channels#enterprise-controls) untuk default pada setiap paket                                                                                                                                                                                                   |
| `disableSideloadFlags`                         | Tolak flag CLI `--plugin-dir`, `--plugin-url`, `--agents`, dan `--mcp-config` saat startup. Tanpa ini, pengguna dapat melewati `strictKnownMarketplaces` untuk satu kali berjalan dengan melewatkan flag ini. Lihat [`disableSideloadFlags`](/docs/id/settings#available-settings). Memerlukan Claude Code v2.1.193 atau lebih baru                     |
| `forceRemoteSettingsRefresh`                   | Ketika `true`, memblokir startup CLI hingga pengaturan terkelola jarak jauh segar diambil dan keluar jika pengambilan gagal. Lihat [fail-closed enforcement](/docs/id/server-managed-settings#enforce-fail-closed-startup)                                                                                                                              |
| `pluginTrustMessage`                           | Pesan kustom ditambahkan ke peringatan kepercayaan plugin yang ditampilkan sebelum instalasi                                                                                                                                                                                                                                                       |
| `sandbox.filesystem.allowManagedReadPathsOnly` | Ketika `true`, hanya jalur `filesystem.allowRead` dari pengaturan terkelola yang dihormati. `denyRead` masih digabung dari semua sumber                                                                                                                                                                                                            |
| `sandbox.network.allowManagedDomainsOnly`      | Ketika `true`, hanya `allowedDomains` dan aturan allow `WebFetch(domain:...)` dari pengaturan terkelola yang dihormati. Domain yang tidak diizinkan diblokir secara otomatis tanpa meminta pengguna. Domain yang ditolak masih digabung dari semua sumber                                                                                          |
| `strictKnownMarketplaces`                      | Mengontrol sumber marketplace plugin mana yang dapat ditambahkan dan diinstal pengguna. Lihat [managed marketplace restrictions](/docs/id/plugin-marketplaces#managed-marketplace-restrictions)                                                                                                                                                         |
| `strictPluginOnlyCustomization`                | Blokir skills, agents, hooks, dan MCP servers dari sumber pengguna dan proyek, sehingga mereka hanya dapat berasal dari plugins atau pengaturan terkelola. `true` mengunci keempat permukaan; array seperti `["skills", "hooks"]` hanya mengunci yang dinamai. Lihat [`strictPluginOnlyCustomization`](/docs/id/settings#strictpluginonlycustomization) |
| `wslInheritsWindowsSettings`                   | Ketika `true` dalam kunci registri Windows HKLM atau `C:\Program Files\ClaudeCode\managed-settings.json`, WSL membaca pengaturan terkelola dari rantai kebijakan Windows selain `/etc/claude-code`. Lihat [Settings files](/docs/id/settings#settings-files)                                                                                            |

`disableBypassPermissionsMode` biasanya ditempatkan dalam pengaturan terkelola untuk memberlakukan kebijakan organisasi, tetapi berfungsi dari cakupan apa pun. Pengguna dapat mengaturnya dalam pengaturan mereka sendiri untuk mengunci diri mereka sendiri dari mode bypass.

<Note>
  Pada paket Team dan Enterprise, Owner mengaktifkan atau menonaktifkan [Remote Control](/docs/id/remote-control) dan [sesi web](/docs/id/claude-code-on-the-web) di seluruh organisasi dalam [pengaturan admin Claude Code](https://claude.ai/admin-settings/claude-code). Remote Control dapat secara tambahan dinonaktifkan per perangkat dengan pengaturan [`disableRemoteControl`](/docs/id/settings#available-settings). Sesi web tidak memiliki kunci pengaturan terkelola per perangkat.
</Note>

<h2 id="settings-precedence">
  Prioritas pengaturan
</h2>

Aturan izin mengikuti [prioritas pengaturan](/docs/id/settings#settings-precedence) yang sama dengan semua pengaturan Claude Code lainnya:

1. **Pengaturan terkelola**: tidak dapat ditimpa oleh tingkat lain apa pun, termasuk argumen baris perintah
2. **Argumen baris perintah**: penggantian sesi sementara
3. **Pengaturan proyek lokal** (`.claude/settings.local.json`)
4. **Pengaturan proyek bersama** (`.claude/settings.json`)
5. **Pengaturan pengguna** (`~/.claude/settings.json`)

Jika alat ditolak di tingkat mana pun, tidak ada tingkat lain yang dapat mengizinkannya. Misalnya, penolakan pengaturan terkelola tidak dapat ditimpa oleh `--allowedTools`, dan `--disallowedTools` dapat menambahkan pembatasan di luar apa yang ditentukan pengaturan terkelola.

Hal yang sama berlaku di seluruh cakupan pengaturan: jika pengaturan pengguna mengizinkan izin dan pengaturan proyek menolaknya, aturan penolakan memblokir izin tersebut. Kebalikannya juga benar: penolakan tingkat pengguna memblokir izin tingkat proyek, karena aturan penolakan dari cakupan apa pun dievaluasi sebelum aturan izin.

Host penyematan dapat menyediakan kebijakan terkelola tambahan melalui opsi SDK `managedSettings` ketika [`parentSettingsBehavior`](/docs/id/settings#settings-precedence) diatur ke `"merge"`; nilai penyemat dapat memperketat kebijakan tetapi tidak dapat melonggarkannya.

<h2 id="project-allow-rules-and-workspace-trust">
  Aturan izin proyek dan kepercayaan ruang kerja
</h2>

Aturan `permissions.allow` dan entri `permissions.additionalDirectories` dalam `.claude/settings.json` proyek memberikan kemampuan, jadi Claude Code menerapkannya hanya setelah Anda menerima [dialog kepercayaan ruang kerja](/docs/id/security#additional-safeguards) untuk ruang kerja tersebut. Sampai saat itu, Claude Code membaca aturan tetapi tidak menerapkannya. Dialog kepercayaan mencantumkan aturan izin dan direktori tambahan yang akan diberikan folder sehingga Anda dapat meninjau sebelum menerima. Aturan `deny` dan `ask` tidak terpengaruh, karena hanya membatasi.

Claude Code menyimpan kepercayaan per ruang kerja, dikunci pada akar repositori git atau, di luar repositori, direktori tempat Anda memulai Claude Code. Ketika Anda memulai di direktori home Anda, kepercayaan disimpan hanya untuk sesi saat ini dan tidak ditulis ke disk; lihat catatan [safeguard tambahan](/docs/id/security#additional-safeguards). Mempercayai direktori induk tidak menerapkan aturan izin proyek bersarang.

`.claude/settings.local.json` adalah file Anda sendiri, jadi pemeriksaan kepercayaan ruang kerja biasanya tidak berlaku untuk itu. Ketika repositori dapat menyediakan file, seperti ketika file tersebut di-commit ke git atau `.claude` adalah symlink, aturan izin dan direktori tambahannya melalui pemeriksaan kepercayaan seperti pengaturan proyek.

Claude Code menjalankan git untuk memeriksa apakah repositori menyediakan file, dan menjalankan pemeriksaan itu hanya dalam folder yang dicakup oleh dialog kepercayaan yang diterima, untuk folder itu atau untuk salah satu direktori induknya. Dalam sesi interaktif dalam folder yang belum Anda percayai, aturan izin dan direktori tambahan dalam `.claude/settings.local.json` melalui pemeriksaan kepercayaan seperti pengaturan proyek sampai Anda menerima dialog, kecuali sesi berjalan di home konfigurasi Anda sendiri seperti yang dijelaskan di bawah. Dari dua pengecualian di bawah, hanya pengecualian home konfigurasi yang berlaku sebelum dialog, karena tidak perlu menjalankan git. Menentukan bahwa direktori tidak berada di dalam repositori git menggunakan pemeriksaan git yang sama, jadi pengecualian tidak-di-dalam-repositori berlaku setelah dialog kepercayaan yang mencakup folder diterima. Sebelum v2.1.207, `.claude/settings.local.json` yang tidak dilacak menerapkan aturan izinnya dalam folder itu sebelum Anda menerima dialog.

Aturan izin dan direktori tambahan dalam `.claude/settings.local.json` juga berlaku tanpa kepercayaan ruang kerja dalam dua kasus:

* Direktori tempat Anda memulai Claude Code tidak berada di dalam repositori git.
* Sesi berjalan di home konfigurasi Anda sendiri: direktori home Anda atau direktori apa pun yang subdirektori `.claude` Anda telah atur sebagai [`CLAUDE_CONFIG_DIR`](/docs/id/env-vars).

Dalam kedua kasus, file adalah file yang Anda buat daripada file yang dapat disediakan repositori, dan `.claude/settings.local.json` yang di-commit repositori masih memerlukan kepercayaan ruang kerja. Versi 2.1.196 hingga 2.1.199 memperlakukan file sebagai disediakan repositori di ruang kerja tersebut, mengabaikan aturan izinnya, dan mencetak peringatan [`this workspace has not been trusted`](/docs/id/errors#workspace-has-not-been-trusted) ke stderr. Dua pengecualian di atas cocok dengan v2.1.195 dan sebelumnya dan dipulihkan dalam v2.1.200.

Juga mulai dari v2.1.200, ruang kerja yang aturan izin atau direktori tambahannya masih tidak diterapkan, tetapi yang tidak pernah menampilkan dialog kepercayaan karena direktori induk sudah dipercaya, menampilkan dialog saat berikutnya Anda memulai Claude Code di sana secara interaktif. Dialog menawarkan dua pilihan:

* **Ya, saya mempercayai folder ini**: menyimpan kepercayaan untuk ruang kerja tersebut dan menerapkan aturan dalam sesi yang sama.
* **Tidak, lanjutkan tanpa izin ini**: terus bekerja dengan aturan tersebut diabaikan. Dialog muncul lagi di sesi berikutnya.

Dalam [mode non-interaktif](/docs/id/headless) dengan `-p`, tidak ada dialog yang muncul dan aturan tetap diabaikan.

<h2 id="example-configurations">
  Contoh konfigurasi
</h2>

[Repositori](https://github.com/anthropics/claude-code/tree/main/examples/settings) ini mencakup konfigurasi pengaturan pemula untuk skenario penerapan umum. Gunakan ini sebagai titik awal dan sesuaikan dengan kebutuhan Anda.

<h2 id="see-also">
  Lihat juga
</h2>

* [Settings](/docs/id/settings): referensi konfigurasi lengkap termasuk tabel pengaturan izin
* [Configure auto mode](/docs/id/auto-mode-config): beri tahu pengklasifikasi mode auto infrastruktur mana yang dipercaya organisasi Anda
* [Sandboxing](/docs/id/sandboxing): isolasi sistem file dan jaringan tingkat OS untuk perintah Bash
* [Authentication](/docs/id/authentication): atur akses pengguna ke Claude Code
* [Security](/docs/id/security): perlindungan keamanan dan praktik terbaik
* [Hooks](/docs/id/hooks-guide): otomatisasi alur kerja dan perluas evaluasi izin
