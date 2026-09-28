> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Buat plugin Claude Code

> Bangun plugin Claude Code pertama Anda dari direktori kosong, uji tanpa marketplace, dan konversi setup .claude/ yang sudah ada.

Plugin adalah direktori skills, agents, hooks, dan MCP servers, ditambah file `plugin.json`, yang disebut manifest, yang memberi nama pada plugin. Claude Code memuat direktori sebagai satu unit, sehingga Anda dapat membagikannya dengan rekan kerja, memasangnya di beberapa proyek, atau menerbitkannya ke marketplace.

Halaman ini untuk orang-orang yang menulis plugin mereka sendiri.

<Note>
  Kasus-kasus ini tercakup di halaman lain:

  * **Memasang plugin orang lain**: lihat [Install plugins](/docs/id/plugins/install)
  * **Tidak yakin Anda memerlukan plugin**: lihat [Decide whether you need a plugin](/docs/id/plugins/overview#decide-whether-you-need-a-plugin) di overview
  * **Pengguna plugin Anda berada di claude.ai atau di Cowork**: folder yang sama dipasang di sana dengan subset komponen yang berbeda. Lihat [Plugins on claude.ai and in Cowork](https://claude.com/docs/plugins/overview)
</Note>

Mulai dari bagian yang sesuai dengan apa yang sudah Anda miliki:

* **Belum ada apa-apa**: ikuti [Create your first plugin](#create-your-first-plugin), kemudian [Develop without a marketplace](#develop-without-a-marketplace) dan [Test and debug](#test-and-debug).
* **File di bawah `.claude/` sudah ada**: lakukan walkthrough first-plugin sekali untuk mempelajari layoutnya, kemudian ikuti [Convert an existing `.claude/` setup](#convert-an-existing-claude-setup).

<h2 id="decide-when-to-use-a-plugin">
  Tentukan kapan menggunakan plugin
</h2>

Skills, agents, hooks, dan MCP servers semuanya bekerja standalone di proyek Anda atau direktori home. Pertahankan setup standalone itu selama melayani satu proyek atau hanya Anda. Buat plugin ketika Anda ingin berbagi setup dengan rekan kerja, memasangnya di beberapa proyek, atau menerbitkan rilis yang diversi.

Ketika Anda memindahkan skills, agents, hooks, dan MCP config standalone ke plugin, lokasi dan nama mereka berubah:

* **Di mana file-file itu pergi**: di bawah direktori plugin sendiri, yang disebut plugin root, sebagai `skills/`, `agents/`, `hooks/hooks.json`, dan `.mcp.json`.
* **Bagaimana mereka dinamai**: plugin skills dan agents mendapatkan nama plugin sebagai prefix, seperti `/my-plugin:hello`, sehingga dua plugin dapat masing-masing menyediakan skill `hello` tanpa bertabrakan.

Untuk memindahkan setup yang sudah ada ke plugin, lihat [Convert an existing `.claude/` setup](#convert-an-existing-claude-setup).

<h2 id="create-your-first-plugin">
  Buat plugin pertama Anda
</h2>

Dalam walkthrough ini, Anda membuat plugin yang satu-satunya komponennya adalah satu skill, sebuah greeting, dan menjalankannya dengan `--plugin-dir`, yang memuat plugin untuk satu sesi tanpa memasangnya. Plugin dapat menampung campuran apa pun dari [components](/docs/id/plugins/components), seperti skills, agents, hooks, dan MCP servers, dan tidak ada yang diperlukan; satu skill adalah contoh terkecil yang menunjukkan layout.

Anda memerlukan Claude Code [installed and signed in](/docs/id/quickstart#step-1-install-claude-code).

Buka terminal di direktori tempat Anda ingin menyimpan plugin, seperti `~/projects`, dan jalankan perintah dalam langkah-langkah ini darinya. Anda dapat menyimpan plugin di mana saja, karena Anda meneruskan jalurnya ke Claude Code ketika Anda memulai sesi.

<Steps>
  <Step title="Buat direktori plugin">
    Buat direktori plugin, dengan folder `.claude-plugin/` di dalamnya untuk menampung manifest:

    ```bash theme={null}
    mkdir -p my-first-plugin/.claude-plugin
    ```
  </Step>

  <Step title="Tulis manifest">
    [manifest](/docs/id/plugins/manifest-reference) adalah file JSON bernama `plugin.json` yang memberi tahu Claude Code nama plugin dan mendeskripsikannya. Simpan yang ini sebagai `my-first-plugin/.claude-plugin/plugin.json`:

    ```json my-first-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-first-plugin",
      "description": "A greeting plugin to learn the basics",
      "version": "1.0.0",
      "author": {
        "name": "Your Name"
      }
    }
    ```

    Empat field melakukan ini:

    * **`name`**: diperlukan. Ini mengidentifikasi plugin dan menjadi prefix pada setiap skill dan agent yang disediakan plugin. Jangan masukkan spasi di dalamnya.
    * **`description`**: teks yang dilihat pengguna untuk plugin di `/plugin`.
    * **`version`**: opsional. Menetapkannya membuat pengguna tetap pada versi itu sampai Anda mengubahnya; [Release a new version](/docs/id/plugins/host-marketplace#release-a-new-version) mengatakan kapan harus menetapkan atau menghilangkannya.
    * **`author`**: siapa yang dikreditkan. `name` diperlukan di dalamnya; `email` dan `url` opsional.

    Setiap field lainnya ada di [manifest reference](/docs/id/plugins/manifest-reference#fields).

    Hanya `plugin.json` yang masuk ke dalam `.claude-plugin/`. Skill yang Anda tambahkan selanjutnya langsung di bawah `my-first-plugin/`, di sebelah folder itu.
  </Step>

  <Step title="Tambahkan skill">
    Satu-satunya komponen plugin ini adalah skill. Setiap skill adalah direktori di bawah `skills/` yang berisi file `SKILL.md`. Buat direktori skill:

    ```bash theme={null}
    mkdir -p my-first-plugin/skills/hello
    ```

    Kemudian buat `my-first-plugin/skills/hello/SKILL.md` dengan konten ini:

    ```markdown my-first-plugin/skills/hello/SKILL.md theme={null}
    ---
    name: hello
    description: Greet the user with a friendly message
    disable-model-invocation: true
    ---

    Greet the user warmly and ask how you can help them today.
    ```

    Baris `disable-model-invocation: true` berarti Claude tidak menjalankan skill sendiri, jadi hanya Anda yang memicunya. Hapus baris itu dari skill yang ingin Anda jalankan sendiri oleh Claude. Perintah skill menggabungkan nama plugin dan nama skill, jadi Anda menjalankan yang ini sebagai `/my-first-plugin:hello`. Untuk field frontmatter lainnya, lihat [skill frontmatter reference](/docs/id/skills#frontmatter-reference).
  </Step>

  <Step title="Validasi plugin">
    Periksa manifest dan frontmatter skill sebelum Anda menjalankan apa pun:

    ```bash theme={null}
    claude plugin validate ./my-first-plugin
    ```

    Perintah mencetak jalur manifest yang diperiksa dan `✔ Validation passed`. Jika mencetak `✘ Validation failed` sebagai gantinya, setiap baris di atas baris hasil itu memberi nama field yang harus diperbaiki. Cari setiap pesan di bawah [`claude plugin validate` reports errors](/docs/id/plugins/troubleshooting#claude-plugin-validate-reports-errors).
  </Step>

  <Step title="Jalankan Claude Code dengan plugin">
    Mulai sesi dengan plugin dimuat:

    ```bash theme={null}
    claude --plugin-dir ./my-first-plugin
    ```

    Setelah Claude Code dimulai, jalankan skill:

    ```text theme={null}
    /my-first-plugin:hello
    ```

    Claude membalas dengan greeting.
  </Step>
</Steps>

Plugin dimuat hanya dalam sesi yang Anda mulai dengan `--plugin-dir`. Untuk terus bekerja padanya tanpa flag, atau untuk menguji build `.zip`, lihat [Develop without a marketplace](#develop-without-a-marketplace).

<h3 id="share-the-plugin">
  Bagikan plugin Anda
</h3>

Plugin yang Anda bangun dengan [Create your first plugin](#create-your-first-plugin) hanya ada di mesin Anda. Ketika sudah siap untuk orang lain, ada tiga cara untuk mendapatkannya kepada mereka:

* **Kirimkan langsung ke beberapa orang**: berikan mereka direktori plugin atau `.zip` darinya, dan tidak ada yang perlu dipublikasikan. Lihat [Share a plugin without a marketplace](/docs/id/plugins/publish#share-a-plugin-without-a-marketplace).
* **Daftarkan di marketplace Anda sendiri**: rekan kerja menambahkan marketplace Anda sekali dan memasang plugin berdasarkan nama, dan mereka menerima update Anda. Lihat [Publish through your own marketplace](/docs/id/plugins/publish#publish-through-your-own-marketplace).
* **Kirimkan ke community marketplace Anthropic**: setelah terdaftar, siapa pun yang menambahkan marketplace itu dapat memasangnya. Lihat [Submit to the community marketplace](/docs/id/plugins/publish#submit-to-the-community-marketplace).

<h3 id="plugin-layout">
  Plugin layout
</h3>

Setiap jenis [component](/docs/id/plugins/components), seperti skills, agents, hooks, dan MCP servers, masuk ke direktori tetap di bawah plugin root, yang merupakan direktori yang Anda teruskan ke `--plugin-dir`. Tambahkan hanya direktori yang Anda gunakan. Untuk mengklik melalui direktori plugin lengkap dan membaca apa yang dilakukan setiap file, buka [plugin explorer](/docs/id/plugins/components#explore-the-plugin-directory).

Tabel mencantumkan direktori yang paling banyak digunakan plugin untuk memulai, dan [full layout](/docs/id/plugins/manifest-reference#standard-layout) mencantumkan sisanya.

| Lokasi                       | Isi                                                                                                                                         |
| :--------------------------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| `.claude-plugin/plugin.json` | Manifest. Ketika Anda memuat plugin dengan `--plugin-dir` dan tidak memiliki manifest, Claude Code memberi nama plugin setelah direktorinya |
| `skills/`                    | Satu direktori `<name>/SKILL.md` per skill                                                                                                  |
| `commands/`                  | File Markdown datar, bentuk lama dari skills. Gunakan `skills/` untuk plugin baru                                                           |
| `agents/`                    | Satu file Markdown per subagent                                                                                                             |
| `hooks/hooks.json`           | Konfigurasi hook: key `"hooks"` tingkat atas yang nilainya memiliki bentuk yang sama dengan `hooks` dalam file settings                     |
| `.mcp.json`                  | Definisi MCP server                                                                                                                         |

<Warning>
  Hanya `plugin.json` yang masuk ke dalam `.claude-plugin/`. Komponen yang disimpan di sana tidak dimuat.

  Plugin root adalah direktori plugin sendiri, bukan `~/.claude/` itu sendiri. `.mcp.json` yang disimpan di `~/.claude/.mcp.json` tidak dimuat.
</Warning>

<h2 id="develop-without-a-marketplace">
  Kembangkan tanpa marketplace
</h2>

Anda tidak memerlukan [marketplace](/docs/id/plugins/overview#get-plugins-from-a-marketplace) untuk menjalankan plugin yang Anda tulis. Muat langsung dari disk atau URL sebagai gantinya:

* [`--plugin-dir`](#load-a-directory-or-archive-for-one-session): memuat direktori atau arsip `.zip` untuk satu sesi.
* [`--plugin-url`](#fetch-an-archive-from-a-url-for-one-session): mengambil arsip `.zip` dari URL untuk satu sesi.
* [`claude plugin init`](#scaffold-a-plugin-that-loads-every-session): membuat scaffold plugin di bawah `~/.claude/skills/` yang dimuat setiap sesi.

Jika dua plugin yang dimuat dengan cara berbeda berbagi nama, lihat [Name conflicts](/docs/id/plugins/loading#name-conflicts) untuk mengetahui mana yang Claude Code pertahankan.

<h3 id="load-a-directory-or-archive-for-one-session">
  Muat plugin untuk satu sesi
</h3>

Anda dapat memuat plugin untuk satu sesi dengan tiga cara: dari direktori atau arsip `.zip` di disk dengan `--plugin-dir`, dari URL dengan `--plugin-url`, atau dari variabel lingkungan ketika Anda tidak dapat menambahkan flag. Setiap plugin dimuat hanya untuk sesi itu, dan tidak ada yang ditulis ke settings Anda untuk itu. Ketika Anda mengedit file plugin selama sesi, jalankan `/reload-plugins` untuk memuat perubahan.

<h4 id="from-a-directory-or-zip">
  Dari direktori atau `.zip`
</h4>

Ketika Anda memulai `claude` dari shell Anda, teruskan `--plugin-dir` dengan direktori root plugin atau arsip `.zip` darinya. Ulangi flag untuk memuat beberapa plugin:

```bash theme={null}
claude --plugin-dir ./my-first-plugin --plugin-dir ./other-plugin.zip
```

<h4 id="load-a-folder-of-plugins">
  Dari folder plugin
</h4>

Untuk memuat beberapa plugin dari satu tempat, teruskan folder yang menampungnya, seperti `--plugin-dir ./plugins`. Memuat folder plugin memerlukan Claude Code v2.1.265 atau lebih baru.

Jika folder tidak memiliki direktori `.claude-plugin/` dan tidak memiliki komponen plugin di tingkat atasnya, Claude Code memperlakukannya sebagai folder plugin. Setiap subfolder langsung yang memiliki manifest `.claude-plugin/plugin.json` kemudian dimuat sebagai plugin terpisah. Semua yang lain di folder dilewati tanpa kesalahan, termasuk subfolder yang tidak memiliki manifest. Jika plugin di folder tidak dimuat, periksa bahwa subfoldernya memiliki `.claude-plugin/plugin.json`.

Dalam sesi interaktif, Anda juga dapat menambah dan menghapus plugin di folder setelah startup:

* Subfolder yang Anda tambahkan dimuat sebagai plugin baru setelah manifestnya ada.
* Ketika Anda menghapus subfolder, pluginnya dibongkar.

Pesan muncul dalam sesi untuk setiap perubahan ini. Jika memuat atau membongkar plugin di tengah percakapan akan [invalidate the prompt cache](/docs/id/prompt-caching#enabling-or-disabling-a-plugin), perubahan ditahan sebagai gantinya, dan pesan memberi tahu Anda untuk menjalankan `/reload-plugins` untuk menerapkannya.

<h4 id="fetch-an-archive-from-a-url-for-one-session">
  Dari URL
</h4>

Ketika Anda memulai `claude` dari shell Anda, teruskan `--plugin-url` dengan alamat arsip `.zip`, seperti artefak build yang CI Anda publikasikan:

```bash theme={null}
claude --plugin-url https://example.com/my-first-plugin.zip
```

Claude Code mengunduh arsip saat startup. Untuk memuat beberapa, ulangi flag atau teruskan URL yang dipisahkan spasi dalam satu argumen yang dikutip.

Arahkan flag hanya ke arsip yang Anda kontrol atau percayai.

Jika Claude Code tidak dapat mengambil arsip, atau arsip tidak valid, itu dimulai tanpa plugin dan mencatat kesalahan beban plugin yang dapat Anda tinjau di tab **Errors** manajer `/plugin`.

<h4 id="from-an-environment-variable">
  Dari variabel lingkungan
</h4>

Untuk memuat plugin dalam sesi di mana Anda tidak dapat menambahkan flag `--plugin-dir`, daftarkan jalur absolutnya dalam variabel lingkungan [`CLAUDE_CODE_PLUGIN_DIRS`](/docs/id/env-vars#variables) sebagai gantinya. Claude Code memuat setiap jalur saat memuat jalur `--plugin-dir`. Plugin ini dimuat sebagai tambahan untuk yang Anda teruskan dengan `--plugin-dir`. [Project and local settings can't set this variable](/docs/id/settings-reference#variables-claude-code-ignores-in-env). `CLAUDE_CODE_PLUGIN_DIRS` memerlukan Claude Code v2.1.280 atau lebih baru.

Managed settings dapat mematikan `--plugin-dir` dan `CLAUDE_CODE_PLUGIN_DIRS`. Lihat [Flags that load a plugin for one session](/docs/id/plugins/cli-reference#flags-that-load-a-plugin-for-one-session). Untuk menguji plugin bersama dengan plugin yang bergantung padanya, lihat [Test a plugin and its dependency locally](/docs/id/plugins/dependencies#test-a-plugin-and-its-dependency-locally).

<h3 id="scaffold-a-plugin-that-loads-every-session">
  Buat plugin dimuat di setiap sesi
</h3>

Direktori skills pribadi Anda adalah `~/.claude/skills/`. Claude Code memuat folder apa pun di sana yang berisi `.claude-plugin/plugin.json` sebagai plugin di setiap sesi, tanpa flag dan tanpa langkah install. `claude plugin init` membuat scaffold satu dari plugin ini untuk Anda.

<h4 id="scaffold-the-plugin-with-claude-plugin-init">
  Buat scaffold plugin dengan `claude plugin init`
</h4>

`claude plugin init` menulis plugin starter di bawah `~/.claude/skills/`. Memerlukan Claude Code v2.1.157 atau lebih baru. Buat scaffold satu dari shell Anda:

```bash theme={null}
claude plugin init my-tool
```

Perintah membuat `~/.claude/skills/my-tool/` dengan `.claude-plugin/plugin.json` dan root `SKILL.md`. Ini mencetak `✔ Created plugin "my-tool" at ~/.claude/skills/my-tool` diikuti oleh `It will auto-load next session as my-tool@skills-dir. Run /reload-plugins to load it now.`

Teruskan `--with skills` untuk membuat scaffold `claude plugin init` skill di bawah `skills/` untuk Anda. Nilai `--with` lainnya ada di [plugin commands reference](/docs/id/plugins/cli-reference#plugin-init).

<h4 id="skill-names-in-a-scaffolded-plugin">
  Beri nama skills plugin
</h4>

Root skill di `~/.claude/skills/my-tool/SKILL.md` juga merupakan personal skill, jadi Anda memanggilnya sebagai `/my-tool`, bukan `/my-tool:my-tool`. Skills yang Anda tambahkan di bawah `skills/` di dalam plugin mendapatkan prefix nama plugin, seperti `/my-tool:example`.

<h4 id="stop-loading-the-plugin">
  Hentikan pemuatan plugin
</h4>

Untuk menghentikan pemuatan plugin yang dibuat scaffold, hapus direktorinya, atau jalankan `claude plugin disable my-tool@skills-dir` di shell Anda dengan nama `my-tool@skills-dir` yang dicetak `claude plugin init`. Dalam ID `my-tool@skills-dir`, `skills-dir` berdiri di mana nama marketplace akan berada, karena plugin dimuat dari direktori skills Anda daripada dari marketplace.

<h4 id="load-a-plugin-for-everyone-in-one-repository">
  Bagikan plugin melalui repository
</h4>

`claude plugin init` menulis plugin ke direktori skills pribadi Anda di `~/.claude/skills/`, sehingga dimuat untuk Anda di setiap proyek. Untuk membuat plugin dimuat untuk semua orang di satu repository, buat layout yang sama sendiri di `<project>/.claude/skills/<name>/`, termasuk `.claude-plugin/plugin.json`. Lihat [Plugins shared through a repository](/docs/id/plugins/loading#plugins-shared-through-a-repository) untuk kondisi di mana Claude Code memuat itu.

<h2 id="test-and-debug">
  Uji dan debug
</h2>

Ketika perubahan pada plugin Anda tidak muncul, kerjakan pemeriksaan ini secara berurutan. Masing-masing memberi tahu Anda apa yang dilakukan Claude Code dengan plugin:

1. Di shell Anda, jalankan `claude plugin validate <path>`. Ini memeriksa manifest dan frontmatter setiap file skill, agent, dan command, dan keluar `0` pada `Validation passed`. Tambahkan `--strict` untuk gagal pada peringatan juga. Exit codes dan penanganan direktori ada di [plugin commands reference](/docs/id/plugins/cli-reference#plugin-validate).
2. Dalam sesi yang berjalan, jalankan `/reload-plugins` untuk menerapkan edit yang Anda buat di disk. Ini mencetak satu baris `Reloaded:` dengan hitungan. Kemudian konfirmasi skill dimuat dengan mengetik perintah `/plugin-name:skill` atau dengan menemukan plugin di tab **Installed** `/plugin`.
3. Dalam sesi yang sama, jalankan `/plugin`. Tab **Installed** mencantumkan plugin Anda dan, dalam detail plugin, komponen yang ditemukan Claude Code. Tab **Errors** mencantumkan apa yang gagal dimuat dan mengapa, seperti jalur dalam manifest Anda yang tidak ada.
4. Kembali di shell Anda, jalankan `claude plugin list`. Ini mencetak plugin sesi-saja dan direktori-skills dalam bagian mereka sendiri dengan `Status: ✔ loaded` atau kesalahan beban. Untuk menyertakan plugin yang Anda kembangkan, teruskan `--plugin-dir` dengan jalurnya sebelum `plugin list`.

Untuk memeriksa MCP server, jalankan `/mcp` dalam sesi untuk melihat status server. Ketika server sehat, `/mcp` mencantumkannya sebagai terhubung. Jika tidak, lihat [MCP servers that don't start](/docs/id/plugins/troubleshooting#invalid-mcp-server-config-for-and-mcp-servers-that-dont-start).

Untuk memeriksa hook, picu event yang cocok. Misalnya, minta Claude untuk mengedit file untuk memicu hook `PostToolUse`. Kemudian baca [debug log](/docs/id/hooks#debug-hooks), yang menunjukkan hook mana yang cocok, exit codes mereka, dan output mereka.

Bagian berikutnya mencakup kegagalan yang paling mungkin Anda alami saat mengembangkan, dan [troubleshooting page](/docs/id/plugins/troubleshooting#build-a-plugin) memiliki entri lengkap untuk masing-masing.

<h3 id="a-component-path-isn’t-found">
  Jalur komponen tidak ditemukan
</h3>

Tab **Errors** dari `/plugin` menunjukkan `<component> path not found: <path>`, misalnya `commands path not found`. Jalur komponen dalam manifest Anda, seperti `commands`, `skills`, `agents`, atau `hooks`, menunjuk ke tidak ada. Perbaiki jalur atau buat direktori, kemudian jalankan `/reload-plugins` dalam sesi. Lihat [`commands path not found`](/docs/id/plugins/troubleshooting#commands-path-not-found).

<h3 id="plugin-dir-at-a-marketplace-root-doesn’t-load-the-plugins-under-plugins/">
  `--plugin-dir` di root marketplace tidak memuat plugin di bawah `plugins/`
</h3>

`--plugin-dir` mengambil direktori root plugin, yang berisi `.claude-plugin/plugin.json` dan direktori komponen seperti `skills/`. Jika Anda mengarahkannya ke root marketplace sebagai gantinya, Claude Code tidak membaca `marketplace.json`, jadi plugin di bawah `plugins/` tidak dimuat, dan Anda tidak melihat kesalahan. Arahkan flag ke folder satu plugin, atau tambahkan marketplace. Lihat [entri troubleshooting](/docs/id/plugins/troubleshooting#plugin-dir-loads-a-plugin-with-no-components).

<h3 id="the-plugin-loads-but-its-skills-are-missing">
  Plugin dimuat tetapi skillnya hilang
</h3>

Direktori `skills/` ada di dalam `.claude-plugin/`, atau entri `skills` dalam manifest menunjuk ke file. Pindahkan `skills/` ke plugin root, arahkan setiap entri `skills` ke direktori yang berisi `SKILL.md`, dan jalankan `/reload-plugins` dalam sesi. Lihat [Plugin loads but its skills are missing](/docs/id/plugins/troubleshooting#plugin-loads-but-its-skills-are-missing).

<h3 id="the-userconfig-dialog-never-appears">
  Dialog `userConfig` tidak pernah muncul
</h3>

Dialog untuk opsi [`userConfig`](/docs/id/plugins/components#user-configuration) plugin Anda adalah bagian dari pemasangan melalui `/plugin` dalam sesi. Memuat dengan `--plugin-dir` tidak menampilkannya, begitu juga `claude plugin install` di shell. Dengan plugin dimuat, jalankan `/plugin configure <plugin-name>` dalam sesi untuk membukanya. Lihat [The `userConfig` dialog never appears](/docs/id/plugins/troubleshooting#the-userconfig-dialog-never-appears).

<h3 id="check-that-the-plugin-changes-claude’s-behavior">
  Periksa bahwa plugin mengubah perilaku Claude
</h3>

Plugin yang dimuat tanpa kesalahan masih dapat gagal mengarahkan Claude dengan cara yang Anda maksudkan. `claude plugin eval`, yang Anda jalankan di shell Anda, menjalankan kasus uji Anda dengan dan tanpa plugin dan mencetak perbedaannya. Lihat [Test plugins with evals](/docs/id/plugin-evals), dimulai dengan [Create your first eval suite](/docs/id/plugin-evals#create-your-first-eval-suite).

<h2 id="convert-an-existing-claude-setup">
  Konversi setup `.claude/` yang sudah ada
</h2>

Jika Anda sudah memiliki skills, agents, atau hooks di bawah direktori `.claude/` proyek, Anda dapat memindahkannya ke plugin tanpa menulis ulang.

Jalankan perintah dalam langkah-langkah ini dari root proyek, yang merupakan direktori yang berisi `.claude/`, karena jalur `cp` relatif terhadapnya.

<Steps>
  <Step title="Buat struktur plugin">
    Buat direktori plugin dan folder `.claude-plugin/` di sebelah `.claude/`. Anda dapat memindahkan plugin ke mana saja setelahnya.

    ```bash theme={null}
    mkdir -p my-plugin/.claude-plugin
    ```

    Buat `my-plugin/.claude-plugin/plugin.json`:

    ```json my-plugin/.claude-plugin/plugin.json theme={null}
    {
      "name": "my-plugin",
      "description": "Migrated from standalone configuration",
      "version": "1.0.0"
    }
    ```
  </Step>

  <Step title="Salin file yang sudah ada">
    Salin setiap direktori konfigurasi yang Anda miliki ke plugin root, dan lewati perintah untuk direktori apa pun yang tidak Anda miliki.

    ```bash theme={null}
    cp -r .claude/commands my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/agents my-plugin/
    ```

    ```bash theme={null}
    cp -r .claude/skills my-plugin/
    ```

    Jalankan `ls -a my-plugin` untuk mengonfirmasi bahwa setiap direktori yang Anda salin muncul di sebelah `.claude-plugin`.
  </Step>

  <Step title="Pindahkan hooks Anda">
    Jika Anda memiliki hooks di `.claude/settings.json` atau `.claude/settings.local.json`, buat direktori hooks:

    ```bash theme={null}
    mkdir -p my-plugin/hooks
    ```

    Buat `my-plugin/hooks/hooks.json` dan salin objek `hooks` dari file settings Anda ke dalamnya. Formatnya sama.

    Contoh ini menunjukkan bentuk dengan satu hook yang menjalankan linter pada setiap file yang ditulis atau diedit Claude. Ganti contoh dengan objek `hooks` Anda sendiri.

    ```json my-plugin/hooks/hooks.json theme={null}
    {
      "hooks": {
        "PostToolUse": [
          {
            "matcher": "Write|Edit",
            "hooks": [{ "type": "command", "command": "jq -r '.tool_input.file_path' | xargs npm run lint:fix" }]
          }
        ]
      }
    }
    ```
  </Step>

  <Step title="Uji plugin yang dimigrasikan">
    Muat plugin untuk sesi:

    ```bash theme={null}
    claude --plugin-dir ./my-plugin
    ```

    Periksa setiap komponen di bawah nama barunya:

    * **Skills**: jalankan `/my-plugin:deploy` untuk skill yang sebelumnya `/deploy`.
    * **Subagents**: minta Claude untuk menggunakan agent `my-plugin:reviewer` untuk agent yang sebelumnya `reviewer`.
    * **Hooks**: picu event yang cocok dengan setiap hook.

    Jika ada yang hilang, kerjakan [Test and debug](#test-and-debug).
  </Step>
</Steps>

Sementara yang asli masih di bawah `.claude/`, mereka tetap dimuat bersama salinan plugin:

* **Skills dan agents**: dua set tidak bertabrakan, karena skills dan agents plugin membawa prefix `my-plugin:`. `/deploy` dan `/my-plugin:deploy` keduanya bekerja, dan Claude melihat `reviewer` dan `my-plugin:reviewer` sebagai dua subagent.
* **Hooks**: hooks tidak memiliki prefix, jadi hook yang ada di file settings Anda dan `hooks/hooks.json` berjalan dua kali setiap kali event-nya terjadi.

Setelah Anda mengonfirmasi plugin bekerja, hapus yang asli dari `.claude/` dan hapus objek `hooks` dari file settings Anda.

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Plugin components](/docs/id/plugins/components): tambahkan agents, hooks, MCP servers, LSP servers, dan user configuration ke plugin Anda
* [Test plugins with evals](/docs/id/plugin-evals): tulis kasus eval dan jalankan dengan `claude plugin eval` untuk memeriksa seberapa andal plugin memandu perilaku Claude
* [Publish a plugin](/docs/id/plugins/publish): versi itu, masukkan ke marketplace, dan kirimkan ke community marketplace
* [Plugins on claude.ai and in Cowork](https://claude.com/docs/plugins/overview): folder plugin yang sama dipasang di claude.ai dan di Cowork. Beberapa komponen hanya Claude Code
* [Plugin manifest reference](/docs/id/plugins/manifest-reference): setiap field `plugin.json`, aturan path, dan direktori
* [Skills](/docs/id/skills): tulis skills yang disediakan plugin Anda
* [Anthropic's plugins in the claude-code repository](https://github.com/anthropics/claude-code/tree/main/plugins): contoh lengkap yang dikerjakan dari layout di halaman ini, seperti `feature-dev` dan `code-review`
