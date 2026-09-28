> ## Documentation Index
> Fetch the complete documentation index at: https://code.claude.com/docs/llms.txt
> Use this file to discover all available pages before exploring further.

# Rekomendasikan plugin Anda dari CLI Anda

> Minta pengguna Claude Code untuk memasang plugin marketplace resmi Anda dengan mengeluarkan tag claude-code-hint dari CLI atau SDK Anda.

Jika Anda memelihara CLI atau SDK, alat Anda dapat meminta pengguna Claude Code untuk memasang plugin Anda. Ketika CLI Anda mendeteksi bahwa itu berjalan di dalam Claude Code, buat agar menuliskan tag `<claude-code-hint />` satu baris ke stderr. Claude Code menghapus baris dari output alat Bash dan PowerShell sebelum model melihat output, kemudian menampilkan kepada pengguna prompt pemasangan satu kali.

Halaman ini hanya berlaku jika plugin Anda terdaftar di `claude-plugins-official` atau marketplace lain dengan salah satu [nama marketplace resmi Anthropic](/docs/id/plugins/security#official-marketplace-names). Marketplace komunitas, `claude-community`, bukan salah satunya.

<Note>
  Untuk menerbitkan plugin, lihat [Publish and distribute a plugin](/docs/id/plugins/publish).
</Note>

<h2 id="emit-the-hint">
  Keluarkan hint
</h2>

Keluarkan tag hanya ketika `CLAUDECODE` atau `CLAUDE_CODE_CHILD_SESSION` diatur, sehingga tidak muncul ketika seseorang menjalankan CLI Anda secara langsung.

Claude Code menetapkan `CLAUDECODE=1` dalam perintah yang dijalankannya melalui alat Bash dan PowerShell dan dalam perintah hook. Pada v2.1.172 dan yang lebih baru, itu juga menetapkan `CLAUDE_CODE_CHILD_SESSION=1` di sana. Variabel berbeda dalam proses mana yang membawanya:

* **`CLAUDECODE`**: diatur oleh setiap versi Claude Code. Ekstensi IDE juga menetapkannya di terminal terintegrasi mereka, jadi gerbang pada `CLAUDECODE` saja juga mengeluarkan tag ketika seseorang menjalankan CLI Anda sendiri di salah satu terminal tersebut
* **`CLAUDE_CODE_CHILD_SESSION`**: diatur hanya dalam subproses yang dimulai Claude Code sendiri. Gunakan ketika Anda dapat memerlukan v2.1.172 atau yang lebih baru

[Referensi variabel lingkungan](/docs/id/env-vars) memiliki detailnya.

Contoh berikut gerbang pada `CLAUDECODE` untuk jangkauan terluas dan mengeluarkan hint untuk plugin bernama `example-cli` di marketplace resmi:

<CodeGroup>
  ```javascript Node.js theme={null}
  if (process.env.CLAUDECODE) {
    process.stderr.write(
      '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />\n',
    )
  }
  ```

  ```python Python theme={null}
  import os, sys

  if os.environ.get("CLAUDECODE"):
      print(
          '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />',
          file=sys.stderr,
      )
  ```

  ```go Go theme={null}
  if os.Getenv("CLAUDECODE") != "" {
      fmt.Fprintln(os.Stderr,
          `<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />`)
  }
  ```

  ```shell Shell theme={null}
  if [ -n "$CLAUDECODE" ]; then
    printf '%s\n' '<claude-code-hint v="1" type="plugin" value="example-cli@claude-plugins-official" />' >&2
  fi
  ```
</CodeGroup>

Ganti `example-cli` dengan nama plugin Anda di marketplace resmi.

Anda dapat mengeluarkan hint pada setiap invokasi, karena Claude Code meminta untuk setiap plugin sekali.

Untuk memeriksa emitter, jalankan `CLAUDECODE=1 example-cli` di terminal dan konfirmasi baris tag muncul di stderr, kemudian jalankan `example-cli` tanpa variabel dan konfirmasi tidak ada yang dicetak ekstra.

<h2 id="hint-format">
  Format hint
</h2>

Tag harus menempati barisnya sendiri; Claude Code mengabaikan tag yang tertanam di tengah baris.

Tag mengambil tiga atribut, semuanya diperlukan:

| Atribut | Deskripsi                                                    |
| :------ | :----------------------------------------------------------- |
| `v`     | Versi protokol. `1` adalah satu-satunya nilai yang didukung  |
| `type`  | Jenis hint. `plugin` adalah satu-satunya nilai yang didukung |
| `value` | Pengenal plugin dalam bentuk `name@marketplace`              |

Nilai dapat dikutip ganda atau tidak dikutip; nilai yang tidak dikutip tidak dapat berisi spasi.

Claude Code menghapus baris dari output bahkan ketika `v` atau `type` tidak dikenali.

<h2 id="check-when-the-prompt-appears">
  Periksa kapan prompt muncul
</h2>

Prompt hanya muncul dalam sesi terminal interaktif. Dalam `claude -p` berjalan, dalam berjalan subagent, dan dalam output perintah hook, tag dilepas dan tidak ada prompt yang ditampilkan. Semua pemeriksaan ini juga harus lulus:

* **Resmi dan dapat dipasang**: `value` menamai plugin yang Claude Code temukan di salinan lokal marketplace resmi, yang belum dipasang, dan yang tidak ada kebijakan yang memblokir
* **Analytics aktif**: sesi di mana analytics Claude Code dimatikan tidak pernah meminta, misalnya satu dengan `DISABLE_TELEMETRY`, `DO_NOT_TRACK`, atau `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` diatur, atau satu di penyedia pihak ketiga seperti Amazon Bedrock, di mana [automatic telemetry opt-out](/docs/id/data-usage#default-behaviors-by-api-provider) berlaku
* **Batas frekuensi**: satu prompt per sesi, satu prompt selamanya per plugin terlepas dari jawaban pengguna, dan tidak ada setelah 100 plugin telah diminta di mesin itu
* **Tidak dimatikan**: pengguna belum memilih **No, and don't show plugin installation hints again**
* **Sesi lokal, dihadiri**: workspace sesi bersifat lokal daripada di mesin cloud atau jarak jauh, dan sesi tidak berjalan tanpa pengawasan. Misalnya, sesi dimulai dengan `--cloud`, satu melayani Remote Control, atau anggota tim agent-team tidak pernah meminta

<h2 id="preview-what-the-user-sees">
  Pratinjau apa yang dilihat pengguna
</h2>

Ketika pemeriksaan dalam [Periksa kapan prompt muncul](#check-when-the-prompt-appears) lulus, Claude Code menampilkan dialog **Plugin recommendation** seperti berikut:

```text theme={null}
─────────────────────────────────────────────────────────────
  Plugin recommendation

    The example-cli command suggests installing a plugin.

    Plugin: example-cli
    Marketplace: claude-plugins-official
    Description: Official integration for example-cli deployments

    Would you like to install it?
    ❯ 1. Yes, install
      2. No
      3. No, and don't show plugin installation hints again

─────────────────────────────────────────────────────────────
```

Dialog menamai kata pertama dari perintah shell yang Claude jalankan, sehingga pengguna dapat mendeteksi ketidaksesuaian. Setiap jawaban memiliki satu efek:

* **Yes, install**: memasang plugin di [user scope](/docs/id/plugins/install)
* **No, and don't show plugin installation hints again**: mematikan prompt hint masa depan untuk pengguna itu
* **No answer for 30 seconds**: dihitung sebagai **No**

<h2 id="next-steps">
  Langkah berikutnya
</h2>

* [Publish and distribute a plugin](/docs/id/plugins/publish): rute ke setiap marketplace, termasuk marketplace resmi, yang hint memerlukan
* [Plugin commands reference](/docs/id/plugins/cli-reference#plugin-install): perintah shell yang memasang plugin yang sama di luar sesi
